use std::sync::{
	Arc, Mutex,
	atomic::{AtomicU8, Ordering},
};

use sdl2::{
	controller::Button, event::Event, keyboard::Keycode, pixels::PixelFormatEnum, rect::Rect,
};

use emu_core::{
	controller::ControllerState,
	graphics::{Bitmap, HEIGHT, WIDTH},
};

pub fn sdl_thread(
	texture_ptr: Arc<Mutex<Box<Bitmap>>>,
	shared_controller_state: &AtomicU8,
) -> Result<(), String> {
	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;
	let controller_subsystem = sdl_context.game_controller()?;

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

	let _controller = (0..controller_subsystem.num_joysticks()?)
		.filter_map(|i| {
			if controller_subsystem.is_game_controller(i) {
				controller_subsystem.open(i).ok()
			} else {
				None
			}
		})
		.collect::<Vec<_>>();
	let mut controller_state = ControllerState::new();

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape | Keycode::Q),
					..
				} => break 'running,
				Event::KeyDown {
					keycode: Some(Keycode::Left),
					..
				}
				| Event::ControllerButtonDown {
					button: Button::DPadLeft,
					..
				} => {
					controller_state.set_left(true);
				}
				Event::KeyUp {
					keycode: Some(Keycode::Left),
					..
				}
				| Event::ControllerButtonUp {
					button: Button::DPadLeft,
					..
				} => {
					controller_state.set_left(false);
				}
				Event::KeyDown {
					keycode: Some(Keycode::Right),
					..
				}
				| Event::ControllerButtonDown {
					button: Button::DPadRight,
					..
				} => {
					controller_state.set_right(true);
				}
				Event::KeyUp {
					keycode: Some(Keycode::Right),
					..
				}
				| Event::ControllerButtonUp {
					button: Button::DPadRight,
					..
				} => {
					controller_state.set_right(false);
				}
				Event::KeyDown {
					keycode: Some(Keycode::Up),
					..
				}
				| Event::ControllerButtonDown {
					button: Button::DPadUp,
					..
				} => {
					controller_state.set_up(true);
				}
				Event::KeyUp {
					keycode: Some(Keycode::Up),
					..
				}
				| Event::ControllerButtonUp {
					button: Button::DPadUp,
					..
				} => {
					controller_state.set_up(false);
				}
				Event::KeyDown {
					keycode: Some(Keycode::Down),
					..
				}
				| Event::ControllerButtonDown {
					button: Button::DPadDown,
					..
				} => {
					controller_state.set_down(true);
				}
				Event::KeyUp {
					keycode: Some(Keycode::Down),
					..
				}
				| Event::ControllerButtonUp {
					button: Button::DPadDown,
					..
				} => {
					controller_state.set_down(false);
				}
				Event::KeyDown {
					keycode: Some(Keycode::Z),
					..
				}
				| Event::ControllerButtonDown {
					button: Button::A, ..
				} => {
					controller_state.set_a(true);
				}
				Event::KeyUp {
					keycode: Some(Keycode::Z),
					..
				}
				| Event::ControllerButtonUp {
					button: Button::A, ..
				} => {
					controller_state.set_a(false);
				}
				Event::KeyDown {
					keycode: Some(Keycode::X),
					..
				}
				| Event::ControllerButtonDown {
					button: Button::B | Button::X,
					..
				} => {
					controller_state.set_b(true);
				}
				Event::KeyUp {
					keycode: Some(Keycode::X),
					..
				}
				| Event::ControllerButtonUp {
					button: Button::B | Button::X,
					..
				} => {
					controller_state.set_b(false);
				}
				Event::KeyDown {
					keycode: Some(Keycode::Return),
					..
				}
				| Event::ControllerButtonDown {
					button: Button::Start,
					..
				} => {
					controller_state.set_start(true);
				}
				Event::KeyUp {
					keycode: Some(Keycode::Return),
					..
				}
				| Event::ControllerButtonUp {
					button: Button::Start,
					..
				} => {
					controller_state.set_start(false);
				}
				Event::KeyDown {
					keycode: Some(Keycode::RShift),
					..
				}
				| Event::ControllerButtonDown {
					button: Button::Back,
					..
				} => {
					controller_state.set_select(true);
				}
				Event::KeyUp {
					keycode: Some(Keycode::RShift),
					..
				}
				| Event::ControllerButtonUp {
					button: Button::Back,
					..
				} => {
					controller_state.set_select(false);
				}
				_ => {}
			}
		}

		shared_controller_state.store(controller_state.into_bits(), Ordering::SeqCst);

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
