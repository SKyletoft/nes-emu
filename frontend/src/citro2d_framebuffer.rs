use citro2d::{
	Instance, Point, Size,
	pixel_type::Rgba5551,
	render::Target,
	shapes::{MultiColour, Rectangle, RectangleSolid},
	sprites::{Mirroring, Sprite as Citro2dSprite},
	texture::{ColourFormat, Tex},
};
use ctru::prelude::*;
use emu_core::{
	frame::NesFramebuffer,
	perf_stats,
	ppu::{Colour, NesColour, Ppu},
	unsafe_assert,
};

use crate::debug_mode::{BackgroundView, DebugBackgroundMode, DebugMode};

const X_OFFSET: f32 = (400. - 256.) / 2.;
const TOP_SCREEN_W: f32 = 400.;
const TOP_SCREEN_H: f32 = 240.;

struct Sprite {
	sprite: Citro2dSprite,
	mirror_x: bool,
	mirror_y: bool,
	tile: u8,
}

pub struct Citro2DFramebuffer<'a> {
	instance: Instance,
	target: Target<'a>,

	bg1: Citro2dSprite,
	bg2: Citro2dSprite,
	sprites: [Sprite; 64],

	pub hide_left: bool,
	pub hide_right: bool,
	pub debug_mode_enabled: bool,
	pub debug_mode: DebugMode,
	pub debug_background_mode: DebugBackgroundMode,
}

