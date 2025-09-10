use std::ops::{Index, IndexMut};

use crate::{cpu::Cpu, nes_file::NesFile};

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

	fn interpret(mut self) -> Self {
		self
	}
}
