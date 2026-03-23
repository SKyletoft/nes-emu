use emu_core::{
	frame::NesFramebuffer,
	perf_stats,
	ppu::{Colour, NesColour, Ppu},
	unsafe_assert,
};
use sdl2::{
	pixels::{Color, PixelFormatEnum},
	rect::{FPoint, Rect},
	render::{BlendMode, Canvas, Texture, TextureAccess, TextureCreator, Vertex},
	video::{Window, WindowContext},
};

use crate::debug_mode::{BackgroundView, DebugBackgroundMode, DebugMode};

const TILE_SIZE: u32 = 8;
const BG_TILES: u32 = 32;
const BG_SIZE: u32 = TILE_SIZE * BG_TILES;
const PATTERN_TABLE_SIZE: u32 = 128; // 16 tiles × 8 pixels

#[rustfmt::skip]
const SWIZZLE_ORDER: [usize; 64] = [
	 0,  1,  8,  9,  2,  3, 10, 11,
	16, 17, 24, 25, 18, 19, 26, 27,
	 4,  5, 12, 13,  6,  7, 14, 15,
	20, 21, 28, 29, 22, 23, 30, 31,
	32, 33, 40, 41, 34, 35, 42, 43,
	48, 49, 56, 57, 50, 51, 58, 59,
	36, 37, 44, 45, 38, 39, 46, 47,
	52, 53, 60, 61, 54, 55, 62, 63,
];

struct SdlSprite<'tc> {
	texture: Texture<'tc>,
	flip_horizontal: bool,
	flip_vertical: bool,
	uvs: Rect,
}

pub struct SdlFramebuffer<'tc> {
	bg1: Texture<'tc>,
	bg2: Texture<'tc>,
	sprites: [SdlSprite<'tc>; 64],
	framebuffer_texture: Texture<'tc>,
	texture_creator: &'tc TextureCreator<WindowContext>,
	canvas: &'tc mut Canvas<Window>,
	pub hide_left: bool,
	pub hide_right: bool,
	pub debug_mode_enabled: bool,
	pub debug_mode: DebugMode,
	pub debug_background_mode: DebugBackgroundMode,
}

impl<'tc> SdlFramebuffer<'tc> {
	pub fn new(
		tc: &'tc TextureCreator<WindowContext>,
		canvas: &'tc mut Canvas<Window>,
	) -> Result<Self, String> {
		let (win_w, win_h) = canvas.window().size();

		let mut bg1 = tc
			.create_texture_streaming(PixelFormatEnum::ARGB8888, BG_SIZE, BG_SIZE)
			.map_err(|e| e.to_string())?;
		bg1.set_blend_mode(BlendMode::BLEND);

		let mut bg2 = tc
			.create_texture_streaming(PixelFormatEnum::ARGB8888, BG_SIZE, BG_SIZE)
			.map_err(|e| e.to_string())?;
		bg2.set_blend_mode(BlendMode::BLEND);

		let sprites = std::array::from_fn(|_| {
			let mut tex = tc
				.create_texture_streaming(
					PixelFormatEnum::ARGB8888,
					PATTERN_TABLE_SIZE,
					PATTERN_TABLE_SIZE,
				)
				.unwrap();
			tex.set_blend_mode(BlendMode::BLEND);
			SdlSprite {
				texture: tex,
				flip_horizontal: false,
				flip_vertical: false,
				uvs: Rect::new(0, 0, TILE_SIZE, TILE_SIZE),
			}
		});

		let mut framebuffer_texture = tc
			.create_texture(
				PixelFormatEnum::ARGB8888,
				TextureAccess::Target,
				win_w,
				win_h,
			)
			.map_err(|e| e.to_string())?;
		framebuffer_texture.set_blend_mode(BlendMode::BLEND);

		Ok(Self {
			bg1,
			bg2,
			sprites,
			framebuffer_texture,
			texture_creator: tc,
			canvas,
			hide_left: true,
			hide_right: true,
			debug_mode_enabled: false,
			debug_mode: DebugMode::Backgrounds(BackgroundView::Both),
			debug_background_mode: DebugBackgroundMode::Checkerboard,
		})
	}
}

