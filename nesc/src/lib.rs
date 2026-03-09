use std::{
	collections::{BTreeSet, VecDeque},
	fs,
	path::PathBuf,
};

use emu_core::{inst::Inst, mapper::Mapper, nrom128::NROM128, nrom256::NROM256};
use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[allow(clippy::large_enum_variant)]
enum Mappers {
	NROM128(NROM128),
	NROM256(NROM256),
}

impl Mapper for Mappers {
	type Framebuffer = emu_core::frame::NoFramebuffer;

	fn framebuffer(&mut self) -> &mut Self::Framebuffer {
		match self {
			Mappers::NROM128(x) => x.framebuffer(),
			Mappers::NROM256(x) => x.framebuffer(),
		}
	}

	fn get_cpu(&self, adr: u16) -> Option<u8> {
		match self {
			Mappers::NROM128(x) => x.get_cpu(adr),
			Mappers::NROM256(x) => x.get_cpu(adr),
		}
	}

	fn set_cpu(&mut self, adr: u16, val: u8) -> Option<()> {
		match self {
			Mappers::NROM128(x) => x.set_cpu(adr, val),
			Mappers::NROM256(x) => x.set_cpu(adr, val),
		}
	}

	fn get_ppu(&self, adr: u16, ppu: &emu_core::ppu::Ppu) -> Option<u8> {
		match self {
			Mappers::NROM128(x) => x.get_ppu(adr, ppu),
			Mappers::NROM256(x) => x.get_ppu(adr, ppu),
		}
	}

	fn set_ppu(&mut self, adr: u16, ppu: &mut emu_core::ppu::Ppu, val: u8) -> Option<()> {
		match self {
			Mappers::NROM128(x) => x.set_ppu(adr, ppu, val),
			Mappers::NROM256(x) => x.set_ppu(adr, ppu, val),
		}
	}

	fn get_palette_index(&self, half: bool, tile: u8, y: u8, x: u8) -> u8 {
		match self {
			Mappers::NROM128(m) => m.get_palette_index(half, tile, y, x),
			Mappers::NROM256(m) => m.get_palette_index(half, tile, y, x),
		}
	}
}

