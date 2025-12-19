use anyhow::{Result, bail};

use crate::{
	mapper::Mapper,
	ppu::Ppu,
};

#[derive(Debug, Clone)]
pub struct MMC3 {
	prg_banks: [u8; 2],
	chr_2k_banks: [u8; 2],
	chr_1k_banks: [u8; 4],
	prg_roms: [[u8; 8 * 1024]; 32],
	// chr_roms: [],
	prg_mode: Mmc3PrgMode,
	chr_mode: Mmc3ChrMode,
	registers: Mmc3Registers,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Mmc3PrgMode {
	#[default]
	Mode0 = 0,
	Mode1 = 1,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Mmc3ChrMode {
	#[default]
	Mode0 = 0,
	Mode1 = 1,
}

#[derive(Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Mmc3Registers {
	// Mapping
	h8000: u8,
	h8001: u8,
	hA000: u8,
	hA001: u8,
	// Scanlines:
	hC000: u8,
	hC001: u8,
	hE000: u8,
	hE001: u8,
}

impl MMC3 {
	pub fn parse_ines(buffer: &[u8]) -> Result<Box<Self>> {
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

				let mut mapper = Box::new(MMC3 {
					prg_banks: [0; _],
					chr_2k_banks: [0; _],
					chr_1k_banks: [0; _],
					prg_roms: [[0; _]; _],
					prg_mode: Mmc3PrgMode::Mode0,
					chr_mode: Mmc3ChrMode::Mode0,
					registers: Mmc3Registers::default(),
				});

				let MMC3 { prg_roms, .. } = &mut *mapper;
				for (src, dst) in buffer[prg_offset..]
					.chunks(16 * 1024)
					.take(*prg_size as _)
					.flat_map(|slice_16| slice_16.chunks(8 * 1024))
					.zip(prg_roms.iter_mut())
				{
					dst.copy_from_slice(src);
				}

				Ok(mapper)
			}
			_ => bail!("Unknown mapper type {mapper_type}"),
		}
	}
}

impl Mapper for MMC3 {
	fn get_cpu(&self, adr: u16) -> Option<u8> {
		if !(0x4020..=0xFFFF).contains(&adr) {
			return None;
		}

		match self {
			MMC3 {
				prg_banks,
				prg_roms,
				prg_mode: Mmc3PrgMode::Mode0,
				..
			} => match adr {
				0x8000..=0x9FFF => prg_roms[prg_banks[0] as usize]
					.get((adr - 0x8000) as usize)
					.copied(),
				0xA000..=0xBFFF => prg_roms[prg_banks[1] as usize]
					.get((adr - 0xA000) as usize)
					.copied(),
				0xC000..=0xDFFF => prg_roms[30].get((adr - 0xC000) as usize).copied(),
				0xE000..=0xFFFF => prg_roms[31].get((adr - 0xE000) as usize).copied(),
				_ => panic!(
					"Out of bounds read from mapper (should probably be 0? But compare to existing emulators when this happens)"
				),
			},
			MMC3 {
				prg_mode: Mmc3PrgMode::Mode1,
				..
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

	fn set_cpu(&mut self, adr: u16, val: u8) -> Option<()> {
		let MMC3 { registers, .. } = self;
		match adr {
			0x8000..=0x9FFF if adr % 2 == 0 => registers.h8000 = val,
			0x8000..=0x9FFF if adr % 2 == 1 => {
				registers.h8001 = val;
				todo!("Update banks");
			}
			0xA000..=0xBFFF if adr % 2 == 0 => registers.hA000 = val,
			0xA000..=0xBFFF if adr % 2 == 1 => {
				registers.hA001 = val;
				todo!("Update banks");
			}
			0xC000..=0xDFFF if adr % 2 == 0 => registers.hC000 = val,
			0xC000..=0xDFFF if adr % 2 == 1 => {
				registers.hC001 = val;
				todo!("Update banks");
			}
			0xE000..=0xFFFF if adr % 2 == 0 => registers.hE000 = val,
			0xE000..=0xFFFF if adr % 2 == 1 => {
				registers.hE001 = val;
				todo!("Update banks");
			}
			_ => panic!("Out of bounds write to mapper, check against actual emulators"),
		}

		Some(())
	}

	fn get_ppu(&self, adr: u16, ppu: &Ppu) -> Option<u8> {
		// let MMC3 {
		//	chr_2k_banks,
		//	chr_1k_banks,
		//	chr_mode,
		//	// chr_rom,
		//	..
		// } = self;

		// let adr = adr & VRAM_MASK;
		// match adr {
		//	0x0000..=0x1FFF => {
		//		let bank = match chr_mode {
		//			Mmc3ChrMode::Mode0 => todo!(),
		//			Mmc3ChrMode::Mode1 => todo!(),
		//		};
		//		// let idx = bank
		//	}
		//	0x2000..=0x3EFF => ppu.vram.get(adr as usize & 0x07FF).copied(),
		//	0x3F00..=0x3FFF => ppu.palettes.get((adr & 0x1F) as usize).copied(),
		//	_ => None,
		// }
		None
	}

	fn set_ppu(&mut self, adr: u16, ppu: &mut Ppu, val: u8) -> Option<()> {
		None
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn load_smb3() {
		let buffer =
			std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../non-free/SMB3.nes")).unwrap();
		MMC3::parse_ines(&buffer).unwrap();
	}

	#[test]
	fn load_fe1() {
		let buffer = std::fs::read(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/../non-free/FE1EN.nes"
		))
		.unwrap();
		MMC3::parse_ines(&buffer).unwrap();
	}
}
