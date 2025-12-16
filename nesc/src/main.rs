#![allow(dead_code)]

use std::{
	collections::{BTreeSet, VecDeque},
	io::{Read, Seek, SeekFrom, Write},
};

use anyhow::{Result, anyhow, bail};
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

	for inst in insts.iter() {
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
		(Inst::Jsr(_), Some(_)) => {
			writeln!(out, "\t[[clang::musttail]] return bb_{then:04X}(state);")?
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
		(Inst::JmpAbsolute(_) | Inst::JmpIndirect(_) | Inst::Rti | Inst::Rts, None) => {
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

	writeln!(out, "void return_stp([[maybe_unused]] State *state) {{")?;
	writeln!(
		out,
		"\tprintf(\"Unimplemented\\n%d %X\\n\", state->cpu.pc, state->cpu.pc);"
	)?;
	writeln!(out, "\texit(-1);")?;
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
			system_state.next();
		}

		let inst = system_state.next_inst_pure();
		block.insts.push(inst);
		match inst {
			Inst::Bcc(_)
			| Inst::Bcs(_)
			| Inst::Bmi(_)
			| Inst::Bne(_)
			| Inst::Beq(_)
			| Inst::Bpl(_) => {
				let pc = system_state.cpu.pc;
				system_state.cpu.p.set_bits(0);
				system_state.next();
				let then = system_state.cpu.pc;
				system_state.cpu.pc = pc;
				system_state.cpu.p.set_bits(0b1111_1111);
				system_state.next();
				let otherwise = system_state.cpu.pc;

				queue.push_back(otherwise);
				queue.push_back(then);
				block.then = then;
				block.otherwise = Some(otherwise);
			}
			Inst::JmpAbsolute(_) => {
				system_state.next();
				queue.push_back(system_state.cpu.pc);
			}
			Inst::JmpIndirect(_) => {
				// Todo: Figure out how to deal with this. This should always be an escape hatch for ACE
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
			| Inst::Stp12 => {}
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

fn write_to_switch(rom: &Mapper) -> Result<NamedTempFile> {
	let mut tmpfile = NamedTempFile::new()?;

	#[derive(Debug)]
	enum End {
		Goto,
		Break,
		None,
	}

	#[derive(Debug)]
	enum IsStart {
		Yes,
		No,
	}

	let sorted_instructions: Vec<(IsStart, u16, Inst, End)> = {
		let mut instructions: VecDeque<(u16, Inst)> = (0x8000..=0xFFFD)
			.map(|i| {
				let inst: Inst = [
					rom.get_cpu(i).ok_or_else(|| anyhow!("Can't read rom"))?,
					rom.get_cpu(i + 1)
						.ok_or_else(|| anyhow!("Can't read rom"))?,
					rom.get_cpu(i + 2)
						.ok_or_else(|| anyhow!("Can't read rom"))?,
				]
				.into();
				Ok((i, inst))
			})
			.collect::<Result<_>>()?;
		let mut sorted = Vec::new();

		while let Some((idx, inst)) = instructions.pop_front() {
			let mut next = idx + inst.len() as u16;
			sorted.push((IsStart::No, idx, inst, End::None));
			if inst.ends_bb() {
				continue;
			}
			while let Ok(j) = instructions.binary_search_by_key(&next, |(x, _)| *x) {
				let (idx, inst) = instructions
					.remove(j)
					.expect("Literally just binary searched for it");
				next = idx + inst.len() as u16;
				sorted.push((IsStart::No, idx, inst, End::None));
				if inst.ends_bb() {
					break;
				}
			}
		}

		for i in 0..(sorted.len() - 1) {
			let next = sorted[i].1 + sorted[i].2.len() as u16;
			if sorted[i].2.ends_bb() {
				sorted[i].3 = End::Break;
				sorted[i + 1].0 = IsStart::Yes;
				match sorted[i].2 {
					Inst::Bcc(y)
					| Inst::Bcs(y)
					| Inst::Beq(y)
					| Inst::Bmi(y)
					| Inst::Bne(y)
					| Inst::Bpl(y)
					| Inst::Bvc(y)
					| Inst::Bvs(y) => {
						let next = sorted[i]
							.1
							.wrapping_add(sorted[i].2.len() as u16)
							.wrapping_add(y as i16 as u16);
						let Some(j) = sorted.iter().position(|(_, x, _, _)| *x == next) else {
							continue;
						};
						sorted[j].0 = IsStart::Yes;
					}
					Inst::JmpAbsolute(adr) | Inst::Jsr(adr) => {
						let Some(j) = sorted.iter().position(|(_, x, _, _)| *x == adr.as_u16())
						else {
							continue;
						};
						sorted[j].0 = IsStart::Yes;
					}
					Inst::JmpIndirect(_) => {}
					Inst::Brk => {}
					Inst::Rti => {}
					Inst::Rts => {}
					Inst::Stp => {}
					Inst::Stp10 => {}
					Inst::Stp11 => {}
					Inst::Stp12 => {}
					Inst::Stp2 => {}
					Inst::Stp3 => {}
					Inst::Stp4 => {}
					Inst::Stp5 => {}
					Inst::Stp6 => {}
					Inst::Stp7 => {}
					Inst::Stp8 => {}
					Inst::Stp9 => {}
					_ => panic!(),
				}
			} else if next != sorted[i + 1].1 {
				sorted[i].3 = End::Goto;
				sorted[i + 1].0 = IsStart::Yes;
				let Some(j) = sorted.iter().position(|(_, x, _, _)| *x == next) else {
					continue;
				};
				sorted[j].0 = IsStart::Yes;
			}
		}

		let reset = sorted
			.iter()
			.position(|(_, x, _, _)| {
				*x == u16::from_le_bytes([
					rom.get_cpu(0xFFFC).expect("Cannot read reset vector"),
					rom.get_cpu(0xFFFD).expect("Cannot read reset vector (2)"),
				])
			})
			.ok_or_else(|| anyhow!("Can't find reset vector"))?;
		let interrupt = sorted
			.iter()
			.position(|(_, x, _, _)| {
				*x == u16::from_le_bytes([
					rom.get_cpu(0xFFFA).expect("Cannot read interrupt vector"),
					rom.get_cpu(0xFFFB)
						.expect("Cannot read interrupt vector (2)"),
				])
			})
			.ok_or_else(|| anyhow!("Can't find interrupt vector"))?;
		sorted[reset].0 = IsStart::Yes;
		sorted[interrupt].0 = IsStart::Yes;

		sorted
	};

	writeln!(&mut tmpfile, "#include \"evaluate_instruction.c\"")?;
	writeln!(&mut tmpfile, "#include <stdio.h>")?;
	writeln!(&mut tmpfile, "#include <stdlib.h>")?;
	writeln!(&mut tmpfile)?;

	writeln!(&mut tmpfile, "void nes_game(State *state) {{")?;
	writeln!(&mut tmpfile, "\tswitch (state->cpu.pc) {{")?;
	for ((_, starting_point, inst, _), (_, next, _, _)) in sorted_instructions
		.iter()
		.zip(sorted_instructions.iter().skip(1))
	{
		write!(
			&mut tmpfile,
			"\tcase 0x{starting_point:04X}: b{starting_point:04X}: {}",
			inst.instruction_representation(),
		)?;
		if inst.ends_bb() {
			writeln!(&mut tmpfile, "\t\tbreak;")?;
		} else if starting_point + inst.len() as u16 != *next {
			// All loops must reasonably often go through the switch
			// to communicate with the input and graphics systems
			assert_ne!(inst.len(), 0);
			writeln!(
				&mut tmpfile,
				"\t\tgoto b{:04X};",
				starting_point + inst.len() as u16
			)?;
		}
	}
	writeln!(&mut tmpfile, "\tdefault: bFFFE: bFFFF:")?;
	writeln!(&mut tmpfile, "\t}}")?;
	writeln!(&mut tmpfile, "}}")?;

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
	let rom = Mapper::parse_ines(&buffer)?;
	assert!(
		matches!(&*rom, Mapper::NROM256 { .. }),
		"Blocks are currently identified exclusively by address"
	);

	// let blocks = find_blocks(rom);
	// let mut c = write_to_c(&blocks)?;
	// c.disable_cleanup(true);

	let mut c = write_to_switch(&rom)?;
	c.disable_cleanup(true);

	let mut buf = String::new();
	c.read_to_string(&mut buf)?;
	println!("{buf}");

	let cc_output = std::process::Command::new("clang")
		.args([
			"-x",
			"c",
			"-std=c23",
			"-c",
			"-Oz",
			"-g0",
			"-Wall",
			"-Wextra",
			"-Wno-unused-label",
			"-Werror=conversion",
		])
		.arg(c.path())
		.arg("-I")
		.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../emu-core/inc"))
		.arg("-I")
		.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../emu-core/src"))
		.args(["-o", "mario.o"])
		// .args(["-D", "STATIC_INLINE=[[clang::always_inline]] static inline"])
		.args(["-D", "STATIC_INLINE=[[clang::noinline]]"])
		.output()?;
	println!("{}", String::from_utf8(cc_output.stdout)?);
	eprintln!("{}", String::from_utf8(cc_output.stderr)?);

	if !cc_output.status.success() {
		bail!("Clang failed");
	}

	let ar_output = std::process::Command::new("ar")
		.args(["rcs", "libmario.a", "mario.o"])
		.output()?;
	println!("{}", String::from_utf8(ar_output.stdout)?);
	eprintln!("{}", String::from_utf8(ar_output.stderr)?);

	if !ar_output.status.success() {
		bail!("Ar failed");
	}

	Ok(())
}
