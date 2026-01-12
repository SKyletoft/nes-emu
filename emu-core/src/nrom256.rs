use anyhow::{Result, bail};

use crate::{
	mapper::{Mapper, PatternAddressBuilder},
	ppu::{NesColour, Ppu, VRAM_MASK},
	unsafe_assert, unsafe_unreachable,
};

#[derive(Debug, Clone)]
pub struct NROM256 {
	pub prg_ram: [u8; 8 * 1024],
	pub prg_rom: [u8; 32 * 1024],
	pub chr_rom: [u8; 8 * 1024],

	/// `this[pattern table][tile][y][x]`
	pub parsed_graphics: [[[[u8; 8]; 8]; 256]; 2],
	pub rendered_background: [[[Option<NesColour>; 240]; 256]; 2],
}

impl NROM256 {
	pub fn parse_ines(buffer: &[u8]) -> Result<Box<Self>> {
		let [
			b'N',
			b'E',
			b'S',
			0x1A,
			prg_size,
			_,
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
			0 if *prg_size == 2 => {
				let mut mapper = Box::new(NROM256 {
					prg_ram: [0; _],
					prg_rom: [0; _],
					chr_rom: [0; _],
					parsed_graphics: [[[[0; _]; _]; _]; _],
					rendered_background: [[[None; _]; _]; _],
				});
				let NROM256 {
					prg_rom,
					chr_rom,
					parsed_graphics,
					..
				} = &mut *mapper;
				prg_rom.copy_from_slice(&buffer[prg_offset..prg_offset + 32 * 1024]);
				chr_rom.copy_from_slice(&buffer[chr_offset..chr_offset + 8 * 1024]);

				for half in 0..2 {
					for tile in 0..=255 {
						for y in 0..8 {
							let plane0 = chr_rom[PatternAddressBuilder::new()
								.with_fine_y(y)
								.with_plane(false)
								.with_tile_idx(tile)
								.with_half(half != 0)
								.build()
								.into_bits() as usize];
							let plane1 = chr_rom[PatternAddressBuilder::new()
								.with_fine_y(y)
								.with_plane(true)
								.with_tile_idx(tile)
								.with_half(half != 0)
								.build()
								.into_bits() as usize];
							for x in 0..8 {
								let bit = 7 - x;
								let ret = ((plane1 >> bit) & 1) << 1 | ((plane0 >> bit) & 1);
								parsed_graphics[half][tile as usize][y as usize][x as usize] = ret;
							}
						}
					}
				}

				Ok(mapper)
			}
			0 => bail!("Wrong amount of prg_roms for an NROM"),
			_ => bail!("Unknown mapper type {mapper_type}"),
		}
	}
}

impl Mapper for NROM256 {
	#[inline]
	fn get_cpu(&self, adr: u16) -> Option<u8> {
		let NROM256 {
			prg_ram: ram,
			prg_rom: rom,
			..
		} = self;

		if !(0x4020..=0xFFFF).contains(&adr) {
			return None;
		}

		match adr {
			0x6000..=0x7FFF => ram.get(adr as usize % ram.len()).copied(),
			0x8000..=0xFFFF => rom.get((adr - 0x8000) as usize).copied(),
			_ => panic!("Out of bounds read from mapper, check against actual emulators"),
		}
	}

	#[inline]
	fn set_cpu(&mut self, adr: u16, val: u8) -> Option<()> {
		let NROM256 {
			prg_ram: ram,
			prg_rom: _rom,
			..
		} = self;
		match adr {
			0x6000..=0x7FFF => ram[adr as usize % ram.len()] = val,
			0x8000..=0xFFFF => {}
			_ => panic!("Out of bounds read from mapper, check against actual emulators"),
		}
		Some(())
	}

	#[inline]
	fn get_ppu(&self, adr: u16, ppu: &Ppu) -> Option<u8> {
		let NROM256 { chr_rom, .. } = self;
		let adr = adr & VRAM_MASK;
		match adr {
			0x0000..=0x1FFF => chr_rom.get(adr as usize).copied(),
			0x2000..=0x3EFF => ppu.vram.get(adr as usize & 0x07FF).copied(),
			0x3F00..=0x3FFF => {
				let palettes_raw = ppu.raw_palettes();
				palettes_raw.get((adr & 0x1F) as usize).copied()
			}
			_ => None,
		}
	}

	#[inline]
	fn set_ppu(&mut self, adr: u16, ppu: &mut Ppu, val: u8) -> Option<()> {
		let NROM256 {
			prg_ram: _,
			prg_rom: _,
			chr_rom: _,
			parsed_graphics: _,
			rendered_background: _,
		} = self;
		let adr = adr & VRAM_MASK;
		match adr {
			0x0000..=0x1FFF => Some(()),
			0x3F00..=0x3FFF if adr.is_multiple_of(16) => {
				let col: NesColour = val.try_into().expect("Writing invalid colour to palette");
				ppu.palettes[0][0] = col;
				ppu.palettes[4][0] = col;
				Some(())
			}
			0x3F00..=0x3FFF => {
				let col: NesColour = val.try_into().expect("Writing invalid colour to palette");
				let adr = adr as usize % 0x20;
				ppu.palettes[(adr / 4) % 8][adr % 4] = col;
				Some(())
			}
			0x2000..=0x3EFF => {
				*ppu.vram.get_mut(adr as usize & 0x07FF).unwrap() = val;
				Some(())
			}
			_ => None,
		}
	}

	#[inline]
	fn get_palette_index(&self, half: bool, tile: u8, y: u8, x: u8) -> u8 {
		unsafe { unsafe_assert!(y < 8 && x < 8) };
		self.parsed_graphics[half as usize][tile as usize][y as usize][x as usize]
	}

impl NROM256 {
	fn rerender_tile(&mut self, tilemap: usize, tile_x: i16, tile_y: i16, ppu: &Ppu) {
		let px = tile_x * 8 + if tilemap == 0 { 0 } else { 256 };
		let x = (tile_x * 8) as usize;
		let py = tile_y * 8;

		for line in 0..8 {
			let y = py + line;

			let attrs = crate::interpret::calculate_attribute_bits(px, y, self, ppu);
			let tiles = crate::interpret::calculate_tile_palette_index(px, y, self, ppu);

			let mut buf: [Option<NesColour>; 8] = [None; 8];

			for (i, (attr, tile)) in attrs.zip(tiles).enumerate() {
				buf[i] = crate::interpret::calculate_background_colour(tile, attr, &ppu.palettes);
			}

			for (i, c) in buf.into_iter().enumerate() {
				self.rendered_background[tilemap][x + i][y as usize] = c;
			}
		}
	}

	fn rerender_background(&mut self, ppu: &Ppu) {
		for tilemap in 0..2 {
			for tile_x in 0..32 {
				for tile_y in 0..30 {
					self.rerender_tile(tilemap, tile_x, tile_y, ppu);
				}
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn load_smb1() {
		let buffer =
			std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../non-free/SMB1.nes")).unwrap();
		NROM256::parse_ines(&buffer).unwrap();
	}
}