impl NesFramebuffer for SdlFramebuffer<'_> {
	fn update_tile(
		&mut self,
		tile_data: impl Iterator<Item = Option<NesColour>>,
		x: usize,
		y: usize,
		x_offset: usize,
	) {
		let mut buffer = [0u32; 64];
		for (col, i) in tile_data.zip(SWIZZLE_ORDER.iter().copied()) {
			unsafe { unsafe_assert!(i < 64) };
			buffer[i] = nes_colour_to_argb8888(col);
		}

		let bg = if x_offset == 0 {
			&mut self.bg1
		} else {
			&mut self.bg2
		};

		let byte_buffer: &[u8] = bytemuck::cast_slice(&buffer);
		let rect = Rect::new((x * 8) as i32, (y * 8) as i32, TILE_SIZE, TILE_SIZE);
		bg.update(rect, byte_buffer, 8 * 4).unwrap();
	}

	fn update_sprite(
		&mut self,
		sprite_data: impl Iterator<Item = Option<NesColour>>,
		sprite_idx: usize,
		tile_idx: u8,
	) {
		let mut buffer = [0u32; PATTERN_TABLE_SIZE as usize * PATTERN_TABLE_SIZE as usize];

		let mut sprite_data = sprite_data;

		const TILE_COUNT: usize = 256usize.isqrt();

		for tile_row in 0..TILE_COUNT {
			for tile_col in 0..TILE_COUNT {
				for swizzle_idx in SWIZZLE_ORDER.iter().copied() {
					let pixel_x_in_tile = swizzle_idx % 8;
					let pixel_y_in_tile = swizzle_idx / 8;

					let pixel_x = tile_col * 8 + pixel_x_in_tile;
					let pixel_y = tile_row * 8 + pixel_y_in_tile;
					let buffer_idx = pixel_y * PATTERN_TABLE_SIZE as usize + pixel_x;

					unsafe { unsafe_assert!(buffer_idx < buffer.len()) };

					let col = sprite_data.next().unwrap_or(None);
					buffer[buffer_idx] = nes_colour_to_argb8888(col);
				}
			}
		}

		let byte_buffer: &[u8] = bytemuck::cast_slice(&buffer);
		self.sprites[sprite_idx]
			.texture
			.update(None, byte_buffer, PATTERN_TABLE_SIZE as usize * 4)
			.unwrap();

		unsafe { unsafe_assert!(sprite_idx < 64) };
		let tile_x = (tile_idx % 16) as i32;
		let tile_y = (tile_idx / 16) as i32;

		let src_rect = Rect::new(
			tile_x * TILE_SIZE as i32,
			tile_y * TILE_SIZE as i32,
			TILE_SIZE,
			TILE_SIZE,
		);
		self.sprites[sprite_idx].uvs = src_rect;
	}

	fn set_mirroring(&mut self, sprite_idx: usize, horizontal: bool, vertical: bool) {
		unsafe { unsafe_assert!(sprite_idx < 64) };
		self.sprites[sprite_idx].flip_horizontal = horizontal;
		self.sprites[sprite_idx].flip_vertical = vertical;
	}

	fn render(&mut self, ppu: &Ppu, lines: &[(i16, i16); 240]) {
		if !self.debug_mode_enabled {
			self.render_nes_frame(ppu, lines);
		} else {
			match self.debug_mode {
				DebugMode::Backgrounds(view) => {
					draw_debug_background(&mut *self.canvas, self.debug_background_mode, ppu);
					render_backgrounds_debug(self.canvas, &self.bg1, &self.bg2, view);
				}
				DebugMode::Sprites(idx) => {
					draw_debug_background(&mut *self.canvas, self.debug_background_mode, ppu);
					let sprite_uv = self.sprites[idx as usize].uvs;
					render_sprite_debug(
						self.canvas,
						&self.sprites[idx as usize].texture,
						sprite_uv,
						idx,
					);
				}
			}
		}
		perf_stats::stop_gpu();
		self.canvas.present();
	}
}

