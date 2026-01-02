#![allow(unused, dead_code)]

use std::{
	ffi::{CStr, c_char},
	sync::{Arc, Mutex},
	time::{Duration, Instant},
};

use bitfields::bitfield;

use crate::{
	apu::Apu,
	controller::Controller,
	cpu::{Cpu, P},
	graphics::{self, Bitmap},
	inst::Inst,
	mapper::Mapper,
	nrom256::NROM256,
	ppu::{DoubleWriter, NesColour, Ppu, Scroll, Sprite},
};

pub const PPU_STARTUP_TIME: u64 = 2500;
const PPUADDR_MASK: u16 = (1 << 14) - 1;
pub enum InterruptTiming {
	Clear,
	Waiting,
	Ready,
}

pub struct State<M: Mapper> {
	pub cpu: Cpu,
	pub rest: Box<StateTail<M>>,
}

pub struct StateTail<M: Mapper> {
	pub ppu: Ppu,
	pub apu: Apu,
	pub controller1: Controller,
	pub controller2: Controller,
	pub rom: M,
	pub ram: [u8; 2048],
	pub cpu_bus: u8,
	pub ppu_bus: u8,
	pub output_texture: Arc<Mutex<Box<Bitmap>>>,
	pub current_texture: Box<Bitmap>,
	pub cycles: u64,
	pub ppu_runahead: u64,
	pub interrupt_requested: InterruptTiming,
}

impl<M: Mapper> State<M> {
	pub fn new(rom: Box<M>, output_texture: Arc<Mutex<Box<Bitmap>>>) -> Self {
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
		let current_texture = Box::new(graphics::empty_bitmap());
		let cycles = 8;
		let ppu_runahead = 0;
		let interrupt_requested = InterruptTiming::Clear;

		Self {
			cpu,
			rest: Box::new(StateTail {
				ppu,
				rom: *rom,
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
				ppu_runahead,
			}),
		}
	}

	pub fn next_step(mut self) -> Self {
		let inst = self.next_inst();
		inst.evaluate(self)
	}

	pub fn next(&mut self) {
		let inst = self.next_inst();
		unsafe {
			(&raw mut *self).write(inst.evaluate((&raw mut *self).read()));
		}
	}

	pub fn next_inst(&mut self) -> Inst {
		let code = [
			self.mem_pure(self.cpu.pc),
			self.mem_pure(self.cpu.pc.wrapping_add(1)),
			self.mem_pure(self.cpu.pc.wrapping_add(2)),
		];
		let res: Inst = code.into();
		// To set the open bus
		let _ = self.mem(self.cpu.pc.wrapping_sub(1).wrapping_add(res.len() as u16));
		res
	}

	pub fn next_inst_pure(&self) -> Inst {
		let code = [
			self.mem_pure(self.cpu.pc),
			self.mem_pure(self.cpu.pc.wrapping_add(1)),
			self.mem_pure(self.cpu.pc.wrapping_add(2)),
		];
		code.into()
	}

	fn read_ppu_pure(&self, adr: u16) -> u8 {
		match adr % 8 {
			0 => self.rest.ppu_bus,
			1 => self.rest.ppu_bus,
			2 => {
				let status: u8 = self.rest.ppu.status.into();
				let bus = self.rest.ppu_bus;
				(status & 0b1110_0000) | (bus & 0b0001_1111)
			}
			3 => self.rest.ppu_bus,
			4 => self.rest.ppu.oam_data,
			5 => self.rest.ppu_bus,
			6 => self.rest.ppu_bus,
			7 => self.rest.ppu.data_cache,
			_ => unreachable!(),
		}
	}

	fn read_ppu(&mut self, adr: u16) -> u8 {
		let res = self.read_ppu_pure(adr);
		self.rest.ppu_bus = res;
		match adr % 8 {
			2 => {
				self.rest.ppu.status.set_vblank(false);
				self.rest.ppu.double_writer = DoubleWriter::default();
			}
			7 => {
				self.rest.ppu.data_cache = self
					.rest
					.rom
					.get_ppu(self.rest.ppu.adr, &self.rest.ppu)
					.expect("Ppu data adr should always be inbounds");
				self.rest.ppu.adr =
					(self.rest.ppu.adr + self.rest.ppu.ctrl.vram_increment_value()) & PPUADDR_MASK;
			}
			0 | 1 | 3 | 4 | 5 | 6 => {}
			_ => unreachable!(),
		}
		res
	}

