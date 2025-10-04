#![allow(unused, dead_code)]

use std::sync::{Arc, Mutex};

use crate::{
	cpu::{Cpu, P},
	drawing::{self, Bitmap},
	inst::Inst,
	nes_file::Mapper,
	ppu::{Ppu, Sprite},
};

// REMEMBER TO REFLECT ANY CHANGES IN `cpu.h`
#[repr(C)]
pub struct State {
	pub cpu: Cpu,
	pub ppu: Ppu,
	pub rom: Box<Mapper>,
	pub ram: [u8; 2048],
	pub bus: u8,
	pub output_texture: Arc<Mutex<Bitmap>>,
	pub current_texture: Bitmap,
	pub cycles: u64,
}

#[unsafe(no_mangle)]
pub unsafe fn state_get_mem(ptr: *mut State, adr: u16) -> u8 {
	let state = unsafe { &mut *ptr };
	state.mem(adr)
}

#[unsafe(no_mangle)]
pub unsafe fn state_set_mem(ptr: *mut State, adr: u16, val: u8) {
	let state = unsafe { &mut *ptr };
	state.set_mem(adr, val);
}

#[unsafe(no_mangle)]
pub unsafe fn state_step_ppu(ptr: *mut State) {
	unsafe { &mut *ptr }.step_ppu();
}

#[unsafe(no_mangle)]
pub unsafe fn state_step_ppu_many(ptr: *mut State, times: u32) {
	unsafe { (&mut *ptr) }.cycles += times as u64;
	for _ in 0..(times * 3) {
		unsafe {
			state_step_ppu(ptr);
		}
	}
}

impl State {
	pub fn new(rom: Box<Mapper>, output_texture: Arc<Mutex<Bitmap>>) -> Self {
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
		let ppu = Ppu::default();
		let bus = 0;
		let current_texture = drawing::empty_bitmap();
		let cycles = 0;

		Self {
			cpu,
			ppu,
			rom,
			ram,
			bus,
			output_texture,
			current_texture,
			cycles,
		}
	}

	pub fn next_inst(&self) -> Inst {
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
		println!("stealth-reading ppu at {adr:04X} ({:02X})", self.ppu.status.into_bits());
		match adr % 8 {
			0 => self.bus,
			1 => self.bus,
			2 => {
				let status: u8 = self.ppu.status.into();
				let bus = self.bus;
				(status & 0b1110_0000) | (bus & 0b0001_1111)
			}
			3 => self.bus,
			4 => self.ppu.oam_data,
			5 => self.bus,
			6 => self.bus,
			7 => self.ppu.data,
			_ => unreachable!(),
		}
	}

	fn read_ppu(&mut self, adr: u16) -> u8 {
		println!("reading ppu at {adr:04X}");
		let res = self.read_ppu_pure(adr);
		match adr % 8 {
			2 => {
				self.ppu.status.set_vblank(false);
				println!("cleared vblank by reading");
			}
			_ => unreachable!(),
		}
		res
	}

	fn write_ppu(&mut self, adr: u16, val: u8) {
		match adr % 8 {
			0 => self.ppu.ctrl.set_bits(val),
			1 => self.ppu.mask.set_bits(val),
			2 => {}
			3 => self.ppu.oam_adr = val,
			4 => self.ppu.oam_data = val,
			5 => todo!(),
			6 => todo!(),
			7 => self.ppu.data = val,
			_ => unreachable!(),
		}
	}

	pub(crate) fn mem_pure(&self, adr: u16) -> u8 {
		match adr {
			0x0000..0x0800 => self.ram[adr as usize],
			0x0800..0x2000 => self.ram[(adr % 2048) as usize],
			0x2000..0x4000 => self.read_ppu_pure(adr),
			0x4000..0x4018 => todo!(),
			0x4018..0x4020 => todo!(),
			0x4020..=0xFFFF => self.rom.get_cpu(adr).expect("Invalid address for ROM"),
		}
	}

	pub fn mem(&mut self, adr: u16) -> u8 {
		let res = match adr {
			0x0000..0x0800 => self.ram[adr as usize],
			0x0800..0x2000 => self.ram[(adr % 2048) as usize],
			0x2000..0x4000 => self.read_ppu(adr),
			0x4000..0x4018 => todo!(),
			0x4018..0x4020 => todo!(),
			0x4020..=0xFFFF => self.rom.get_cpu(adr).expect("Invalid address for ROM"),
		};
		self.bus = res;
		res
	}

	pub fn set_mem(&mut self, adr: u16, val: u8) {
		match adr {
			0x0000..0x0800 => self.ram[adr as usize] = val,
			0x0800..0x2000 => self.ram[(adr % 2048) as usize] = val,
			0x2000..0x4000 => self.write_ppu(adr, val),
			0x4000..0x4018 => todo!(),
			0x4018..0x4020 => todo!(),
			0x4020..=0xFFFF => self.rom.set_cpu(adr, val).expect("Invalid address for ROM"),
		}
		self.bus = val;
	}

	pub fn set_vblank(&mut self) {
		println!("vblank!");
		if self.ppu.ctrl.nmi_enable() {}
		if self.ppu.cycles > 27384 {
			self.ppu.status.set_vblank(true);
		}
	}

	pub fn step_ppu(&mut self) {
		println!("{} {} {}", self.ppu.scanline, self.ppu.dot, self.ppu.cycles);
		self.ppu.cycles += 1;

		if (0..240).contains(&self.ppu.scanline) && (0..255).contains(&self.ppu.dot) {
			let mut sprites: [Sprite; 64] = self.ppu.oam;

			// Stable sort: Primarily by x, then by prio, lastly by index.
			sprites.sort_by(|l, r| {
				l.x.cmp(&r.x)
					.then(l.attr.priority().cmp(&r.attr.priority()))
			});

			let colour = sprites
				.iter()
				.filter(|sprite| self.ppu.sprite_is_visible_y(sprite))
				.take(8)
				.find(|sprite| self.ppu.sprite_is_visible_x(sprite))
				.map(|s| self.ppu.sprite_get_colour(s))
				.unwrap_or_else(|| self.ppu.background_get_colour());
			self.current_texture[self.ppu.scanline as usize][self.ppu.dot as usize] = colour;
		}
		if self.ppu.scanline == 241 && self.ppu.dot == 0 {
			self.set_vblank();
		}
		if self.ppu.scanline == 0 && self.ppu.dot == 0 && self.ppu.status.vblank() {
			println!("Cleared vblank by waiting");
			self.ppu.status.set_vblank(false);
		}
		self.ppu.dot += 1;
		self.ppu.scanline += self.ppu.dot / 341;
		self.ppu.dot %= 341;
		self.ppu.frame += (self.ppu.scanline / 262) as u64;
		self.ppu.scanline %= 262;
	}
}
