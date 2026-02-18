use citro2d::{
	Instance, Point, Size,
	pixel_type::{Rgba8, Rgba5551},
	render::Target,
	shapes::RectangleSolid,
	sprites::{Mirroring, Sprite},
	texture::{ColourFormat, Tex},
};
use ctru::prelude::*;
use emu_core::{
	frame::NesFramebuffer,
	graphics::Colour,
	mapper::Mapper,
	ppu::{NesColour, Ppu},
	unsafe_assert,
};

const X_OFFSET: f32 = (400. - 256.) / 2.;

pub struct Citro2DFramebuffer<'a> {
	instance: Instance,
	target: Target<'a>,

	bg1: Sprite,
	bg2: Sprite,
	sprites: [Sprite; 64],

	pub hide_left: bool,
	pub hide_right: bool,
}

impl<'a> Citro2DFramebuffer<'a> {
	pub fn new(gfx: &'a Gfx) -> Result<Self, citro2d::error::Error> {
		let top_screen = gfx.top_screen.borrow_mut();
		let instance = Instance::new()?;
		let target = Target::new(top_screen)?;

		let mut bg1 = Sprite::from_tex(Tex::new(8 * 32, 8 * 32, ColourFormat::Rgba5551));
		bg1.set_pos((X_OFFSET, 0.));
		bg1.set_depth(0.5);

		let mut bg2 = Sprite::from_tex(Tex::new(8 * 32, 8 * 32, ColourFormat::Rgba5551));
		bg2.set_pos((X_OFFSET, 0.));
		bg2.set_depth(0.5);

		let sprites = std::array::from_fn(|_| {
			let mut s = Sprite::from_tex(Tex::new(8, 8, ColourFormat::Rgba5551));
			s.set_size((8., 8.));
			s.set_mirroring(Mirroring::MirrorXY);
			s
		});

		let hide_left = true;
		let hide_right = true;

		Ok(Self {
			instance,
			target,
			bg1,
			bg2,
			sprites,
			hide_left,
			hide_right,
		})
	}
}

impl NesFramebuffer for Citro2DFramebuffer<'_> {
	fn update_tile(
		&mut self,
		tile_data: impl Iterator<Item = Option<NesColour>>,
		x: usize,
		y: usize,
		x_offset: usize,
	) {
		let mut buffer = [[Rgba5551::TRANSPARENT; 8]; 8];
		let bg = if x_offset == 0 {
			&mut self.bg1
		} else {
			&mut self.bg2
		};
		for (col, pixel) in tile_data.zip(buffer.iter_mut().flat_map(|l| l.iter_mut())) {
			*pixel = nes_colour_to_rgba5551(col);
		}
		bg.texture_mut()
			.unwrap()
			.swizzle_and_update_tile(buffer, y as _, x as _);
	}

	fn update_sprite(&mut self, sprite_data: impl Iterator<Item = Option<NesColour>>, idx: usize) {
		let mut buffer = [[Rgba5551::TRANSPARENT; 8]; 8];
		for (pixel, col) in buffer
			.iter_mut()
			.flat_map(|xs: &mut [Rgba5551; 8]| xs.iter_mut())
			.zip(sprite_data)
		{
			*pixel = nes_colour_to_rgba5551(col);
		}
		self.sprites[idx]
			.texture_mut()
			.unwrap()
			.swizzle_and_update_tile(buffer, 0, 0);
	}

	fn render<M: Mapper>(&mut self, m: &M, ppu: &Ppu, lines: &[(i16, i16); 240]) {
		let (dirty_sprites, dirty_tiles) = m.dirty_tiles();

		for (dirty_tiles, x_offset) in dirty_tiles.into_iter().zip([0, 256].into_iter()) {
			for (x, y) in dirty_tiles.iter().enumerate().flat_map(|(x, row)| {
				row.iter()
					.enumerate()
					.filter(|(_, b)| **b)
					.map(move |(y, _)| (x, y))
			}) {
				let tile_data = m.get_bg_pixels((x + x_offset / 8) as i16, y as i16, ppu);
				self.update_tile(tile_data, x, y, x_offset);
			}
		}

		for (idx, dirty) in dirty_sprites.iter().copied().enumerate() {
			if dirty {
				let sprite_data = m.get_sprite_pixels(idx, ppu);
				self.update_sprite(sprite_data, idx);
			}
		}

		for (idx, sprite) in self.sprites.iter_mut().enumerate() {
			sprite.set_depth(if ppu.oam[idx].attr.priority() {
				0.1
			} else {
				0.9
			});
			sprite.set_pos((X_OFFSET + ppu.oam[idx].x as f32, ppu.oam[idx].y as f32));
		}

		let background_slices =
			lines
				.chunk_by(|l, r| l.0 == r.0 && l.1 + 1 == r.1)
				.scan(0, |acc, curr| {
					let old_acc = *acc;
					*acc += curr.len();
					Some((curr[0].0, curr[0].1, old_acc, curr.len() as i16))
				});
		self.instance.render_target(&mut self.target, |_, t| {
			let emu_core::graphics::Colour {
				blue, green, red, ..
			} = emu_core::graphics::Colour::from_const(ppu.palettes[0][0]);
			t.clear(citro2d::render::Colour::new(red, green, blue));

			if ppu.mask.show_spr() {
				for sp in self.sprites.iter().filter(|sp| sp.depth() <= 0.5) {
					t.render_2d_shape(sp);
				}
			}

			if ppu.mask.show_bg() {
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

					t.render_2d_shape(&self.bg1);
					t.render_2d_shape(&self.bg2);
				}
			}

			if ppu.mask.show_spr() {
				for sp in self.sprites.iter().filter(|sp| sp.depth() > 0.5) {
					t.render_2d_shape(sp);
				}
			}

			if self.hide_left {
				t.render_2d_shape(&RectangleSolid {
					point: Point {
						x: 0.,
						y: 0.,
						z: 1.,
					},
					size: Size {
						width: 72.,
						height: 240.,
					},
					color: Rgba8::new(0, 0, 0),
				});
			}
			if self.hide_right {
				t.render_2d_shape(&RectangleSolid {
					point: Point {
						x: 328.,
						y: 0.,
						z: 1.,
					},
					size: Size {
						width: 72.,
						height: 240.,
					},
					color: Rgba8::new(0, 0, 0),
				});
			}
		});
	}
}

fn nes_colour_to_rgba5551(value: Option<NesColour>) -> Rgba5551 {
	let Some(value) = value else {
		return Rgba5551::TRANSPARENT;
	};
	const fn convert_colour(c: NesColour) -> Rgba5551 {
		let Colour {
			blue,
			green,
			red,
			alpha,
		} = Colour::from_const(c);
		let mut ret = Rgba5551::new();
		ret.set_red(red >> 3);
		ret.set_green(green >> 3);
		ret.set_blue(blue >> 3);
		ret.set_alpha(alpha != 0);
		ret
	}
	const TRANSLATED_COLOURS: [Rgba5551; 64] = NesColour::PALETTE.map(convert_colour);
	unsafe { unsafe_assert!((0..64).contains(&(value as usize))) };
	TRANSLATED_COLOURS[value as usize]
}
