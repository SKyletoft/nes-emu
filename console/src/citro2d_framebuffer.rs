use citro2d::{
	Instance,
	pixel_type::Rgba5551,
	render::Target,
	sprites::{Mirroring, Sprite},
	texture::{ColourFormat, Tex},
};
use ctru::prelude::*;
use emu_core::{frame::NesFramebuffer, mapper::Mapper, ppu::Ppu};

use crate::colour::nes_colour_to_rgba5551;

const X_OFFSET: f32 = (400. - 256.) / 2.;

pub struct Citro2DFramebuffer<'a> {
	instance: Instance,
	target: Target<'a>,

	bg1: Sprite,
	bg2: Sprite,
	sprites: [Sprite; 64],
}

impl<'a> Citro2DFramebuffer<'a> {
	pub fn new(gfx: &'a Gfx) -> Result<Self, citro2d::error::Error> {
		let top_screen = gfx.top_screen.borrow_mut();
		let instance = Instance::new()?;
		let target = Target::new(top_screen)?;

		let mut bg1 = Sprite::from_tex(Tex::new(8 * 32, 8 * 32, ColourFormat::Rgba5551));
		bg1.set_pos((X_OFFSET, 0.));

		let mut bg2 = Sprite::from_tex(Tex::new(8 * 32, 8 * 32, ColourFormat::Rgba5551));
		bg2.set_pos((X_OFFSET, 0.));

		let sprites = std::array::from_fn(|_| {
			let mut s = Sprite::from_tex(Tex::new(8, 8, ColourFormat::Rgba5551));
			s.set_size((8., 8.));
			s.set_mirroring(Mirroring::MirrorXY);
			s
		});

		Ok(Self {
			instance,
			target,
			bg1,
			bg2,
			sprites,
		})
	}
}

impl NesFramebuffer for Citro2DFramebuffer<'_> {
	fn render<M: Mapper>(&mut self, m: &M, ppu: &Ppu, lines: &[(i16, i16); 240]) {
		let mut buffer = [[Rgba5551::TRANSPARENT; 8]; 8];

		let (dirty_sprites, [dirty_tiles_bg1, dirty_tiles_bg2]) = m.dirty_tiles();

		for (x, y) in dirty_tiles_bg1.iter().enumerate().flat_map(|(x, row)| {
			row.iter()
				.enumerate()
				.filter(|(_, b)| **b)
				.map(move |(y, _)| (x, y))
		}) {
			for (col, pixel) in (0..8)
				.flat_map(move |dy| (0..8).map(move |dx| (dx, dy)))
				.map(|(dx, dy)| {
					nes_colour_to_rgba5551(m.get_bg_pixel(
						(x * 8 + dx) as i16,
						(y * 8 + dy) as i16,
						ppu,
						&ppu.palettes,
					))
				})
				.zip(buffer.iter_mut().flat_map(|l| l.iter_mut()))
			{
				*pixel = col;
			}
			self.bg1
				.texture_mut()
				.unwrap()
				.swizzle_and_update_tile(buffer, y as _, x as _);
		}

		for (x, y) in dirty_tiles_bg2.iter().enumerate().flat_map(|(x, row)| {
			row.iter()
				.enumerate()
				.filter(|(_, b)| **b)
				.map(move |(y, _)| (x, y))
		}) {
			for (col, pixel) in (0..8)
				.flat_map(move |dy| (0..8).map(move |dx| (dx, dy)))
				.map(|(dx, dy)| {
					nes_colour_to_rgba5551(m.get_bg_pixel(
						256 + (x * 8 + dx) as i16,
						(y * 8 + dy) as i16,
						ppu,
						&ppu.palettes,
					))
				})
				.zip(buffer.iter_mut().flat_map(|l| l.iter_mut()))
			{
				*pixel = col;
			}
			self.bg2
				.texture_mut()
				.unwrap()
				.swizzle_and_update_tile(buffer, y as _, x as _);
		}

		for ((idx, sprite), dirty) in self
			.sprites
			.iter_mut()
			.enumerate()
			.zip(dirty_sprites)
		{
			if dirty {
				let data = m.get_sprite_pixels(idx, ppu);
				for (pixel, col) in buffer
					.iter_mut()
					.flat_map(|xs: &mut [Rgba5551; 8]| xs.iter_mut())
					.zip(data)
				{
					*pixel = nes_colour_to_rgba5551(col);
				}
				sprite
					.texture_mut()
					.unwrap()
					.swizzle_and_update_tile(buffer, 0, 0);
			}
			sprite.set_pos((X_OFFSET + ppu.oam[idx].x as f32, ppu.oam[idx].y as f32));
		}

		self.instance.render_target(&mut self.target, |_, t| {
			let emu_core::graphics::Colour {
				blue, green, red, ..
			} = emu_core::graphics::Colour::from_const(ppu.palettes[0][0]);
			t.clear(citro2d::render::Colour::new(red, green, blue));

			let background_slices =
				lines
					.chunk_by(|l, r| l.0 == r.0 && l.1 + 1 == r.1)
					.scan(0, |acc, curr| {
						let old_acc = *acc;
						*acc += curr.len();
						Some((curr[0].0, curr[0].1, old_acc, curr.len() as i16))
					});
			for (x_offset, y_offset, y, height) in background_slices {
				self.bg1.set_size((256., height as f32));
				self.bg2.set_size((256., height as f32));

				let x1 = {
					let base = X_OFFSET - x_offset as f32;
					if base < X_OFFSET - 256. {
						base + 512.
					} else {
						base
					}
				};
				self.bg1.set_pos((x1, y as f32));
				self.bg1.set_mirroring(Mirroring::Custom {
					left: 1.,
					right: 0.,
					top: y_offset as f32 / 256.,
					bottom: (y_offset + height) as f32 / 256.,
				});
				t.render_2d_shape(&self.bg1);

				let x2 = {
					let base = X_OFFSET + 256. - x_offset as f32;
					if base < X_OFFSET - 256. {
						base + 512.
					} else {
						base
					}
				};
				self.bg2.set_pos((x2, y as f32));
				self.bg2.set_mirroring(Mirroring::Custom {
					left: 1.,
					right: 0.,
					top: y_offset as f32 / 256.,
					bottom: (y_offset + height) as f32 / 256.,
				});
				t.render_2d_shape(&self.bg2);
			}

			for sp in self.sprites.iter() {
				t.render_2d_shape(sp);
			}
		});
	}
}
