use std::sync::{Arc, Mutex};

use crate::{
	cpu::{Cpu, P},
	drawing::{self, Bitmap},
	inst::Inst,
	nes_file::Mapper,
	ppu::Ppu,
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
		todo!()
	}
}
