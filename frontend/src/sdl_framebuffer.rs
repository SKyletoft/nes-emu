use emu_core::{
	frame::NesFramebuffer,
	graphics::{Colour, WIDTH},
	mapper::Mapper,
	ppu::{NesColour, Ppu},
};
use sdl2::{
	pixels::{Color, PixelFormatEnum},
	rect::{FPoint, Rect},
	render::{BlendMode, Canvas, Texture, TextureCreator, Vertex},
	video::{Window, WindowContext},
};

const TILE_SIZE: u32 = 8;
const BG_TILES: u32 = 32;
const BG_SIZE: u32 = TILE_SIZE * BG_TILES;

pub struct SdlFramebuffer<'tc> {
	bg1: Texture<'tc>,
	bg2: Texture<'tc>,
	sprites: [Texture<'tc>; 64],
	canvas: &'tc mut Canvas<Window>,
	pub hide_left: bool,
	pub hide_right: bool,
}

impl<'tc> SdlFramebuffer<'tc> {
	pub fn new(
		tc: &'tc TextureCreator<WindowContext>,
		canvas: &'tc mut Canvas<Window>,
	) -> Result<Self, String> {
		let mut bg1 = tc
			.create_texture_streaming(PixelFormatEnum::ARGB8888, BG_SIZE, BG_SIZE)
			.map_err(|e| e.to_string())?;
		bg1.set_blend_mode(BlendMode::Blend);

		let mut bg2 = tc
			.create_texture_streaming(PixelFormatEnum::ARGB8888, BG_SIZE, BG_SIZE)
			.map_err(|e| e.to_string())?;
		bg2.set_blend_mode(BlendMode::Blend);

		let sprites = std::array::from_fn(|_| {
			let mut tex = tc
				.create_texture_streaming(PixelFormatEnum::ARGB8888, TILE_SIZE, TILE_SIZE)
				.unwrap();
			tex.set_blend_mode(BlendMode::Blend);
			tex
		});

		Ok(Self {
			bg1,
			bg2,
			sprites,
			canvas,
			hide_left: true,
			hide_right: true,
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
		let mut buffer = [[0u32; 8]; 8];
		for (col, pixel) in tile_data.zip(buffer.iter_mut().flat_map(|l| l.iter_mut())) {
			*pixel = nes_colour_to_argb8888(col);
		}

		let bg = if x_offset == 0 {
			&mut self.bg1
		} else {
			&mut self.bg2
		};

		let byte_buffer: &[u8] = bytemuck::cast_slice(&buffer);
		let rect = Rect::new((x * 8) as i32, (y * 8) as i32, TILE_SIZE, TILE_SIZE);
		let _ = bg.update(rect, byte_buffer, 8 * 4);
	}

	fn update_sprite(&mut self, sprite_data: impl Iterator<Item = Option<NesColour>>, idx: usize) {
		let mut buffer = [[0u32; 8]; 8];
		for (pixel, col) in buffer
			.iter_mut()
			.flat_map(|xs| xs.iter_mut())
			.zip(sprite_data)
		{
			*pixel = nes_colour_to_argb8888(col);
		}

		let byte_buffer: &[u8] = bytemuck::cast_slice(&buffer);
		let _ = self.sprites[idx].update(None, byte_buffer, 8 * 4);
	}

	fn render<M: Mapper>(&mut self, m: &M, ppu: &Ppu, lines: &[(i16, i16); 240]) {
		let canvas = &mut *self.canvas;

		let bg_colour = Colour::from_const(ppu.palettes[0][0]);
		canvas.set_draw_color(Color::RGBA(
			bg_colour.red,
			bg_colour.green,
			bg_colour.blue,
			bg_colour.alpha,
		));
		canvas.clear();

		let (win_w, win_h) = canvas.window().size();
		let size = win_w.min(win_h);
		let dst_x = ((win_w - size) / 2) as i32;
		let dst_y = ((win_h - size) / 2) as i32;
		let dst_w = size as i32;
		let dst_h = size as i32;

		let scale_num_x = dst_w as i64;
		let scale_num_y = dst_h as i64;
		const SCALE_DENOM_X: i64 = WIDTH as i64;
		const SCALE_DENOM_Y: i64 = 240;

		if ppu.mask.show_spr() {
			for (idx, sprite) in self.sprites.iter().enumerate() {
				if ppu.oam[idx].attr.priority() && ppu.oam[idx].is_visible() {
					let nes_x = ppu.oam[idx].x as i64;
					let nes_y = ppu.oam[idx].y as i64 + 1;
					let left = dst_x as i64 + (nes_x * scale_num_x) / SCALE_DENOM_X;
					let right =
						dst_x as i64 + ((nes_x + TILE_SIZE as i64) * scale_num_x) / SCALE_DENOM_X;
					let top = dst_y as i64 + (nes_y * scale_num_y) / SCALE_DENOM_Y;
					let bottom =
						dst_y as i64 + ((nes_y + TILE_SIZE as i64) * scale_num_y) / SCALE_DENOM_Y;
					let sprite_dst = Rect::new(
						left as i32,
						top as i32,
						(right - left) as u32,
						(bottom - top) as u32,
					);
					let _ = canvas.copy(sprite, None, Some(sprite_dst));
				}
			}
		}

		if ppu.mask.show_bg() {
			let background_slices =
				lines
					.chunk_by(|l, r| l.0 == r.0 && l.1 + 1 == r.1)
					.scan(0, |acc, curr| {
						let old_acc = *acc;
						*acc += curr.len();
						Some((curr[0].0, curr[0].1, old_acc, curr.len() as i16))
					});
			for (x_offset, y_offset, y_start, height) in background_slices {
				let top = dst_y as i64 + (y_start as i64 * scale_num_y) / SCALE_DENOM_Y;
				let bottom =
					dst_y as i64 + ((y_start as i64 + height as i64) * scale_num_y) / SCALE_DENOM_Y;
				let src_y = (y_offset as u32) % BG_SIZE;

				let x1 = {
					let base = dst_x as i64 - (x_offset as i64 * scale_num_x) / SCALE_DENOM_X;
					let min_x = dst_x as i64 - (WIDTH as i64 * scale_num_x) / SCALE_DENOM_X;
					if base < min_x {
						base + (512 * scale_num_x) / SCALE_DENOM_X
					} else {
						base
					}
				};
				let src_rect = Rect::new(0, src_y as i32, BG_SIZE, height as u32);
				let dst_rect = Rect::new(
					x1 as i32,
					top as i32,
					((BG_SIZE as i64 * scale_num_x) / SCALE_DENOM_X) as u32,
					(bottom - top) as u32,
				);
				let _ = canvas.copy(&self.bg1, Some(src_rect), Some(dst_rect));

				let x2 = {
					let base = dst_x as i64 + (WIDTH as i64 * scale_num_x) / SCALE_DENOM_X
						- (x_offset as i64 * scale_num_x) / SCALE_DENOM_X;
					let min_x = dst_x as i64 - (WIDTH as i64 * scale_num_x) / SCALE_DENOM_X;
					if base < min_x {
						base + (512 * scale_num_x) / SCALE_DENOM_X
					} else {
						base
					}
				};
				let dst_rect = Rect::new(
					x2 as i32,
					top as i32,
					((BG_SIZE as i64 * scale_num_x) / SCALE_DENOM_X) as u32,
					(bottom - top) as u32,
				);
				let _ = canvas.copy(&self.bg2, Some(src_rect), Some(dst_rect));
			}
		}

		if ppu.mask.show_spr() {
			for (idx, sprite) in self.sprites.iter().enumerate() {
				if !ppu.oam[idx].attr.priority() && ppu.oam[idx].is_visible() {
					let nes_x = ppu.oam[idx].x as i64;
					let nes_y = ppu.oam[idx].y as i64 + 1;
					let left = dst_x as i64 + (nes_x * scale_num_x) / SCALE_DENOM_X;
					let right =
						dst_x as i64 + ((nes_x + TILE_SIZE as i64) * scale_num_x) / SCALE_DENOM_X;
					let top = dst_y as i64 + (nes_y * scale_num_y) / SCALE_DENOM_Y;
					let bottom =
						dst_y as i64 + ((nes_y + TILE_SIZE as i64) * scale_num_y) / SCALE_DENOM_Y;
					let sprite_dst = Rect::new(
						left as i32,
						top as i32,
						(right - left) as u32,
						(bottom - top) as u32,
					);
					let _ = canvas.copy(sprite, None, Some(sprite_dst));
				}
			}
		}

		let left_width = dst_x;
		let right_width = win_w as i32 - (dst_x + dst_w);
		let top_height = dst_y;
		let bottom_height = win_h as i32 - (dst_y + dst_h);

		if self.hide_left {
			draw_horizontal_gradient(canvas, 0.0, left_width as f32, win_h as f32, 64, 0);
		}
		if self.hide_right {
			draw_horizontal_gradient(
				canvas,
				(dst_x + dst_w) as f32,
				right_width as f32,
				win_h as f32,
				0,
				64,
			);
		}
		draw_vertical_gradient(canvas, 0.0, top_height as f32, win_w as f32, 64, 0);
		draw_vertical_gradient(
			canvas,
			(dst_y + dst_h) as f32,
			bottom_height as f32,
			win_w as f32,
			0,
			64,
		);

		canvas.present();
	}
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
	let _ = canvas.render_geometry(&vertices, None, &indices);
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
	let _ = canvas.render_geometry(&vertices, None, &indices);
}
