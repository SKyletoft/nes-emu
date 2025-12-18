use std::{
	collections::VecDeque,
	io::{Write, Read, Seek, SeekFrom},
};

use anyhow::{Result, anyhow, bail};
use emu_core::{inst::Inst, nes_file::Mapper};
use tempfile::NamedTempFile;

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
	writeln!(&mut tmpfile, "\tstatic uint16_t history[10] = {{}};")?;
	writeln!(&mut tmpfile, "\tstatic size_t history_index = 0;")?;
	writeln!(&mut tmpfile)?;
	writeln!(&mut tmpfile, "\thistory[history_index] = state->cpu.pc;")?;
	writeln!(&mut tmpfile, "\thistory_index = (history_index + 1) % 10;")?;
	writeln!(&mut tmpfile)?;
	writeln!(&mut tmpfile, "\tswitch (state->cpu.pc) {{")?;
	// for ((_, starting_point, inst, _), (_, next, _, _)) in sorted_instructions
	//	.iter()
	//	.zip(sorted_instructions.iter().skip(1))
	// {
	//	write!(
	//		&mut tmpfile,
	//		"\tcase 0x{starting_point:04X}: b{starting_point:04X}: {}",
	//		inst.instruction_representation(),
	//	)?;
	//	if inst.ends_bb() {
	//		writeln!(&mut tmpfile, "\t\tbreak;")?;
	//	} else if starting_point + inst.len() as u16 != *next {
	//		// All loops must reasonably often go through the switch
	//		// to communicate with the input and graphics systems
	//		assert_ne!(inst.len(), 0);
	//		writeln!(
	//			&mut tmpfile,
	//			"\t\tgoto b{:04X};",
	//			starting_point + inst.len() as u16
	//		)?;
	//	}
	// }
	for (is_start, pc, inst, end) in sorted_instructions {
		match is_start {
			IsStart::Yes => write!(
				&mut tmpfile,
				"\tcase 0x{pc:04X}: b{pc:04X}: {}",
				inst.instruction_representation(),
			)?,
			IsStart::No => write!(
				&mut tmpfile,
				"\t                    {}",
				inst.instruction_representation()
			)?,
		};
		match end {
			End::Goto => writeln!(&mut tmpfile, "\t\tgoto b{:04X};", pc + inst.len() as u16)?,
			End::Break => writeln!(&mut tmpfile, "\t\tbreak;")?,
			End::None => {}
		}
	}
	writeln!(&mut tmpfile, "\tdefault: bFFFE: bFFFF:")?;
	writeln!(
		&mut tmpfile,
		"\t\tprintf(\"ERROR: Hit default case (0x%X)\\n\", state->cpu.pc);"
	)?;
	writeln!(
		&mut tmpfile,
		"\t\tprintf(\n\t\t\t\"%X %X %X %X %X\\n\",\n\t\t\thistory[(history_index + 9) % 10],\n\t\t\thistory[(history_index + 8) % 10],\n\t\t\thistory[(history_index + 7) % 10],\n\t\t\thistory[(history_index + 6) % 10],\n\t\t\thistory[(history_index + 5) % 10],\n\t\t\thistory[(history_index + 4) % 10]\n\t\t);"
	)?;
	writeln!(&mut tmpfile, "\t\texit(-1);")?;
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

	let mut c = write_to_switch(&rom)?;
	c.disable_cleanup(true);

	let mut buf = String::new();
	c.read_to_string(&mut buf)?;
	println!("{buf}");

	let cc_output = std::process::Command::new("arm-none-eabi-gcc")
		.args([
			"-x",
			"c",
			"-std=c23",
			"-c",
			"-O3",
			"-g0",
			"-Wall",
			"-Wextra",
			"-Wno-unused-label",
			"-Wno-implicit-fallthrough",
			// "-Werror=conversion",

			"-mfloat-abi=hard",
			"-mtune=mpcore",
			"-mtp=soft",
			"-march=armv6k",
		])
		.arg(c.path())
		.arg("-I")
		.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../emu-core/inc"))
		.arg("-I")
		.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../emu-core/src"))
		.args(["-o", "mario.o"])
		// .args(["-D", "STATIC_INLINE=[[clang::always_inline]] static inline"])
		// .args(["-D", "STATIC_INLINE=[[clang::noinline]]"])
		.output()?;
	println!("{}", String::from_utf8(cc_output.stdout)?);
	eprintln!("{}", String::from_utf8(cc_output.stderr)?);

	if !cc_output.status.success() {
		bail!("Clang failed");
	}

	let ar_output = std::process::Command::new("arm-none-eabi-ar")
		.args(["rcs", "libmario.a", "mario.o"])
		.output()?;
	println!("{}", String::from_utf8(ar_output.stdout)?);
	eprintln!("{}", String::from_utf8(ar_output.stderr)?);

	if !ar_output.status.success() {
		bail!("Ar failed");
	}

	Ok(())
}