#[proc_macro]
pub fn compile_nes_to_rust(input: TokenStream) -> TokenStream {
	let path_lit = parse_macro_input!(input as LitStr);
	let rel_path = path_lit.value();

	let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
	let path = PathBuf::from(manifest_dir).join(rel_path);

	let buffer = fs::read(&path)
		.unwrap_or_else(|e| panic!("failed to read ROM '{}': {}", path.display(), e));

	let (rom, constants) = parse_ines(&buffer);

	let mut fns = Vec::new();
	let mut branches = Vec::new();
	let mut starting_points = BTreeSet::new();

	let sorted_instructions = collect_starting_points(&rom);

	for func in sorted_instructions.chunk_by(|_, (is_start, ..)| *is_start != IsStart::Yes) {
		assert!(
			func.iter()
				.skip(1)
				.all(|(is_start, ..)| *is_start == IsStart::No)
		);
		assert!(
			func.iter()
				.take(func.len() - 1)
				.all(|(_, _, _, end)| *end == End::Continue)
		);
		let Some((_, pc, ..)) = func.first() else {
			panic!()
		};

		let insts = func
			.iter()
			.map(|(_, _, i, _)| {
				let call = i
					.instruction_representation()
					.parse::<proc_macro2::TokenStream>()
					.unwrap();
				quote! { state = #call }
			})
			.collect::<Vec<_>>();

		let ident = syn::Ident::new(&format!("b_{pc:04x}"), proc_macro2::Span::call_site());

		starting_points.insert(pc);
		fns.push(quote! {
			fn #ident<M: emu_core::mapper::Mapper>(mut state: State<M>) -> State<M> {
				#(#insts)*
				state
			}
		});
	}

	for i in 0x8000..=0xFFFF {
		let id = syn::Ident::new("id", proc_macro2::Span::call_site());
		if starting_points.contains(&i) {
			let ident = syn::Ident::new(&format!("b_{i:04x}"), proc_macro2::Span::call_site());
			branches.push(quote! { #i => #ident, });
		} else {
			branches.push(quote! { #i => #id, });
		}
	}
	quote! {
		#[allow(unused_imports)]
		use emu_core::{evaluate_instruction::*, interpret::State, nrom128::NROM128, nrom256::NROM256};

		#(#fns)*

		fn id<T>(x: T) -> T { x }

		fn b_ffff<M: emu_core::mapper::Mapper>(state: State<M>) -> State<M> { state }

		pub fn nes_game<M: emu_core::mapper::Mapper>(state: &mut State<M>) {
			unsafe {
				let mut local: State<M> = (&raw mut *state).read();
				(&raw mut *state).write(
					match local.cpu.pc {
						0..0x8000 => id,
						#(#branches)*
					}(local)
				)
			}
		}

		#constants
	}
	.into()
}

fn parse_ines(buffer: &[u8]) -> (Mappers, proc_macro2::TokenStream) {
	let [b'N', b'E', b'S', 0x1A, prg_size, _, flags_6, flags_7, ..] = &buffer[0..16] else {
		panic!("Invalid file");
	};

	let trainer_present = flags_6 & (1 << 2) != 0;
	assert!(!trainer_present); // Not really, but please error early when I hit a game with one.
	let mapper_type = (*flags_7 & 0xF0) | *flags_6 >> 4;
	match mapper_type {
		0 if *prg_size == 1 => {
			let parsed_file = NROM128::parse_ines(buffer).unwrap();
			let lit1 = Literal::byte_string(&parsed_file.prg_ram);
			let lit2 = Literal::byte_string(&parsed_file.prg_rom);
			let lit3 = Literal::byte_string(&parsed_file.chr_rom);
			let lit4 = Literal::byte_string(unsafe {
				std::mem::transmute::<&[[[[u8; 8]; 8]; 256]; 2], &[u8; 32768]>(
					&parsed_file.parsed_graphics,
				)
			});
			let mapper_literal = quote! {
				pub const MAPPER: NROM128 = NROM128 {
					prg_ram: *#lit1,
					prg_rom: *#lit2,
					chr_rom: *#lit3,
					parsed_graphics: unsafe {
						std::mem::transmute::<[u8; 32768], [[[[u8; 8]; 8]; 256]; 2]>(*#lit4)
					},
				};
			};
			(Mappers::NROM128(parsed_file), mapper_literal)
		}
		0 if *prg_size == 2 => {
			let parsed_file = NROM256::parse_ines(buffer).unwrap();
			let lit1 = Literal::byte_string(&parsed_file.prg_ram);
			let lit2 = Literal::byte_string(parsed_file.prg_rom);
			let lit3 = Literal::byte_string(parsed_file.chr_rom);
			let lit4 = Literal::byte_string(unsafe {
				std::mem::transmute::<&[[[[u8; 8]; 8]; 256]; 2], &[u8; 32768]>(
					parsed_file.parsed_graphics,
				)
			});
			let mapper_literal = quote! {
				pub const MAPPER: NROM256 = NROM256 {
					framebuffer: emu_core::frame::NoFramebuffer,
					prg_ram: *#lit1,
					prg_rom: &*#lit2,
					chr_rom: &*#lit3,
					parsed_graphics: &unsafe {
						std::mem::transmute::<[u8; 32768], [[[[u8; 8]; 8]; 256]; 2]>(*#lit4)
					},
					hitbox_background: [[[false;_];_];_],
					hitbox_sprite_0: [false; _],
				};
			};
			(Mappers::NROM256(parsed_file), mapper_literal)
		}
		x => panic!("Unsupported Mapper: {x} ({prg_size})"),
	}
}

#[derive(Debug, PartialEq)]
enum End {
	Goto(u16),
	Break,
	Continue,
}

#[derive(Debug, PartialEq)]
enum IsStart {
	Yes,
	No,
}

fn collect_starting_points(rom: &Mappers) -> Vec<(IsStart, u16, Inst, End)> {
	let mut instructions: VecDeque<(u16, Inst)> = (0x8000..=0xFFFD)
		.map(|i| {
			let inst: Inst = [
				rom.get_cpu(i).unwrap(),
				rom.get_cpu(i + 1).unwrap(),
				rom.get_cpu(i + 2).unwrap(),
			]
			.into();
			(i, inst)
		})
		.collect();
	let mut sorted = Vec::new();

	while let Some((idx, inst)) = instructions.pop_front() {
		let mut next = idx + inst.size() as u16;
		sorted.push((IsStart::No, idx, inst, End::Continue));
		if inst.ends_bb() {
			continue;
		}
		while let Ok(j) = instructions.binary_search_by_key(&next, |(x, _)| *x) {
			let (idx, inst) = instructions
				.remove(j)
				.expect("Literally just binary searched for it");
			next = idx + inst.size() as u16;
			sorted.push((IsStart::No, idx, inst, End::Continue));
			if inst.ends_bb() {
				break;
			}
		}
	}

	for i in 0..(sorted.len() - 1) {
		let next = sorted[i].1 + sorted[i].2.size() as u16;
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
						.wrapping_add(sorted[i].2.size() as u16)
						.wrapping_add(y as i16 as u16);
					let Some(j) = sorted.iter().position(|(_, x, ..)| *x == next) else {
						continue;
					};
					sorted[j].0 = IsStart::Yes;
					let adr = sorted[j].1;
					if let Some((_, _, _, e @ End::Continue)) = sorted.get_mut(j.wrapping_sub(1)) {
						*e = End::Goto(adr);
					}
				}
				Inst::JmpAbsolute(adr) | Inst::Jsr(adr) => {
					let Some(j) = sorted.iter().position(|(_, x, ..)| *x == adr.as_u16()) else {
						continue;
					};
					sorted[j].0 = IsStart::Yes;
					let adr = sorted[j].1;
					if let Some((_, _, _, e @ End::Continue)) = sorted.get_mut(j.wrapping_sub(1)) {
						*e = End::Goto(adr);
					}
				}
				Inst::JmpIndirect(_) => {}
				Inst::Brk => {}
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
				_ => panic!(),
			}
		} else if next != sorted[i + 1].1 {
			sorted[i].3 = End::Goto(next);
			sorted[i + 1].0 = IsStart::Yes;
			let Some(j) = sorted.iter().position(|(_, x, ..)| *x == next) else {
				continue;
			};
			sorted[j].0 = IsStart::Yes;
		}
	}

	let hidden_labels = [
		u16::from_le_bytes([
			rom.get_cpu(0xFFFC).expect("Cannot read reset vector"),
			rom.get_cpu(0xFFFD).expect("Cannot read reset vector (2)"),
		]),
		u16::from_le_bytes([
			rom.get_cpu(0xFFFA).expect("Cannot read reset vector"),
			rom.get_cpu(0xFFFB).expect("Cannot read reset vector (2)"),
		]),
		0x80AE,
		0x80ED,
		0x80F0,
		0x814D,
		0x818D,
		0x81AB,
		0x8211,
		0x8231,
		0x8245,
		0x825C,
		0x8267,
		0x8299,
		0x82E9,
		0x8335,
		0x850D,
		0x853F,
		0x858B,
		0x85B0,
		0x85BF,
		0x85E3,
		0x8643,
		0x8652,
		0x8657,
		0x865A,
		0x8693,
		0x86A8,
		0x86B6,
		0x86C5,
		0x86CA,
		0x86DD,
		0x86F1,
		0x86FF,
		0x8704,
		0x8827,
		0x8898,
		0x8943,
		0x895A,
		0x89EE,
		0x8A64,
		0x8A7D,
		0x8A82,
		0x8E41,
		0x8E53,
		0x8F0B,
		0x8F86,
		0x8FDC,
		0x8FE4,
		0x8FF3,
		0x9040,
		0x9061,
		0x907D,
		0x90E3,
		0x9131,
		0x9178,
		0x917D,
		0x91CD,
		0x9215,
		0x9218,
		0x9224,
		0x9237,
		0x924F,
		0x9267,
		0x92E5,
		0x940E,
		0x94C2,
		0x94DC,
		0x9503,
		0x951A,
		0x9539,
		0x9558,
		0x9568,
		0x95A5,
		0x95B1,
		0x95C7,
		0x95E6,
		0x95FE,
		0x9630,
		0x96C5,
		0x96F2,
		0x96F9,
		0x9708,
		0x970D,
		0x973A,
		0x9756,
		0x9806,
		0x983F,
		0x984E,
		0x986F,
		0x9882,
		0x9898,
		0x98C6,
		0x98E5,
		0x991C,
		0x996B,
		0x9979,
		0x997C,
		0x997F,
		0x99B4,
		0x99D0,
		0x99F2,
		0x9A19,
		0x9A2E,
		0x9A59,
		0x9A8B,
		0x9A97,
		0x9ABC,
		0x9ADC,
		0x9AE6,
		0x9B01,
		0x9B11,
		0x9B1C,
		0x9B41,
		0x9B46,
		0x9B4B,
		0x9B51,
		0x9B85,
		0x9BA5,
		0x9C06,
		0x9C28,
		0xAF01,
		0xAF08,
		0xAF0B,
		0xAF13,
		0xAF19,
		0xAF1C,
		0xAF2C,
		0xAF3B,
		0xAF4A,
		0xB005,
		0xB0AB,
		0xB125,
		0xB12E,
		0xB14F,
		0xB152,
		0xB15D,
		0xB171,
		0xB1B1,
		0xB1E5,
		0xB1ED,
		0xB22B,
		0xB232,
		0xB269,
		0xB2BA,
		0xB2CA,
		0xB2CF,
		0xB2D5,
		0xB31B,
		0xB321,
		0xB330,
		0xB334,
		0xB361,
		0xB369,
		0xB37A,
		0xB382,
		0xB3AA,
		0xB3B0,
		0xB3B6,
		0xB474,
		0xB47E,
		0xB48F,
		0xB4CC,
		0xB4D1,
		0xB4DC,
		0xB524,
		0xB52B,
		0xB598,
		0xB59D,
		0xB5A5,
		0xB5AB,
		0xB635,
		0xB669,
		0xB67D,
		0xB680,
		0xB683,
		0xB6D0,
		0xB6DB,
		0xB6E8,
		0xB769,
		0xB7A9,
		0xB81A,
		0xB871,
		0xB8A8,
		0xB8BA,
		0xB8EF,
		0xB8F8,
		0xB908,
		0xB915,
		0xB94F,
		0xB95B,
		0xB97C,
		0xB989,
		0xB9C9,
		0xB9E1,
		0xBA27,
		0xBA3C,
		0xBA45,
		0xBA4A,
		0xBA5D,
		0xBA7F,
		0xBAAD,
		0xBAEB,
		0xBB34,
		0xBB3B,
		0xBB54,
		0xBBDE,
		0xBBF1,
		0xBC39,
		0xBC71,
		0xBC99,
		0xBCA4,
		0xBCA7,
		0xBCB0,
		0xBCE1,
		0xBD12,
		0xBD31,
		0xBD47,
		0xBDB4,
		0xBDB6,
		0xBDD2,
		0xBDDF,
		0xBDFD,
		0xBE00,
		0xBE0F,
		0xBE1B,
		0xBE26,
		0xBE3A,
		0xBE83,
		0xBE8E,
		0xBE99,
		0xBE9C,
		0xBEA1,
		0xBEA8,
		0xBEB6,
		0xBEBE,
		0xBEC1,
		0xBFD4,
		0xBFE8,
		0xC14D,
		0xC156,
		0xC15A,
		0xC163,
		0xC192,
		0xC1C8,
		0xC1D5,
		0xC211,
		0xC215,
		0xC328,
		0xC36B,
		0xC375,
		0xC378,
		0xC385,
		0xC3A4,
		0xC3AD,
		0xC3EC,
		0xC437,
		0xC45C,
		0xC4A8,
		0xC4B0,
		0xC4E3,
		0xC57D,
		0xC5BB,
		0xC654,
		0xC6CC,
		0xC75B,
		0xC80B,
		0xC812,
		0xC83F,
		0xC845,
		0xC8E0,
		0xC8EE,
		0xC8F1,
		0xC8F4,
		0xC935,
		0xC938,
		0xC941,
		0xC95F,
		0xC962,
		0xC96E,
		0xC971,
		0xC9B0,
		0xC9B8,
		0xC9E5,
		0xCA24,
		0xCA69,
		0xCA6B,
		0xCA9B,
		0xCAAD,
		0xCAE8,
		0xCAEF,
		0xCB56,
		0xCB63,
		0xCBA7,
		0xCC31,
		0xCC36,
		0xCC4A,
		0xCCC3,
		0xCD4B,
		0xCD5D,
		0xCD94,
		0xCDAA,
		0xCDEA,
		0xCEDF,
		0xCEE5,
		0xCEF7,
		0xCF07,
		0xCF35,
		0xCF5D,
		0xCF71,
		0xCF83,
		0xD227,
		0xD2BC,
		0xD2D9,
		0xD2F2,
		0xD301,
		0xD312,
		0xD34E,
		0xD35E,
		0xD393,
		0xD399,
		0xD3A2,
		0xD3A5,
		0xD3B4,
		0xD3C4,
		0xD5F8,
		0xD60C,
		0xD60F,
		0xD64F,
		0xD6C6,
		0xD6FC,
		0xD71F,
		0xD791,
		0xD79B,
		0xD7DB,
		0xD7F7,
		0xD833,
		0xD85B,
		0xD85D,
		0xD868,
		0xD874,
		0xD887,
		0xD8AF,
		0xD8B3,
		0xD8FD,
		0xD944,
		0xD987,
		0xD98C,
		0xD9AC,
		0xD9C7,
		0xDA0A,
		0xDA47,
		0xDA4B,
		0xDA5E,
		0xDA6C,
		0xDA7B,
		0xDA92,
		0xDADE,
		0xDB02,
		0xDB22,
		0xDB2A,
		0xDB2E,
		0xDB32,
		0xDB53,
		0xDB62,
		0xDB64,
		0xDB68,
		0xDB71,
		0xDB86,
		0xDB88,
		0xDB9F,
		0xDBC7,
		0xDBCB,
		0xDBDE,
		0xDBEA,
		0xDC6F,
		0xDC7A,
		0xDC7E,
		0xDCC1,
		0xDCC4,
		0xDCC9,
		0xDCCB,
		0xDCCF,
		0xDCDA,
		0xDCFE,
		0xDD04,
		0xDD06,
		0xDD18,
		0xDD22,
		0xDD32,
		0xDD47,
		0xDD73,
		0xDD7C,
		0xDDA1,
		0xDDA6,
		0xDDAC,
		0xDDAE,
		0xDDB8,
		0xDDE2,
		0xDDFA,
		0xDE6D,
		0xDEC1,
		0xDF54,
		0xDF59,
		0xDFDC,
		0xDFEA,
		0xDFFD,
		0xDFFF,
		0xE051,
		0xE082,
		0xE08A,
		0xE0C2,
		0xE11A,
		0xE152,
		0xE168,
		0xE179,
		0xE191,
		0xE1BD,
		0xE1C5,
		0xE1D6,
		0xE1DC,
		0xE277,
		0xE335,
		0xE344,
		0xE354,
		0xE364,
		0xE370,
		0xE372,
		0xE40B,
		0xE4E4,
		0xE537,
		0xE567,
		0xE603,
		0xE609,
		0xE60D,
		0xE661,
		0xE6B0,
		0xE8D9,
		0xE8EB,
		0xE96D,
		0xE9D6,
		0xE9E2,
		0xE9ED,
		0xEA08,
		0xEA43,
		0xEA73,
		0xEABE,
		0xEAC2,
		0xEB9A,
		0xEBA0,
		0xEC02,
		0xEC82,
		0xECBE,
		0xECD5,
		0xED6F,
		0xED75,
		0xEDBB,
		0xEDC9,
		0xEDD6,
		0xEF0C,
		0xEF2C,
		0xEF50,
		0xEF62,
		0xEF75,
		0xEFAE,
		0xF01A,
		0xF02B,
		0xF039,
		0xF041,
		0xF04A,
		0xF04D,
		0xF055,
		0xF0FD,
		0xF145,
		0xF160,
		0xF16E,
		0xF1C5,
		0xF1DA,
		0xF31D,
		0xF321,
		0xF333,
		0xF354,
		0xF357,
		0xF393,
		0xF3C9,
		0xF3F7,
		0xF41F,
		0xF423,
		0xF42B,
		0xF42F,
		0xF437,
		0xF43B,
		0xF445,
		0xF586,
		0xF59A,
		0xF59E,
		0xF5A2,
		0xF5B5,
		0xF5F2,
		0xF627,
		0xF698,
		0xF69C,
		0xF6AE,
		0xF761,
		0xF78D,
		0xF7A2,
		0xF7C0,
		0xF7EC,
		0xF84E,
		0xF895,
		0xF899,
		0xF8A1,
		0xF8E9,
		0xF905,
	];

	for &adr in hidden_labels.iter() {
		let idx = sorted.iter().position(|(_, x, ..)| *x == adr).unwrap();
		sorted[idx].0 = IsStart::Yes;
		let adr = sorted[idx].1;
		if let Some((_, _, _, e @ End::Continue)) = sorted.get_mut(idx.wrapping_sub(1)) {
			*e = End::Goto(adr);
		}
	}

	sorted
}