impl<'a> Citro2DFramebuffer<'a> {
	pub fn new(gfx: &'a Gfx) -> Result<Self, citro2d::error::Error> {
		let top_screen = gfx.top_screen.borrow_mut();
		let instance = Instance::new()?;
		let target = Target::new(top_screen)?;

		let mut bg1 = Citro2dSprite::from_tex(Tex::new(8 * 32, 8 * 32, ColourFormat::Rgba5551));
		bg1.set_pos((X_OFFSET, 0.));
		bg1.set_depth(0.5);

		let mut bg2 = Citro2dSprite::from_tex(Tex::new(8 * 32, 8 * 32, ColourFormat::Rgba5551));
		bg2.set_pos((X_OFFSET, 0.));
		bg2.set_depth(0.5);

		let sprites = std::array::from_fn(|_| {
			let sprite = Citro2dSprite::from_tex(Tex::new(8 * 16, 8 * 16, ColourFormat::Rgba5551))
				.with_size((8., 8.))
				.with_mirroring(&Mirroring::Normal);
			Sprite {
				sprite,
				mirror_x: false,
				mirror_y: false,
				tile: 0,
			}
		});

		let hide_left = true;
		let hide_right = true;
		let debug_mode_enabled = false;
		let debug_mode = DebugMode::Backgrounds(BackgroundView::Both);
		let debug_background_mode = DebugBackgroundMode::Checkerboard;

		Ok(Self {
			instance,
			target,
			bg1,
			bg2,
			sprites,
			hide_left,
			hide_right,
			debug_mode_enabled,
			debug_mode,
			debug_background_mode,
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
		let bg = if x_offset == 0 {
			&mut self.bg1
		} else {
			&mut self.bg2
		};
		let tile = bg
			.texture_mut()
			.unwrap()
			.raw_flat_tile::<Rgba5551>(x as _, y as _);
		for (col, pixel) in tile_data.zip(tile.iter_mut()) {
			*pixel = nes_colour_to_rgba5551(col);
		}
	}

	fn update_sprite(
		&mut self,
		sprite_data: impl Iterator<Item = Option<NesColour>>,
		sprite_idx: usize,
		tile_idx: u8,
	) {
		unsafe { unsafe_assert!(sprite_idx < 64) };
		let tile = self.sprites[sprite_idx]
			.sprite
			.texture_mut()
			.unwrap()
			.raw_texture_mut();
		for (col, pixel) in sprite_data.zip(tile.iter_mut()) {
			*pixel = nes_colour_to_rgba5551(col);
		}
		self.sprites[sprite_idx].tile = tile_idx;
	}

	fn set_mirroring(&mut self, sprite_idx: usize, horizontal: bool, vertical: bool) {
		unsafe { unsafe_assert!(sprite_idx < 64) };
		let sprite = &mut self.sprites[sprite_idx];
		sprite.mirror_x = horizontal;
		sprite.mirror_y = vertical;
	}

	fn render(&mut self, ppu: &Ppu, lines: &[(i16, i16); 240]) {
		if !self.debug_mode_enabled {
			self.render_nes_frame(ppu, lines);
			return;
		}

		match self.debug_mode {
			DebugMode::Backgrounds(view) => {
				self.render_backgrounds_debug(view, self.debug_background_mode, ppu)
			}
			DebugMode::Sprites(idx) => {
				self.render_sprite_debug(idx, ppu, self.debug_background_mode)
			}
		}
	}
}

impl Citro2DFramebuffer<'_> {
	fn render_nes_frame(&mut self, ppu: &Ppu, lines: &[(i16, i16); 240]) {
		for (idx, sprite) in self.sprites.iter_mut().enumerate() {
			sprite.sprite.set_size((8., 8.));
			sprite.sprite.set_depth(if ppu.oam[idx].attr.priority() {
				0.1
			} else {
				0.9
			});
			sprite.sprite.set_pos((
				X_OFFSET + ppu.oam[idx].x as f32 + 4.,
				1. + ppu.oam[idx].y as f32 + 4.,
			));
			sprite.sprite.set_centre((4., 4.));

			let tile_x = sprite.tile % 16;
			let tile_y = sprite.tile / 16;

			match (sprite.mirror_x, sprite.mirror_y) {
				(false, false) => {
					sprite.sprite.set_mirroring(&Mirroring::Custom {
						top: (16 - tile_y) as f32 / 16.,
						bottom: (15 - tile_y) as f32 / 16.,
						left: tile_x as f32 / 16.,
						right: (tile_x + 1) as f32 / 16.,
					});
					sprite.sprite.set_angle(0.);
				}
				(false, true) => {
					sprite.sprite.set_mirroring(&Mirroring::Custom {
						top: (16 - tile_y) as f32 / 16.,
						bottom: (15 - tile_y) as f32 / 16.,
						right: tile_x as f32 / 16.,
						left: (tile_x + 1) as f32 / 16.,
					});
					sprite.sprite.set_angle(180_f32.to_radians());
				}
				(true, false) => {
					sprite.sprite.set_mirroring(&Mirroring::Custom {
						top: (16 - tile_y) as f32 / 16.,
						bottom: (15 - tile_y) as f32 / 16.,
						left: (tile_x + 1) as f32 / 16.,
						right: tile_x as f32 / 16.,
					});
					sprite.sprite.set_angle(0.);
				}
				(true, true) => {
					sprite.sprite.set_mirroring(&Mirroring::Custom {
						top: (16 - tile_y) as f32 / 16.,
						bottom: (15 - tile_y) as f32 / 16.,
						left: tile_x as f32 / 16.,
						right: (tile_x + 1) as f32 / 16.,
					});
					sprite.sprite.set_angle(180_f32.to_radians());
				}
			};
		}

		let background_slices =
			lines
				.chunk_by(|l, r| l.0 == r.0 && l.1 + 1 == r.1)
				.scan(0, |acc, curr| {
					*acc += curr.len() as i16;
					Some((curr[0].0, curr[0].1, *acc))
				});

		self.instance.render_target(&mut self.target, |_, t| {
			perf_stats::start_gpu();

			let Colour {
				blue, green, red, ..
			} = Colour::from_const(ppu.palettes[0][0]);
			t.clear(citro2d::render::Colour::new(red, green, blue));

			if ppu.mask.show_spr() {
				for sp in self.sprites.iter().filter(|sp| sp.sprite.depth() <= 0.5) {
					t.render_2d_shape(&sp.sprite);
				}
			}

			if ppu.mask.show_bg() {
				for (x_offset, y_start, y_end) in background_slices {
					let height = (y_end - y_start) as f32;
					self.bg1.set_size((256., height));
					self.bg2.set_size((256., height));

					let mirroring = Mirroring::Custom {
						left: 0.,
						right: 1.,
						bottom: (256 - y_end) as f32 / 256.,
						top: (256 - y_start) as f32 / 256.,
					};

					let x1 = {
						let base = X_OFFSET - x_offset as f32;
						if base < X_OFFSET - 256. {
							base + 512.
						} else {
							base
						}
					};
					let x2 = {
						let base = X_OFFSET + 256. - x_offset as f32;
						if base < X_OFFSET - 256. {
							base + 512.
						} else {
							base
						}
					};

					for (bg, x) in [(&mut self.bg1, x1), (&mut self.bg2, x2)].into_iter() {
						for offset in [-512., 0., 512.].into_iter() {
							bg.set_pos((x + offset, y_start as f32));
							bg.set_mirroring(&mirroring.clone());
							t.render_2d_shape(bg);
						}
					}
				}
			}

			if ppu.mask.show_spr() {
				for sp in self.sprites.iter().filter(|sp| sp.sprite.depth() > 0.5) {
					t.render_2d_shape(&sp.sprite);
				}
			}

			if self.hide_left {
				t.render_2d_shape(&Rectangle {
					point: Point {
						x: 0.,
						y: 0.,
						z: 1.,
					},
					size: Size {
						width: X_OFFSET,
						height: 240.,
					},
					multi_colour: MultiColour {
						top_left: citro2d::render::Colour::new(64, 64, 64),
						top_right: citro2d::render::Colour::new(0, 0, 0),
						bottom_left: citro2d::render::Colour::new(64, 64, 64),
						bottom_right: citro2d::render::Colour::new(0, 0, 0),
					},
				});
			}
			if self.hide_right {
				t.render_2d_shape(&Rectangle {
					point: Point {
						x: X_OFFSET + 256.,
						y: 0.,
						z: 1.,
					},
					size: Size {
						width: X_OFFSET,
						height: 240.,
					},
					multi_colour: MultiColour {
						top_left: citro2d::render::Colour::new(0, 0, 0),
						top_right: citro2d::render::Colour::new(64, 64, 64),
						bottom_left: citro2d::render::Colour::new(0, 0, 0),
						bottom_right: citro2d::render::Colour::new(64, 64, 64),
					},
				});
			}

			perf_stats::stop_gpu();
		});
	}

	fn render_backgrounds_debug(
		&mut self,
		view: BackgroundView,
		mode: DebugBackgroundMode,
		ppu: &Ppu,
	) {
		self.instance.render_target(&mut self.target, |_, t| {
			perf_stats::start_gpu();

			render_background(mode, ppu, t);

			match view {
				BackgroundView::Both => {
					let content_w = 512.;
					let content_h = 256.;
					let scale = (TOP_SCREEN_W / content_w).min(TOP_SCREEN_H / content_h);
					let scaled_w = 256. * scale;
					let scaled_h = 256. * scale;
					let x_offset = (TOP_SCREEN_W - scaled_w * 2.) / 2.;
					let y_offset = (TOP_SCREEN_H - scaled_h) / 2.;

					self.bg1.set_size((scaled_w, scaled_h));
					self.bg2.set_size((scaled_w, scaled_h));
					self.bg1.set_pos((x_offset, y_offset));
					self.bg2.set_pos((x_offset + scaled_w, y_offset));
					self.bg1.set_mirroring(&Mirroring::Normal);
					self.bg2.set_mirroring(&Mirroring::Normal);

					t.render_2d_shape(&self.bg1);
					t.render_2d_shape(&self.bg2);
				}
				BackgroundView::Bg1Only | BackgroundView::Bg2Only => {
					let bg = if view == BackgroundView::Bg1Only {
						&mut self.bg1
					} else {
						&mut self.bg2
					};
					bg.set_size((256., 256.));
					bg.set_pos(((TOP_SCREEN_W - 256.) / 2., 0.));
					bg.set_mirroring(&Mirroring::Normal);

					t.render_2d_shape(bg);
				}
			}

			perf_stats::stop_gpu();
		});
	}

	fn render_sprite_debug(&mut self, sprite_idx: u8, ppu: &Ppu, mode: DebugBackgroundMode) {
		let sprite = &ppu.oam[sprite_idx as usize];
		let tile_idx = sprite.tile;

		const SPRITE_SIZE: f32 = 128.;
		let sprite_pos = (
			(TOP_SCREEN_W - SPRITE_SIZE) / 2.,
			(TOP_SCREEN_H - SPRITE_SIZE) / 2.,
		);

		let sp = &mut self.sprites[sprite_idx as usize];
		sp.sprite.set_size((SPRITE_SIZE, SPRITE_SIZE));
		sp.sprite.set_centre((0., 0.));
		sp.sprite.set_pos(sprite_pos);
		sp.sprite.set_mirroring(&Mirroring::Normal);

		let tile_x = ((tile_idx % 16) as f32) * 8.;
		let tile_y = ((tile_idx / 16) as f32) * 8.;
		const TILE_SIZE: f32 = 8.;

		self.instance.render_target(&mut self.target, |_, t| {
			perf_stats::start_gpu();

			render_background(mode, ppu, t);

			t.render_2d_shape(&self.sprites[sprite_idx as usize].sprite);

			let sides = [
				(TILE_SIZE + 2., 1., -1., -1.),
				(TILE_SIZE + 2., 1., -1., TILE_SIZE),
				(1., TILE_SIZE + 2., -1., -1.),
				(1., TILE_SIZE + 2., TILE_SIZE, -1.),
			];
			for (width, height, x_off, y_off) in sides {
				t.render_2d_shape(&RectangleSolid {
					point: Point {
						x: sprite_pos.0 + x_off + tile_x,
						y: sprite_pos.1 + y_off + tile_y,
						z: 1.,
					},
					size: Size { width, height },
					colour: citro2d::render::Colour::new(255, 0, 0),
				});
			}

			perf_stats::stop_gpu();
		});

		println!("Sprite {} / 64 - Tile {}", sprite_idx, tile_idx);
	}
}

fn render_background(mode: DebugBackgroundMode, ppu: &Ppu, t: &mut Target<'_>) {
	match mode {
		DebugBackgroundMode::Black => {
			t.clear(citro2d::render::Colour::new(0, 0, 0));
		}
		DebugBackgroundMode::White => {
			t.clear(citro2d::render::Colour::new(255, 255, 255));
		}
		DebugBackgroundMode::Checkerboard => {
			t.clear(citro2d::render::Colour::new(200, 200, 200));
			let tile_size: f32 = 16.;
			let mut y = 0.;
			while y < TOP_SCREEN_H {
				let mut x = if (y / tile_size) as i32 % 2 == 0 {
					0.
				} else {
					tile_size
				};
				while x < TOP_SCREEN_W {
					t.render_2d_shape(&RectangleSolid {
						point: Point { x, y, z: 0. },
						size: Size {
							width: tile_size,
							height: tile_size,
						},
						colour: citro2d::render::Colour::new(255, 255, 255),
					});
					x += tile_size * 2.;
				}
				y += tile_size;
			}
		}
		DebugBackgroundMode::Palette0 => {
			let Colour {
				red, green, blue, ..
			} = Colour::from_const(ppu.palettes[0][0]);
			t.clear(citro2d::render::Colour::new(red, green, blue));
		}
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
