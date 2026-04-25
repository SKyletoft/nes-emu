use anyhow::{Result, bail};

use crate::{
	frame::{NesFramebuffer, NoFramebuffer},
	mapper::{Mapper, PatternAddressBuilder},
	ppu::{Ppu, Sprite, VRAM_MASK},
	unsafe_assert, unsafe_unreachable,
};

#[rustfmt::skip]
const SWIZZLE_ORDER_2D: [(usize, usize); 64] = [
	(0,0), (0,1), (1,0), (1,1),
	(0,2), (0,3), (1,2), (1,3),
	(2,0), (2,1), (3,0), (3,1),
	(2,2), (2,3), (3,2), (3,3),
	(0,4), (0,5), (1,4), (1,5),
	(0,6), (0,7), (1,6), (1,7),
	(2,4), (2,5), (3,4), (3,5),
	(2,6), (2,7), (3,6), (3,7),
	(4,0), (4,1), (5,0), (5,1),
	(4,2), (4,3), (5,2), (5,3),
	(6,0), (6,1), (7,0), (7,1),
	(6,2), (6,3), (7,2), (7,3),
	(4,4), (4,5), (5,4), (5,5),
	(4,6), (4,7), (5,6), (5,7),
	(6,4), (6,5), (7,4), (7,5),
	(6,6), (6,7), (7,6), (7,7),
];

#[derive(Debug, Clone)]
pub struct Nrom<const SIZE: usize, F: NesFramebuffer = NoFramebuffer> {
	pub framebuffer: F,
	pub prg_ram: [u8; 8 * 1024],
	pub prg_rom: &'static [u8; SIZE],
	pub chr_rom: &'static [u8; 8 * 1024],

	/// `this[pattern table][tile][y][x]`
	pub parsed_graphics: &'static [[[[u8; 8]; 8]; 256]; 2],
	pub hitbox_background: [[[bool; 240]; 256]; 2],
	pub hitbox_sprite_0: [bool; 64],
}

impl<const SIZE: usize> Nrom<SIZE> {
	pub fn parse_ines(buffer: &[u8]) -> Result<Self> {
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

		let expected_prg_banks = (SIZE / (16 * 1024)) as u8;

		match mapper_type {
			0 if *prg_size == expected_prg_banks => {
				let mut prg_rom = Box::new([0; SIZE]);
				let mut chr_rom = Box::new([0; 8 * 1024]);
				let mut parsed_graphics = Box::new([[[[0; 8]; 8]; 256]; 2]);

				prg_rom.copy_from_slice(&buffer[prg_offset..prg_offset + SIZE]);
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

				Ok(Nrom {
					framebuffer: NoFramebuffer,
					prg_ram: [0; 8 * 1024],
					prg_rom: Box::leak(prg_rom),
					chr_rom: Box::leak(chr_rom),
					parsed_graphics: Box::leak(parsed_graphics),
					hitbox_background: [[[false; _]; _]; _],
					hitbox_sprite_0: [false; _],
				})
			}
			0 => bail!("Wrong amount of prg_roms for an NROM"),
			_ => bail!("Unknown mapper type {mapper_type}"),
		}
	}

	pub fn with_framebuffer<NewF: NesFramebuffer>(self, framebuffer: NewF) -> Nrom<SIZE, NewF> {
		Nrom {
			framebuffer,
			prg_ram: self.prg_ram,
			prg_rom: self.prg_rom,
			chr_rom: self.chr_rom,
			parsed_graphics: self.parsed_graphics,
			hitbox_background: self.hitbox_background,
			hitbox_sprite_0: self.hitbox_sprite_0,
		}
	}
}

impl<const SIZE: usize, F: NesFramebuffer> Mapper for Nrom<SIZE, F> {
	type Framebuffer = F;

	#[inline]
	fn framebuffer(&mut self) -> &mut Self::Framebuffer {
		&mut self.framebuffer
	}

	#[inline]
	fn get_cpu(&self, adr: u16) -> Option<u8> {
		let Nrom {
			prg_ram: ram,
			prg_rom: rom,
			..
		} = self;

		if !(0x4020..=0xFFFF).contains(&adr) {
			return None;
		}

		match adr {
			0x6000..=0x7FFF => ram.get(adr as usize % ram.len()).copied(),
			0x8000..=0xFFFF => rom.get((adr - 0x8000) as usize % SIZE).copied(),
			_ => panic!("Out of bounds read from mapper, check against actual emulators"),
		}
	}

