use anyhow::{Result, bail};

use crate::inst::{self, Inst};

#[derive(Debug, Copy, Clone)]
pub enum Mapper {
	MMC3 {
		prg_banks: [u8; 2],
		chr_2k_banks: [u8; 2],
		chr_1k_banks: [u8; 4],
		prg_roms: [[u8; 8 * 1024]; 32],
		// chr_roms: [],
	},
}

impl TryFrom<u8> for Mapper {
	type Error = anyhow::Error;

	fn try_from(value: u8) -> Result<Self> {
		match value {
			4 | 118 | 119 => Ok(Self::MMC3 {
				prg_banks: Default::default(),
				chr_2k_banks: Default::default(),
				chr_1k_banks: Default::default(),
				prg_roms: [[0; _]; _]
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

		// Parse PRG ROM banks
		let prg_roms = buffer[prg_offset..]
			.chunks(16 * 1024)
			.take(*prg_size as _)
			.map(|slice| {
				let mut arr = [0; _];
				if slice.len() != 16 * 1024 {
					bail!("Incorrect length of memory bank {}", slice.len());
				}
				arr.copy_from_slice(slice);
				Ok(arr)
			})
			.collect::<Result<Vec<_>>>()?;

		let programs = prg_roms
			.iter()
			.map(|txt| {
				let mut txt = txt.as_slice();
				let mut out = Vec::new();
				while !txt.is_empty() {
					let idx = (16384 - txt.len()) as _;
					let Ok(inst) = inst::parse_instruction(&mut txt) else {
						break;
					};
					out.push((idx, inst));
				}
				Ok(out)
			})
			.collect::<Result<Vec<_>>>()?;

		// Parse CHR ROM banks
		let chr_roms = buffer[chr_offset..]
			.chunks(8 * 1024)
			.take(*chr_size as _)
			.map(|slice| {
				let mut arr = [0; _];
				if slice.len() != 8 * 1024 {
					bail!("Incorrect length of memory bank {}", slice.len());
				}
				arr.copy_from_slice(slice);
				Ok(arr)
			})
			.collect::<Result<Vec<_>>>()?;

		Ok(NesFile {
			prg_roms,
			programs,
			chr_roms,
			mapper,
		})
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
