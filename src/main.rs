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

fn display(state: &State) -> String {
	use std::fmt::Write;

	let cpu::Cpu {
		a, x, y, s, p, pc, ..
	} = state.cpu;

	let c = p.c() as u8;
	let z = p.z() as u8;
	let i = p.i() as u8;
	let d = p.d() as u8;
	let b = p.b() as u8;
	let v = p.v() as u8;
	let n = p.n() as u8;

	let inst = state.next_inst();

	let line = state.ppu.scanline;
	let dot = state.ppu.dot;
	let frame = state.ppu.frame % 10000;

	let mut out = String::new();

	writeln!(&mut out, "┌─CPU──────────────────────────┐").unwrap();
	writeln!(
		&mut out,
		"│ A:{a:02X} X:{x:02X} Y:{y:02X} SP:{s:02X} pc:{pc:04X} │"
	)
	.unwrap();
	writeln!(&mut out, "│ C:{c} Z:{z} I:{i} D:{d} B:{b} V:{v} N:{n}  │").unwrap();
	writeln!(&mut out, "├─PPU──────────────────────────┤").unwrap();
	writeln!(
		&mut out,
		"│ line:{line:03} dot:{dot:03} frame: {frame:04} │"
	)
	.unwrap();
	writeln!(&mut out, "└──────────────────────────────┘").unwrap();
	writeln!(&mut out, "Next: {inst:X?}").unwrap();
	writeln!(&mut out).unwrap();

	out
}

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

		print!("{}", display(&system_state));
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
