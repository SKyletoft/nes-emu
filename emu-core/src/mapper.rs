use bitfields::bitfield;

use crate::{
	ppu::{NesColour, Ppu},
	unsafe_assert, unsafe_unreachable,
};

pub trait Mapper {
	fn get_cpu(&self, adr: u16) -> Option<u8>;
	fn set_cpu(&mut self, adr: u16, val: u8) -> Option<()>;
	fn get_ppu(&self, adr: u16, ppu: &Ppu) -> Option<u8>;
	fn set_ppu(&mut self, adr: u16, ppu: &mut Ppu, val: u8) -> Option<()>;
	fn get_palette_index(&self, half: bool, tile: u8, y: u8, x: u8) -> u8;

	fn get_bg_pixel(
		&self,
		tilemap_x: i16,
		tilemap_y: i16,
		ppu: &Ppu,
		palettes: &[[NesColour; 4]; 8],
	) -> Option<NesColour>
	where
		Self: Sized,
	{
		unsafe { unsafe_assert!((0..512).contains(&tilemap_x)) };
		unsafe { unsafe_assert!((0..480).contains(&tilemap_y)) };

		let Some(attribute) =
			crate::interpret::calculate_attribute_bits(tilemap_x, tilemap_y, self, ppu)
				.nth(tilemap_x as usize % 8)
		else { unsafe { unsafe_unreachable!() } };
		let Some(tile) =
			crate::interpret::calculate_tile_palette_index(tilemap_x, tilemap_y, self, ppu)
				.nth(tilemap_x as usize % 8)
		else { unsafe { unsafe_unreachable!() } };

		crate::interpret::calculate_background_colour(tile, attribute, palettes)
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
