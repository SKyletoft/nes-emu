use anyhow::{Result, bail};

use crate::{
	frame::NesFramebuffer,
	mapper::{Mapper, PatternAddressBuilder},
	ppu::{NesColour, Ppu, Sprite, VRAM_MASK},
	unsafe_assert, unsafe_unreachable,
};

#[rustfmt::skip]
const SWIZZLE_ORDER_2D: [(usize, usize); 64] = [
	(0,0), (1,0), (0,1), (1,1),
	(2,0), (3,0), (2,1), (3,1),
	(0,2), (1,2), (0,3), (1,3),
	(2,2), (3,2), (2,3), (3,3),
	(4,0), (5,0), (4,1), (5,1),
	(6,0), (7,0), (6,1), (7,1),
	(4,2), (5,2), (4,3), (5,3),
	(6,2), (7,2), (6,3), (7,3),
	(0,4), (1,4), (0,5), (1,5),
	(2,4), (3,4), (2,5), (3,5),
	(0,6), (1,6), (0,7), (1,7),
	(2,6), (3,6), (2,7), (3,7),
	(4,4), (5,4), (4,5), (5,5),
	(6,4), (7,4), (6,5), (7,5),
	(4,6), (5,6), (4,7), (5,7),
	(6,6), (7,6), (6,7), (7,7),
];

#[derive(Debug, Clone)]
pub struct NROM256<F: NesFramebuffer = crate::frame::NoFramebuffer> {
	pub framebuffer: F,
	pub prg_ram: [u8; 8 * 1024],
	pub prg_rom: &'static [u8; 32 * 1024],
	pub chr_rom: &'static [u8; 8 * 1024],

	/// `this[pattern table][tile][y][x]`
	pub parsed_graphics: &'static [[[[u8; 8]; 8]; 256]; 2],
	pub hitbox_background: [[[bool; 240]; 256]; 2],
	pub rendered_sprites: [[Option<NesColour>; 64]; 64],
	pub hitbox_sprites: [[bool; 64]; 64],
}

impl NROM256 {
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

		match mapper_type {
			0 if *prg_size == 2 => {
				let mut prg_rom = Box::new([0; _]);
				let mut chr_rom = Box::new([0; _]);
				let mut parsed_graphics = Box::new([[[[0; _]; _]; _]; _]);
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

				Ok(NROM256 {
					framebuffer: crate::frame::NoFramebuffer,
					prg_ram: [0; _],
					prg_rom: Box::leak(prg_rom),
					chr_rom: Box::leak(chr_rom),
					parsed_graphics: Box::leak(parsed_graphics),
					hitbox_background: [[[false; _]; _]; _],
					rendered_sprites: [[None; _]; _],
					hitbox_sprites: [[false; _]; _],
				})
			}
			0 => bail!("Wrong amount of prg_roms for an NROM"),
			_ => bail!("Unknown mapper type {mapper_type}"),
		}
	}

	pub fn with_framebuffer<NewF: NesFramebuffer>(self, framebuffer: NewF) -> NROM256<NewF> {
		NROM256 {
			framebuffer,
			prg_ram: self.prg_ram,
			prg_rom: self.prg_rom,
			chr_rom: self.chr_rom,
			parsed_graphics: self.parsed_graphics,
			hitbox_background: self.hitbox_background,
			rendered_sprites: self.rendered_sprites,
			hitbox_sprites: self.hitbox_sprites,
		}
	}
}

impl<F: NesFramebuffer> Mapper for NROM256<F> {
	type Framebuffer = F;