	fn write_ppu(&mut self, adr: u16, val: u8) {
		self.rest.ppu_bus = val;
		match adr % 8 {
			0 => self.rest.ppu.ctrl.set_bits(val),
			1 => self.rest.ppu.mask.set_bits(val),
			2 => {}
			3 => {}
			4 => self.rest.ppu.oam_data = val,
			5 => {
				if let Some((x, y)) = self.rest.ppu.double_writer.write(val) {
					self.rest.ppu.scroll = Scroll { x, y };
				}
			}
			6 => {
				if let Some((hi, lo)) = self.rest.ppu.double_writer.write(val) {
					let adr = u16::from_be_bytes([hi, lo]) & PPUADDR_MASK;
					self.rest.ppu.adr = adr;
					let x = ((lo & 0b11111) << 3) | (self.rest.ppu.scroll.x & 0b111);
					let y = ((adr & 0b11111_00000) >> 2 | ((adr >> 12) & 0b111)) as u8;
					let nn = (adr >> 10) & 0b11;
					self.rest.ppu.ctrl.set_nametable(nn as _);
					self.rest.ppu.scroll = Scroll { x, y };
				}
			}
			7 => {
				self.rest
					.rom
					.set_ppu(self.rest.ppu.adr, &mut self.rest.ppu, val)
					.expect("All PPU writes should be inbounds");
				self.rest.ppu.adr =
					(self.rest.ppu.adr + self.rest.ppu.ctrl.vram_increment_value()) & PPUADDR_MASK;
			}
			_ => unreachable!(),
		}
	}

	pub fn write_apu(&mut self, adr: u16, val: u8) {
		match adr {
			0x4000..0x4014 => {
				let raw_bytes: &mut [u8; 0x14] = self.rest.apu.registers_as_raw_bytes_mut();
				raw_bytes[(adr & 0xFF) as usize] = val;
			}
			0x4014 => panic!("4014 is not an APU register"),
			0x4015 => self.rest.apu.write_status(val),
			0x4017 => self.rest.apu.frame_counter = val,
			_ => {}
		}
	}

	pub(crate) fn mem_pure(&self, adr: u16) -> u8 {
		match adr {
			0x0000..0x0800 => self.rest.ram[adr as usize],
			0x0800..0x2000 => self.rest.ram[(adr % 2048) as usize],
			0x2000..0x4000 => self.read_ppu_pure(adr),
			0x4000..0x4015 => self.rest.cpu_bus,
			0x4015 => {
				(self.rest.apu.status.into_bits() & 0b1101_1111) | (self.rest.cpu_bus & 0b0010_0000)
			}
			0x4016 => {
				(self.rest.controller1.read_pure() & 0b0000_0111)
					| (self.rest.cpu_bus & 0b1111_1000)
			}
			0x4017 => {
				(self.rest.controller2.read_pure() & 0b0000_0111)
					| (self.rest.cpu_bus & 0b1111_1000)
			}
			0x4018..0x4020 => panic!("Cpu test mode is disabled"),
			0x4020..=0xFFFF => self.rest.rom.get_cpu(adr).expect("Invalid address for ROM"),
		}
	}

	pub fn mem(&mut self, adr: u16) -> u8 {
		let res = match adr {
			0x0000..0x0800 => self.rest.ram[adr as usize],
			0x0800..0x2000 => self.rest.ram[(adr % 2048) as usize],
			0x2000..0x4000 => self.read_ppu(adr),
			0x4000..0x4015 => self.rest.cpu_bus,
			0x4015 => {
				(self.rest.apu.status.into_bits() & 0b1101_1111) | (self.rest.cpu_bus & 0b0010_0000)
			}
			0x4016 => {
				(self.rest.controller1.read() & 0b0000_0111) | (self.rest.cpu_bus & 0b1111_1000)
			}
			0x4017 => {
				(self.rest.controller2.read() & 0b0000_0111) | (self.rest.cpu_bus & 0b1111_1000)
			}
			0x4018..0x4020 => panic!("Cpu test mode is disabled"),
			0x4020..=0xFFFF => self.rest.rom.get_cpu(adr).expect("Invalid address for ROM"),
		};
		self.rest.cpu_bus = res;
		res
	}

