use std::{
	collections::{BTreeSet, VecDeque},
	io::{Seek, SeekFrom, Write},
};

use anyhow::{Result, bail};
use emu_core::{graphics, inst::Inst, interpret::State, nes_file::Mapper};
use tempfile::NamedTempFile;

#[derive(Debug)]
struct Block {
	starting_at: u16,
	insts: Vec<Inst>,
	then: u16,
	otherwise: Option<u16>,
}

fn to_h<T: Write>(Block { starting_at, .. }: &Block, out: &mut T) -> Result<()> {
	writeln!(out, "void bb_{starting_at:04X}(State *state);")?;
	Ok(())
}

fn to_c<T: Write>(
	Block {
		starting_at,
		insts,
		then,
		otherwise,
	}: &Block,
	out: &mut T,
) -> Result<()> {
	writeln!(out, "void bb_{starting_at:04X}(State *state) {{")?;

	for inst in insts.iter().take(insts.len().saturating_sub(1)) {
		write!(out, "\t{}", inst.instruction_representation())?;
	}

	fn branch<T: Write>(out: &mut T, cond: &str, then: u16, other: u16) -> Result<()> {
		writeln!(out, "\tif ({cond}) {{")?;
		writeln!(out, "\t\t[[clang::musttail]] return bb_{then:04X}(state);")?;
		writeln!(out, "\t}} else {{")?;
		writeln!(out, "\t\t[[clang::musttail]] return bb_{other:04X}(state);")?;
		writeln!(out, "\t}}")?;
		Ok(())
	}

	match (insts.last().unwrap(), otherwise) {
		(Inst::Bcc(_), Some(other)) => branch(out, "state->cpu.p.C == 0", *then, *other)?,
		(Inst::Bcs(_), Some(other)) => branch(out, "state->cpu.p.C != 0", *then, *other)?,
		(Inst::Bpl(_), Some(other)) => branch(out, "state->cpu.p.N == 0", *then, *other)?,
		(Inst::Bmi(_), Some(other)) => branch(out, "state->cpu.p.N != 0", *then, *other)?,
		(Inst::Bne(_), Some(other)) => branch(out, "state->cpu.p.Z == 0", *then, *other)?,
		(Inst::Beq(_), Some(other)) => branch(out, "state->cpu.p.Z != 0", *then, *other)?,
		(Inst::Bvc(_), Some(other)) => branch(out, "state->cpu.p.V == 0", *then, *other)?,
		(Inst::Bvs(_), Some(other)) => branch(out, "state->cpu.p.V != 0", *then, *other)?,
		(inst @ Inst::Jsr(_), Some(_)) => {
			writeln!(out, "\t{}", inst.instruction_representation())?;
			writeln!(out, "\t[[clang::musttail]] return bb_{then:04X}(state);")?;
		}
		(
			Inst::Stp
			| Inst::Stp2
			| Inst::Stp3
			| Inst::Stp4
			| Inst::Stp5
			| Inst::Stp6
			| Inst::Stp7
			| Inst::Stp8
			| Inst::Stp9
			| Inst::Stp10
			| Inst::Stp11
			| Inst::Stp12,
			None,
		) => writeln!(out, "\t[[clang::musttail]] return return_stp(state);")?,
		(inst @ (Inst::JmpAbsolute(_) | Inst::JmpIndirect(_) | Inst::Rti | Inst::Rts), None) => {
			write!(out, "\t{}", inst.instruction_representation())?;
			writeln!(out, "\t[[clang::musttail]] return jump_table(state);")?
		}
		e => bail!("Invalid bb ({e:?})"),
	}

	write!(out, "}}\n\n")?;

	Ok(())
}

fn c_helpers<T: Write>(blocks: &[Block], out: &mut T) -> Result<()> {
	writeln!(out, "void jump_table(State *state) {{")?;
	writeln!(out, "\tswitch (state->cpu.pc) {{")?;
	for &Block { starting_at, .. } in blocks.iter() {
		writeln!(
			out,
			"\tcase 0x{starting_at:04X}: [[clang::musttail]] return bb_{starting_at:04X}(state);"
		)?;
	}
	writeln!(out, "\tdefault: {{")?;
	writeln!(out, "\t\tprintf(\"Unknown block %X\\n\", state->cpu.pc);")?;
	writeln!(out, "\t\texit(-1);")?;
	writeln!(out, "\t}}")?;
	writeln!(out, "\t}}")?;
	writeln!(out, "}}\n")?;

	writeln!(out, "void return_stp(State *state) {{")?;
	writeln!(out, "\t\tprintf(\"Unimplemented\\n\");")?;
	writeln!(out, "\t\texit(-1);")?;
	writeln!(out, "}}\n")?;

	Ok(())
}

