use anyhow::{Result, bail};

use crate::inst::{
	ADC, AHX, AND, ASL, BIT, CMP, CPX, CPY, DCP, DEC, EOR, INC, ISC, Inst, JMP, LAX, LDA, LDX, LDY,
	LSR, ORA, RLA, ROL, ROR, RRA, SAX, SBC, SLO, SRE, STA, STX, STY,
};

pub struct NesFile {
	pub prg_roms: Vec<[u8; 16 * 1024]>,
	pub programs: Vec<Vec<(u16, Inst)>>,
	pub chr_roms: Vec<[u8; 8 * 1024]>,
}

impl TryFrom<Vec<u8>> for NesFile {
	type Error = anyhow::Error;

	fn try_from(buffer: Vec<u8>) -> Result<Self, Self::Error> {
		let [
			b'N',
			b'E',
			b'S',
			0x1A,
			prg_size,
			chr_size,
			flags_6,
			_flags_7,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
		] = &buffer[0..16]
		else {
			bail!("Missing header!");
		};

		let trainer_present = flags_6 & 0x04 != 0;
		let trainer_offset = if trainer_present { 512 } else { 0 };
		let prg_offset = 16 + trainer_offset;
		let chr_offset = prg_offset + (*prg_size as usize * 16 * 1024);

		// Parse PRG ROM banks
		let mut prg_roms = Vec::new();
		for i in 0..*prg_size {
			let start = prg_offset + (i as usize * 16 * 1024);
			let end = start + (16 * 1024);

			if end > buffer.len() {
				bail!("PRG ROM data is too short");
			}

			let mut bank = [0u8; 16 * 1024];
			bank.copy_from_slice(&buffer[start..end]);
			prg_roms.push(bank);
		}

		let programs = prg_roms
			.iter()
			.map(|txt| {
				let mut txt = txt.as_slice();
				let mut out = Vec::new();
				while !txt.is_empty() {
					let idx = (16384 - txt.len()) as _;
					let inst = parse_instruction(&mut txt)?;
					out.push((idx, inst));
				}
				Ok(out)
			})
			.collect::<Result<Vec<_>>>()?;

		// Parse CHR ROM banks
		let mut chr_roms = Vec::new();
		for i in 0..*chr_size {
			let start = chr_offset + (i as usize * 8 * 1024);
			let end = start + (8 * 1024);

			if end > buffer.len() {
				bail!("CHR ROM data is too short");
			}

			let mut bank = [0u8; 8 * 1024];
			bank.copy_from_slice(&buffer[start..end]);
			chr_roms.push(bank);
		}

		Ok(NesFile {
			prg_roms,
			programs,
			chr_roms,
		})
	}
}

impl NesFile {
	fn parse_bb(
		prg_roms: &[[u8; 16 * 1024]],
		stack_ptr: u16,
		rom_bank: usize,
	) -> Result<Vec<Inst>> {
		let mut out = Vec::new();
		let mut slice = &prg_roms[rom_bank][stack_ptr as usize..];
		loop {
			let inst = parse_instruction(&mut slice).unwrap();
			out.push(inst);
			if inst.ends_bb() {
				break;
			}
		}
		Ok(out)
	}
}

