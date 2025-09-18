use anyhow::{Result, bail};

#[derive(Debug, Copy, Clone)]
pub enum Mapper {
	MMC3 {
		prg_banks: [u8; 2],
		chr_2k_banks: [u8; 2],
		chr_1k_banks: [u8; 4],
		prg_roms: [[u8; 8 * 1024]; 32],
		// chr_roms: [],
		prg_mode: Mmc3PrgMode,
	},
}

#[derive(Debug, Copy, Clone)]
pub enum Mmc3PrgMode {
	Mode0 = 0,
	Mode1 = 1,
}

impl TryFrom<u8> for Mapper {
	type Error = anyhow::Error;

	fn try_from(value: u8) -> Result<Self> {
		match value {
			4 | 118 | 119 => Ok(Self::MMC3 {
				prg_banks: Default::default(),
				chr_2k_banks: Default::default(),
				chr_1k_banks: Default::default(),
				prg_roms: [[0; _]; _],
				prg_mode: Mmc3PrgMode::Mode0,
			}),
			_ => bail!("{value}"),
		}
	}
}

impl TryFrom<Vec<u8>> for Mapper {
	type Error = anyhow::Error;

	fn try_from(buffer: Vec<u8>) -> Result<Self, Self::Error> {
		let [
			b'N',
			b'E',
			b'S',
			0x1A,
			prg_size,
			chr_size,
			flags_6,
			flags_7,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
		] = &buffer[0..16]
		else {
			bail!("Missing header!");
		};

		let trainer_present = flags_6 & (1 << 2) != 0;
		assert!(!trainer_present); // Not really, but please error early when I hit a game with one.
		let trainer_offset = if trainer_present { 512 } else { 0 };
		let prg_offset = 16 + trainer_offset;
		let chr_offset = prg_offset + (*prg_size as usize * 16 * 1024);
		let mapper_type = (*flags_7 & 0xF0) | *flags_6 >> 4;

		match mapper_type {
			4 | 118 | 119 => {
				if *prg_size != 16 {
					bail!("Wrong amount of prg_roms for an MMC3 mapper");
				}

				let mut prg_roms = [[0; _]; _];
				for (src, dst) in buffer[prg_offset..]
					.chunks(16 * 1024)
					.take(*prg_size as _)
					.flat_map(|slice_16| slice_16.chunks(8 * 1024))
					.zip(prg_roms.iter_mut())
				{
					dst.copy_from_slice(src);
				}

				Ok(Mapper::MMC3 {
					prg_banks: [0; _],
					chr_2k_banks: [0; _],
					chr_1k_banks: [0; _],
					prg_roms,
					prg_mode: Mmc3PrgMode::Mode0,
				})
			}
			_ => bail!("Unknown mapper type {mapper_type}"),
		}
	}
}

impl Mapper {
	pub fn get_cpu(&self, adr: u16) -> Option<u8> {
		if !(0x4020..=0xFFFF).contains(&adr) {
			return None;
		}

		match self {
			Mapper::MMC3 {
				prg_banks,
				chr_2k_banks,
				chr_1k_banks,
				prg_roms,
				prg_mode: Mmc3PrgMode::Mode0,
			} => match adr {
				0x8000..=0x9FFF => todo!("Bank 6"),
				0xA000..=0xBFFF => todo!("Bank 7"),
				0xC000..=0xDFFF => todo!("Fixed"),
				0xE000..=0xFFFF => todo!("Last bank, fixed"),
				_ => panic!(
					"Out of bounds read from mapper (should probably be 0? But compare to existing emulators when this happens)"
				),
			},
			Mapper::MMC3 {
				prg_banks,
				chr_2k_banks,
				chr_1k_banks,
				prg_roms,
				prg_mode: Mmc3PrgMode::Mode1,
			} => match adr {
				0x8000..=0x9FFF => todo!("Fixed"),
				0xA000..=0xBFFF => todo!("Bank 7"),
				0xC000..=0xDFFF => todo!("Bank 6"),
				0xE000..=0xFFFF => todo!("Last bank, fixed"),
				_ => panic!(
					"Out of bounds read from mapper (should probably be 0? But compare to existing emulators when this happens)"
				),
			},
		}
	}

	pub fn set_cpu(&mut self, adr: u16) -> Option<()> {
		todo!()
	}

	pub fn get_ppu(&self, adr: u16) -> Option<()> {
		todo!()
	}

	pub fn set_ppu(&mut self, adr: u16) -> Option<()> {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn load_smb3() {
		let buffer = std::fs::read("non-free/SMB3.nes").unwrap();
		Mapper::try_from(buffer).unwrap();
	}
}