impl SdlFramebuffer<'_> {
	fn render_nes_frame(&mut self, ppu: &Ppu, lines: &[(i16, i16); 240]) {
		let canvas = &mut *self.canvas;

		let bg_colour = Colour::from_const(ppu.palettes[0][0]);
		canvas.set_draw_color(Color::RGBA(
			bg_colour.red,
			bg_colour.green,
			bg_colour.blue,
			bg_colour.alpha,
		));

		let (win_w, win_h) = canvas.window().size();
		let tex_w = std::cmp::max(256, 240 * win_w / win_h);
		let tex_h = 240u32;
		let tex_query = self.framebuffer_texture.query();
		if (tex_w, tex_h) != (tex_query.width, tex_query.height) {
			self.framebuffer_texture = self
				.texture_creator
				.create_texture(
					PixelFormatEnum::ARGB8888,
					TextureAccess::Target,
					tex_w,
					tex_h,
				)
				.unwrap();
			self.framebuffer_texture.set_blend_mode(BlendMode::BLEND);
		}

		canvas
			.with_texture_canvas(&mut self.framebuffer_texture, |tex_canvas| {
				tex_canvas.set_draw_color(Color::RGBA(
					bg_colour.red,
					bg_colour.green,
					bg_colour.blue,
					bg_colour.alpha,
				));
				tex_canvas.clear();

				let dst_x = ((tex_w - 256) / 2) as i32;
				let dst_y = 0;
				let dst_w = 256;
				let dst_h = 240;

				let scale_num_x = dst_w as i64;
				let scale_num_y = dst_h as i64;
				const WIDTH: i64 = 256;
				const SCALE_DENOM_X: i64 = 256;
				const SCALE_DENOM_Y: i64 = 240;

				if ppu.mask.show_spr() {
					for (idx, sprite) in self.sprites.iter().enumerate() {
						if ppu.oam[idx].attr.priority() && ppu.oam[idx].is_visible() {
							let nes_x = ppu.oam[idx].x as i64;
							let nes_y = ppu.oam[idx].y as i64 + 1;
							let left = dst_x as i64 + (nes_x * scale_num_x) / SCALE_DENOM_X;
							let right = dst_x as i64
								+ ((nes_x + TILE_SIZE as i64) * scale_num_x) / SCALE_DENOM_X;
							let top = dst_y as i64 + (nes_y * scale_num_y) / SCALE_DENOM_Y;
							let bottom = dst_y as i64
								+ ((nes_y + TILE_SIZE as i64) * scale_num_y) / SCALE_DENOM_Y;
							let sprite_dst = Rect::new(
								left as i32,
								top as i32,
								(right - left) as u32,
								(bottom - top) as u32,
							);
							tex_canvas
								.copy_ex(
									&sprite.texture,
									Some(sprite.uvs),
									Some(sprite_dst),
									0.0,
									None,
									sprite.flip_horizontal,
									sprite.flip_vertical,
								)
								.unwrap();
						}
					}
				}

				if ppu.mask.show_bg() {
					let background_slices = lines
						.chunk_by(|l, r| l.0 == r.0 && l.1 + 1 == r.1)
						.scan(0, |acc, curr| {
							let old_acc = *acc;
							*acc += curr.len();
							Some((curr[0].0, curr[0].1, old_acc, curr.len() as i16))
						});
					for (x_offset, y_offset, y_start, height) in background_slices {
						let top = dst_y as i64 + (y_start as i64 * scale_num_y) / SCALE_DENOM_Y;
						let bottom = dst_y as i64
							+ ((y_start as i64 + height as i64) * scale_num_y) / SCALE_DENOM_Y;
						let src_y = (y_offset as u32) % BG_SIZE;
						let src_rect = Rect::new(0, src_y as i32, BG_SIZE, height as u32);
						let dst_w = ((BG_SIZE as i64 * scale_num_x) / SCALE_DENOM_X) as u32;
						let dst_h = (bottom - top) as u32;

						let x1 = {
							let base =
								dst_x as i64 - (x_offset as i64 * scale_num_x) / SCALE_DENOM_X;
							let min_x = dst_x as i64 - (WIDTH * scale_num_x) / SCALE_DENOM_X;
							if base < min_x {
								base + (512 * scale_num_x) / SCALE_DENOM_X
							} else {
								base
							}
						};
						let x2 = {
							let base = dst_x as i64 + (WIDTH * scale_num_x) / SCALE_DENOM_X
								- (x_offset as i64 * scale_num_x) / SCALE_DENOM_X;
							let min_x = dst_x as i64 - (WIDTH * scale_num_x) / SCALE_DENOM_X;
							if base < min_x {
								base + (512 * scale_num_x) / SCALE_DENOM_X
							} else {
								base
							}
						};

						for (bg, x) in [(&self.bg1, x1), (&self.bg2, x2)].into_iter() {
							for offset in [-512, 0, 512].into_iter() {
								let dst_rect = Rect::new(
									(x + offset * scale_num_x / SCALE_DENOM_X) as i32,
									top as i32,
									dst_w,
									dst_h,
								);
								tex_canvas.copy(bg, Some(src_rect), Some(dst_rect)).unwrap();
							}
						}
					}
				}

				if ppu.mask.show_spr() {
					for (idx, sprite) in self.sprites.iter().enumerate() {
						if !ppu.oam[idx].attr.priority() && ppu.oam[idx].is_visible() {
							let nes_x = ppu.oam[idx].x as i64;
							let nes_y = ppu.oam[idx].y as i64 + 1;
							let left = dst_x as i64 + (nes_x * scale_num_x) / SCALE_DENOM_X;
							let right = dst_x as i64
								+ ((nes_x + TILE_SIZE as i64) * scale_num_x) / SCALE_DENOM_X;
							let top = dst_y as i64 + (nes_y * scale_num_y) / SCALE_DENOM_Y;
							let bottom = dst_y as i64
								+ ((nes_y + TILE_SIZE as i64) * scale_num_y) / SCALE_DENOM_Y;
							let sprite_dst = Rect::new(
								left as i32,
								top as i32,
								(right - left) as u32,
								(bottom - top) as u32,
							);
							tex_canvas
								.copy_ex(
									&sprite.texture,
									Some(sprite.uvs),
									Some(sprite_dst),
									0.0,
									None,
									sprite.flip_horizontal,
									sprite.flip_vertical,
								)
								.unwrap();
						}
					}
				}
			})
			.unwrap();

		let content_height = (15 * (win_w as i64) / 16) as i32;
		let original_content_width = (((win_h as i64) * 16 / 15) as i32).min(win_w as i32);
		let left_width = (win_w as i32 - original_content_width) / 2;
		let top_height = (win_h as i32 - content_height) / 2;
		let dst =
			(win_w * 15 / 16 <= win_h).then(|| Rect::new(0, top_height, win_w, win_w * 15 / 16));

		canvas.copy(&self.framebuffer_texture, None, dst).unwrap();
		if self.hide_left {
			draw_horizontal_gradient(canvas, 0.0, left_width as f32, win_h as f32, 64, 0);
		}
		if self.hide_right {
			draw_horizontal_gradient(
				canvas,
				(win_w as i32 - left_width) as f32,
				left_width as f32,
				win_h as f32,
				0,
				64,
			);
		}
		draw_vertical_gradient(canvas, 0.0, top_height as f32, win_w as f32, 64, 0);
		draw_vertical_gradient(
			canvas,
			(win_h as i32 - top_height) as f32,
			top_height as f32,
			win_w as f32,
			0,
			64,
		);
	}
}

