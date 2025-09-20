use crate::{
	cpu::{Cpu, P},
	inst::{self, Inst},
	nes_file::Mapper,
	ppu::Ppu,
};

// Actually RAM ends at 0x07FF, but it's then repeated four times for some reason.
const END_OF_RAM: u16 = 0x1FFF;

// REMEMBER TO REFLECT ANY CHANGES IN `cpu.h`
#[repr(C)]
pub struct State {
	pub cpu: Cpu,
	pub ppu: Ppu,
	pub rom: Box<Mapper>,
	pub ram: [u8; 2048],
}

#[unsafe(no_mangle)]
pub unsafe fn state_get_mem(ptr: *const State, adr: u16) -> u8 {
	let state = unsafe { &*ptr };
	state.mem(adr)
}

#[unsafe(no_mangle)]
pub unsafe fn state_set_mem(ptr: *mut State, adr: u16, val: u8) {
	let state = unsafe { &mut *ptr };
	state.rom.set_cpu(adr, val).unwrap();
}

impl State {
	pub fn new(rom: Mapper) -> Self {
		let rom = Box::new(rom);

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

		Self { cpu, ppu, rom, ram }
	}

	pub fn next_inst(&self) -> Inst {
		let arr = [
			self.mem(self.cpu.pc),
			self.mem(self.cpu.pc + 1),
			self.mem(self.cpu.pc + 2),
			self.mem(self.cpu.pc + 3),
		];
		let mut slice = arr.as_slice();
		inst::parse_instruction(&mut slice)
			.expect("Instruction parse can only fail if there aren't enough operands")
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

	pub fn mem(&self, adr: u16) -> u8 {
		match adr {
			0x0000..0x0800 => self.ram[adr as usize],
			0x0800..0x2000 => self.ram[(adr % 2048) as usize],
			0x2000..0x2008 => todo!(),
			0x2008..0x4000 => todo!(),
			0x4000..0x4018 => todo!(),
			0x4018..0x4020 => todo!(),
			0x4020..=0xFFFF => self.rom.get_cpu(adr).expect("Invalid address for ROM"),
		}
	}
}
