use std::sync::{Arc, Mutex};

use bytemuck::{Pod, Zeroable};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, rect::Rect};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Pod, Zeroable)]
pub struct Colour {
	pub blue: u8,
	pub green: u8,
	pub red: u8,
	pub alpha: u8,
}

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 240;

pub type Bitmap = [[Colour; WIDTH]; HEIGHT];

pub const fn empty_bitmap() -> Bitmap {
	[[Colour {
		blue: 0,
		green: 0,
		red: 0,
		alpha: 0,
	}; _]; _]
}

pub fn new_bitmap() -> Arc<Mutex<Box<Bitmap>>> {
	Arc::new(Mutex::new(Box::new(empty_bitmap())))
}

pub fn sdl_thread(texture_ptr: Arc<Mutex<Box<Bitmap>>>) -> Result<(), String> {
	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;

	let window = video_subsystem
		.window("Pixel Test", 800, 600)
		.resizable()
		.position_centered()
		.build()
		.map_err(|e| e.to_string())?;

	let mut canvas = window
		.into_canvas()
		.present_vsync()
		.build()
		.map_err(|e| e.to_string())?;
	let texture_creator = canvas.texture_creator();

	let mut texture = texture_creator
		.create_texture_streaming(PixelFormatEnum::ARGB8888, WIDTH as _, HEIGHT as _)
		.map_err(|e| e.to_string())?;

	let mut event_pump = sdl_context.event_pump()?;

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape | Keycode::Q),
					..
				} => break 'running,
				_ => {}
			}
		}

		let (win_w, win_h) = canvas.window().size();
		let size = win_w.min(win_h);

		let dst = Rect::new(
			((win_w - size) / 2) as i32,
			((win_h - size) / 2) as i32,
			size,
			size,
		);

		texture.with_lock(None, |buffer: &mut [u8], _: usize| {
			let texture_ptr = texture_ptr
				.lock()
				.expect("Mutex poisoned, not dealing with that");

			let texture_buffer: &Bitmap = &texture_ptr;
			let texture_buffer: &[u8] = bytemuck::cast_slice(texture_buffer);
			buffer.copy_from_slice(texture_buffer);
		})?;

		canvas.set_draw_color(sdl2::pixels::Color::BLACK);
		canvas.clear();
		canvas.copy(&texture, None, Some(dst))?;

		let (win_w, win_h) = canvas.window().size();

		let left_width = dst.x();
		for i in 0..left_width {
			let t = i as f32 / left_width as f32;
			let val = (64.0 * (1.0 - t)) as u8; // fade to black
			canvas.set_draw_color(sdl2::pixels::Color::RGB(val, val, val));
			canvas.fill_rect(Rect::new(i, 0, 1, win_h))?;
		}

		let right_width = win_w as i32 - (dst.x() + dst.width() as i32);
		for i in 0..right_width {
			let t = i as f32 / right_width as f32;
			let val = (64.0 * t) as u8; // fade to grey
			let x = dst.x() + dst.width() as i32 + i;
			canvas.set_draw_color(sdl2::pixels::Color::RGB(val, val, val));
			canvas.fill_rect(Rect::new(x, 0, 1, win_h))?;
		}

		let top_height = dst.y();
		for j in 0..top_height {
			let t = j as f32 / top_height as f32;
			let val = (64.0 * (1.0 - t)) as u8;
			canvas.set_draw_color(sdl2::pixels::Color::RGB(val, val, val));
			canvas.fill_rect(Rect::new(0, j, win_w, 1))?;
		}

		let bottom_height = win_h as i32 - (dst.y() + dst.height() as i32);
		for j in 0..bottom_height {
			let t = j as f32 / bottom_height as f32;
			let val = (64.0 * t) as u8;
			let y = dst.y() + dst.height() as i32 + j;
			canvas.set_draw_color(sdl2::pixels::Color::RGB(val, val, val));
			canvas.fill_rect(Rect::new(0, y, win_w, 1))?;
		}

		canvas.present();
	}

	Ok(())
}