	fn framebuffer(&mut self) -> &mut Self::Framebuffer {
		&mut self.framebuffer
	}

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
				if old != col && is_bg_pal {
					self.rerender_background_targetted(ppu, pal_idx as u8);
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
	fn get_bg_pixel(&self, _: i16, _: i16, _: &Ppu) -> Option<NesColour>
	where
		Self: Sized,
	{
		panic!("This function is to be removed!")
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
	fn get_sprite_pixels(
		&self,
		sprite_idx: usize,
		_: &Ppu,
	) -> impl Iterator<Item = Option<NesColour>> {
		unsafe { unsafe_assert!(sprite_idx < self.rendered_sprites.len()) };
		self.rendered_sprites[sprite_idx].into_iter()
	}

	#[inline]
	fn get_sprite_visible(&self, sprite_idx: usize, _: &Ppu) -> impl Iterator<Item = bool> {
		unsafe { unsafe_assert!(sprite_idx < self.rendered_sprites.len()) };
		self.hitbox_sprites[sprite_idx].into_iter()
	}

	#[inline]
	fn set_sprite(&mut self, ppu: &mut Ppu, new: Sprite, idx: usize) {
		unsafe { unsafe_assert!(idx < ppu.oam.len()) };
		let old = ppu.oam[idx];
		ppu.oam[idx] = new;
		if new.attr.palette() != old.attr.palette() || new.tile != old.tile {
			self.rerender_sprite(ppu, idx);
		}
	}
}

impl<F: NesFramebuffer> NROM256<F> {
	fn rerender_sprite(&mut self, ppu: &mut Ppu, idx: usize) {
		unsafe { unsafe_assert!(idx < ppu.oam.len()) };

		let sprite = ppu.oam[idx];
		let calc = |y, x| {
			let palette_index = {
				unsafe { unsafe_assert!(y < 8 && x < 8) };
				self.parsed_graphics[ppu.ctrl.sprite_pattern_table() as usize][sprite.tile as usize]
					[y as usize][x as usize]
			};
			if palette_index == 0 {
				return None;
			}
			unsafe { unsafe_assert!((0..4).contains(&sprite.attr.palette())) };
			unsafe { unsafe_assert!((0..4).contains(&palette_index)) };
			let col_idx = sprite.attr.palette() as u16 * 4 + palette_index as u16;
			unsafe { unsafe_assert!((0..16).contains(&col_idx)) };

			let Some(raw_col) = ({
				let adr = (0x3F10 + col_idx) & VRAM_MASK;
				let palettes_raw = ppu.raw_palettes();
				palettes_raw.get((adr & 0x1F) as usize).copied()
			}) else {
				unsafe { unsafe_unreachable!("Palette RAM must be in-bounds") }
			};
			let col = NesColour::try_from(raw_col).expect("Game used invalid colour");
			Some(col)
		};

		let colour_data: &mut [[Option<NesColour>; 8]; 8] =
			unsafe { std::mem::transmute(&mut self.rendered_sprites[idx]) };

		match (sprite.attr.flip_h(), sprite.attr.flip_v()) {
			(false, false) => {
				for (y, row) in colour_data.iter_mut().enumerate() {
					for (x, col) in row.iter_mut().enumerate() {
						*col = calc(y as u8, x as u8);
					}
				}
			}
			(false, true) => {
				for (y, row) in colour_data.iter_mut().rev().enumerate() {
					for (x, col) in row.iter_mut().enumerate() {
						*col = calc(y as u8, x as u8);
					}
				}
			}
			(true, false) => {
				for (y, row) in colour_data.iter_mut().enumerate() {
					for (x, col) in row.iter_mut().rev().enumerate() {
						*col = calc(y as u8, x as u8);
					}
				}
			}
			(true, true) => {
				for (y, row) in colour_data.iter_mut().rev().enumerate() {
					for (x, col) in row.iter_mut().rev().enumerate() {
						*col = calc(y as u8, x as u8);
					}
				}
			}
		}

		self.hitbox_sprites[idx] = self.rendered_sprites[idx].map(|c| c.is_some());

		let sprite_data = SWIZZLE_ORDER_2D.iter().copied().map(|(y, x)| {
			let offset = y * 8 + x;
			unsafe { unsafe_assert!(idx < self.rendered_sprites.len()) };
			unsafe { unsafe_assert!(offset < 64) };
			self.rendered_sprites[idx][offset]
		});
		self.framebuffer.update_sprite(sprite_data, idx);
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
