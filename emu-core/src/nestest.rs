use std::{
	fs::File,
	io::{BufRead, BufReader},
	str::FromStr,
};

use crate::{interpret::State, mapper::Mapper, nrom::NROM128};

const AUTOMATION_ENTRY_POINT: u16 = 0xC000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NestestCpuState {
	a: u8,
	x: u8,
	y: u8,
	p: u8,
	sp: u8,
	cyc: usize,
}

fn parse_prefixed_hex_u8(line: &str, prefix: &str) -> Result<u8, String> {
	let start = line
		.find(prefix)
		.ok_or_else(|| format!("Missing field {prefix} in nestest line: {line}"))?;
	let value = &line[start + prefix.len()..];
	let value = value
		.get(..2)
		.ok_or_else(|| format!("Missing value for {prefix} in nestest line: {line}"))?;
	u8::from_str_radix(value, 16)
		.map_err(|_| format!("Invalid hex value for {prefix}: {value} in line: {line}"))
}

fn parse_prefixed_usize(line: &str, prefix: &str) -> Result<usize, String> {
	let start = line
		.find(prefix)
		.ok_or_else(|| format!("Missing field {prefix} in nestest line: {line}"))?;
	let value = &line[start + prefix.len()..];
	let value = value
		.split_whitespace()
		.next()
		.ok_or_else(|| format!("Missing numeric value after {prefix} in nestest line: {line}"))?;
	value
		.parse::<usize>()
		.map_err(|_| format!("Invalid numeric value for {prefix}: {value} in line: {line}"))
}

impl FromStr for NestestCpuState {
	type Err = String;

	fn from_str(line: &str) -> Result<Self, Self::Err> {
		Ok(Self {
			a: parse_prefixed_hex_u8(line, "A:")?,
			x: parse_prefixed_hex_u8(line, "X:")?,
			y: parse_prefixed_hex_u8(line, "Y:")?,
			p: parse_prefixed_hex_u8(line, "P:")?,
			sp: parse_prefixed_hex_u8(line, "SP:")?,
			cyc: parse_prefixed_usize(line, "CYC:")?,
		})
	}
}

impl<M: Mapper> From<&State<M>> for NestestCpuState {
	fn from(state: &State<M>) -> Self {
		Self {
			a: state.cpu.a,
			x: state.cpu.x,
			y: state.cpu.y,
			p: state.cpu.p.into_bits(),
			sp: state.cpu.s,
			cyc: state.rest.cycles,
		}
	}
}

#[test]
fn nestest_matches_reference_log_cpu_state() {
	let rom_bytes = std::fs::read(concat!(
		env!("CARGO_MANIFEST_DIR"),
		"/../non-free/nestest.nes"
	))
	.unwrap();
	let rom = NROM128::parse_ines(&rom_bytes).unwrap();
	let mut state = State::new(rom);
	state.cpu.pc = AUTOMATION_ENTRY_POINT;
	state.rest.cycles = 7;

	let reader = BufReader::new(
		File::open(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/../reference-logs/nestest.log"
		))
		.unwrap(),
	);

	for (idx, line) in reader.lines().enumerate() {
		let line_number = idx + 1;
		let line = line.unwrap();
		let expected = line.parse::<NestestCpuState>().unwrap_or_else(|error| {
			panic!("Failed to parse reference-logs/nestest.log:{line_number}: {error}")
		});
		let actual = NestestCpuState::from(&state);

		assert_eq!(
			actual,
			expected,
			"Mismatch at reference-logs/nestest.log:{line_number}\nexpected: {expected:?}\nactual  : {actual:?}\nraw line: {line}\n{}",
			state.display(),
		);

		state.next();
	}
}
