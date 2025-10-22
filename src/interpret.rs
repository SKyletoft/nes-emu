#![allow(unused, dead_code)]

use std::sync::{Arc, Mutex};

use crate::{
	apu::Apu,
	controller::Controller,
	cpu::{Cpu, P},
	drawing::{self, Bitmap},
	inst::Inst,
	nes_file::Mapper,
	ppu::{DoubleWriter, NesColour, Ppu, Scroll, Sprite},
};

use bitfields::bitfield;

pub const PPU_STARTUP_TIME: u64 = 2500;
const PPUADDR_MASK: u16 = (1 << 14) - 1;
pub enum InterruptTiming {
	Clear,
	Waiting,
	Ready,
}

// REMEMBER TO REFLECT ANY CHANGES IN `cpu.h`
#[repr(C)]
pub struct State {
	pub cpu: Cpu,
	pub ppu: Ppu,
	pub apu: Apu,
	pub controller1: Controller,
	pub controller2: Controller,
	pub rom: Box<Mapper>,
	pub ram: [u8; 2048],
	pub cpu_bus: u8,
	pub ppu_bus: u8,
	pub output_texture: Arc<Mutex<Box<Bitmap>>>,
	pub current_texture: Box<Bitmap>,
	pub cycles: u64,
	pub interrupt_requested: InterruptTiming,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn state_get_mem(ptr: *mut State, adr: u16) -> u8 {
	let state = unsafe { &mut *ptr };
	state.mem(adr)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn state_set_mem(ptr: *mut State, adr: u16, val: u8) {
	let state = unsafe { &mut *ptr };
	state.set_mem(adr, val);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn state_step_ppu(ptr: *mut State) {
	let state = unsafe { &mut *ptr };
	state.cycles += 1;
	state.step_ppu();
	state.step_ppu();
	state.step_ppu();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn state_set_bus(ptr: *mut State, val: u8) {
	unsafe { &mut *ptr }.cpu_bus = val;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn state_step_ppu_many(ptr: *mut State, times: u32) {
	let state = unsafe { &mut *ptr };
	for _ in 0..times {
		state.cycles += 1;
		state.step_ppu();
		state.step_ppu();
		state.step_ppu();
	}
	state.check_interrupt();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn state_check_interrupt(ptr: *mut State) {
	let state = unsafe { &mut *ptr };
	state.check_interrupt();
}

impl State {
	pub fn new(rom: Box<Mapper>, output_texture: Arc<Mutex<Box<Bitmap>>>) -> Self {
		let pc = u16::from_le_bytes([
			rom.get_cpu(0xFFFC).expect("Cannot read reset vector"),
			rom.get_cpu(0xFFFD).expect("Cannot read reset vector (2)"),
		]);

		let cpu = Cpu {
			a: 0,
			x: 0,
			y: 0,
			s: 0xFD,
			p: P::new(),
			pc,
		};

		let ram = [0; 2048];
		let apu = Apu::default();
		let ppu = Ppu::default();
		let controller1 = Controller::default();
		let controller2 = Controller::default();
		let cpu_bus = 0;
		let ppu_bus = 0;
		let current_texture = Box::new(drawing::empty_bitmap());
		let cycles = 8;
		let interrupt_requested = InterruptTiming::Clear;

		Self {
			cpu,
			ppu,
			rom,
			ram,
			cpu_bus,
			ppu_bus,
			output_texture,
			current_texture,
			cycles,
			apu,
			controller1,
			controller2,
			interrupt_requested,
		}
	}

	pub fn next_inst(&mut self) -> Inst {
		let code = [
			self.mem_pure(self.cpu.pc),
			self.mem_pure(self.cpu.pc + 1),
			self.mem_pure(self.cpu.pc + 2),
		];
		let res: Inst = code.into();
		// To set the open bus
		let _ = self.mem(self.cpu.pc - 1 + res.len() as u16);
		res
	}

	pub fn next_inst_pure(&self) -> Inst {
		let code = [
			self.mem_pure(self.cpu.pc),
			self.mem_pure(self.cpu.pc + 1),
			self.mem_pure(self.cpu.pc + 2),
		];
		code.into()
	}

	pub fn next_step(mut self) -> Self {
		let inst = self.next_inst();
		inst.evaluate(&mut self);

		self
	}

	pub fn next(&mut self) {
		let inst = self.next_inst();
		inst.evaluate(self);
	}

	fn read_ppu_pure(&self, adr: u16) -> u8 {
		match adr % 8 {
			0 => self.ppu_bus,
			1 => self.ppu_bus,
			2 => {
				let status: u8 = self.ppu.status.into();
				let bus = self.ppu_bus;
				(status & 0b1110_0000) | (bus & 0b0001_1111)
			}
			3 => self.ppu_bus,
			4 => self.ppu.oam_data,
			5 => self.ppu_bus,
			6 => self.ppu_bus,
			7 => self.ppu.data_cache,
			_ => unreachable!(),
		}
	}

	fn read_ppu(&mut self, adr: u16) -> u8 {
		let res = self.read_ppu_pure(adr);
		self.ppu_bus = res;
		match adr % 8 {
			2 => {
				self.ppu.status.set_vblank(false);
				self.ppu.double_writer = DoubleWriter::default();
			}
			7 => {
				self.ppu.data_cache = self
					.rom
					.get_ppu(self.ppu.adr, &self.ppu)
					.expect("Ppu data adr should always be inbounds");
				self.ppu.adr = (self.ppu.adr + self.ppu.ctrl.vram_increment_value()) & PPUADDR_MASK;
			}
			_ => unreachable!(),
		}
		res
	}

	fn write_ppu(&mut self, adr: u16, val: u8) {
		self.ppu_bus = val;
		match adr % 8 {
			0 => self.ppu.ctrl.set_bits(val),
			1 => self.ppu.mask.set_bits(val),
			2 => {}
			3 => self.ppu.oam_adr = val,
			4 => self.ppu.oam_data = val,
			5 => {
				if let Some((x, y)) = self.ppu.double_writer.write(val) {
					self.ppu.scroll = Scroll { x, y };
				}
			}
			6 => {
				if let Some((hi, lo)) = self.ppu.double_writer.write(val) {
					self.ppu.adr = u16::from_be_bytes([hi, lo]) & PPUADDR_MASK;
				}
			}
			7 => {
				self.rom
					.set_ppu(self.ppu.adr, &mut self.ppu, val)
					.expect("All PPU writes should be inbounds");
				self.ppu.adr = (self.ppu.adr + self.ppu.ctrl.vram_increment_value()) & PPUADDR_MASK;
			}
			_ => unreachable!(),
		}
	}

	pub fn write_apu(&mut self, adr: u16, val: u8) {
		match adr {
			0x4000..0x4014 => {
				let raw_bytes: &mut [u8; 0x14] = self.apu.registers_as_raw_bytes_mut();
				raw_bytes[(adr & 0xFF) as usize] = val;
			}
			0x4014 => panic!("4014 is not an APU register"),
			0x4015 => self.apu.write_status(val),
			0x4017 => self.apu.frame_counter = val,
			_ => {}
		}
	}

	pub(crate) fn mem_pure(&self, adr: u16) -> u8 {
		match adr {
			0x0000..0x0800 => self.ram[adr as usize],
			0x0800..0x2000 => self.ram[(adr % 2048) as usize],
			0x2000..0x4000 => self.read_ppu_pure(adr),
			0x4000..0x4015 => self.cpu_bus,
			0x4015 => (self.apu.status.into_bits() & 0b1101_1111) | (self.cpu_bus & 0b0010_0000),
			0x4016 => (self.controller1.read_pure() & 0b0000_0111) | (self.cpu_bus & 0b1111_1000),
			0x4017 => (self.controller2.read_pure() & 0b0000_0111) | (self.cpu_bus & 0b1111_1000),
			0x4018..0x4020 => panic!("Cpu test mode is disabled"),
			0x4020..=0xFFFF => self.rom.get_cpu(adr).expect("Invalid address for ROM"),
		}
	}

	pub fn mem(&mut self, adr: u16) -> u8 {
		let res = match adr {
			0x0000..0x0800 => self.ram[adr as usize],
			0x0800..0x2000 => self.ram[(adr % 2048) as usize],
			0x2000..0x4000 => self.read_ppu(adr),
			0x4000..0x4015 => self.cpu_bus,
			0x4015 => (self.apu.status.into_bits() & 0b1101_1111) | (self.cpu_bus & 0b0010_0000),
			0x4016 => (self.controller1.read() & 0b0000_0111) | (self.cpu_bus & 0b1111_1000),
			0x4017 => (self.controller2.read() & 0b0000_0111) | (self.cpu_bus & 0b1111_1000),
			0x4018..0x4020 => panic!("Cpu test mode is disabled"),
			0x4020..=0xFFFF => self.rom.get_cpu(adr).expect("Invalid address for ROM"),
		};
		self.cpu_bus = res;
		res
	}

	pub fn set_mem(&mut self, adr: u16, val: u8) {
		match adr {
			0x0000..0x0800 => self.ram[adr as usize] = val,
			0x0800..0x2000 => self.ram[(adr % 2048) as usize] = val,
			0x2000..0x4000 => self.write_ppu(adr, val),
			0x4000..0x4014 | 0x4015 | 0x4017 => self.write_apu(adr, val),
			0x4014 => self.dma_transfer(val),
			0x4016 => {
				self.controller1.write(val);
				self.controller2.write(val);
			}
			0x4018..0x4020 => panic!("Cpu test mode is disabled"),
			0x4020..=0xFFFF => self.rom.set_cpu(adr, val).expect("Invalid address for ROM"),
		}
		// Writing to CPU-internal registers doesn't set the bus.
		if adr != 0x4015 {
			self.cpu_bus = val;
		}
	}

	fn dma_transfer(&mut self, page: u8) {
		if self.cycles % 2 == 1 {
			self.cycles += 2;
			for _ in 0..6 {
				self.step_ppu();
			}
		} else {
			self.cycles += 1;
			for _ in 0..3 {
				self.step_ppu();
			}
		}
		for (from, to) in (0..256).map(|i| (((page as u16) << 8) | i, i as usize)) {
			let val = self.mem(from);
			self.cycles += 1;
			for _ in 0..3 {
				self.step_ppu();
			}

			let buf: &mut [u8] = bytemuck::cast_slice_mut(&mut self.ppu.oam);
			buf[to] = val;
			self.cycles += 1;
			for _ in 0..3 {
				self.step_ppu();
			}
		}
	}

	pub fn set_vblank(&mut self) {
		if self.ppu.ctrl.nmi_enable() {
			let hi = self.mem(0xFFFB);
			let lo = self.mem(0xFFFA);
			self.set_mem(0x0100 + self.cpu.s as u16, (self.cpu.pc >> 8) as u8);
			self.set_mem(0x00FF + self.cpu.s as u16, self.cpu.pc as u8);
			self.set_mem(0x00FE + self.cpu.s as u16, self.cpu.p.into_bits());
			self.cpu.pc = u16::from_be_bytes([hi, lo]);
			self.cpu.s = self.cpu.s.wrapping_sub(3);
			self.cycles += 7;
			for _ in 0..21 {
				self.step_ppu();
			}
		}
		self.interrupt_requested = InterruptTiming::Clear;
	}

	pub fn check_interrupt(&mut self) {
		match self.interrupt_requested {
			InterruptTiming::Clear => {}
			InterruptTiming::Waiting => self.interrupt_requested = InterruptTiming::Ready,
			InterruptTiming::Ready => self.set_vblank(),
		}
	}

	pub fn step_ppu(&mut self) {
		self.ppu.cycles += 1;

		if (0..240).contains(&self.ppu.scanline) && (0..255).contains(&self.ppu.dot) {
			let mut sprites: [Sprite; 64] = self.ppu.oam;

			// Stable sort: Primarily by x, then by prio, lastly by index.
			sprites.sort_by(|l, r| {
				l.x.cmp(&r.x)
					.then(l.attr.priority().cmp(&r.attr.priority()))
			});

			let sprite_0_hit = {
				let sprite_0 = &self.ppu.oam[0];
				self.ppu.mask.show_spr()
					&& self.ppu.sprite_is_visible_x(sprite_0)
					&& self.ppu.sprite_is_visible_y(sprite_0)
					&& self.sprite_get_colour(sprite_0).is_some()
			};
			self.ppu
				.status
				.set_sprite_0_hit(self.ppu.status.sprite_0_hit() | sprite_0_hit);

			let colour = sprites
				.iter()
				.filter(|sprite| self.ppu.sprite_is_visible_y(sprite))
				.take(8)
				.find(|sprite| self.ppu.sprite_is_visible_x(sprite))
				.and_then(|s| self.sprite_get_colour(s))
				.unwrap_or_else(|| self.background_get_colour());
			self.current_texture[self.ppu.scanline as usize][self.ppu.dot as usize] = colour.into();
		}
		self.ppu.dot += 1;
		self.ppu.scanline += self.ppu.dot / 341;
		self.ppu.dot %= 341;
		if self.ppu.scanline == 261 {
			self.ppu.scanline = -1;
			self.ppu.status.set_sprite_0_hit(false);
		}

		// Dot crawl
		if self.ppu.scanline == -1
			&& self.ppu.dot == 339
			&& (self.ppu.mask.show_bg() || self.ppu.mask.show_spr())
		{
			self.ppu.dot = 340;
		}

		if self.ppu.scanline == 241 && self.ppu.dot == 6 {
			self.interrupt_requested = InterruptTiming::Ready;
			self.ppu.status.set_vblank(true);
		}
		if self.ppu.scanline == 0 && self.ppu.dot == 0 && self.ppu.status.vblank() {
			self.ppu.status.set_vblank(false);
		}

		// Why frames count from the start of vblank and not the start of frames, I don't
		// know. Again, matching Mesen's behaviour.
		if self.ppu.dot == 0 && self.ppu.scanline == 240 {
			self.ppu.frame += 1;
			let mut texture = self.output_texture.lock().unwrap();
			std::mem::swap(&mut self.current_texture, &mut texture);
		}
	}

	pub fn sprite_get_colour(&self, sprite: &Sprite) -> Option<NesColour> {
		if !self.ppu.mask.show_spr() {
			return None;
		}

		assert!(self.ppu.scroll.x == 0 && self.ppu.scroll.y == 0);
		let x = (self.ppu.dot + self.ppu.scroll.x as i16) % 512;
		let y = (self.ppu.scanline + self.ppu.scroll.y as i16) % 480;

		let pixel_x = sprite.x as i16 - self.ppu.dot;
		let pixel_y = sprite.y as i16 - self.ppu.scanline;

		assert!((0..8).contains(&pixel_x), "{pixel_x}");
		assert!((0..8).contains(&pixel_y), "{pixel_y}");

		let pixel_x = if sprite.attr.flip_h() {
			pixel_x
		} else {
			7 - pixel_x
		};
		let pixel_y = if sprite.attr.flip_v() {
			pixel_y
		} else {
			7 - pixel_y
		};

		assert!((0..8).contains(&pixel_x), "{pixel_x}");
		assert!((0..8).contains(&pixel_y), "{pixel_y}");

		let (plane0, plane1) = self.read_pattern_table(
			pixel_y as _,
			sprite.tile,
			self.ppu.ctrl.sprite_pattern_table(),
		);

		// Combine bits to get 0-3 palette index
		let bit = pixel_x;
		let palette_index = ((plane1 >> bit) & 1) << 1 | ((plane0 >> bit) & 1);

		if palette_index == 0 {
			return None;
		}
		assert!((0..4).contains(&sprite.attr.palette()));
		assert!((0..4).contains(&palette_index));
		let col_idx = sprite.attr.palette() as u16 * 4 + palette_index as u16;
		assert!((0..16).contains(&col_idx));
		let raw_col = self
			.rom
			.get_ppu(0x3F10 + col_idx, &self.ppu)
			.expect("Palette RAM must be in-bounds");
		let col = NesColour::try_from(raw_col).expect("Game used invalid colour");
		Some(col)
	}

	pub fn read_pattern_table(&self, fine_y: u8, tile_id: u8, half: bool) -> (u8, u8) {
		let plane0 = self
			.rom
			.get_ppu(
				PatternAddressBuilder::new()
					.with_fine_y(fine_y)
					.with_plane(false)
					.with_tile_idx(tile_id)
					.with_half(half)
					.build()
					.into_bits(),
				&self.ppu,
			)
			.expect("Pattern table read failed");
		let plane1 = self
			.rom
			.get_ppu(
				PatternAddressBuilder::new()
					.with_fine_y(fine_y)
					.with_plane(true)
					.with_tile_idx(tile_id)
					.with_half(half)
					.build()
					.into_bits(),
				&self.ppu,
			)
			.expect("Pattern table read failed");

		(plane0, plane1)
	}

	pub fn background_get_colour(&self) -> NesColour {
		if !self.ppu.mask.show_bg() {
			return NesColour::Black;
		}

		let x = (self.ppu.dot + self.ppu.scroll.x as i16) % 512;
		let y = (self.ppu.scanline + self.ppu.scroll.y as i16) % 480;
		assert!(self.ppu.ctrl.nametable() == 0); // Not really, but I'm not taking this into account yet
		let nametable_adr = match (x, y) {
			(0..256, 0..240) => 0x2000,
			(256..512, 0..240) => 0x2400,
			(0..256, 240..480) => 0x2800,
			(256..512, 240..480) => 0x2C00,
			(..0, _) | (_, ..0) | (512.., _) | (_, 480..) => panic!("Out of bounds tile access!"),
		};

		let tile_x = (x % 256 / 8) as u16;
		let tile_y = (y % 240 / 8) as u16;
		let pixel_x = (x % 8) as u16;
		let pixel_y = (y % 8) as u16;

		let tile_idx = (tile_y << 5) + tile_x;

		// Fetch tile index from nametable
		let tile_id = self
			.rom
			.get_ppu(nametable_adr + tile_idx, &self.ppu)
			.expect("Nametable read failed");

		let (plane0, plane1) = self.read_pattern_table(
			pixel_y as _,
			tile_id,
			self.ppu.ctrl.background_pattern_table(),
		);

		// Combine bits to get 0-3 palette index
		let bit = 7 - pixel_x;
		let tile_palette_index = ((plane1 >> bit) & 1) << 1 | ((plane0 >> bit) & 1);

		// Fetch attribute byte
		let attribute_table_base = nametable_adr + 0x3C0;
		let attribute_addr = attribute_table_base + (tile_y / 4) * 8 + tile_x / 4;
		let attribute_byte = self
			.rom
			.get_ppu(attribute_addr, &self.ppu)
			.expect("Attribute table read failed");

		// Select correct quadrant (2 bits)
		let shift = ((tile_y % 4) / 2) * 4 + ((tile_x % 4) / 2) * 2;
		let attribute_bits = (attribute_byte >> shift) & 0b11;

		// Combine tile bits with attribute to get final 0-15 palette index
		let palette_index = (attribute_bits << 2) | tile_palette_index;

		// Temporary placeholder palette (replace with full NES palette mapping)
		let palette = [
			NesColour::RedDark,
			NesColour::BlueDark,
			NesColour::GreenDark,
			NesColour::YellowDark,
			NesColour::RedLight,
			NesColour::BlueLight,
			NesColour::GreenLight,
			NesColour::YellowLight,
			NesColour::ChartreuseDark,
			NesColour::AzureDark,
			NesColour::OrangeDark,
			NesColour::MagentaDark,
			NesColour::ChartreuseLight,
			NesColour::AzureLight,
			NesColour::OrangeLight,
			NesColour::MagentaLight,
		];
		palette[palette_index as usize]
	}

	pub fn display(&self) -> String {
		use std::fmt::Write;

		let crate::cpu::Cpu {
			a, x, y, s, p, pc, ..
		} = self.cpu;

		let n = if p.n() { 'N' } else { 'n' };
		let v = if p.v() { 'V' } else { 'v' };
		let d = if p.d() { 'D' } else { 'd' };
		let i = if p.i() { 'I' } else { 'i' };
		let z = if p.z() { 'Z' } else { 'z' };
		let c = if p.c() { 'C' } else { 'c' };
		let b = if p.b() { '+' } else { '-' };
		let u = if p.u() { '+' } else { '-' };
		let cbus = self.cpu_bus;
		let pbus = self.ppu_bus;

		let inst = self.next_inst_pure();

		let crate::ppu::Ppu {
			ctrl,
			mask,
			status,
			scanline,
			dot,
			..
		} = self.ppu;
		let frame = self.ppu.frame % 10000;

		let mut out = String::new();

		let s0 = self.mem_pure(0x01FF);
		let s1 = self.mem_pure(0x01FE);
		let s2 = self.mem_pure(0x01FD);
		let s3 = self.mem_pure(0x01FC);
		let s4 = self.mem_pure(0x01FB);
		let s5 = self.mem_pure(0x01FA);
		let s6 = self.mem_pure(0x01F9);
		let s7 = self.mem_pure(0x01F8);
		let s8 = self.mem_pure(0x01F7);
		let s9 = self.mem_pure(0x01F6);

		let cycles = self.cycles;

		let cache = self.ppu.data_cache;
		let ppu_adr = self.ppu.adr;
		let ppu_cycles = self.ppu.cycles;

		writeln!(&mut out, "┌─CPU───────────────────────────┐").unwrap();
		writeln!(
			&mut out,
			"│ A:{a:02X} X:{x:02X} Y:{y:02X} SP:{s:02X} pc:{pc:04X}  │"
		)
		.unwrap();
		writeln!(
			&mut out,
			"│ P:{n}{v}{u}{b}{d}{i}{z}{c} bus:{cbus:04X}, {pbus:04X}     │"
		)
		.unwrap();
		writeln!(&mut out, "│ Cycles: {cycles:<10}            │").unwrap();
		writeln!(&mut out, "├─Stack─────────────────────────┤").unwrap();
		writeln!(
			&mut out,
			"│ {s0:02X},{s1:02X},{s2:02X},{s3:02X},{s4:02X},{s5:02X},{s6:02X},{s7:02X},{s8:02X},{s9:02X} │"
		)
		.unwrap();
		writeln!(&mut out, "├─PPU───────────────────────────┤").unwrap();
		writeln!(
			&mut out,
			"│ line:{scanline:03} dot:{dot:03} frame: {frame:04}  │"
		)
		.unwrap();
		writeln!(
			&mut out,
			"│ ctrl:{:02X} mask:{:02X} status:{:02X}     │",
			ctrl.into_bits(),
			mask.into_bits(),
			status.into_bits()
		)
		.unwrap();
		writeln!(
			&mut out,
			"│ cache:{cache:02X} adr:{ppu_adr:04X}             │",
		)
		.unwrap();
		writeln!(&mut out, "│ Cycles: {ppu_cycles:<10}            │").unwrap();
		writeln!(&mut out, "└───────────────────────────────┘").unwrap();
		writeln!(&mut out, "Next: {inst:X?}").unwrap();
		writeln!(&mut out).unwrap();

		out
	}
}

#[bitfield(u16)]
struct PatternAddress {
	#[bits(3)]
	fine_y: u8,
	#[bits(1)]
	plane: bool,
	#[bits(8)]
	tile_idx: u8,
	#[bits(1)]
	half: bool,
	#[bits(3, default = 0)]
	__unused: u8,
}
