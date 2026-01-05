use std::{
	cell::RefMut,
	time::{Duration, Instant},
};

use ctru::{
	prelude::*,
	services::gfx::{Screen, Swap, TopScreen},
};
use emu_core::{
	controller::ControllerState,
	frame::NesFramebuffer,
	graphics::Colour,
	interpret::State,
	ppu::NesColour,
	unsafe_assert,
};

#[repr(C)]
struct Bgr8 {
	blue: u8,
	green: u8,
	red: u8,
}

struct ConsoleFramebuffer<'a> {
	screen: RefMut<'a, TopScreen>,
	/// Must be updated when screen is swapped
	unsafe_raw_frame_buf: &'a mut [[Bgr8; 240]; 400],
}

impl<'a> ConsoleFramebuffer<'a> {
	fn new(mut screen: RefMut<'a, TopScreen>) -> Self {
		let frame_buf = screen.raw_framebuffer();
		let unsafe_raw_frame_buf =
			unsafe { std::mem::transmute::<_, &mut [[Bgr8; 240]; 400]>(&mut *frame_buf.ptr) };
		ConsoleFramebuffer {
			screen,
			unsafe_raw_frame_buf,
		}
	}
}

impl<'a> NesFramebuffer for ConsoleFramebuffer<'a> {
	#[inline]
	fn set(&mut self, x: usize, y: usize, col: NesColour) {
		let Colour {
			blue, green, red, ..
		} = col.into();
		unsafe { unsafe_assert!(y < 400 && x < 240) };
		let x = 239 - x;
		let y = (400 - 256) / 2 + y;
		self.unsafe_raw_frame_buf[y][x] = Bgr8 { blue, green, red };
	}

	#[inline]
	fn swap(&mut self) {
		self.screen.swap_buffers();
		let frame_buf = self.screen.raw_framebuffer();
		let unsafe_raw_frame_buf =
			unsafe { std::mem::transmute::<_, &mut [[Bgr8; 240]; 400]>(&mut *frame_buf.ptr) };
		self.unsafe_raw_frame_buf = unsafe_raw_frame_buf;
	}
}

fn main() {
	let apt = Apt::new().unwrap();
	let mut hid = Hid::new().unwrap();
	let gfx = Gfx::new().unwrap();
	let _console = Console::new(gfx.bottom_screen.borrow_mut());

	let game = Box::new(game::MAPPER.clone());
	let mut system_state = State::new(game, ConsoleFramebuffer::new(gfx.top_screen.borrow_mut()));

	let mut last_frame = 0;

	let mut cpu_dur = Duration::new(0, 0);
	let mut ppu_dur = Duration::new(0, 0);
	while apt.main_loop() {
		let before = Instant::now();
		while system_state.rest.ppu_runahead <= 341 {
			game::nes_game(&mut system_state);
		}
		let after = Instant::now();
		cpu_dur += after - before;

		let before_ppu = Instant::now();
		system_state.catch_up_ppu();
		let after_ppu = Instant::now();
		ppu_dur += after_ppu - before_ppu;

		let frame = system_state.rest.ppu.frame;
		if last_frame == frame {
			continue;
		}
		last_frame = frame;

		let frame_count = system_state.rest.ppu.frame;

		let cpu_time = cpu_dur.as_millis();
		let ppu_time = ppu_dur.as_millis();
		println!("{frame_count:5}: {cpu_time:3}ms {ppu_time:3}ms");
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
