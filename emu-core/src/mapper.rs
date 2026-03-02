use bitfields::bitfield;

use crate::{
	frame::NesFramebuffer,
	ppu::{NesColour, Ppu, Sprite},
	unsafe_assert, unsafe_unreachable,
};

pub trait Mapper {
	type Framebuffer: NesFramebuffer;
	fn framebuffer(&mut self) -> &mut Self::Framebuffer;

	fn get_cpu(&self, adr: u16) -> Option<u8>;
	fn set_cpu(&mut self, adr: u16, val: u8) -> Option<()>;
	fn get_ppu(&self, adr: u16, ppu: &Ppu) -> Option<u8>;
	fn set_ppu(&mut self, adr: u16, ppu: &mut Ppu, val: u8) -> Option<()>;
	fn get_palette_index(&self, half: bool, tile: u8, y: u8, x: u8) -> u8;

	fn get_bg_visible(&self, tilemap_x: i16, tilemap_y: i16, ppu: &Ppu) -> bool
	where
		Self: Sized,
	{
		unsafe { unsafe_assert!((0..512).contains(&tilemap_x)) };
		unsafe { unsafe_assert!((0..480).contains(&tilemap_y)) };

		let Some(tile) =
			crate::interpret::calculate_tile_palette_index(tilemap_x, tilemap_y, self, ppu)
				.nth(tilemap_x as usize % 8)
		else {
			unsafe { unsafe_unreachable!() }
		};
		tile != 0
	}

	fn get_sprite_0_visible(&self, ppu: &Ppu) -> impl Iterator<Item = bool> {
		let sprite = ppu.oam[0];
		let calc =
			|y, x| 0 != self.get_palette_index(ppu.ctrl.sprite_pattern_table(), sprite.tile, y, x);

		let mut colour_data = std::array::from_fn::<_, 8, _>(move |y| {
			std::array::from_fn::<_, 8, _>(|x| calc(y as u8, x as u8))
		});

		if sprite.attr.flip_h() {
			colour_data.iter_mut().for_each(|xs| xs.reverse());
		}
		if sprite.attr.flip_v() {
			colour_data.reverse();
		}

		let colour_data = unsafe { std::mem::transmute::<[[bool; 8]; 8], [bool; 64]>(colour_data) };
		colour_data.into_iter()
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