fn find_blocks(rom: Box<Mapper>) -> Vec<Block> {
	let mut blocks = Vec::new();
	let mut queue = VecDeque::new();
	queue.push_back(u16::from_le_bytes([
		rom.get_cpu(0xFFFC).expect("Cannot read reset vector"),
		rom.get_cpu(0xFFFD).expect("Cannot read reset vector (2)"),
	]));
	queue.push_back(u16::from_le_bytes([
		rom.get_cpu(0xFFFA).expect("Cannot read interrupt vector"),
		rom.get_cpu(0xFFFB)
			.expect("Cannot read interrupt vector (2)"),
	]));
	let mut visited = BTreeSet::new();

	let mut system_state = State::new(rom, graphics::new_bitmap());
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
			| Inst::Beq(offset)
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
			Inst::JmpIndirect(_) => {
				// let then = u16::from_le_bytes([
				//	system_state.rom.get_cpu(adr.as_u16()).ok_or(anyhow!(
				//		"Cannot read indirect jump (is it in RAM? 0x{adr:04X})"
				//	))?,
				//	system_state.rom.get_cpu(adr.as_u16() + 1).ok_or(anyhow!(
				//		"Cannot read indirect jump (2) (is it in RAM? 0x{adr:04X})"
				//	))?,
				// ]);
				// block.then = then;
				// queue.push_back(then);
			}
			Inst::Jsr(fn_adr) => {
				let then = fn_adr.as_u16();
				let otherwise = system_state.cpu.pc.wrapping_add(inst.len() as _);
				block.then = then;
				block.otherwise = Some(otherwise);
				queue.push_back(then);
				queue.push_back(otherwise);
			}
			Inst::Rti => {}
			Inst::Rts => {}
			Inst::Stp | Inst::Stp2 => {}
			e => panic!("Not a valid basic block end: {e:X?}\n{block:#?}"),
		}

		blocks.push(block);
	}

	blocks.sort_unstable_by_key(|b| b.starting_at);

	blocks
}

fn write_to_c(blocks: &[Block]) -> Result<NamedTempFile> {
	let mut tmpfile = NamedTempFile::new()?;

	writeln!(&mut tmpfile, "#include \"evaluate_instruction.c\"")?;
	writeln!(&mut tmpfile, "#include <stdio.h>")?;
	writeln!(&mut tmpfile, "#include <stdlib.h>")?;
	writeln!(&mut tmpfile)?;

	writeln!(&mut tmpfile, "void jump_table(State *state);")?;
	writeln!(&mut tmpfile, "void return_stp(State *state);")?;

	writeln!(&mut tmpfile)?;

	for block in blocks.iter() {
		to_h(block, &mut tmpfile)?;
	}
	writeln!(&mut tmpfile)?;

	for block in blocks.iter() {
		to_c(block, &mut tmpfile)?;
	}

	c_helpers(blocks, &mut tmpfile)?;

	tmpfile.seek(SeekFrom::Start(0))?;

	Ok(tmpfile)
}

fn main() -> Result<()> {
	let path = std::env::args().nth(1).unwrap_or_else(|| {
		concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/../non-free/SMB1.nes" // "/../non-free/AccuracyCoin.nes"
		)
		.into()
	});
	let buffer = std::fs::read(path)?;
	let rom = Mapper::parse_ines(buffer)?;
	assert!(
		matches!(&*rom, Mapper::NROM256 { .. }),
		"Blocks are currently identified exclusively by address"
	);

	let blocks = find_blocks(rom);
	let c = write_to_c(&blocks)?;

	let out_dir = tempfile::tempdir()?;

	let cc_output = std::process::Command::new("clang")
		.args([
			"-x", "c", "-std=c23", "-c", "-Og", "-g3", "-Wall", "-Wextra",
		])
		.arg(c.path())
		.arg("-I")
		.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../emu-core/inc"))
		.arg("-I")
		.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../emu-core/src"))
		.arg("-working-directory")
		.arg(out_dir.path())
		.output()?;

	println!("{}", String::from_utf8(cc_output.stdout)?);
	eprintln!("{}", String::from_utf8(cc_output.stderr)?);

	Ok(())
}
