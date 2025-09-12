use std::ops::{Index, IndexMut};

use crate::{
	cpu::Cpu,
	evaluate_instruction,
	nes_file::{self, NesFile},
};

use anyhow::{Result, bail};

// Actually RAM ends at 0x07FF, but it's then repeated four times for some reason.
const END_OF_RAM: u16 = 0x1FFF;

pub struct State {
	pub cpu: Cpu,
	pub game: NesFile,
	pub ram: Ram,
}

pub struct Ram {
	mem: [u8; 2048],
}

impl Ram {
	pub fn new() -> Self {
		Self { mem: [0; _] }
	}

	pub fn get(&self, idx: usize) -> Result<&u8> {
		if idx > 8192 {
			bail!("Out of bounds RAM-access");
		}
		let idx = idx % 2048;
		Ok(unsafe { self.mem.get_unchecked(idx) })
	}

	pub fn get_mut(&mut self, idx: usize) -> Result<&mut u8> {
		if idx > 8192 {
			bail!("Out of bounds RAM-access");
		}
		let idx = idx % 2048;
		Ok(unsafe { self.mem.get_unchecked_mut(idx) })
	}

	pub fn get_copied_slice(&self, idx: usize) -> Result<[u8; 4]> {
		Ok([
			*self.get(idx)?,
			*self.get(idx + 1)?,
			*self.get(idx + 2)?,
			*self.get(idx + 3)?,
		])
	}
}

impl Index<usize> for Ram {
	type Output = u8;

	fn index(&self, index: usize) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl IndexMut<usize> for Ram {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
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

		let ram = Ram::new();

		Self { cpu, game, ram }
	}

	pub fn interpret(mut self) -> Self {
		let inst = if self.cpu.pc < END_OF_RAM {
			let arr = self.ram.get_copied_slice(self.cpu.pc as _).unwrap();
			let mut slice = arr.as_slice();
			nes_file::parse_instruction(&mut slice)
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

	pub fn interpret_in_place(&mut self) {
		let ptr = self as *mut State;
		let mut state = unsafe { ptr.read() };
		state = state.interpret();
		unsafe { ptr.write(state) };
	}
}