fn draw_debug_background(canvas: &mut Canvas<Window>, mode: DebugBackgroundMode, ppu: &Ppu) {
	let (win_w, win_h) = canvas.window().size();

	match mode {
		DebugBackgroundMode::Black => {
			canvas.set_draw_color(Color::RGB(0, 0, 0));
			canvas.clear();
		}
		DebugBackgroundMode::White => {
			canvas.set_draw_color(Color::RGB(255, 255, 255));
			canvas.clear();
		}
		DebugBackgroundMode::Checkerboard => {
			let tile_size = 16;

			for y in (0..win_h).step_by(tile_size as usize) {
				for x in (0..win_w).step_by(tile_size as usize) {
					// Alternate between white and light grey
					let is_white = ((x / tile_size) + (y / tile_size)) % 2 == 0;
					let colour = if is_white {
						Color::RGB(255, 255, 255)
					} else {
						Color::RGB(200, 200, 200)
					};

					canvas.set_draw_color(colour);
					canvas
						.fill_rect(Rect::new(
							x as i32,
							y as i32,
							tile_size.min(win_w - x),
							tile_size.min(win_h - y),
						))
						.ok();
				}
			}
		}
		DebugBackgroundMode::Palette0 => {
			let bg_colour = Colour::from_const(ppu.palettes[0][0]);
			canvas.set_draw_color(Color::RGB(bg_colour.red, bg_colour.green, bg_colour.blue));
			canvas.clear();
		}
	}
}

