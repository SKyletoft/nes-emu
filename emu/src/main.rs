mod drawing;

use std::sync::{
	Arc, Mutex,
	atomic::{AtomicBool, AtomicU8, Ordering},
};

use emu_core::{graphics::Bitmap, interpret::State, nrom256::NROM256};

#[cfg(feature = "precompiled")]
#[link(name = "mario", kind = "static")]
unsafe extern "C" {
	pub fn nes_game(state: &mut State<NROM256>) -> c_int;
}

fn emulation_loop(
	shared_texture: Arc<Mutex<Box<Bitmap>>>,
	controller_state: &AtomicU8,
	kill: &AtomicBool,
) {
	// let path = std::env::args().nth(1).unwrap_or_else(|| {
	//	concat!(
	//		env!("CARGO_MANIFEST_DIR"),
	//		"/../non-free/SMB1.nes" // "/../non-free/AccuracyCoin.nes"
	//	)
	//	.into()
	// });
	// dbg!(&path);
	// let buffer = std::fs::read(path).unwrap();

	let buffer = *include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../non-free/SMB1.nes"));
	let game = NROM256::parse_ines(&buffer).unwrap();
	let mut system_state = State::new(game, shared_texture);

	// let mut buf = String::new();
	while kill.load(Ordering::Relaxed) {
		*system_state.controller1.state_mut() = controller_state.load(Ordering::SeqCst);

		#[cfg(not(feature = "precompiled"))]
		system_state.next();

		#[cfg(feature = "precompiled")]
		unsafe { nes_game(&mut system_state); }

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