	pub fn set_mem(&mut self, adr: u16, val: u8) {
		match adr {
			0x0000..0x0800 => self.rest.ram[adr as usize] = val,
			0x0800..0x2000 => self.rest.ram[(adr % 2048) as usize] = val,
			0x2000..0x4000 => self.write_ppu(adr, val),
			0x4000..0x4014 | 0x4015 | 0x4017 => self.write_apu(adr, val),
			0x4014 => self.dma_transfer(val),
			0x4016 => {
				self.rest.controller1.write(val);
				self.rest.controller2.write(val);
			}
			0x4018..0x4020 => panic!("Cpu test mode is disabled"),
			0x4020..=0xFFFF => self
				.rest
				.rom
				.set_cpu(adr, val)
				.expect("Invalid address for ROM"),
		}
		// Writing to CPU-internal registers doesn't set the bus.
		if adr != 0x4015 {
			self.rest.cpu_bus = val;
		}
	}

	fn dma_transfer(&mut self, page: u8) {
		if self.rest.cycles % 2 == 1 {
			self.rest.cycles += 2;
			self.rest.ppu_runahead += 6;
		} else {
			self.rest.cycles += 1;
			self.rest.ppu_runahead += 3;
		}
		for (from, to) in (0..256).map(|i| (((page as u16) << 8) | i, i as usize)) {
			let val = self.mem(from);
			self.rest.cycles += 1;
			self.rest.ppu_runahead += 3;

			let buf: &mut [u8] = bytemuck::cast_slice_mut(&mut self.rest.ppu.oam);
			buf[to] = val;
			self.rest.cycles += 1;
			self.rest.ppu_runahead += 3;
		}
	}

	pub fn set_vblank(&mut self) {
		if self.cpu.p.i() && self.rest.ppu.ctrl.nmi_enable() {
			let hi = self.mem(0xFFFB);
			let lo = self.mem(0xFFFA);
			self.set_mem(0x0100 + self.cpu.s as u16, (self.cpu.pc >> 8) as u8);
			self.set_mem(0x00FF + self.cpu.s as u16, self.cpu.pc as u8);
			self.set_mem(0x00FE + self.cpu.s as u16, self.cpu.p.into_bits());
			self.cpu.pc = u16::from_be_bytes([hi, lo]);
			self.cpu.s = self.cpu.s.wrapping_sub(3);
			self.rest.cycles += 7;
			self.rest.ppu_runahead += 21;
		}
		self.rest.interrupt_requested = InterruptTiming::Clear;
	}

	pub fn check_interrupt(&mut self) {
		match self.rest.interrupt_requested {
			InterruptTiming::Clear => {}
			InterruptTiming::Waiting => self.rest.interrupt_requested = InterruptTiming::Ready,
			InterruptTiming::Ready => self.set_vblank(),
		}
	}

	pub fn catch_up_ppu(&mut self) {
		let mut remaining = self.rest.ppu_runahead;

		#[cfg(test)] // During tests catchup should be run on every instruction
		assert!(remaining.is_multiple_of(3) && remaining <= 7 * 3);

		debug_assert!(
			self.rest.ppu.dot == 0 || self.rest.ppu.dot == 128 || self.rest.ppu.dot >= 256
		);
		while self.rest.ppu_runahead > 128
			|| (self.rest.ppu.dot > 255 && self.rest.ppu_runahead > 0)
		{
			let cycles = if self.rest.ppu.dot > 255 || self.rest.ppu.dot == 128 {
				(341 - self.rest.ppu.dot as u64).min(self.rest.ppu_runahead)
			} else {
				debug_assert_eq!(self.rest.ppu.dot, 0);
				128
			};
			self.step_ppu_batch(cycles);
			self.rest.ppu_runahead -= cycles;
			debug_assert!(
				self.rest.ppu.dot == 0 || self.rest.ppu.dot == 128 || self.rest.ppu.dot >= 256,
				"{}",
				self.rest.ppu.dot
			);
		}

		self.check_interrupt();
	}

