use crate::{cpu::Cpu, nes_file::NesFile};

struct State {
	cpu: Cpu,
	game: NesFile,
}

impl State {
	fn interpret(mut self) -> Self {
		self
	}
}
