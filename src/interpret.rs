#![allow(unused, dead_code)]

use std::sync::{Arc, Mutex};

use crate::{
	apu::Apu,
	controller::Controller,
	cpu::{Cpu, P},
	drawing::{self, Bitmap},
	inst::Inst,
	nes_file::Mapper,
	ppu::{Ppu, Sprite},
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
	pub bus: u8,
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
		let bus = 0;
		let current_texture = drawing::empty_bitmap();
		let cycles = 8;

		Self {
			cpu,
			ppu,
			rom,
			ram,
			bus,
			output_texture,
			current_texture,
			cycles,
			apu,
			controller1,
			controller2,
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
		let res = self.read_ppu_pure(adr);
		match adr % 8 {
			2 => {
				self.ppu.status.set_vblank(false);
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

	pub fn write_apu(&mut self, addr: u16, val: u8) {
		match addr {
			0x4000 => self.apu.pulse1.sweep = val,
			0x4001 => self.apu.pulse1.timer_low = val,
			0x4002 => self.apu.pulse1.timer_high = val,
			0x4003 => self.apu.pulse1.control = val,
			0x4004 => self.apu.pulse2.sweep = val,
			0x4005 => self.apu.pulse2.timer_low = val,
			0x4006 => self.apu.pulse2.timer_high = val,
			0x4007 => self.apu.pulse2.control = val,
			0x4008 => self.apu.triangle.timer_low = val,
			0x4009 => self.apu.triangle.timer_high = val,
			0x400A => self.apu.triangle.control = val,
			0x400C => self.apu.noise.timer_low = val,
			0x400D => self.apu.noise.timer_high = val,
			0x400E => self.apu.noise.control = val,
			0x4010 => self.apu.dmc.timer_low = val,
			0x4011 => self.apu.dmc.timer_high = val,
			0x4012 => self.apu.dmc.control = val,
			0x4015 => self.apu.status = val,
			0x4017 => self.apu.frame_counter = val,
			_ => {}
		}
	}

	pub(crate) fn mem_pure(&self, adr: u16) -> u8 {
		match adr {
			0x0000..0x0800 => self.ram[adr as usize],
			0x0800..0x2000 => self.ram[(adr % 2048) as usize],
			0x2000..0x4000 => self.read_ppu_pure(adr),
			0x4000..0x4015 => self.bus,
			0x4015..0x4018 => self.apu.status,
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
			0x4000..0x4015 => self.bus,
			0x4015..0x4018 => self.apu.status,
			0x4016 => self.controller1.into_bits(),
			0x4017 => self.controller2.into_bits(),
			0x4018..0x4020 => panic!("Cpu test mode is disabled"),
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
			0x4000..0x4018 => { /* Audio stuff */ }
			0x4018..0x4020 => { /* Audio + Controller stuff */ }
			0x4020..=0xFFFF => self.rom.set_cpu(adr, val).expect("Invalid address for ROM"),
		}
		self.bus = val;
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