	pub fn step_ppu_batch(&mut self, cycles: u64) {
		debug_assert!(
			self.rest.ppu.dot + (cycles as i16) <= 341,
			"{} + {} = {}",
			self.rest.ppu.dot,
			cycles,
			self.rest.ppu.dot + (cycles as i16)
		);
		let working_range = self.rest.ppu.dot..self.rest.ppu.dot + (cycles as i16);
		if self.rest.ppu.dot == 0 {
			self.calculate_sprite_overflow();
			self.update_sprite_cache();
		}

		if (0..240).contains(&self.rest.ppu.scanline) {
			let sprite_0_constants = self.rest.ppu.mask.show_spr() && self.rest.ppu.mask.show_bg();
			for dot in self.rest.ppu.dot..(self.rest.ppu.dot + (cycles as i16)).min(255) {
				self.rest.ppu.dot = dot;
				let sprite_0_hit = {
					let sprite_0 = &self.rest.ppu.oam[0];
					sprite_0_constants
						&& self.rest.ppu.sprite_is_visible_x(sprite_0)
						&& self.rest.ppu.sprite_is_visible_y(sprite_0)
						&& self.sprite_get_colour(sprite_0).is_some()
						&& self.background_get_colour().is_some()
				};
				self.rest
					.ppu
					.status
					.set_sprite_0_hit(self.rest.ppu.status.sprite_0_hit() | sprite_0_hit);
				self.render_pixel();
			}
		}

		// Dot crawl
		#[cfg(test)]
		if self.rest.ppu.scanline == -1
			&& working_range.contains(&339)
			&& (self.rest.ppu.mask.show_bg() || self.rest.ppu.mask.show_spr())
			&& self.rest.ppu.frame & 1 != 0
		{
			self.rest.ppu.cycles -= 1;
		}

		self.rest.ppu.cycles += cycles;
		let scanline_overflow = working_range.end >= 341;
		self.rest.ppu.scanline += scanline_overflow as i16;
		self.rest.ppu.dot = if scanline_overflow {
			working_range.end - 341
		} else {
			working_range.end
		};

		if self.rest.ppu.scanline == 261 {
			self.rest.ppu.scanline = -1;
			self.rest.ppu.status.set_sprite_0_hit(false);
		}

		if working_range.contains(&65) {
			self.rest
				.ppu
				.status
				.set_sprite_overflow(self.rest.ppu.sprite_overflow_latch);
		}

		if self.rest.ppu.scanline == 241 && working_range.contains(&6) {
			self.rest.interrupt_requested = InterruptTiming::Ready;
			self.rest.ppu.status.set_vblank(true);
		}

		if self.rest.ppu.scanline == 0
			&& working_range.contains(&0)
			&& self.rest.ppu.status.vblank()
		{
			self.rest.ppu.status.set_vblank(false);
		}

		// Why frames count from the start of vblank and not the start of frames, I don't
		// know. Again, matching Mesen's behaviour.
		if scanline_overflow && self.rest.ppu.scanline == 240 {
			self.rest.ppu.frame += 1;
			let mut texture = self.rest.output_texture.lock().unwrap();
			std::mem::swap(&mut self.rest.current_texture, &mut texture);
		}

		debug_assert!(self.rest.ppu.dot < 341, "{}", self.rest.ppu.dot);
	}