fn render_backgrounds_debug(
	canvas: &mut Canvas<Window>,
	bg1: &Texture,
	bg2: &Texture,
	view: BackgroundView,
) {
	let (win_w, win_h) = canvas.window().size();

	match view {
		BackgroundView::Both => {
			let content_w = 512.0;
			let content_h = 256.0;
			let scale = ((win_w as f32 / content_w).min(win_h as f32 / content_h)).max(1.0);
			let scaled_w = content_w * scale;
			let scaled_h = content_h * scale;
			let x_offset = ((win_w as f32 - scaled_w) / 2.0) as i32;
			let y_offset = ((win_h as f32 - scaled_h) / 2.0) as i32;

			canvas
				.copy(
					bg1,
					None,
					Rect::new(
						x_offset,
						y_offset,
						(256.0 * scale) as u32,
						(256.0 * scale) as u32,
					),
				)
				.unwrap();

			canvas
				.copy(
					bg2,
					None,
					Rect::new(
						x_offset + (256.0 * scale) as i32,
						y_offset,
						(256.0 * scale) as u32,
						(256.0 * scale) as u32,
					),
				)
				.unwrap();
		}
		BackgroundView::Bg1Only => {
			let content_w = 256.0;
			let content_h = 256.0;
			let scale = ((win_w as f32 / content_w).min(win_h as f32 / content_h)).max(1.0);
			let scaled_w = content_w * scale;
			let scaled_h = content_h * scale;
			let x_offset = ((win_w as f32 - scaled_w) / 2.0) as i32;
			let y_offset = ((win_h as f32 - scaled_h) / 2.0) as i32;

			canvas
				.copy(
					bg1,
					None,
					Rect::new(x_offset, y_offset, scaled_w as u32, scaled_h as u32),
				)
				.unwrap();
		}
		BackgroundView::Bg2Only => {
			let content_w = 256.0;
			let content_h = 256.0;
			let scale = ((win_w as f32 / content_w).min(win_h as f32 / content_h)).max(1.0);
			let scaled_w = content_w * scale;
			let scaled_h = content_h * scale;
			let x_offset = ((win_w as f32 - scaled_w) / 2.0) as i32;
			let y_offset = ((win_h as f32 - scaled_h) / 2.0) as i32;

			canvas
				.copy(
					bg2,
					None,
					Rect::new(x_offset, y_offset, scaled_w as u32, scaled_h as u32),
				)
				.unwrap();
		}
	}
}

