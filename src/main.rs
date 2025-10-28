mod apu;
mod controller;
mod cpu;
mod drawing;
mod evaluate_instruction;
mod inst;
mod interpret;
mod nes_file;
mod ppu;
mod u15;

#[cfg(test)]
mod tests;

use std::sync::{
	Arc, Mutex,
	atomic::{AtomicBool, AtomicU8, Ordering},
};

use drawing::Bitmap;
use interpret::State;
use nes_file::Mapper;

fn emulation_loop(
	shared_texture: Arc<Mutex<Box<Bitmap>>>,
	controller_state: &AtomicU8,
	kill: &AtomicBool,
) {
	let path = std::env::args()
		.nth(1)
		.unwrap_or_else(|| concat!(env!("CARGO_MANIFEST_DIR"), "/non-free/SMB1.nes").into());
	dbg!(&path);
	let buffer = std::fs::read(path).unwrap();
	let game = Mapper::parse_ines(buffer).unwrap();
	let mut system_state = State::new(game, shared_texture);

	// let mut buf = String::new();
	while kill.load(Ordering::Relaxed) {
		*system_state.controller1.state_mut() = controller_state.load(Ordering::SeqCst);
		system_state.next();
		// print!("{}", system_state.display());
		// buf.clear();
		// std::io::stdin().read_line(&mut buf).unwrap();
	}
}

fn main() {
	let shared_texture = drawing::new_bitmap();
	let controller_state = AtomicU8::new(0);
	let kill_predicate = AtomicBool::new(true);

	let texture_ptr = shared_texture.clone();
	std::thread::scope(|s| {
		s.spawn(|| emulation_loop(texture_ptr, &controller_state, &kill_predicate));
		drawing::sdl_thread(shared_texture, &controller_state).unwrap();
		kill_predicate.store(false, Ordering::SeqCst);
	});
}