	fn render_pixel(&mut self) {
		let visible_sprites_iter = self
			.rest
			.ppu
			.sprite_cache
			.iter()
			.filter_map(|&s| s)
			.filter(|sprite| self.rest.ppu.sprite_is_visible_x(sprite));
		let colour = visible_sprites_iter
			.clone()
			.filter(|s| !s.attr.priority())
			.filter_map(|s| self.sprite_get_colour(&s))
			.chain(self.background_get_colour())
			.chain(
				visible_sprites_iter
					.filter(|s| s.attr.priority())
					.filter_map(|s| self.sprite_get_colour(&s)),
			)
			.next()
			.unwrap_or(self.rest.ppu.palettes[0][0])
			.into();
		self.rest.current_texture[self.rest.ppu.scanline as usize][self.rest.ppu.dot as usize] =
			colour;
	}

	fn update_sprite_cache(&mut self) {
		let mut sprites: [Sprite; 64] = self.rest.ppu.oam;

		// Stable sort: Primarily by x, then by prio, lastly by index.
		sprites.sort_by(|l, r| {
			l.x.cmp(&r.x)
				.then(l.attr.priority().cmp(&r.attr.priority()))
		});

		let mut sprite_cache = [None; _];
		for (cache, sprite) in sprite_cache.iter_mut().zip(
			sprites
				.iter()
				.filter(|sprite| self.rest.ppu.sprite_is_visible_y(sprite))
				.take(8),
		) {
			*cache = Some(*sprite);
		}

		self.rest.ppu.sprite_cache = sprite_cache;
	}

	fn calculate_sprite_overflow(&mut self) {
		let scanline = self.rest.ppu.scanline + 1;
		let height = if self.rest.ppu.ctrl.sprite_size() {
			16
		} else {
			8
		};

		let oam = &self.rest.ppu.oam;

		let mut n: usize = 0;
		// primary OAM byte index
		let mut m: usize = 0;
		// secondary OAM byte index
		let mut overflow = false;

		for _ in 0..64 {
			let y = if m < 32 {
				oam[n / 4].y
			} else {
				// Bug: misaligned read once secondary OAM is full
				let idx = (n + (m & 0x1F)) & 0xFF;
				let spr = &oam[idx / 4];
				bytemuck::cast_slice(oam)[idx % 4]
			};

			if scanline >= y as i16 && scanline < (y as i16 + height) {
				if m >= 32 {
					overflow = true;
					break;
				}
				m += 4;
			}

			n += 4;
		}

		self.rest.ppu.sprite_overflow_latch = overflow;
	}

	pub fn sprite_get_colour(&self, sprite: &Sprite) -> Option<NesColour> {
		if !self.rest.ppu.mask.show_spr() {
			return None;
		}

		let (x, y) = self.rest.ppu.actual_pos();

		let pixel_x = self.rest.ppu.dot - sprite.x as i16;
		let pixel_y = self.rest.ppu.scanline - sprite.y as i16 - 1;

		debug_assert!((0..8).contains(&pixel_x), "{pixel_x}");
		debug_assert!((0..8).contains(&pixel_y), "{pixel_y}");

		let pixel_x = if sprite.attr.flip_h() {
			7 - pixel_x
		} else {
			pixel_x
		};
		let pixel_y = if sprite.attr.flip_v() {
			7 - pixel_y
		} else {
			pixel_y
		};

		debug_assert!((0..8).contains(&pixel_x), "{pixel_x}");
		debug_assert!((0..8).contains(&pixel_y), "{pixel_y}");

		let palette_index = self.read_pattern_table(
			pixel_x as _,
			pixel_y as _,
			sprite.tile,
			self.rest.ppu.ctrl.sprite_pattern_table(),
		);

		if palette_index == 0 {
			return None;
		}

		debug_assert!((0..4).contains(&sprite.attr.palette()));
		debug_assert!((0..4).contains(&palette_index));
		let col_idx = sprite.attr.palette() as u16 * 4 + palette_index as u16;
		debug_assert!((0..16).contains(&col_idx));

		let raw_col = self
			.rest
			.rom
			.get_ppu(0x3F10 + col_idx, &self.rest.ppu)
			.expect("Palette RAM must be in-bounds");
		let col = NesColour::try_from(raw_col).expect("Game used invalid colour");
		Some(col)
	}

