use std::sync::{
	Arc, Mutex,
	atomic::{AtomicU8, Ordering},
};

use emu_core::{
	controller::ControllerState,
	frame::NesFramebuffer,
	graphics::{Bitmap, HEIGHT, WIDTH},
	unsafe_assert,
};
use sdl2::{
	controller::Button,
	event::Event,
	keyboard::Keycode,
	pixels::{Color, PixelFormatEnum},
	rect::Rect,
	render::Canvas,
	video::Window,
};

pub struct SdlFramebuffer {
	pub output_texture: Arc<Mutex<Box<Bitmap>>>,
	pub current_texture: Box<Bitmap>,
}

impl NesFramebuffer for SdlFramebuffer {
	fn render<M: emu_core::mapper::Mapper>(
		&mut self,
		m: &M,
		ppu: &emu_core::ppu::Ppu,
		lines: &[(i16, i16); 240],
	) {
		fn set(
			framebuffer: &mut SdlFramebuffer,
			y: usize,
			x: usize,
			col: emu_core::ppu::NesColour,
		) {
			unsafe { unsafe_assert!((0..HEIGHT).contains(&y)) };
			unsafe { unsafe_assert!((0..WIDTH).contains(&x)) };
			framebuffer.current_texture[y][x] = col.into();
		}

		let bg = ppu.palettes[0][0];

		for dot in 0..256 {
			for (at, _) in lines.iter().copied().enumerate() {
				set(self, at, dot as usize, bg);
			}
		}

		if ppu.mask.show_spr() {
			for (idx, sprite) in ppu
				.oam
				.iter()
				.enumerate()
				.filter(|(_, s)| s.is_visible() && s.attr.priority())
			{
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
						set(self, line as usize, dot as usize, col);
					}
				}
			}
		}

		if ppu.mask.show_bg() {
			for (at, pos) in lines.iter().enumerate() {
				for dot in 0..256 {
					let tilemap_x = (dot + pos.0) % 512;
					let tilemap_y = pos.1; // This is broken, but I'm preserving behaviour for now
					let Some(col) = m.get_bg_pixel(tilemap_x, tilemap_y, ppu) else {
						continue;
					};
					set(self, at, dot as usize, col);
				}
			}
		}

		if ppu.mask.show_spr() {
			for (idx, sprite) in ppu
				.oam
				.iter()
				.enumerate()
				.filter(|(_, s)| s.is_visible() && !s.attr.priority())
			{
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
						set(self, line as usize, dot as usize, col);
					}
				}
			}
		}

		let mut texture = self.output_texture.lock().unwrap();
		std::mem::swap(&mut self.current_texture, &mut texture);
	}

	fn update_tile(
		&mut self,
		_: impl Iterator<Item = Option<emu_core::ppu::NesColour>>,
		_: usize,
		_: usize,
		_: usize,
	) {
	}

	fn update_sprite(
		&mut self,
		_: impl Iterator<Item = Option<emu_core::ppu::NesColour>>,
		_: usize,
	) {
	}
}

fn draw_horizontal_gradient(
	canvas: &mut Canvas<Window>,
	x_start: i32,
	width: i32,
	win_h: u32,
	start_val: f32,
	end_val: f32,
) -> Result<(), String> {
	let steps = 64;
	for i in 0..steps {
		let t0 = i as f32 / steps as f32;
		let t1 = (i + 1) as f32 / steps as f32;
		let val0 = start_val + (end_val - start_val) * t0;
		let val1 = start_val + (end_val - start_val) * t1;
		let rect_width = ((val1 - val0).abs().max(1.0) * width as f32
			/ (end_val - start_val).abs().max(1.0))
		.round() as u32;
		let x = x_start + (width as f32 * t0).round() as i32;
		canvas.set_draw_color(Color::RGB(val0 as u8, val0 as u8, val0 as u8));
		canvas.fill_rect(Rect::new(x, 0, rect_width, win_h))?;
	}
	Ok(())
}

fn draw_vertical_gradient(
	canvas: &mut Canvas<Window>,
	y_start: i32,
	height: i32,
	win_w: u32,
	start_val: f32,
	end_val: f32,
) -> Result<(), String> {
	let steps = 64;
	for i in 0..steps {
		let t0 = i as f32 / steps as f32;
		let t1 = (i + 1) as f32 / steps as f32;
		let val0 = start_val + (end_val - start_val) * t0;
		let val1 = start_val + (end_val - start_val) * t1;
		let rect_height = ((val1 - val0).abs().max(1.0) * height as f32
			/ (end_val - start_val).abs().max(1.0))
		.round() as u32;
		let y = y_start + (height as f32 * t0).round() as i32;
		canvas.set_draw_color(Color::RGB(val0 as u8, val0 as u8, val0 as u8));
		canvas.fill_rect(Rect::new(0, y, win_w, rect_height))?;
	}
	Ok(())
}

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

		let (win_w, win_h) = canvas.window().size();

		let left_width = dst.x();
		let right_width = win_w as i32 - (dst.x() + dst.width() as i32);
		let top_height = dst.y();
		let bottom_height = win_h as i32 - (dst.y() + dst.height() as i32);

		draw_horizontal_gradient(&mut canvas, 0, left_width, win_h, 64., 0.)?;
		draw_horizontal_gradient(
			&mut canvas,
			dst.x() + dst.width() as i32,
			right_width,
			win_h,
			0.,
			64.,
		)?;
		draw_vertical_gradient(&mut canvas, 0, top_height, win_w, 64., 0.)?;
		draw_vertical_gradient(
			&mut canvas,
			dst.y() + dst.height() as i32,
			bottom_height,
			win_w,
			0.,
			64.,
		)?;

		canvas.copy(&texture, None, Some(dst))?;
		canvas.present();
	}

	Ok(())
}