	#[inline]
	fn set_cpu(&mut self, adr: u16, val: u8) -> Option<()> {
		let Nrom {
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
		let Nrom { chr_rom, .. } = self;
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
		let adr = adr & VRAM_MASK;
		match adr {
			0x0000..=0x1FFF => Some(()),
			0x3F00..=0x3FFF if adr.is_multiple_of(16) => {
				let Ok(col) = val.try_into() else {
					unsafe { unsafe_unreachable!("Writing invalid colour to palette") }
				};
				ppu.palettes[0][0] = col;
				ppu.palettes[4][0] = col;
				Some(())
			}
			0x3F00..=0x3FFF => {
				let Ok(col) = val.try_into() else {
					unsafe { unsafe_unreachable!("Writing invalid colour to palette") }
				};
				let adr = adr as usize % 0x20;
				let pal_idx = (adr / 4) % 8;
				let col_idx = adr % 4;
				let old = ppu.palettes[pal_idx][col_idx];
				ppu.palettes[pal_idx][col_idx] = col;
				let is_bg_pal = pal_idx < 4;
				if old == col {
					return Some(());
				}

				if is_bg_pal {
					self.rerender_background_targetted(ppu, pal_idx as u8);
				} else {
					let pattern_table = ppu.ctrl.sprite_pattern_table() as usize;

					let calc = |tile_idx: usize, pixel_x: usize, pixel_y: usize| {
						unsafe { unsafe_assert!(pixel_y < 8 && pixel_x < 8) };
						unsafe { unsafe_assert!(tile_idx < 256) };

						let palette_index =
							self.parsed_graphics[pattern_table][tile_idx][pixel_x][pixel_y];
						if palette_index == 0 {
							return None;
						}

						Some(ppu.palettes[pal_idx][palette_index as usize])
					};

					const TILE_COUNT: usize = 256usize.isqrt();
					let pattern_table_data = (0..TILE_COUNT).flat_map(|tile_row| {
						(0..TILE_COUNT).flat_map(move |tile_col| {
							let tile_idx = tile_row * TILE_COUNT + tile_col;
							SWIZZLE_ORDER_2D
								.iter()
								.copied()
								.map(move |(pixel_x, pixel_y)| calc(tile_idx, pixel_x, pixel_y))
						})
					});

					self.framebuffer.update_sprite_pattern_table(
						pal_idx as u8 - 4,
						ppu.palettes[pal_idx],
						pattern_table_data,
					);
				}
				Some(())
			}
			0x2000..=0x3EFF => {
				*ppu.vram.get_mut(adr as usize % 0x800).unwrap() = val;

				let in_nametable = adr % 0x400;
				if in_nametable < 0x03C0 {
					// Update tile
					let tile_x = (in_nametable % 32) as i16;
					let tile_y = (in_nametable / 32) as i16;
					unsafe { unsafe_assert!((0..32).contains(&tile_x), "{tile_x}") };
					unsafe { unsafe_assert!((0..30).contains(&tile_y), "{tile_y}") };
					let half = (adr - in_nametable) == 0x2400 || (adr - in_nametable) == 0x2C00;
					self.rerender_tile(half as usize, tile_x, tile_y, ppu);
				} else {
					// Update 4x4 tiles
					let in_attr = in_nametable - 0x3C0;

					let attr_x = (in_attr % 8) as i16;
					let attr_y = (in_attr / 8) as i16;

					let tile_x = attr_x * 4;
					let tile_y = attr_y * 4;

					let half = (adr - in_nametable) == 0x2400 || (adr - in_nametable) == 0x2C00;

					for y in tile_y..(tile_y + 4).min(30) {
						for x in tile_x..(tile_x + 4).min(32) {
							self.rerender_tile(half as usize, x, y, ppu);
						}
					}
				}
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

	#[inline]
	fn get_bg_visible(&self, tilemap_x: i16, tilemap_y: i16, _: &Ppu) -> bool
	where
		Self: Sized,
	{
		unsafe { unsafe_assert!((0..512).contains(&tilemap_x)) };
		unsafe { unsafe_assert!((0..240).contains(&tilemap_y)) };
		let tilemap = (tilemap_x >= 256) as usize;
		let tilemap_x = tilemap_x as usize % 256;
		let tilemap_y = tilemap_y as usize;
		self.hitbox_background[tilemap][tilemap_x][tilemap_y]
	}

	#[inline]
	fn get_sprite_0_visible(&self, _: &Ppu) -> impl Iterator<Item = bool> {
		self.hitbox_sprite_0.into_iter()
	}

	#[inline]
	fn set_sprite(&mut self, ppu: &mut Ppu, new: Sprite, idx: usize) {
		unsafe { unsafe_assert!(idx < ppu.oam.len()) };
		let old = ppu.oam[idx];
		ppu.oam[idx] = new;
		if new.attr.palette() != old.attr.palette() || new.tile != old.tile {
			self.update_sprite(ppu, idx);
		}
		self.framebuffer().update_sprite(
			idx,
			new.tile,
			new.attr.flip_h(),
			new.attr.flip_v(),
			new.attr.palette(),
		);
	}
}

impl<const SIZE: usize, F: NesFramebuffer> Nrom<SIZE, F> {
	fn update_sprite(&mut self, ppu: &mut Ppu, idx: usize) {
		unsafe { unsafe_assert!(idx < ppu.oam.len()) };

		let sprite = ppu.oam[idx];

		self.framebuffer.update_sprite(
			idx,
			sprite.tile,
			sprite.attr.flip_h(),
			sprite.attr.flip_v(),
			sprite.attr.palette(),
		);

		if idx == 0 {
			let pattern_table = ppu.ctrl.sprite_pattern_table() as usize;
			let calc = |tile_idx: usize, pixel_x: usize, pixel_y: usize| {
				unsafe { unsafe_assert!(pixel_y < 8 && pixel_x < 8) };
				unsafe { unsafe_assert!(tile_idx < 256) };

				let palette_index = self.parsed_graphics[pattern_table][tile_idx][pixel_x][pixel_y];
				palette_index != 0
			};

			for (pixel_x, pixel_y) in SWIZZLE_ORDER_2D.iter().copied() {
				let ret = calc(sprite.tile as usize, pixel_x, pixel_y);
				let pixel_idx = pixel_x * 8 + pixel_y;
				unsafe { unsafe_assert!(pixel_idx < 64) };
				self.hitbox_sprite_0[pixel_idx] = ret
			}
		}
	}

	fn rerender_tile(&mut self, tilemap: usize, tile_x: i16, tile_y: i16, ppu: &Ppu) {
		let tile_x_pixels = tile_x * 8 + if tilemap == 0 { 0 } else { 256 };
		let tile_y_pixels = tile_y * 8;

		unsafe { unsafe_assert!((0..512).contains(&tile_x_pixels)) };
		unsafe { unsafe_assert!((0..480).contains(&tile_y_pixels)) };
		let nametable_adr = match (tile_x_pixels, tile_y_pixels) {
			(0..256, 0..240) => 0x2000,
			(256..512, 0..240) => 0x2400,
			(0..256, 240..480) => 0x2800,
			(256..512, 240..480) => 0x2C00,
			(..0, _) | (_, ..0) | (512.., _) | (_, 480..) => unsafe { unsafe_unreachable!() },
		};

		let chr_rom: &'static [u8; _] = self.chr_rom;
		let Some(tile_id) = ({
			let adr = (nametable_adr + ((tile_y << 5) | tile_x)) as u16 & VRAM_MASK;
			match adr {
				0x0000..=0x1FFF => chr_rom.get(adr as usize).copied(),
				0x2000..=0x3EFF => ppu.vram.get(adr as usize & 0x07FF).copied(),
				0x3F00..=0x3FFF => {
					let palettes_raw = ppu.raw_palettes();
					palettes_raw.get((adr & 0x1F) as usize).copied()
				}
				_ => None,
			}
		}) else {
			unsafe { unsafe_unreachable!() }
		};

		let attr = crate::interpret::calculate_attribute_bits(
			tile_x * 8 + if tilemap == 0 { 0 } else { 256 },
			tile_y * 8,
			self,
			ppu,
		);

		let parsed_graphics: &'static _ = self.parsed_graphics;
		let hitbox_background = &mut self.hitbox_background;
		let half = ppu.ctrl.background_pattern_table();
		let tile_data = SWIZZLE_ORDER_2D.iter().copied().map(|(pixel_y, pixel_x)| {
			let tile = parsed_graphics[half as usize][tile_id as usize][pixel_y][pixel_x];
			hitbox_background[tilemap][(tile_x * 8 + pixel_x as i16) as usize]
				[(tile_y * 8 + pixel_y as i16) as usize] = tile != 0;
			crate::interpret::calculate_background_colour(tile, attr, &ppu.palettes)
		});

		self.framebuffer.update_tile(
			tile_data,
			tile_x as usize,
			tile_y as usize,
			if tilemap != 0 { 256 } else { 0 },
		);
	}

	fn rerender_background_targetted(&mut self, ppu: &Ppu, pal: u8) {
		for tilemap in 0..2 {
			for tile_x in 0..32 {
				for tile_y in 0..30 {
					let attr = crate::interpret::calculate_attribute_bits(
						tile_x * 8 + if tilemap == 0 { 0 } else { 256 },
						tile_y * 8,
						self,
						ppu,
					);
					if pal == attr {
						self.rerender_tile(tilemap, tile_x, tile_y, ppu);
					}
				}
			}
		}
	}
}

pub type NROM128<F = NoFramebuffer> = Nrom<{ 16 * 1024 }, F>;
pub type NROM256<F = NoFramebuffer> = Nrom<{ 32 * 1024 }, F>;

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn load_nestest() {
		let buffer = std::fs::read(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/../non-free/nestest.nes"
		))
		.unwrap();
		NROM128::parse_ines(&buffer).unwrap();
	}

	#[test]
	fn load_smb1() {
		let buffer =
			std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../non-free/SMB1.nes")).unwrap();
		NROM256::parse_ines(&buffer).unwrap();
	}
}
