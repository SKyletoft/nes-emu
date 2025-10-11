#![allow(unused, dead_code)]

use std::sync::{Arc, Mutex};

use crate::{
	apu::Apu,
	controller::Controller,
	cpu::{Cpu, P},
	drawing::{self, Bitmap},
	inst::Inst,
	nes_file::Mapper,
	ppu::{DoubleWriter, Ppu, Scroll, Sprite},
};

pub const PPU_STARTUP_TIME: u64 = 2500;

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
	pub output_texture: Arc<Mutex<Bitmap>>,
	pub current_texture: Bitmap,
	pub cycles: u64,
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
	unsafe { &mut *ptr }.step_ppu();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn state_set_bus(ptr: *mut State, val: u8) {
	unsafe { &mut *ptr }.cpu_bus = val;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn state_step_ppu_many(ptr: *mut State, times: u32) {
	for _ in 0..times {
		unsafe { (&mut *ptr) }.cycles += 1;
		unsafe {
			state_step_ppu(ptr);
			state_step_ppu(ptr);
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
		let apu = Apu::default();
		let ppu = Ppu::default();
		let controller1 = Controller::new();
		let controller2 = Controller::new();
		let cpu_bus = 0;
		let ppu_bus = 0;
		let current_texture = drawing::empty_bitmap();
		let cycles = 8;

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
					self.ppu.adr = u16::from_be_bytes([hi, lo]) & 0b0011_1111;
				}
			}
			7 => {
				self.rom
					.set_ppu(adr, &mut self.ppu, val)
					.expect("All PPU writes should be inbounds");
				self.ppu.adr = (self.ppu.adr + self.ppu.ctrl.vram_increment_value()) & 0b0011_1111;
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
			0x4016 => self.controller1.into_bits(),
			0x4017 => self.controller2.into_bits(),
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
			0x4016 => self.controller1.into_bits(),
			0x4017 => self.controller2.into_bits(),
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
			0x4000..0x4018 => { /* Audio stuff */ }
			0x4018..0x4020 => { /* Audio + Controller stuff */ }
			0x4020..=0xFFFF => self.rom.set_cpu(adr, val).expect("Invalid address for ROM"),
		}
		// Writing to CPU-internal registers doesn't set the bus.
		if adr != 0x4015 {
			self.cpu_bus = val;
		}
	}

	pub fn set_vblank(&mut self) {
		if self.ppu.ctrl.nmi_enable() {}
		self.ppu.status.set_vblank(true);
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

			let colour = sprites
				.iter()
				.filter(|sprite| self.ppu.sprite_is_visible_y(sprite))
				.take(8)
				.find(|sprite| self.ppu.sprite_is_visible_x(sprite))
				.map(|s| self.ppu.sprite_get_colour(s))
				.unwrap_or_else(|| self.ppu.background_get_colour());
			self.current_texture[self.ppu.scanline as usize][self.ppu.dot as usize] = colour.into();
		}
		if self.ppu.scanline == 241 && self.ppu.dot == 0 {
			self.set_vblank();
		}
		if self.ppu.scanline == 0 && self.ppu.dot == 0 && self.ppu.status.vblank() {
			self.ppu.status.set_vblank(false);
		}

		self.ppu.dot += 1;
		self.ppu.scanline += self.ppu.dot / 341;
		self.ppu.dot %= 341;
		if self.ppu.scanline == 261 {
			self.ppu.scanline = -1;
		}

		// Why frames count from the start of vblank and not the start of frames, I don't
		// know. Again, matching Mesen's behaviour.
		if self.ppu.dot == 0 && self.ppu.scanline == 240 {
			self.ppu.frame += 1;
			let mut texture = self.output_texture.lock().unwrap();
			std::mem::swap(&mut self.current_texture, &mut texture);
		}
	}
}
