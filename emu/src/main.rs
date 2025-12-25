mod drawing;

use std::{
	collections::BTreeSet,
	sync::{
		Arc, Mutex,
		atomic::{AtomicBool, AtomicU8, Ordering},
	},
	time::{Duration, Instant},
};

use emu_core::{graphics::Bitmap, interpret::State, nrom256::NROM256};

fn emulation_loop(
	shared_texture: Arc<Mutex<Box<Bitmap>>>,
	controller_state: &AtomicU8,
	kill: &AtomicBool,
) {

	let buffer = *include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../non-free/SMB1.nes"));
	let game = NROM256::parse_ines(&buffer).unwrap();
	let mut system_state = State::new(game, shared_texture);
	let mut visited = BTreeSet::new();

	let mut last_frame = 0;

	// let mut buf = String::new();
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

		if system_state.ppu.frame != last_frame {
			static LAST_TIME: Mutex<Option<Instant>> = Mutex::new(None);
			let mut last_time = LAST_TIME.lock().unwrap();
			let to_sleep = match &mut *last_time {
				None => Duration::from_millis(16),
				Some(last_time) => {
					let now = Instant::now();
					(Duration::from_millis(1000) / 60).saturating_sub(now - *last_time)
				}
			};
			std::thread::sleep(to_sleep);
			*last_time = Some(Instant::now());

			last_frame = system_state.ppu.frame;
		}

		// print!("{}", system_state.display());
		// buf.clear();
		// std::io::stdin().read_line(&mut buf).unwrap();
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
