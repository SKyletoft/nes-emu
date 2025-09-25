use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, rect::Rect};
use std::sync::{Arc, Mutex};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Colour {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8,
}

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 240;

pub type Bitmap = [[Colour; WIDTH]; HEIGHT];

pub fn empty_bitmap() -> Bitmap {
	[[Colour::default(); _]; _]
}

pub fn new_bitmap() -> Arc<Mutex<Bitmap>> {
	Arc::new(Mutex::new(empty_bitmap()))
}

pub fn sdl_thread(texture_ptr: Arc<Mutex<Bitmap>>) -> Result<(), String> {
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
			// Bytemuck can't handle nested arrays?
			for (src, dst) in texture_ptr
				.iter()
				.flat_map(|line| line.iter())
				.zip(buffer.chunks_exact_mut(4))
			{
				dst[0] = src.alpha;
				dst[1] = src.red;
				dst[2] = src.green;
				dst[3] = src.blue;
			}
		})?;

		canvas.set_draw_color(sdl2::pixels::Color::BLACK);
		canvas.clear();
		canvas.copy(&texture, None, Some(dst))?;
		canvas.present();

		// std::thread::sleep(Duration::from_millis(16));
	}

	Ok(())
}
