use std::ops::{Index, IndexMut};

use crate::{cpu::Cpu, inst, nes_file::NesFile};

use anyhow::{Result, bail};

// Actually RAM ends at 0x07FF, but it's then repeated four times for some reason.
const END_OF_RAM: u16 = 0x1FFF;

pub struct State {
	pub cpu: Cpu,
	pub game: NesFile,
	pub ram: [u8; 2048],
}

impl State {
	pub fn new(game: NesFile) -> Self {
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

		let ram = [0; _];

		Self { cpu, game, ram }
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
			let memory_bank = 0;
			let idx = self.game.programs[memory_bank]
				.binary_search_by_key(&self.cpu.pc, |(x, _)| *x)
				.unwrap();
			self.game.programs[memory_bank][idx].1
		};
		inst.evaluate(&mut self.cpu);

		self
	}
}
