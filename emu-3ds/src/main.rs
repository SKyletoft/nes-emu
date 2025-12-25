use std::{hint::assert_unchecked, time::Instant};

use ctru::{
	prelude::*,
	services::gfx::{Screen, Swap},
};
use emu_core::{
	controller::ControllerState,
	graphics::{Bitmap, Colour},
	interpret::State,
	nrom256::NROM256,
};

#[repr(C)]
struct Bgr8 {
	blue: u8,
	green: u8,
	red: u8,
}

fn update_screen(gfx: &Gfx, nes_screen: &Bitmap) {
	let mut top_screen = gfx.top_screen.borrow_mut();
	let frame_buf = top_screen.raw_framebuffer();
	let screen = unsafe { std::mem::transmute::<_, &mut [[Bgr8; 240]; 400]>(&mut *frame_buf.ptr) };

	for (
		x,
		y,
		Colour {
			blue, green, red, ..
		},
	) in nes_screen
		.iter()
		.enumerate()
		.flat_map(|(x, line)| line.iter().enumerate().map(move |(y, px)| (x, y, *px)))
	{
		let x = 239 - x;
		let y = (400 - 256) / 2 + y;
		debug_assert!(y < 400 && x < 240);
		unsafe { assert_unchecked(y < 400 && x < 240) };
		screen[y][x] = Bgr8 { blue, green, red };
	}

	top_screen.swap_buffers();
}

fn main() {
	let apt = Apt::new().unwrap();
	let mut hid = Hid::new().unwrap();
	let gfx = Gfx::new().unwrap();
	let _console = Console::new(gfx.bottom_screen.borrow_mut());

	let shared_texture = emu_core::graphics::new_bitmap();

	let buffer = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../non-free/SMB1.nes"));
	let game = NROM256::parse_ines(buffer).unwrap();
	let mut system_state = State::new(game, shared_texture);

	let mut last_frame = u64::MAX;

	let mut frame_timing = Instant::now();

	while apt.main_loop() {
		let res = macro_expanded::nes_game(&mut system_state);
		if res != 0 {
			system_state.next();
		}

		let frame = system_state.ppu.frame;
		if last_frame == frame {
			continue;
		}

		last_frame = frame;
		update_screen(&gfx, system_state.current_texture.as_ref());
		let now = Instant::now();
		println!("{frame:5}: {:?}", now - frame_timing);
		frame_timing = now;
		// gfx.wait_for_vblank();

		hid.scan_input();

		let mut c = ControllerState::new();
		c.set_a(hid.keys_held().contains(KeyPad::A));
		c.set_b(hid.keys_held().contains(KeyPad::B));
		c.set_start(hid.keys_held().contains(KeyPad::START));
		c.set_select(hid.keys_held().contains(KeyPad::SELECT));
		c.set_up(hid.keys_held().contains(KeyPad::DPAD_UP));
		c.set_down(hid.keys_held().contains(KeyPad::DPAD_DOWN));
		c.set_left(hid.keys_held().contains(KeyPad::DPAD_LEFT));
		c.set_right(hid.keys_held().contains(KeyPad::DPAD_RIGHT));
		*system_state.controller1.state_mut() = c.into_bits();

		if hid.keys_down().contains(KeyPad::SELECT) {
			break;
		}
	}
}
