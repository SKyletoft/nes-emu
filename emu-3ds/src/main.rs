use std::{
	sync::{
		Arc, Mutex,
		atomic::{AtomicBool, AtomicU8, Ordering},
	},
	u8,
};

use ctru::{
	prelude::*,
	services::{
		gfx::{Screen, Swap},
		gspgpu::FramebufferFormat,
	},
};
use emu_core::{
	controller::ControllerState,
	graphics::{Bitmap, Colour},
	interpret::State,
	nes_file::Mapper,
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
		screen[(400 - 256) / 2 + y][239 - x] = Bgr8 { blue, green, red };
	}

	top_screen.swap_buffers();
}

fn main() {
	let apt = Apt::new().unwrap();
	let mut hid = Hid::new().unwrap();
	let gfx = Gfx::new().unwrap();
	let _console = Console::new(gfx.bottom_screen.borrow_mut());

	let shared_texture = emu_core::graphics::new_bitmap();
	let controller_state = AtomicU8::new(0);
	let kill_predicate = AtomicBool::new(true);

	let buffer = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../non-free/SMB1.nes"));
	let game = Mapper::parse_ines(buffer).unwrap();
	let mut system_state = State::new(game, shared_texture);

	let mut last_frame = u64::MAX;
	let mut last_line = i16::MAX;
	while apt.main_loop() {
		system_state.next();
		let frame = system_state.ppu.frame;
		if last_frame != frame {
			update_screen(&gfx, system_state.current_texture.as_ref());
			last_frame = frame;
			gfx.wait_for_vblank();
		}

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
