use crate::{cpu::Cpu, nes_file::NesFile};

// Actually RAM ends at 0x07FF, but it's then repeated four times for some reason.
const END_OF_RAM: u16 = 0x1FFF;

pub struct State {
	pub cpu: Cpu,
	pub game: NesFile,
	// ram: _
}

impl State {
	fn new(game: NesFile) -> Self {
		let cpu = Cpu {
			a: 0,
			x: 0,
			y: 0,
			s: 0,
			p: crate::cpu::P {
				_bitfield_align_1: [],
				_bitfield_1: crate::cpu::P::new_bitfield_1(0, 0, 0, 0, 0, 0, 0, 0),
			},
			pc: 0,
		};

		// TODO: Create a memory mapping of RAM that has 2k wrapped four times.

		Self { cpu, game }
	}

	fn interpret(mut self) -> Self {
		self
	}
}
