mod cpu;
mod drawing;
mod evaluate_instruction;
mod inst;
mod interpret;
mod nes_file;
mod ppu;

use std::sync::{Arc, Mutex};

use drawing::Bitmap;
use interpret::State;
use nes_file::Mapper;

fn display(state: &State) {
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

	println!("┌─CPU──────────────────────────┐");
	println!("│ A:{a:02X} X:{x:02X} Y:{y:02X} SP:{s:02X} pc:{pc:04X} │");
	println!("│ C:{c} Z:{z} I:{i} D:{d} B:{b} V:{v} N:{n}  │");
	println!("├─PPU──────────────────────────┤");
	println!("│ line:{line:03} dot:{dot:03} frame: {frame:04} │");
	println!("└──────────────────────────────┘");
	println!("Next: {inst:X?}");
	println!();
}

fn emulation_loop(shared_texture: Arc<Mutex<Bitmap>>) {
	let path = std::env::args()
		.nth(1)
		.unwrap_or_else(|| "../non-free/SMB1.nes".into());
	dbg!(&path);
	let buffer = std::fs::read(path).unwrap();
	let game = Mapper::parse_ines(buffer).unwrap();
	let mut system_state = State::new(game, shared_texture);

	let mut buf = String::new();
	loop {
		system_state.next();

		display(&system_state);
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

#[cfg(test)]
mod test {
	use crate::{cpu::P, drawing, inst::Inst, interpret::State, nes_file::Mapper};

	#[test]
	fn smb3_first_few() {
		let buffer = std::fs::read("non-free/SMB3.nes").unwrap();
		let game = Mapper::parse_ines(buffer).unwrap();
		let mut state = State::new(game, drawing::new_bitmap());
		assert_eq!(state.next_inst(), Inst::Sei);
		assert_eq!(state.cpu.pc, 0xFF40);
		state.next();
		assert_eq!(state.next_inst(), Inst::Cld);
		state.next();
		assert_eq!(state.next_inst(), Inst::LdaImmediate(0));
		assert_eq!(state.cpu.a, 0);
		assert!(!state.cpu.p.d()); // A bit late for some reason
		state.next();
		assert_eq!(state.next_inst(), Inst::StaAbsolute(0x2001u16.into()));
		state.next();
		assert_eq!(state.next_inst(), Inst::LdaImmediate(8));
		state.next();
		assert_eq!(state.next_inst(), Inst::StaAbsolute(0x2000u16.into()));
		assert_eq!(state.cpu.a, 8);
		assert_eq!(state.cpu.pc, 0xFF49);
		assert_eq!(state.cpu.s, 0xFD);
	}

	#[test]
	fn smb3_first_jsr() {
		let buffer = std::fs::read("non-free/SMB3.nes").unwrap();
		let game = Mapper::parse_ines(buffer).unwrap();
		let mut state = State::new(game, drawing::new_bitmap());
		for _ in 0..7 {
			state.next();
		}
		// Wait for PPU to init...
		for i in 0..(25559 / 2) {
			assert_eq!(state.cpu.pc, 0xFF4E);
			assert_eq!(state.next_inst(), Inst::LdaAbsolute(0x2002u16.into()));
			state.next();
			assert_eq!(state.cpu.pc, 0xFF51);
			assert_eq!(state.next_inst(), Inst::Bpl(-5), "Loop: {i}");
			state.next();
		}
		assert_eq!(state.cpu.pc, 0xFF53);
		assert_eq!(state.next_inst(), Inst::Dex);
	}
}
