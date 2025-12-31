use std::{
	hint::assert_unchecked,
	time::{Duration, Instant},
};

use ctru::{
	prelude::*,
	services::gfx::{Screen, Swap},
};
use emu_core::{
	controller::ControllerState,
	graphics::{Bitmap, Colour},
	interpret::State,
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

	let game = Box::new(game::MAPPER.clone());
	let mut system_state = State::new(game, shared_texture);

	let mut last_frame = 0;

	let mut frame_timing = Instant::now();

	let mut ppu_catchup_cycle = 0;

	let mut cpu_dur = Duration::new(0, 0);
	let mut ppu_dur = Duration::new(0, 0);
	while apt.main_loop() {
		let before = Instant::now();
		game::nes_game(&mut system_state);
		let after = Instant::now();
		cpu_dur += after - before;

		ppu_catchup_cycle += 1;
		if ppu_catchup_cycle == 10 {
			let before_ppu = Instant::now();
			system_state.catch_up_ppu();
			let after_ppu = Instant::now();
			ppu_dur += after_ppu - before_ppu;
			ppu_catchup_cycle = 0;
		}
		let after_graphics = Instant::now();

		let frame = system_state.rest.ppu.frame;
		if last_frame == frame {
			continue;
		}
		last_frame = frame;

		let before_copy = Instant::now();
		update_screen(&gfx, system_state.rest.current_texture.as_ref());
		let after_copy = Instant::now();
		let frame_count = system_state.rest.ppu.frame;

		let cpu_time = cpu_dur.as_millis();
		let ppu_time = ppu_dur.as_millis();
		let copy_time = (after_copy - before_copy).as_millis();
		println!("{frame_count:5}: {cpu_time:3}ms {ppu_time:3}ms {copy_time:3}ms");
		cpu_dur = Duration::new(0, 0);
		ppu_dur = Duration::new(0, 0);
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
		*system_state.rest.controller1.state_mut() = c.into_bits();

		if hid.keys_down().contains(KeyPad::SELECT) {
			break;
		}
	}
}
