mod apu;
mod controller;
mod cpu;
mod drawing;
mod evaluate_instruction;
mod inst;
mod interpret;
mod nes_file;
mod ppu;

#[cfg(test)]
mod tests;

use std::sync::{Arc, Mutex};

use drawing::Bitmap;
use interpret::State;
use nes_file::Mapper;

fn emulation_loop(shared_texture: Arc<Mutex<Bitmap>>) {
	let path = std::env::args()
		.nth(1)
		.unwrap_or_else(|| "../non-free/SMB1.nes".into());
	dbg!(&path);
	let buffer = std::fs::read(path).unwrap();
	let game = Mapper::parse_ines(buffer).unwrap();
	let mut system_state = State::new(game, shared_texture);

	// let mut buf = String::new();
	loop {
		system_state.next();
		print!("{}", system_state.display());
		// buf.clear();
		// std::io::stdin().read_line(&mut buf).unwrap();
	}
}

fn main() {
	let shared_texture = drawing::new_bitmap();

	let texture_ptr = shared_texture.clone();
	let _emulation = std::thread::spawn(|| emulation_loop(texture_ptr));
	drawing::sdl_thread(shared_texture).unwrap();

	_emulation.join().unwrap();
}
