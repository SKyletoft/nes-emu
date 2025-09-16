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
		let trainer_offset = if trainer_present { 512 } else { 0 };
		let prg_offset = 16 + trainer_offset;
		let chr_offset = prg_offset + (*prg_size as usize * 16 * 1024);
		let mapper_type = (*flags_7 & 0xF0) | *flags_6 >> 4;

		match mapper_type {
			4 | 118 | 119 => {}
			_ => bail!("Unknown mapper type {mapper_type}"),
		}

	}
}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn load_smb3() {
		let buffer = std::fs::read("non-free/SMB3.nes").unwrap();
		NesFile::try_from(buffer).unwrap();
	}

	#[test]
	fn all_opcodes_parse() {
		let mut buf = [0, 0, 0, 0];
		for byte in u8::MIN..=u8::MAX {
			buf[0] = byte;
			let mut code = buf.as_slice();

			let res = inst::parse_instruction(&mut code);
			assert!(res.is_ok());
		}
	}
}
