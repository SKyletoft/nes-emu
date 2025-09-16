use crate::{cpu::Cpu, inst, nes_file::Mapper};

// Actually RAM ends at 0x07FF, but it's then repeated four times for some reason.
const END_OF_RAM: u16 = 0x1FFF;

#[repr(C)]
pub struct State {
	pub cpu: Cpu,
	pub rom: Box<Mapper>,
	pub ram: [u8; 2048],
}

impl State {
	pub fn new(rom: Mapper) -> Self {
		let cpu = Cpu {
			a: 0,
			x: 0,
			y: 0,
			s: 0xFD,
			p: crate::cpu::P {
				_bitfield_align_1: [],
				_bitfield_1: crate::cpu::P::new_bitfield_1(0, 0, 1, 0, 0, 0, 0, 0),
			},
			pc: 0xFFFC,
		};

		let ram = [0; 2048];
		let rom = Box::new(rom);

		Self { cpu, rom, ram }
	}

	pub fn interpret(mut self) -> Self {
		let inst = if self.cpu.pc < END_OF_RAM {
			let arr = [
				self.ram[(self.cpu.pc as usize + 0) % 2048],
				self.ram[(self.cpu.pc as usize + 1) % 2048],
				self.ram[(self.cpu.pc as usize + 2) % 2048],
				self.ram[(self.cpu.pc as usize + 3) % 2048],
			];
			let mut slice = arr.as_slice();
			inst::parse_instruction(&mut slice)
				.expect("Instruction parse can only fail if there aren't enough operands")
		} else {
			// Yes this is stupid, but it's temporary until the ROM-code is recompiled to x86
			todo!()
		};
		inst.evaluate(&mut self.cpu);

		self
	}

	pub fn mem(&self, adr: u16) -> u8 {
		match adr {
			0x0000..0x0800 => self.ram[adr as usize],
			0x0800..0x2000 => self.ram[(adr % 2048) as usize],
			0x2000..0x2008 => todo!(),
			0x2008..0x4000 => todo!(),
			0x4000..0x4018 => todo!(),
			0x4018..0x4020 => todo!(),
			0x4020..=0xFFFF => match &*self.rom {
				Mapper::MMC3 { .. } => {
					if adr < 0x8000 {
						return 0;
					}

					todo!()
				}
			},
		}
	}
}
