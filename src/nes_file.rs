use anyhow::{Result, bail};

use crate::inst::{self, Inst};

pub struct NesFile {
	pub prg_roms: Vec<[u8; 16 * 1024]>,
	pub programs: Vec<Vec<(u16, Inst)>>,
	pub chr_roms: Vec<[u8; 8 * 1024]>,
	pub mapper: Mapper,
}

impl TryFrom<Vec<u8>> for NesFile {
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
		let mapper = Mapper::try_from((*flags_7 & 0xF0) | *flags_6 >> 4)?;

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
			.take(*prg_size as _)
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
			mapper
		})
	}
}

impl NesFile {
	fn parse_bb(
		prg_roms: &[[u8; 16 * 1024]],
		stack_ptr: u16,
		rom_bank: usize,
	) -> Result<Vec<Inst>> {
		let mut out = Vec::new();
		let mut slice = &prg_roms[rom_bank][stack_ptr as usize..];
		loop {
			let inst = inst::parse_instruction(&mut slice).unwrap();
			out.push(inst);
			if inst.ends_bb() {
				break;
			}
		}
		Ok(out)
	}
}

enum Mapper {
	MMC3,
}

impl TryFrom<u8> for Mapper {
	type Error = anyhow::Error;

	fn try_from(value: u8) -> Result<Self> {
		match value {
			4 => Ok(Self::MMC3),
			_ => bail!("{value}"),
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
