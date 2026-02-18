use std::{
	sync::{
		Arc, Mutex,
		atomic::{AtomicBool, AtomicU8, Ordering},
	},
	time::{Duration, Instant},
};

use emu_core::{
	graphics::{self, Bitmap},
	interpret::State,
};

use crate::sdl_framebuffer;

fn emulation_loop(
	shared_texture: Arc<Mutex<Box<Bitmap>>>,
	controller_state: &AtomicU8,
	kill: &AtomicBool,
) {
	// let game = Box::new(game::MAPPER.clone());
	let game =
		emu_core::nrom256::NROM256::parse_ines(include_bytes!("../../non-free/SMB1.nes")).unwrap();
	let mut system_state = State::new(
		game,
		sdl_framebuffer::SdlFramebuffer {
			output_texture: shared_texture,
			current_texture: Box::new(graphics::empty_bitmap()),
		},
	);

	let mut frame_last = 0;
	let mut last_time = Instant::now();

	while kill.load(Ordering::Relaxed) {
		*system_state.rest.controller1.state_mut() = controller_state.load(Ordering::SeqCst);

		// game::nes_game(&mut system_state);
		system_state.next();

		system_state.catch_up_ppu();

		let frame_now = system_state.rest.ppu.frame;
		if frame_now == frame_last {
			continue;
		}
		frame_last = frame_now;

		let now = Instant::now();
		std::thread::sleep((Duration::from_millis(1000) / 60).saturating_sub(now - last_time));
		last_time = Instant::now();
	}
}

pub fn main() {
	let shared_texture = emu_core::graphics::new_bitmap();
	let controller_state = AtomicU8::new(0);
	let kill_predicate = AtomicBool::new(true);

	let texture_ptr = shared_texture.clone();
	std::thread::scope(|s| {
		s.spawn(|| emulation_loop(texture_ptr, &controller_state, &kill_predicate));
		sdl_framebuffer::sdl_thread(shared_texture, &controller_state).unwrap();
		kill_predicate.store(false, Ordering::SeqCst);
	});
}