pub fn parse_instruction(code: &mut &[u8]) -> Result<Inst> {
	match code {
		// ADC instructions
		[0x69, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::ADC(ADC::Immediate(*imm)))
		}
		[0x65, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ADC(ADC::ZeroPage(*addr)))
		}
		[0x75, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ADC(ADC::ZeroPageX(*addr)))
		}
		[0x6D, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ADC(ADC::Absolute(*x)))
		}
		[0x7D, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ADC(ADC::AbsoluteX(*x)))
		}
		[0x79, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ADC(ADC::AbsoluteY(*x)))
		}
		[0x61, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ADC(ADC::IndirectX(*addr)))
		}
		[0x71, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ADC(ADC::IndirectY(*addr)))
		}

		// AND instructions
		[0x29, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::AND(AND::Immediate(*imm)))
		}
		[0x25, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::AND(AND::ZeroPage(*addr)))
		}
		[0x35, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::AND(AND::ZeroPageX(*addr)))
		}
		[0x2D, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::AND(AND::Absolute(*x)))
		}
		[0x3D, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::AND(AND::AbsoluteX(*x)))
		}
		[0x39, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::AND(AND::AbsoluteY(*x)))
		}
		[0x21, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::AND(AND::IndirectX(*addr)))
		}
		[0x31, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::AND(AND::IndirectY(*addr)))
		}

		// ASL instructions
		[0x0A, rest @ ..] => {
			*code = rest;
			Ok(Inst::ASL(ASL::Accumulator))
		}
		[0x06, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ASL(ASL::ZeroPage(*addr)))
		}
		[0x16, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ASL(ASL::ZeroPageX(*addr)))
		}
		[0x0E, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ASL(ASL::Absolute(*x)))
		}
		[0x1E, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ASL(ASL::AbsoluteX(*x)))
		}

		// BCC instruction
		[0x90, rel, rest @ ..] => {
			*code = rest;
			Ok(Inst::BCC(*rel as i8))
		}

		// BCS instruction
		[0xB0, rel, rest @ ..] => {
			*code = rest;
			Ok(Inst::BCS(*rel as i8))
		}

		// BEQ instruction
		[0xF0, rel, rest @ ..] => {
			*code = rest;
			Ok(Inst::BEQ(*rel as i8))
		}

		// BIT instructions
		[0x24, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::BIT(BIT::ZeroPage(*addr)))
		}
		[0x2C, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::BIT(BIT::Absolute(*x)))
		}

		// BMI instruction
		[0x30, rel, rest @ ..] => {
			*code = rest;
			Ok(Inst::BMI(*rel as i8))
		}

		// BNE instruction
		[0xD0, rel, rest @ ..] => {
			*code = rest;
			Ok(Inst::BNE(*rel as i8))
		}

		// BPL instruction
		[0x10, rel, rest @ ..] => {
			*code = rest;
			Ok(Inst::BPL(*rel as i8))
		}

		// BRK instruction
		[0x00, rest @ ..] => {
			*code = rest;
			Ok(Inst::BRK)
		}

		// BVC instruction
		[0x50, rel, rest @ ..] => {
			*code = rest;
			Ok(Inst::BVC(*rel as i8))
		}

		// BVS instruction
		[0x70, rel, rest @ ..] => {
			*code = rest;
			Ok(Inst::BVS(*rel as i8))
		}

		// CLC instruction
		[0x18, rest @ ..] => {
			*code = rest;
			Ok(Inst::CLC)
		}

		// CLD instruction
		[0xD8, rest @ ..] => {
			*code = rest;
			Ok(Inst::CLD)
		}

		// CLI instruction
		[0x58, rest @ ..] => {
			*code = rest;
			Ok(Inst::CLI)
		}

		// CLV instruction
		[0xB8, rest @ ..] => {
			*code = rest;
			Ok(Inst::CLV)
		}

		// CMP instructions
		[0xC9, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::CMP(CMP::Immediate(*imm)))
		}
		[0xC5, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::CMP(CMP::ZeroPage(*addr)))
		}
		[0xD5, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::CMP(CMP::ZeroPageX(*addr)))
		}
		[0xCD, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::CMP(CMP::Absolute(*x)))
		}
		[0xDD, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::CMP(CMP::AbsoluteX(*x)))
		}
		[0xD9, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::CMP(CMP::AbsoluteY(*x)))
		}
		[0xC1, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::CMP(CMP::IndirectX(*addr)))
		}
		[0xD1, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::CMP(CMP::IndirectY(*addr)))
		}

		// CPX instructions
		[0xE0, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::CPX(CPX::Immediate(*imm)))
		}
		[0xE4, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::CPX(CPX::ZeroPage(*addr)))
		}
		[0xEC, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::CPX(CPX::Absolute(*x)))
		}

		// CPY instructions
		[0xC0, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::CPY(CPY::Immediate(*imm)))
		}
		[0xC4, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::CPY(CPY::ZeroPage(*addr)))
		}
		[0xCC, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::CPY(CPY::Absolute(*x)))
		}

		// DEC instructions
		[0xC6, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::DEC(DEC::ZeroPage(*addr)))
		}
		[0xD6, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::DEC(DEC::ZeroPageX(*addr)))
		}
		[0xCE, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::DEC(DEC::Absolute(*x)))
		}
		[0xDE, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::DEC(DEC::AbsoluteX(*x)))
		}

		// DEX instruction
		[0xCA, rest @ ..] => {
			*code = rest;
			Ok(Inst::DEX)
		}

		// DEY instruction
		[0x88, rest @ ..] => {
			*code = rest;
			Ok(Inst::DEY)
		}

		// EOR instructions
		[0x49, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::EOR(EOR::Immediate(*imm)))
		}
		[0x45, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::EOR(EOR::ZeroPage(*addr)))
		}
		[0x55, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::EOR(EOR::ZeroPageX(*addr)))
		}
		[0x4D, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::EOR(EOR::Absolute(*x)))
		}
		[0x5D, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::EOR(EOR::AbsoluteX(*x)))
		}
		[0x59, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::EOR(EOR::AbsoluteY(*x)))
		}
		[0x41, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::EOR(EOR::IndirectX(*addr)))
		}
		[0x51, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::EOR(EOR::IndirectY(*addr)))
		}

		// INC instructions
		[0xE6, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::INC(INC::ZeroPage(*addr)))
		}
		[0xF6, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::INC(INC::ZeroPageX(*addr)))
		}
		[0xEE, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::INC(INC::Absolute(*x)))
		}
		[0xFE, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::INC(INC::AbsoluteX(*x)))
		}

		// INX instruction
		[0xE8, rest @ ..] => {
			*code = rest;
			Ok(Inst::INX)
		}

		// INY instruction
		[0xC8, rest @ ..] => {
			*code = rest;
			Ok(Inst::INY)
		}

		// JMP instructions
		[0x4C, lo, hi, rest @ ..] => {
			*code = rest;
			let x = u16::from_le_bytes([*lo, *hi]);
			Ok(Inst::JMP(JMP::Absolute(x)))
		}
		[0x6C, lo, hi, rest @ ..] => {
			*code = rest;
			let x = u16::from_le_bytes([*lo, *hi]);
			Ok(Inst::JMP(JMP::Indirect(x)))
		}

		// JSR instruction
		[0x20, lo, hi, rest @ ..] => {
			*code = rest;
			let x = u16::from_le_bytes([*lo, *hi]);
			Ok(Inst::JSR(x))
		}

		// LDA instructions
		[0xA9, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDA(LDA::Immediate(*imm)))
		}
		[0xA5, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDA(LDA::ZeroPage(*addr)))
		}
		[0xB5, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDA(LDA::ZeroPageX(*addr)))
		}
		[0xAD, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDA(LDA::Absolute(*x)))
		}
		[0xBD, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDA(LDA::AbsoluteX(*x)))
		}
		[0xB9, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDA(LDA::AbsoluteY(*x)))
		}
		[0xA1, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDA(LDA::IndirectX(*addr)))
		}
		[0xB1, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDA(LDA::IndirectY(*addr)))
		}

		// LDX instructions
		[0xA2, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDX(LDX::Immediate(*imm)))
		}
		[0xA6, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDX(LDX::ZeroPage(*addr)))
		}
		[0xB6, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDX(LDX::ZeroPageY(*addr)))
		}
		[0xAE, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDX(LDX::Absolute(*x)))
		}
		[0xBE, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDX(LDX::AbsoluteY(*x)))
		}

		// LDY instructions
		[0xA0, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDY(LDY::Immediate(*imm)))
		}
		[0xA4, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDY(LDY::ZeroPage(*addr)))
		}
		[0xB4, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDY(LDY::ZeroPageX(*addr)))
		}
		[0xAC, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDY(LDY::Absolute(*x)))
		}
		[0xBC, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::LDY(LDY::AbsoluteX(*x)))
		}

		// LSR instructions
		[0x4A, rest @ ..] => {
			*code = rest;
			Ok(Inst::LSR(LSR::Accumulator))
		}
		[0x46, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LSR(LSR::ZeroPage(*addr)))
		}
		[0x56, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LSR(LSR::ZeroPageX(*addr)))
		}
		[0x4E, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::LSR(LSR::Absolute(*x)))
		}
		[0x5E, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::LSR(LSR::AbsoluteX(*x)))
		}

		// NOP instruction
		[0xEA, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}

		// ORA instructions
		[0x09, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::ORA(ORA::Immediate(*imm)))
		}
		[0x05, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ORA(ORA::ZeroPage(*addr)))
		}
		[0x15, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ORA(ORA::ZeroPageX(*addr)))
		}
		[0x0D, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ORA(ORA::Absolute(*x)))
		}
		[0x1D, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ORA(ORA::AbsoluteX(*x)))
		}
		[0x19, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ORA(ORA::AbsoluteY(*x)))
		}
		[0x01, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ORA(ORA::IndirectX(*addr)))
		}
		[0x11, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ORA(ORA::IndirectY(*addr)))
		}

		// PHA instruction
		[0x48, rest @ ..] => {
			*code = rest;
			Ok(Inst::PHA)
		}

		// PHP instruction
		[0x08, rest @ ..] => {
			*code = rest;
			Ok(Inst::PHP)
		}

		// PLA instruction
		[0x68, rest @ ..] => {
			*code = rest;
			Ok(Inst::PLA)
		}

		// PLP instruction
		[0x28, rest @ ..] => {
			*code = rest;
			Ok(Inst::PLP)
		}

		// ROL instructions
		[0x2A, rest @ ..] => {
			*code = rest;
			Ok(Inst::ROL(ROL::Accumulator))
		}
		[0x26, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ROL(ROL::ZeroPage(*addr)))
		}
		[0x36, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ROL(ROL::ZeroPageX(*addr)))
		}
		[0x2E, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ROL(ROL::Absolute(*x)))
		}
		[0x3E, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ROL(ROL::AbsoluteX(*x)))
		}

		// ROR instructions
		[0x6A, rest @ ..] => {
			*code = rest;
			Ok(Inst::ROR(ROR::Accumulator))
		}
		[0x66, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ROR(ROR::ZeroPage(*addr)))
		}
		[0x76, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ROR(ROR::ZeroPageX(*addr)))
		}
		[0x6E, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ROR(ROR::Absolute(*x)))
		}
		[0x7E, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ROR(ROR::AbsoluteX(*x)))
		}

		// RTI instruction
		[0x40, rest @ ..] => {
			*code = rest;
			Ok(Inst::RTI)
		}

		// RTS instruction
		[0x60, rest @ ..] => {
			*code = rest;
			Ok(Inst::RTS)
		}

		// SBC instructions
		[0xE9, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::SBC(SBC::Immediate(*imm)))
		}
		[0xE5, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SBC(SBC::ZeroPage(*addr)))
		}
		[0xF5, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SBC(SBC::ZeroPageX(*addr)))
		}
		[0xED, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::SBC(SBC::Absolute(*x)))
		}
		[0xFD, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::SBC(SBC::AbsoluteX(*x)))
		}
		[0xF9, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::SBC(SBC::AbsoluteY(*x)))
		}
		[0xE1, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SBC(SBC::IndirectX(*addr)))
		}
		[0xF1, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SBC(SBC::IndirectY(*addr)))
		}

		// SEC instruction
		[0x38, rest @ ..] => {
			*code = rest;
			Ok(Inst::SEC)
		}

		// SED instruction
		[0xF8, rest @ ..] => {
			*code = rest;
			Ok(Inst::SED)
		}

		// SEI instruction
		[0x78, rest @ ..] => {
			*code = rest;
			Ok(Inst::SEI)
		}

		// STA instructions
		[0x85, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::STA(STA::ZeroPage(*addr)))
		}
		[0x95, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::STA(STA::ZeroPageX(*addr)))
		}
		[0x8D, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::STA(STA::Absolute(*x)))
		}
		[0x9D, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::STA(STA::AbsoluteX(*x)))
		}
		[0x99, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::STA(STA::AbsoluteY(*x)))
		}
		[0x81, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::STA(STA::IndirectX(*addr)))
		}
		[0x91, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::STA(STA::IndirectY(*addr)))
		}

		// STX instructions
		[0x86, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::STX(STX::ZeroPage(*addr)))
		}
		[0x96, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::STX(STX::ZeroPageY(*addr)))
		}
		[0x8E, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::STX(STX::Absolute(*x)))
		}

		// STY instructions
		[0x84, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::STY(STY::ZeroPage(*addr)))
		}
		[0x94, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::STY(STY::ZeroPageX(*addr)))
		}
		[0x8C, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::STY(STY::Absolute(*x)))
		}

		// TAX instruction
		[0xAA, rest @ ..] => {
			*code = rest;
			Ok(Inst::TAX)
		}

		// TAY instruction
		[0xA8, rest @ ..] => {
			*code = rest;
			Ok(Inst::TAY)
		}

		// TSX instruction
		[0xBA, rest @ ..] => {
			*code = rest;
			Ok(Inst::TSX)
		}

		// TXA instruction
		[0x8A, rest @ ..] => {
			*code = rest;
			Ok(Inst::TXA)
		}

		// TXS instruction
		[0x9A, rest @ ..] => {
			*code = rest;
			Ok(Inst::TXS)
		}

		// TYA instruction
		[0x98, rest @ ..] => {
			*code = rest;
			Ok(Inst::TYA)
		}

		// LAX instructions (unofficial)
		[0xA7, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LAX(LAX::ZeroPage(*addr)))
		}
		[0xB7, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LAX(LAX::ZeroPageY(*addr)))
		}
		[0xAF, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::LAX(LAX::Absolute(*x)))
		}
		[0xBF, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::LAX(LAX::AbsoluteY(*x)))
		}
		[0xA3, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LAX(LAX::IndirectX(*addr)))
		}
		[0xB3, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::LAX(LAX::IndirectY(*addr)))
		}

		// SAX instructions (unofficial)
		[0x87, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SAX(SAX::ZeroPage(*addr)))
		}
		[0x97, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SAX(SAX::ZeroPageY(*addr)))
		}
		[0x8F, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::SAX(SAX::Absolute(*x)))
		}
		[0x83, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SAX(SAX::IndirectX(*addr)))
		}

		// DCP instructions (unofficial)
		[0xC7, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::DCP(DCP::ZeroPage(*addr)))
		}
		[0xD7, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::DCP(DCP::ZeroPageX(*addr)))
		}
		[0xCF, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::DCP(DCP::Absolute(*x)))
		}
		[0xDF, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::DCP(DCP::AbsoluteX(*x)))
		}
		[0xDB, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::DCP(DCP::AbsoluteY(*x)))
		}
		[0xC3, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::DCP(DCP::IndirectX(*addr)))
		}
		[0xD3, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::DCP(DCP::IndirectY(*addr)))
		}

		// ISC instructions (unofficial)
		[0xE7, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ISC(ISC::ZeroPage(*addr)))
		}
		[0xF7, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ISC(ISC::ZeroPageX(*addr)))
		}
		[0xEF, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ISC(ISC::Absolute(*x)))
		}
		[0xFF, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ISC(ISC::AbsoluteX(*x)))
		}
		[0xFB, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::ISC(ISC::AbsoluteY(*x)))
		}
		[0xE3, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ISC(ISC::IndirectX(*addr)))
		}
		[0xF3, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::ISC(ISC::IndirectY(*addr)))
		}

		// RLA instructions (unofficial)
		[0x27, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::RLA(RLA::ZeroPage(*addr)))
		}
		[0x37, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::RLA(RLA::ZeroPageX(*addr)))
		}
		[0x2F, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::RLA(RLA::Absolute(*x)))
		}
		[0x3F, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::RLA(RLA::AbsoluteX(*x)))
		}
		[0x3B, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::RLA(RLA::AbsoluteY(*x)))
		}
		[0x23, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::RLA(RLA::IndirectX(*addr)))
		}
		[0x33, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::RLA(RLA::IndirectY(*addr)))
		}

		// RRA instructions (unofficial)
		[0x67, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::RRA(RRA::ZeroPage(*addr)))
		}
		[0x77, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::RRA(RRA::ZeroPageX(*addr)))
		}
		[0x6F, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::RRA(RRA::Absolute(*x)))
		}
		[0x7F, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::RRA(RRA::AbsoluteX(*x)))
		}
		[0x7B, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::RRA(RRA::AbsoluteY(*x)))
		}
		[0x63, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::RRA(RRA::IndirectX(*addr)))
		}
		[0x73, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::RRA(RRA::IndirectY(*addr)))
		}

		// SLO instructions (unofficial)
		[0x07, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SLO(SLO::ZeroPage(*addr)))
		}
		[0x17, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SLO(SLO::ZeroPageX(*addr)))
		}
		[0x0F, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::SLO(SLO::Absolute(*x)))
		}
		[0x1F, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::SLO(SLO::AbsoluteX(*x)))
		}
		[0x1B, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::SLO(SLO::AbsoluteY(*x)))
		}
		[0x03, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SLO(SLO::IndirectX(*addr)))
		}
		[0x13, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SLO(SLO::IndirectY(*addr)))
		}

		// SRE instructions (unofficial)
		[0x47, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SRE(SRE::ZeroPage(*addr)))
		}
		[0x57, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SRE(SRE::ZeroPageX(*addr)))
		}
		[0x4F, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::SRE(SRE::Absolute(*x)))
		}
		[0x5F, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::SRE(SRE::AbsoluteX(*x)))
		}
		[0x5B, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::SRE(SRE::AbsoluteY(*x)))
		}
		[0x43, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SRE(SRE::IndirectX(*addr)))
		}
		[0x53, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::SRE(SRE::IndirectY(*addr)))
		}

		// ANC instructions (unofficial)
		[0x0B, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::ANC(*imm))
		}
		[0x2B, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::ANC(*imm))
		}

		// ALR instructions (unofficial)
		[0x4B, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::ALR(*imm))
		}

		// ARR instructions (unofficial)
		[0x6B, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::ARR(*imm))
		}

		// AXS instructions (unofficial)
		[0xCB, imm, rest @ ..] => {
			*code = rest;
			Ok(Inst::AXS(*imm))
		}

		// LAS instructions (unofficial)
		[0xBB, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::LAS(*x))
		}

		// TAS instructions (unofficial)
		[0x9B, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::TAS(*x))
		}

		// SHY instructions (unofficial)
		[0x9C, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::SHY(*x))
		}

		// SHX instructions (unofficial)
		[0x9E, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::SHX(*x))
		}

		// AHX instructions (unofficial)
		[0x9F, x, rest @ ..] => {
			*code = rest;
			Ok(Inst::AHX(AHX::AbsoluteY(*x)))
		}
		[0x93, addr, rest @ ..] => {
			*code = rest;
			Ok(Inst::AHX(AHX::IndirectY(*addr)))
		}

		// NOP instructions (unofficial)
		[0x1A, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x3A, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x5A, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x7A, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xDA, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xFA, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}

		// Generated invalid op-codes, to be looked up later
		[0x02, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x04, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x0C, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x12, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x14, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x1C, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x22, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x32, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x34, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x3C, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x42, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x44, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x52, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x54, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x5C, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x62, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x64, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x72, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x74, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x7C, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x80, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x82, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x89, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x8B, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0x92, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xAB, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xB2, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xC2, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xD2, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xD4, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xDC, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xE2, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xEB, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xF2, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xF4, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}
		[0xFC, rest @ ..] => {
			*code = rest;
			Ok(Inst::NOP)
		}

		// Default case - unknown instruction
		x => Err(anyhow::anyhow!("Unknown opcode: {:02x?}", &x[..1])),
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn load_smb3() {
		let buffer = std::fs::read("non-free/SMB3.nes").unwrap();
		NesFile::try_from(buffer).unwrap();
	}

	#[test]
	fn all_opcodes_parse() {
		let mut buf = [0, 0, 0, 0];
		for byte in u8::MIN..=u8::MAX {
			buf[0] = byte;
			let mut code = buf.as_slice();

			let res = parse_instruction(&mut code);
			assert!(res.is_ok());
		}
	}
}
