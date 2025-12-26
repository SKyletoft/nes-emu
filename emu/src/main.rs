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
	let game = Box::new(macro_expanded::MAPPER.clone());
	let mut system_state = State::new(game, shared_texture);
	let mut visited = BTreeSet::new();

	let mut frame_last = 0;
	let mut ppu_last = 0;

	while kill.load(Ordering::Relaxed) {
		*system_state.controller1.state_mut() = controller_state.load(Ordering::SeqCst);

		let broke = macro_expanded::nes_game(&mut system_state);
		if broke != 0 {
			if visited.insert(broke) {
				println!("0x{broke:04X}");
			}
			while !system_state.next_inst_pure().ends_bb() {
				system_state.next();
			}
			system_state.next();
		}

		let ppu_now = system_state.ppu_runahead;
		if ppu_last + 1000 >= ppu_now {
			continue;
		}
		println!("{ppu_last} -> {ppu_now}");
		ppu_last = ppu_now;
		system_state.catch_up_ppu();

		let frame_now = system_state.ppu.frame;
		if frame_now == frame_last {
			continue;
		}
		frame_last = frame_now;

		static LAST_TIME: Mutex<Option<Instant>> = Mutex::new(None);
		let mut last_time = LAST_TIME.lock().unwrap();
		let to_sleep = match &mut *last_time {
			None => Duration::from_millis(16),
			Some(last_time) => {
				let now = Instant::now();
				(Duration::from_millis(1000) / 60).saturating_sub(now - *last_time)
			}
		};
		println!("{to_sleep:?} sleep! {}", system_state.ppu.frame);
		std::thread::sleep(to_sleep);
		*last_time = Some(Instant::now());
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