	pub fn read_pattern_table(&self, fine_x: u8, fine_y: u8, tile_id: u8, half: bool) -> u8 {
		let plane0 = self
			.rest
			.rom
			.get_ppu(
				PatternAddressBuilder::new()
					.with_fine_y(fine_y)
					.with_plane(false)
					.with_tile_idx(tile_id)
					.with_half(half)
					.build()
					.into_bits(),
				&self.rest.ppu,
			)
			.expect("Pattern table read failed");
		let plane1 = self
			.rest
			.rom
			.get_ppu(
				PatternAddressBuilder::new()
					.with_fine_y(fine_y)
					.with_plane(true)
					.with_tile_idx(tile_id)
					.with_half(half)
					.build()
					.into_bits(),
				&self.rest.ppu,
			)
			.expect("Pattern table read failed");

		let bit = 7 - fine_x;
		((plane1 >> bit) & 1) << 1 | ((plane0 >> bit) & 1)
	}

	pub fn background_get_colour(&self) -> Option<NesColour> {
		if !self.rest.ppu.mask.show_bg() {
			return None;
		}

		let (x, y) = self.rest.ppu.actual_pos();
		let nametable_adr = match (x, y) {
			(0..256, 0..240) => 0x2000,
			(256..512, 0..240) => 0x2400,
			(0..256, 240..480) => 0x2800,
			(256..512, 240..480) => 0x2C00,
			(..0, _) | (_, ..0) | (512.., _) | (_, 480..) => return None,
		};

		let tile_x = (x % 256 / 8) as u16;
		let tile_y = (y % 240 / 8) as u16;
		let pixel_x = (x % 8) as u16;
		let pixel_y = (y % 8) as u16;

		let tile_idx = (tile_y << 5) + tile_x;

		// Fetch tile index from nametable
		let tile_id = self
			.rest
			.rom
			.get_ppu(nametable_adr + tile_idx, &self.rest.ppu)
			.expect("Nametable read failed");

		let tile_palette_index = self.read_pattern_table(
			pixel_x as _,
			pixel_y as _,
			tile_id,
			self.rest.ppu.ctrl.background_pattern_table(),
		);

		if tile_palette_index == 0 {
			return None;
		}

		let attribute_table_base = nametable_adr + 0x3C0;
		let attribute_addr = attribute_table_base + (tile_y / 4) * 8 + tile_x / 4;
		let attribute_byte = self
			.rest
			.rom
			.get_ppu(attribute_addr, &self.rest.ppu)
			.expect("Attribute table read failed");
		let shift = ((tile_y % 4) / 2) * 4 + ((tile_x % 4) / 2) * 2;
		let attribute_bits = (attribute_byte >> shift) & 0b11;

		debug_assert!((0..4).contains(&attribute_bits));
		debug_assert!((0..4).contains(&tile_palette_index));
		let col_idx = attribute_bits as u16 * 4 + tile_palette_index as u16;
		debug_assert!((0..16).contains(&col_idx));

		let col = self.rest.ppu.palettes[attribute_bits as usize][tile_palette_index as usize];
		Some(col)
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
		let cbus = self.rest.cpu_bus;
		let pbus = self.rest.ppu_bus;

		let inst = self.next_inst_pure();

		let crate::ppu::Ppu {
			ctrl,
			mask,
			status,
			scanline,
			dot,
			..
		} = self.rest.ppu;
		let frame = self.rest.ppu.frame % 10000;

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

		let cycles = self.rest.cycles;

		let cache = self.rest.ppu.data_cache;
		let ppu_adr = self.rest.ppu.adr;
		let ppu_cycles = self.rest.ppu.cycles;
		let Scroll {
			x: scroll_x,
			y: scroll_y,
		} = self.rest.ppu.scroll;

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
			"│ cache:{cache:02X} adr:{ppu_adr:04X}       x:{scroll_x:03} │",
		)
		.unwrap();
		writeln!(
			&mut out,
			"│ Cycles: {ppu_cycles:<10}      y:{scroll_y:03} │"
		)
		.unwrap();
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
	#[bits(3, default = 0u8)]
	__unused: u8,
}
