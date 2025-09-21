use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, rect::Rect};
use std::{
	sync::{Arc, atomic::AtomicPtr},
	time::Duration,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Colour {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8,
}

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

pub type Bitmap = [[Colour; WIDTH]; HEIGHT];

pub fn main() {
	let texture = Arc::new(Box::new([[Colour::default(); _]; _]));
	sdl_thread(texture);
}

pub fn sdl_thread(texture: Arc<Box<Bitmap>>) -> Result<(), String> {
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

	// Fill texture pixel by pixel
	texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
		for y in 0..HEIGHT {
			for x in 0..WIDTH {
				let offset = y * pitch + x * 4;
				// Simple gradient pattern
				buffer[offset] = x as u8; // B
				buffer[offset + 1] = y as u8; // G
				buffer[offset + 2] = 128; // R
				buffer[offset + 3] = 255; // A
			}
		}
	})?;

	let mut event_pump = sdl_context.event_pump()?;

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
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

		canvas.set_draw_color(sdl2::pixels::Color::BLACK);
		canvas.clear();
		canvas.copy(&texture, None, Some(dst))?;
		canvas.present();

		// std::thread::sleep(Duration::from_millis(16));
	}

	Ok(())
}
