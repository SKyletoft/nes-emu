mod cpu;
mod evaluate_instruction;
mod inst;
mod interpret;
mod nes_file;
mod ppu;
mod drawing;

use interpret::State;
use nes_file::Mapper;

fn display(state: &State) {
	let cpu::Cpu {
		a, x, y, s, p, pc, ..
	} = state.cpu;

	let c = p.c();
	let z = p.z();
	let i = p.i();
	let d = p.d();
	let b = p.b();
	let v = p.v();
	let n = p.n();

	let inst = state.next_inst();

	println!("┌─CPU──────────────────────────┐");
	println!("│ A:{a:02X} X:{x:02X} Y:{y:02X} SP:{s:02X} pc:{pc:04X} │");
	println!("│ C:{c} Z:{z} I:{i} D:{d} B:{b} V:{v} N:{n}  │");
	println!("└──────────────────────────────┘");
	println!("Next: {inst:X?}");
}

fn main() {
	// let path = std::env::args()
	//	.nth(1)
	//	.unwrap_or_else(|| "../non-free/SMB3.nes".into());
	// dbg!(&path);
	// let buffer = std::fs::read(path).unwrap();
	// let game = Mapper::parse_ines(buffer).unwrap();
	// let mut system_state = State::new(game);

	// let mut buf = String::new();
	// loop {
	//	system_state.next();

	//	display(&system_state);
	//	buf.clear();
	//	std::io::stdin().read_line(&mut buf).unwrap();
	// }

	drawing::main();
}

#[cfg(test)]
mod test {
	use crate::{
		cpu::P,
		inst::{Inst, LDA, STA},
		interpret::State,
		nes_file::Mapper,
	};

	#[test]
	fn smb3_first_few() {
		let buffer = std::fs::read("non-free/SMB3.nes").unwrap();
		let game = Mapper::parse_ines(buffer).unwrap();
		let mut state = State::new(game);
		assert_eq!(state.next_inst(), Inst::SEI);
		assert_eq!(state.cpu.pc, 0xFF40);
		state.next();
		assert_eq!(state.next_inst(), Inst::CLD);
		state.next();
		assert_eq!(state.next_inst(), Inst::LDA(LDA::Immediate(0)));
		assert_eq!(state.cpu.a, 0);
		assert!(!state.cpu.p.contains(P::D)); // A bit late for some reason
		state.next();
		assert_eq!(state.next_inst(), Inst::STA(STA::Absolute(0x2001)));
		state.next();
		assert_eq!(state.next_inst(), Inst::LDA(LDA::Immediate(8)));
		state.next();
		assert_eq!(state.next_inst(), Inst::STA(STA::Absolute(0x2000)));
		assert_eq!(state.cpu.a, 8);
		assert_eq!(state.cpu.pc, 0xFF49);
		assert_eq!(state.cpu.s, 0xFD);
	}

	#[test]
	fn smb3_first_jsr() {
		let buffer = std::fs::read("non-free/SMB3.nes").unwrap();
		let game = Mapper::parse_ines(buffer).unwrap();
		let mut state = State::new(game);
		for _ in 0..7 {
			state.next();
		}
		// Wait for PPU to init...
		for i in 0..(25559 / 2) {
			assert_eq!(state.cpu.pc, 0xFF4E);
			assert_eq!(state.next_inst(), Inst::LDA(LDA::Absolute(0x2002)));
			state.next();
			assert_eq!(state.cpu.pc, 0xFF51);
			assert_eq!(state.next_inst(), Inst::BPL(-5), "Loop: {i}");
			state.next();
		}
		assert_eq!(state.cpu.pc, 0xFF53);
		assert_eq!(state.next_inst(), Inst::DEX);
	}
}
