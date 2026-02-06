use std::cell::RefMut;

use ctru::{
	prelude::*,
	services::gfx::{Screen, Swap, TopScreen},
};
use emu_core::{
	frame::NesFramebuffer,
	mapper::Mapper,
	ppu::{NesColour, Ppu},
	unsafe_assert,
};

use crate::ColourFormat;

pub struct ConsoleFramebuffer<'a> {
	gfx: &'a Gfx,
	screen: RefMut<'a, TopScreen>,
	/// Must be updated when screen is swapped
	unsafe_raw_frame_buf: &'a mut [[ColourFormat; 240]; 400],
}

impl<'a> ConsoleFramebuffer<'a> {
	pub fn new(gfx: &'a Gfx) -> Self {
		let mut screen = gfx.top_screen.borrow_mut();
		let frame_buf = screen.raw_framebuffer();
		let unsafe_raw_frame_buf =
			unsafe { std::mem::transmute::<_, &mut [[ColourFormat; 240]; 400]>(frame_buf.ptr) };
		ConsoleFramebuffer {
			gfx,
			screen,
			unsafe_raw_frame_buf,
		}
	}
}

impl<'a> NesFramebuffer for ConsoleFramebuffer<'a> {
	#[inline]
	fn set(&mut self, y: usize, x: usize, col: NesColour) {
		unsafe { unsafe_assert!(x < 400 && y < 240) };
		let y = 239 - y;
		let x = (400 - 256) / 2 + x;
		self.unsafe_raw_frame_buf[x][y] = col.into();
	}

	#[inline]
	fn swap(&mut self) {
		self.screen.swap_buffers();
		let frame_buf = self.screen.raw_framebuffer();
		let unsafe_raw_frame_buf =
			unsafe { std::mem::transmute::<_, &mut [[ColourFormat; 240]; 400]>(frame_buf.ptr) };
		self.unsafe_raw_frame_buf = unsafe_raw_frame_buf;
		self.gfx.wait_for_vblank();
	}

	fn render<M: Mapper>(&mut self, m: &M, ppu: &Ppu, lines: &[(i16, i16); 240]) {
		let bg = ppu.palettes[0][0];

		if ppu.mask.show_bg() {
			for (at, pos) in lines.iter().enumerate() {
				for dot in 0..256 {
					let tilemap_x = (dot + pos.0) % 512;
					let tilemap_y = pos.1; // This is broken, but I'm preserving behaviour for now
					let palettes = ppu.palettes;
					let col = m
						.get_bg_pixel(tilemap_x, tilemap_y, ppu, &palettes)
						.unwrap_or(bg);
					self.set(at, dot as usize, col);
				}
			}
		}

		if ppu.mask.show_spr() {
			for (idx, sprite) in ppu.oam.iter().enumerate().filter(|(_, s)| s.is_visible()) {
				let sprite_y = sprite.y as i16 + 1; // Hardware bug
				let sprite_x = sprite.x as i16;
				let sprite_pixels = m.get_sprite_pixels(idx, ppu);
				let y_range = sprite_y..(sprite_y + 8);
				let x_range = sprite_x..(sprite_x + 8);
				for ((line, dot), col) in y_range
					.flat_map(move |l| x_range.clone().map(move |d| (l, d)))
					.zip(sprite_pixels)
					.filter(|((l, d), _)| *l < 240 && *d < 256)
				{
					if let Some(col) = col {
						self.set(line as usize, dot as usize, col);
					}
				}
			}
		}
		self.swap();
	}
}
