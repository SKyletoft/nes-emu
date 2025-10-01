use std::sync::{Arc, Mutex};

use crate::{
	cpu::{Cpu, P},
	drawing::{self, Bitmap, Colour},
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

		Self {
			cpu,
			ppu,
			rom,
			ram,
			bus,
			output_texture,
			current_texture,
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

	fn read_ppu(&self, adr: u16) -> u8 {
		match adr % 8 {
			0 => self.bus,
			1 => self.bus,
			2 => self.ppu.status.into(),
			3 => self.bus,
			4 => self.ppu.oam_data.into(),
			5 => self.bus,
			6 => self.bus,
			7 => self.ppu.data.into(),
			_ => unreachable!(),
		}
	}

	fn write_ppu(&self, adr: u16, val: u8) {
		todo!()
	}

	fn mem_pure(&self, adr: u16) -> u8 {
		match adr {
			0x0000..0x0800 => self.ram[adr as usize],
			0x0800..0x2000 => self.ram[(adr % 2048) as usize],
			0x2000..0x4000 => self.read_ppu(adr),
			0x4000..0x4018 => todo!(),
			0x4018..0x4020 => todo!(),
			0x4020..=0xFFFF => self.rom.get_cpu(adr).expect("Invalid address for ROM"),
		}
	}

	pub fn mem(&mut self, adr: u16) -> u8 {
		let res = self.mem_pure(adr);
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
		if self.ppu.ctrl.nmi_enable() {
			self.ppu.status.checked_set_vblank(true).unwrap();
		}
	}

	pub fn step_ppu(&mut self) {
		println!("{} {}", self.ppu.scanline, self.ppu.dot);
		match (self.ppu.scanline, self.ppu.dot) {
			(263.., _) | (_, 342..) => panic!("{} {}", self.ppu.scanline, self.ppu.dot),
			(0..240, 0..255) => {
				let mut sprites: [Sprite; 64] = self.ppu.oam.into();

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

				self.ppu.dot += 1;
			}
			(262, 255) => {
				self.ppu.scanline = 0;
				self.ppu.dot = 0;
				self.ppu.frame += 1;
			}
			(240, 0) => {
				self.set_vblank();
				self.ppu.dot += 1;
			}
			(_, 341) => {
				self.ppu.dot = 0;
				self.ppu.scanline += 1;
			}

			_ => {
				self.ppu.dot += 1;
			}
		}
	}
}
