mod drawing;

use std::{
	collections::BTreeSet,
	sync::{
		Arc, Mutex,
		atomic::{AtomicBool, AtomicU8, Ordering},
	},
	time::{Duration, Instant},
};

use emu_core::{graphics::Bitmap, interpret::State};

fn emulation_loop(
	shared_texture: Arc<Mutex<Box<Bitmap>>>,
	controller_state: &AtomicU8,
	kill: &AtomicBool,
) {
	let game = Box::new(game::MAPPER.clone());
	let mut system_state = State::new(game, shared_texture);
	// let mut visited = BTreeSet::new();

	let mut frame_last = 0;
	let mut last_time = Instant::now();

	while kill.load(Ordering::Relaxed) {
		*system_state.rest.controller1.state_mut() = controller_state.load(Ordering::SeqCst);

		// let broke =
		game::nes_game(&mut system_state);
		// if broke != 0 {
		//	if visited.insert(broke) {
		//		println!("0x{broke:04X}");
		//	}
		//	while !system_state.next_inst_pure().ends_bb() {
		//		system_state.next();
		//	}
		//	system_state.next();
		// }

		if system_state.rest.ppu_runahead > 600 {
			continue;
		}
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

fn main() {
	let shared_texture = emu_core::graphics::new_bitmap();
	let controller_state = AtomicU8::new(0);
	let kill_predicate = AtomicBool::new(true);

	let texture_ptr = shared_texture.clone();
	std::thread::scope(|s| {
		s.spawn(|| emulation_loop(texture_ptr, &controller_state, &kill_predicate));
		drawing::sdl_thread(shared_texture, &controller_state).unwrap();
		kill_predicate.store(false, Ordering::SeqCst);
	});
}
