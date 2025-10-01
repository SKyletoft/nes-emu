#[cfg(test)]
fn fceux_log(state: &State) -> String {
	let cpu::Cpu {
		a, x, y, s, p, pc, ..
	} = state.cpu;
	let inst = state.next_inst();

	// fceux flag order: N V - B D I Z C
	let n = if p.n() { 'N' } else { 'n' };
	let v = if p.v() { 'V' } else { 'v' };
	let u = 'u'; // always unused/reserved
	let b = if p.b() { 'B' } else { 'b' };
	let d = if p.d() { 'D' } else { 'd' };
	let i = if p.i() { 'I' } else { 'i' };
	let z = if p.z() { 'Z' } else { 'z' };
	let c = if p.c() { 'C' } else { 'c' };

	let byte_str = {
		let mut s = String::new();
		for offset in 0..inst.len() {
			let mem = state.mem_pure(state.cpu.pc + offset as u16);
			write!(&mut s, "{mem:02X} ").unwrap();
		}
		s.pop(); // Remove trailing space
		s
	};

	let mut out = format!(
		"A:{:02X} X:{:02X} Y:{:02X} S:{:02X} {}   ${:04X}:  {:<8}",
		a,
		x,
		y,
		s,
		format!("{n}{v}{u}{b}{d}{i}{z}{c}"),
		pc,
		byte_str,
	);

	// Add instruction
	let mut instruction_str = String::new();
	print_instruction(inst, &mut instruction_str).unwrap();
	out.push_str(&instruction_str);

	out
}

#[cfg(test)]
mod test {
	use crate::{drawing, inst::Inst, interpret::State, nes_file::Mapper};

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

	#[test]
	fn fceux_log_1() {
		use std::fs::File;
		use std::io::{BufRead, BufReader};

		let buffer = std::fs::read("non-free/SMB1.nes").unwrap();
		let game = Mapper::parse_ines(buffer).unwrap();
		let mut state = State::new(game, drawing::new_bitmap());
		let file = File::open("reference-logs/SMB1.log").unwrap();
		let reader = BufReader::new(file);

		for (i, line) in reader.lines().enumerate() {
			let line = line.unwrap();
			let ours = crate::fceux_log(&state);
			// Normalize whitespace for comparison
			let normalized_ours = ours.trim_end();
			let normalized_line = line.trim_end();
			
			assert_eq!(
				normalized_ours, 
				normalized_line,
				"Mismatch at line {i}\n ours: {ours}\n ref : {line}"
			);
			state.next();
		}
	}
}