fn render_sprite_debug(
	canvas: &mut Canvas<Window>,
	sprite_tex: &Texture,
	sprite_uv: Rect,
	sprite_idx: u8,
) {
	let (win_w, win_h) = canvas.window().size();

	let base_size = PATTERN_TABLE_SIZE as f32;
	let min_scale = 4.0;
	let scale = ((win_w as f32 / base_size).min(win_h as f32 / base_size) * 0.5).max(min_scale);
	let scaled_size = base_size * scale;
	let x_offset = ((win_w as f32 - scaled_size) / 2.0) as i32;
	let y_offset = ((win_h as f32 - scaled_size) / 2.0) as i32;

	canvas
		.copy(
			sprite_tex,
			None,
			Rect::new(x_offset, y_offset, scaled_size as u32, scaled_size as u32),
		)
		.unwrap();

	let rect_x = x_offset + (sprite_uv.x() as f32 * scale) as i32;
	let rect_y = y_offset + (sprite_uv.y() as f32 * scale) as i32;
	let rect_w = (sprite_uv.width() as f32 * scale) as u32;
	let rect_h = (sprite_uv.height() as f32 * scale) as u32;

	canvas.set_draw_color(Color::RGB(255, 0, 0));

	for offset in 0..3 {
		let rect = Rect::new(
			rect_x - 1 + offset,
			rect_y - 1 + offset,
			rect_w + 2 - (offset as u32 * 2),
			rect_h + 2 - (offset as u32 * 2),
		);
		canvas.draw_rect(rect).unwrap();
	}

	let tile_idx = (sprite_uv.x() / TILE_SIZE as i32) + (sprite_uv.y() / TILE_SIZE as i32) * 16;
	println!("Sprite {} / 64 - Tile {}", sprite_idx, tile_idx);
}

fn nes_colour_to_argb8888(value: Option<NesColour>) -> u32 {
	let Some(value) = value else {
		return 0x00000000;
	};
	const fn convert_colour(c: NesColour) -> u32 {
		let Colour {
			blue,
			green,
			red,
			alpha,
		} = Colour::from_const(c);
		(alpha as u32) << 24 | (red as u32) << 16 | (green as u32) << 8 | (blue as u32)
	}
	const TRANSLATED_COLOURS: [u32; 64] = NesColour::PALETTE.map(convert_colour);
	TRANSLATED_COLOURS[value as usize]
}

fn draw_horizontal_gradient(
	canvas: &mut Canvas<Window>,
	x: f32,
	width: f32,
	height: f32,
	start_val: u8,
	end_val: u8,
) {
	if width <= 0.0 || height <= 0.0 {
		return;
	}
	let vertices = [
		Vertex {
			position: FPoint::new(x, 0.0),
			color: Color::RGB(start_val, start_val, start_val),
			tex_coord: FPoint::new(0.0, 0.0),
		},
		Vertex {
			position: FPoint::new(x + width, 0.0),
			color: Color::RGB(end_val, end_val, end_val),
			tex_coord: FPoint::new(0.0, 0.0),
		},
		Vertex {
			position: FPoint::new(x + width, height),
			color: Color::RGB(end_val, end_val, end_val),
			tex_coord: FPoint::new(0.0, 0.0),
		},
		Vertex {
			position: FPoint::new(x, height),
			color: Color::RGB(start_val, start_val, start_val),
			tex_coord: FPoint::new(0.0, 0.0),
		},
	];
	let indices: [[i32; 3]; 2] = [[0, 1, 2], [2, 3, 0]];
	canvas.render_geometry(&vertices, None, &indices).unwrap();
}

fn draw_vertical_gradient(
	canvas: &mut Canvas<Window>,
	y: f32,
	height: f32,
	width: f32,
	start_val: u8,
	end_val: u8,
) {
	if height <= 0.0 || width <= 0.0 {
		return;
	}
	let vertices = [
		Vertex {
			position: FPoint::new(0.0, y),
			color: Color::RGB(start_val, start_val, start_val),
			tex_coord: FPoint::new(0.0, 0.0),
		},
		Vertex {
			position: FPoint::new(width, y),
			color: Color::RGB(start_val, start_val, start_val),
			tex_coord: FPoint::new(0.0, 0.0),
		},
		Vertex {
			position: FPoint::new(width, y + height),
			color: Color::RGB(end_val, end_val, end_val),
			tex_coord: FPoint::new(0.0, 0.0),
		},
		Vertex {
			position: FPoint::new(0.0, y + height),
			color: Color::RGB(end_val, end_val, end_val),
			tex_coord: FPoint::new(0.0, 0.0),
		},
	];
	let indices: [[i32; 3]; 2] = [[0, 1, 2], [2, 3, 0]];
	canvas.render_geometry(&vertices, None, &indices).unwrap();
}
