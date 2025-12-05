use std::{
	collections::{BTreeSet, VecDeque},
	fmt::Write,
};

use anyhow::{Result, bail};
use emu_core::{graphics, inst::Inst, interpret::State, nes_file::Mapper};

#[derive(Debug)]
struct Block {
	starting_at: u16,
	insts: Vec<Inst>,
	then: u16,
	otherwise: Option<u16>,
}

fn to_c(
	Block {
		starting_at,
		insts,
		then,
		otherwise,
	}: &Block,
	out: &mut String,
) -> Result<()> {
	writeln!(out, "void bb_{starting_at:04X}(State *state) {{")?;

	for inst in insts.iter().take(insts.len().saturating_sub(1)) {
		write!(out, "\t{}", inst.instruction_representation())?;
	}

	match (insts.last().unwrap(), otherwise) {
		(Inst::Bcc(_), Some(other)) => {
			writeln!(out, "\tif (state->cpu.p.c == 0) {{")?;
			writeln!(out, "\t\t[[clang::musttail]] return bb_{then:04X}(state);")?;
			writeln!(out, "\t}} else {{")?;
			writeln!(out, "\t\t[[clang::musttail]] return bb_{other:04X}(state);")?;
			writeln!(out, "\t}}")?;
		}
		(Inst::Bcs(_), Some(other)) => {
			writeln!(out, "\tif (state->cpu.p.c != 0) {{")?;
			writeln!(out, "\t\t[[clang::musttail]] return bb_{then:04X}(state);")?;
			writeln!(out, "\t}} else {{")?;
			writeln!(out, "\t\t[[clang::musttail]] return bb_{other:04X}(state);")?;
			writeln!(out, "\t}}")?;
		}
		(Inst::Bmi(_), Some(other)) => {
			writeln!(out, "\tif (state->cpu.p.n != 0) {{")?;
			writeln!(out, "\t\t[[clang::musttail]] return bb_{then:04X}(state);")?;
			writeln!(out, "\t}} else {{")?;
			writeln!(out, "\t\t[[clang::musttail]] return bb_{other:04X}(state);")?;
			writeln!(out, "\t}}")?;
		}
		(Inst::Bpl(_), Some(other)) => {
			writeln!(out, "\tif (state->cpu.p.n == 0) {{")?;
			writeln!(out, "\t\t[[clang::musttail]] return bb_{then:04X}(state);")?;
			writeln!(out, "\t}} else {{")?;
			writeln!(out, "\t\t[[clang::musttail]] return bb_{other:04X}(state);")?;
			writeln!(out, "\t}}")?;
		}
		(Inst::Bne(_), Some(other)) => {}
		(Inst::JmpAbsolute(_), None) => {}
		(Inst::Jsr(_), None) => {}
		(Inst::Rti, None) => {}
		(Inst::Rts, None) => {}
		(Inst::Stp, None) => {}
		_ => bail!("Invalid bb"),
	}

	write!(out, "}}\n\n")?;

	Ok(())
}

fn main() -> Result<()> {
	let path = std::env::args().nth(1).unwrap_or_else(|| {
		concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/../non-free/SMB1.nes" // "/../non-free/AccuracyCoin.nes"
		)
		.into()
	});
	let buffer = std::fs::read(path).unwrap();
	let rom = Mapper::parse_ines(buffer).unwrap();
	assert!(
		matches!(&*rom, Mapper::NROM256 { .. }),
		"Blocks are currently identified exclusively by address"
	);
	let mut system_state = State::new(rom, graphics::new_bitmap());

	let mut blocks = Vec::new();
	let mut queue = VecDeque::new();
	queue.push_back(system_state.cpu.pc);
	let mut visited = BTreeSet::new();

	while let Some(adr) = queue.pop_front() {
		if visited.contains(&adr) || adr < 0x4020 {
			continue;
		}
		visited.insert(adr);
		system_state.cpu.pc = adr;

		let mut block = Block {
			starting_at: adr,
			insts: Vec::new(),
			then: 0,
			otherwise: None,
		};

		while let inst = system_state.next_inst_pure()
			&& !inst.ends_bb()
		{
			block.insts.push(inst);
			system_state.cpu.pc = system_state.cpu.pc.wrapping_add(inst.len() as _);
		}

		let inst = system_state.next_inst_pure();
		block.insts.push(inst);
		match inst {
			Inst::Bcc(offset)
			| Inst::Bcs(offset)
			| Inst::Bmi(offset)
			| Inst::Bne(offset)
			| Inst::Bpl(offset) => {
				let then = system_state.cpu.pc.wrapping_add(offset as u16);
				let otherwise = system_state.cpu.pc.wrapping_add(inst.len() as _);
				queue.push_back(otherwise);
				queue.push_back(then);
				block.then = then;
				block.otherwise = Some(otherwise);
			}
			Inst::JmpAbsolute(adr) => {
				let then = adr.as_u16();
				block.then = then;
				queue.push_back(then);
			}
			Inst::Jsr(fn_adr) => {
				queue.push_back(fn_adr.as_u16());
			}
			Inst::Rti => {}
			Inst::Rts => {}
			Inst::Stp => {}
			e => panic!("Not a valid basic block end: {e:X?}\n{block:#?}"),
		}

		blocks.push(block);
	}

	blocks.sort_unstable_by_key(|b| b.starting_at);

	let mut c_code = String::new();
	writeln!(&mut c_code, "#include \"evaluate_instruction.c\"\n")?;

	for block in blocks.into_iter() {
		to_c(&block, &mut c_code)?;
	}

	println!("{c_code}");

	Ok(())
}
