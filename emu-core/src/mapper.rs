use bitfields::bitfield;

use crate::{
	frame::NesFramebuffer,
	ppu::{NesColour, Ppu, Sprite},
	unsafe_assert, unsafe_unreachable,
};

pub trait Mapper {
	type Framebuffer: NesFramebuffer;
	fn framebuffer(&mut self) -> &mut Self::Framebuffer;
	// fn render(&mut self, ppu: &Ppu, lines: &[(i16, i16); 240]);

	fn get_cpu(&self, adr: u16) -> Option<u8>;
	fn set_cpu(&mut self, adr: u16, val: u8) -> Option<()>;
	fn get_ppu(&self, adr: u16, ppu: &Ppu) -> Option<u8>;
	fn set_ppu(&mut self, adr: u16, ppu: &mut Ppu, val: u8) -> Option<()>;
	fn get_palette_index(&self, half: bool, tile: u8, y: u8, x: u8) -> u8;

	fn dirty_tiles(&self) -> ([bool; 64], [[[bool; 30]; 32]; 2]) {
		([true; _], [[[true; _]; _]; _])
	}
	fn reset_dirty(&mut self) {}

	fn get_bg_pixel(&self, tilemap_x: i16, tilemap_y: i16, ppu: &Ppu) -> Option<NesColour>
	where
		Self: Sized,
	{
		unsafe { unsafe_assert!((0..512).contains(&tilemap_x)) };
		unsafe { unsafe_assert!((0..480).contains(&tilemap_y)) };

		let attribute = crate::interpret::calculate_attribute_bits(tilemap_x, tilemap_y, self, ppu);
		let Some(tile) =
			crate::interpret::calculate_tile_palette_index(tilemap_x, tilemap_y, self, ppu)
				.nth(tilemap_x as usize % 8)
		else {
			unsafe { unsafe_unreachable!() }
		};

		crate::interpret::calculate_background_colour(tile, attribute, &ppu.palettes)
	}

	fn get_sprite_pixels(
		&self,
		sprite_idx: usize,
		ppu: &Ppu,
	) -> impl Iterator<Item = Option<NesColour>> {
		let sprite = ppu.oam[sprite_idx];
		let calc = |y, x| {
			let palette_index =
				self.get_palette_index(ppu.ctrl.sprite_pattern_table(), sprite.tile, y, x);
			if palette_index == 0 {
				return None;
			}
			unsafe { unsafe_assert!((0..4).contains(&sprite.attr.palette())) };
			unsafe { unsafe_assert!((0..4).contains(&palette_index)) };
			let col_idx = sprite.attr.palette() as u16 * 4 + palette_index as u16;
			unsafe { unsafe_assert!((0..16).contains(&col_idx)) };

			let Some(raw_col) = self.get_ppu(0x3F10 + col_idx, ppu) else {
				unsafe { unsafe_unreachable!("Palette RAM must be in-bounds") }
			};
			let col = NesColour::try_from(raw_col).expect("Game used invalid colour");
			Some(col)
		};

		let mut colour_data = std::array::from_fn::<_, 8, _>(move |y| {
			std::array::from_fn::<_, 8, _>(|x| calc(y as u8, x as u8))
		});

		if sprite.attr.flip_h() {
			colour_data.iter_mut().for_each(|xs| xs.reverse());
		}
		if sprite.attr.flip_v() {
			colour_data.reverse();
		}

		let colour_data = unsafe {
			std::mem::transmute::<[[Option<NesColour>; 8]; 8], [Option<NesColour>; 64]>(colour_data)
		};
		colour_data.into_iter()
	}

	fn get_bg_pixels(
		&self,
		tile_x: i16,
		tile_y: i16,
		ppu: &Ppu,
	) -> impl Iterator<Item = Option<NesColour>>
	where
		Self: Sized,
	{
		let tilemap_x = (tile_x % 32) * 8;
		let tilemap_y = tile_y * 8;
		unsafe { unsafe_assert!((0..512).contains(&tilemap_x)) };
		unsafe { unsafe_assert!((0..240).contains(&tilemap_y)) };
		(0..8)
			.flat_map(move |dy| (0..8).map(move |dx| (dx, dy)))
			.map(move |(dx, dy)| self.get_bg_pixel(tilemap_x + dx, tilemap_y + dy, ppu))
	}

	fn set_sprite(&mut self, ppu: &mut Ppu, sprite: Sprite, idx: usize) {
		unsafe { unsafe_assert!(idx < ppu.oam.len()) };
		ppu.oam[idx] = sprite;
	}
}

#[bitfield(u16)]
pub struct PatternAddress {
	#[bits(3)]
	fine_y: u8,
	#[bits(1)]
	plane: bool,
	#[bits(8)]
	tile_idx: u8,
	#[bits(1)]
	half: bool,
	#[bits(3, default = 0u8)]
	__unused: u8,
}
