#![allow(unused, clippy::upper_case_acronyms)]

use crate::{cpu::Cpu, evaluate_instruction::*};

use anyhow::{Result, bail};

// Auto-generated NES CPU instruction set
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Inst {
	ADC(ADC),
	AND(AND),
	ASL(ASL),
	BCC(i8),
	BCS(i8),
	BEQ(i8),
	BIT(BIT),
	BMI(i8),
	BNE(i8),
	BPL(i8),
	BRK,
	BVC(i8),
	BVS(i8),
	CLC,
	CLD,
	CLI,
	CLV,
	CMP(CMP),
	CPX(CPX),
	CPY(CPY),
	DEC(DEC),
	DEX,
	DEY,
	EOR(EOR),
	INC(INC),
	INX,
	INY,
	JMP(JMP),
	JSR(u16),
	LDA(LDA),
	LDX(LDX),
	LDY(LDY),
	LSR(LSR),
	NOP,
	ORA(ORA),
	PHA,
	PHP,
	PLA,
	PLP,
	ROL(ROL),
	ROR(ROR),
	RTI,
	RTS,
	SBC(SBC),
	SEC,
	SED,
	SEI,
	STA(STA),
	STX(STX),
	STY(STY),
	TAX,
	TAY,
	TSX,
	TXA,
	TXS,
	TYA,
	LAX(LAX),
	SAX(SAX),
	DCP(DCP),
	ISC(ISC),
	RLA(RLA),
	RRA(RRA),
	SLO(SLO),
	SRE(SRE),
	ANC(u8),
	ALR(u8),
	ARR(u8),
	AXS(u8),
	LAS(u8),
	TAS(u8),
	SHY(u8),
	SHX(u8),
	AHX(AHX),
	NOPU(NOPU),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ADC {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AND {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ASL {
	Accumulator,
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BIT {
	ZeroPage(u8),
	Absolute(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CMP {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CPX {
	Immediate(u8),
	ZeroPage(u8),
	Absolute(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CPY {
	Immediate(u8),
	ZeroPage(u8),
	Absolute(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DEC {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EOR {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum INC {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum JMP {
	Absolute(u16),
	Indirect(u16),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LDA {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LDX {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageY(u8),
	Absolute(u8),
	AbsoluteY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LDY {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LSR {
	Accumulator,
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ORA {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ROL {
	Accumulator,
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ROR {
	Accumulator,
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SBC {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum STA {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum STX {
	ZeroPage(u8),
	ZeroPageY(u8),
	Absolute(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum STY {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LAX {
	ZeroPage(u8),
	ZeroPageY(u8),
	Absolute(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SAX {
	ZeroPage(u8),
	ZeroPageY(u8),
	Absolute(u8),
	IndirectX(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DCP {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ISC {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RLA {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RRA {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SLO {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SRE {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
	IndirectX(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AHX {
	AbsoluteY(u8),
	IndirectY(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NOPU {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8),
	AbsoluteX(u8),
	AbsoluteY(u8),
}

impl Inst {
	pub fn ends_bb(&self) -> bool {
		match self {
			Inst::ADC(..) => false,
			Inst::AND(..) => false,
			Inst::ASL(..) => false,
			Inst::BCC(..) => true,
			Inst::BCS(..) => true,
			Inst::BEQ(..) => true,
			Inst::BIT(..) => false,
			Inst::BMI(..) => true,
			Inst::BNE(..) => true,
			Inst::BPL(..) => true,
			Inst::BRK => false,
			Inst::BVC(..) => true,
			Inst::BVS(..) => true,
			Inst::CLC => false,
			Inst::CLD => false,
			Inst::CLI => false,
			Inst::CLV => false,
			Inst::CMP(..) => false,
			Inst::CPX(..) => false,
			Inst::CPY(..) => false,
			Inst::DEC(..) => false,
			Inst::DEX => false,
			Inst::DEY => false,
			Inst::EOR(..) => false,
			Inst::INC(..) => false,
			Inst::INX => false,
			Inst::INY => false,
			Inst::JMP(..) => true,
			Inst::JSR(..) => true,
			Inst::LDA(..) => false,
			Inst::LDX(..) => false,
			Inst::LDY(..) => false,
			Inst::LSR(..) => false,
			Inst::NOP => false,
			Inst::ORA(..) => false,
			Inst::PHA => false,
			Inst::PHP => false,
			Inst::PLA => false,
			Inst::PLP => false,
			Inst::ROL(..) => false,
			Inst::ROR(..) => false,
			Inst::RTI => true,
			Inst::RTS => true,
			Inst::SBC(..) => false,
			Inst::SEC => false,
			Inst::SED => false,
			Inst::SEI => false,
			Inst::STA(..) => false,
			Inst::STX(..) => false,
			Inst::STY(..) => false,
			Inst::TAX => false,
			Inst::TAY => false,
			Inst::TSX => false,
			Inst::TXA => false,
			Inst::TXS => false,
			Inst::TYA => false,
			Inst::LAX(..) => false,
			Inst::SAX(..) => false,
			Inst::DCP(..) => false,
			Inst::ISC(..) => false,
			Inst::RLA(..) => false,
			Inst::RRA(..) => false,
			Inst::SLO(..) => false,
			Inst::SRE(..) => false,
			Inst::ANC(..) => false,
			Inst::ALR(..) => false,
			Inst::ARR(..) => false,
			Inst::AXS(..) => false,
			Inst::LAS(..) => false,
			Inst::TAS(..) => false,
			Inst::SHY(..) => false,
			Inst::SHX(..) => false,
			Inst::AHX(..) => false,
			Inst::NOPU(..) => false,
		}
	}

	pub fn len(&self) -> u8 {
		match self {
			Inst::ADC(ADC::Immediate(..)) => 2,
			Inst::ADC(ADC::ZeroPage(..)) => 2,
			Inst::ADC(ADC::ZeroPageX(..)) => 2,
			Inst::ADC(ADC::Absolute(..)) => 3,
			Inst::ADC(ADC::AbsoluteX(..)) => 3,
			Inst::ADC(ADC::AbsoluteY(..)) => 3,
			Inst::ADC(ADC::IndirectX(..)) => 2,
			Inst::ADC(ADC::IndirectY(..)) => 2,
			Inst::AND(AND::Immediate(..)) => 2,
			Inst::AND(AND::ZeroPage(..)) => 2,
			Inst::AND(AND::ZeroPageX(..)) => 2,
			Inst::AND(AND::Absolute(..)) => 3,
			Inst::AND(AND::AbsoluteX(..)) => 3,
			Inst::AND(AND::AbsoluteY(..)) => 3,
			Inst::AND(AND::IndirectX(..)) => 2,
			Inst::AND(AND::IndirectY(..)) => 2,
			Inst::ASL(ASL::Accumulator) => 1,
			Inst::ASL(ASL::ZeroPage(..)) => 2,
			Inst::ASL(ASL::ZeroPageX(..)) => 2,
			Inst::ASL(ASL::Absolute(..)) => 3,
			Inst::ASL(ASL::AbsoluteX(..)) => 3,
			Inst::BCC(..) => 2,
			Inst::BCS(..) => 2,
			Inst::BEQ(..) => 2,
			Inst::BIT(BIT::ZeroPage(..)) => 2,
			Inst::BIT(BIT::Absolute(..)) => 3,
			Inst::BMI(..) => 2,
			Inst::BNE(..) => 2,
			Inst::BPL(..) => 2,
			Inst::BRK => 1,
			Inst::BVC(..) => 2,
			Inst::BVS(..) => 2,
			Inst::CLC => 1,
			Inst::CLD => 1,
			Inst::CLI => 1,
			Inst::CLV => 1,
			Inst::CMP(CMP::Immediate(..)) => 2,
			Inst::CMP(CMP::ZeroPage(..)) => 2,
			Inst::CMP(CMP::ZeroPageX(..)) => 2,
			Inst::CMP(CMP::Absolute(..)) => 3,
			Inst::CMP(CMP::AbsoluteX(..)) => 3,
			Inst::CMP(CMP::AbsoluteY(..)) => 3,
			Inst::CMP(CMP::IndirectX(..)) => 2,
			Inst::CMP(CMP::IndirectY(..)) => 2,
			Inst::CPX(CPX::Immediate(..)) => 2,
			Inst::CPX(CPX::ZeroPage(..)) => 2,
			Inst::CPX(CPX::Absolute(..)) => 3,
			Inst::CPY(CPY::Immediate(..)) => 2,
			Inst::CPY(CPY::ZeroPage(..)) => 2,
			Inst::CPY(CPY::Absolute(..)) => 3,
			Inst::DEC(DEC::ZeroPage(..)) => 2,
			Inst::DEC(DEC::ZeroPageX(..)) => 2,
			Inst::DEC(DEC::Absolute(..)) => 3,
			Inst::DEC(DEC::AbsoluteX(..)) => 3,
			Inst::DEX => 1,
			Inst::DEY => 1,
			Inst::EOR(EOR::Immediate(..)) => 2,
			Inst::EOR(EOR::ZeroPage(..)) => 2,
			Inst::EOR(EOR::ZeroPageX(..)) => 2,
			Inst::EOR(EOR::Absolute(..)) => 3,
			Inst::EOR(EOR::AbsoluteX(..)) => 3,
			Inst::EOR(EOR::AbsoluteY(..)) => 3,
			Inst::EOR(EOR::IndirectX(..)) => 2,
			Inst::EOR(EOR::IndirectY(..)) => 2,
			Inst::INC(INC::ZeroPage(..)) => 2,
			Inst::INC(INC::ZeroPageX(..)) => 2,
			Inst::INC(INC::Absolute(..)) => 3,
			Inst::INC(INC::AbsoluteX(..)) => 3,
			Inst::INX => 1,
			Inst::INY => 1,
			Inst::JMP(JMP::Absolute(..)) => 3,
			Inst::JMP(JMP::Indirect(..)) => 3,
			Inst::JSR(..) => 3,
			Inst::LDA(LDA::Immediate(..)) => 2,
			Inst::LDA(LDA::ZeroPage(..)) => 2,
			Inst::LDA(LDA::ZeroPageX(..)) => 2,
			Inst::LDA(LDA::Absolute(..)) => 3,
			Inst::LDA(LDA::AbsoluteX(..)) => 3,
			Inst::LDA(LDA::AbsoluteY(..)) => 3,
			Inst::LDA(LDA::IndirectX(..)) => 2,
			Inst::LDA(LDA::IndirectY(..)) => 2,
			Inst::LDX(LDX::Immediate(..)) => 2,
			Inst::LDX(LDX::ZeroPage(..)) => 2,
			Inst::LDX(LDX::ZeroPageY(..)) => 2,
			Inst::LDX(LDX::Absolute(..)) => 3,
			Inst::LDX(LDX::AbsoluteY(..)) => 3,
			Inst::LDY(LDY::Immediate(..)) => 2,
			Inst::LDY(LDY::ZeroPage(..)) => 2,
			Inst::LDY(LDY::ZeroPageX(..)) => 2,
			Inst::LDY(LDY::Absolute(..)) => 3,
			Inst::LDY(LDY::AbsoluteX(..)) => 3,
			Inst::LSR(LSR::Accumulator) => 1,
			Inst::LSR(LSR::ZeroPage(..)) => 2,
			Inst::LSR(LSR::ZeroPageX(..)) => 2,
			Inst::LSR(LSR::Absolute(..)) => 3,
			Inst::LSR(LSR::AbsoluteX(..)) => 3,
			Inst::NOP => 1,
			Inst::ORA(ORA::Immediate(..)) => 2,
			Inst::ORA(ORA::ZeroPage(..)) => 2,
			Inst::ORA(ORA::ZeroPageX(..)) => 2,
			Inst::ORA(ORA::Absolute(..)) => 3,
			Inst::ORA(ORA::AbsoluteX(..)) => 3,
			Inst::ORA(ORA::AbsoluteY(..)) => 3,
			Inst::ORA(ORA::IndirectX(..)) => 2,
			Inst::ORA(ORA::IndirectY(..)) => 2,
			Inst::PHA => 1,
			Inst::PHP => 1,
			Inst::PLA => 1,
			Inst::PLP => 1,
			Inst::ROL(ROL::Accumulator) => 1,
			Inst::ROL(ROL::ZeroPage(..)) => 2,
			Inst::ROL(ROL::ZeroPageX(..)) => 2,
			Inst::ROL(ROL::Absolute(..)) => 3,
			Inst::ROL(ROL::AbsoluteX(..)) => 3,
			Inst::ROR(ROR::Accumulator) => 1,
			Inst::ROR(ROR::ZeroPage(..)) => 2,
			Inst::ROR(ROR::ZeroPageX(..)) => 2,
			Inst::ROR(ROR::Absolute(..)) => 3,
			Inst::ROR(ROR::AbsoluteX(..)) => 3,
			Inst::RTI => 1,
			Inst::RTS => 1,
			Inst::SBC(SBC::Immediate(..)) => 2,
			Inst::SBC(SBC::ZeroPage(..)) => 2,
			Inst::SBC(SBC::ZeroPageX(..)) => 2,
			Inst::SBC(SBC::Absolute(..)) => 3,
			Inst::SBC(SBC::AbsoluteX(..)) => 3,
			Inst::SBC(SBC::AbsoluteY(..)) => 3,
			Inst::SBC(SBC::IndirectX(..)) => 2,
			Inst::SBC(SBC::IndirectY(..)) => 2,
			Inst::SEC => 1,
			Inst::SED => 1,
			Inst::SEI => 1,
			Inst::STA(STA::ZeroPage(..)) => 2,
			Inst::STA(STA::ZeroPageX(..)) => 2,
			Inst::STA(STA::Absolute(..)) => 3,
			Inst::STA(STA::AbsoluteX(..)) => 3,
			Inst::STA(STA::AbsoluteY(..)) => 3,
			Inst::STA(STA::IndirectX(..)) => 2,
			Inst::STA(STA::IndirectY(..)) => 2,
			Inst::STX(STX::ZeroPage(..)) => 2,
			Inst::STX(STX::ZeroPageY(..)) => 2,
			Inst::STX(STX::Absolute(..)) => 3,
			Inst::STY(STY::ZeroPage(..)) => 2,
			Inst::STY(STY::ZeroPageX(..)) => 2,
			Inst::STY(STY::Absolute(..)) => 3,
			Inst::TAX => 1,
			Inst::TAY => 1,
			Inst::TSX => 1,
			Inst::TXA => 1,
			Inst::TXS => 1,
			Inst::TYA => 1,
			Inst::LAX(LAX::ZeroPage(..)) => 2,
			Inst::LAX(LAX::ZeroPageY(..)) => 2,
			Inst::LAX(LAX::Absolute(..)) => 3,
			Inst::LAX(LAX::AbsoluteY(..)) => 3,
			Inst::LAX(LAX::IndirectX(..)) => 2,
			Inst::LAX(LAX::IndirectY(..)) => 2,
			Inst::SAX(SAX::ZeroPage(..)) => 2,
			Inst::SAX(SAX::ZeroPageY(..)) => 2,
			Inst::SAX(SAX::Absolute(..)) => 3,
			Inst::SAX(SAX::IndirectX(..)) => 2,
			Inst::DCP(DCP::ZeroPage(..)) => 2,
			Inst::DCP(DCP::ZeroPageX(..)) => 2,
			Inst::DCP(DCP::Absolute(..)) => 3,
			Inst::DCP(DCP::AbsoluteX(..)) => 3,
			Inst::DCP(DCP::AbsoluteY(..)) => 3,
			Inst::DCP(DCP::IndirectX(..)) => 2,
			Inst::DCP(DCP::IndirectY(..)) => 2,
			Inst::ISC(ISC::ZeroPage(..)) => 2,
			Inst::ISC(ISC::ZeroPageX(..)) => 2,
			Inst::ISC(ISC::Absolute(..)) => 3,
			Inst::ISC(ISC::AbsoluteX(..)) => 3,
			Inst::ISC(ISC::AbsoluteY(..)) => 3,
			Inst::ISC(ISC::IndirectX(..)) => 2,
			Inst::ISC(ISC::IndirectY(..)) => 2,
			Inst::RLA(RLA::ZeroPage(..)) => 2,
			Inst::RLA(RLA::ZeroPageX(..)) => 2,
			Inst::RLA(RLA::Absolute(..)) => 3,
			Inst::RLA(RLA::AbsoluteX(..)) => 3,
			Inst::RLA(RLA::AbsoluteY(..)) => 3,
			Inst::RLA(RLA::IndirectX(..)) => 2,
			Inst::RLA(RLA::IndirectY(..)) => 2,
			Inst::RRA(RRA::ZeroPage(..)) => 2,
			Inst::RRA(RRA::ZeroPageX(..)) => 2,
			Inst::RRA(RRA::Absolute(..)) => 3,
			Inst::RRA(RRA::AbsoluteX(..)) => 3,
			Inst::RRA(RRA::AbsoluteY(..)) => 3,
			Inst::RRA(RRA::IndirectX(..)) => 2,
			Inst::RRA(RRA::IndirectY(..)) => 2,
			Inst::SLO(SLO::ZeroPage(..)) => 2,
			Inst::SLO(SLO::ZeroPageX(..)) => 2,
			Inst::SLO(SLO::Absolute(..)) => 3,
			Inst::SLO(SLO::AbsoluteX(..)) => 3,
			Inst::SLO(SLO::AbsoluteY(..)) => 3,
			Inst::SLO(SLO::IndirectX(..)) => 2,
			Inst::SLO(SLO::IndirectY(..)) => 2,
			Inst::SRE(SRE::ZeroPage(..)) => 2,
			Inst::SRE(SRE::ZeroPageX(..)) => 2,
			Inst::SRE(SRE::Absolute(..)) => 3,
			Inst::SRE(SRE::AbsoluteX(..)) => 3,
			Inst::SRE(SRE::AbsoluteY(..)) => 3,
			Inst::SRE(SRE::IndirectX(..)) => 2,
			Inst::SRE(SRE::IndirectY(..)) => 2,
			Inst::ANC(..) => 2,
			Inst::ALR(..) => 2,
			Inst::ARR(..) => 2,
			Inst::AXS(..) => 2,
			Inst::LAS(..) => 3,
			Inst::TAS(..) => 3,
			Inst::SHY(..) => 3,
			Inst::SHX(..) => 3,
			Inst::AHX(AHX::AbsoluteY(..)) => 3,
			Inst::AHX(AHX::IndirectY(..)) => 2,
			Inst::NOPU(NOPU::Immediate(..)) => 2,
			Inst::NOPU(NOPU::ZeroPage(..)) => 2,
			Inst::NOPU(NOPU::ZeroPageX(..)) => 2,
			Inst::NOPU(NOPU::Absolute(..)) => 3,
			Inst::NOPU(NOPU::AbsoluteX(..)) => 3,
			Inst::NOPU(NOPU::AbsoluteY(..)) => 3,
		}
	}

	pub fn evaluate(&self, cpu: &mut Cpu) {
		// Iffy cast, but it's the same type handled by bindgen twice from two different header inclusions.
		let cpu = cpu as *mut Cpu as *mut crate::evaluate_instruction::Cpu;
		unsafe {
			match self {
				Inst::ADC(ADC::Immediate(x)) => adc_immediate(cpu, *x),
				Inst::ADC(ADC::ZeroPage(x)) => adc_zero_page(cpu, *x),
				Inst::ADC(ADC::ZeroPageX(x)) => adc_zero_page_x(cpu, *x),
				Inst::ADC(ADC::Absolute(a)) => adc_absolute(cpu, *a),
				Inst::ADC(ADC::AbsoluteX(a)) => adc_absolute_x(cpu, *a),
				Inst::ADC(ADC::AbsoluteY(a)) => adc_absolute_y(cpu, *a),
				Inst::ADC(ADC::IndirectX(x)) => adc_indirect_x(cpu, *x),
				Inst::ADC(ADC::IndirectY(x)) => adc_indirect_y(cpu, *x),
				Inst::AND(AND::Immediate(x)) => and_immediate(cpu, *x),
				Inst::AND(AND::ZeroPage(x)) => and_zero_page(cpu, *x),
				Inst::AND(AND::ZeroPageX(x)) => and_zero_page_x(cpu, *x),
				Inst::AND(AND::Absolute(a)) => and_absolute(cpu, *a),
				Inst::AND(AND::AbsoluteX(a)) => and_absolute_x(cpu, *a),
				Inst::AND(AND::AbsoluteY(a)) => and_absolute_y(cpu, *a),
				Inst::AND(AND::IndirectX(x)) => and_indirect_x(cpu, *x),
				Inst::AND(AND::IndirectY(x)) => and_indirect_y(cpu, *x),
				Inst::ASL(ASL::Accumulator) => asl_accumulator(cpu),
				Inst::ASL(ASL::ZeroPage(x)) => asl_zero_page(cpu, *x),
				Inst::ASL(ASL::ZeroPageX(x)) => asl_zero_page_x(cpu, *x),
				Inst::ASL(ASL::Absolute(a)) => asl_absolute(cpu, *a),
				Inst::ASL(ASL::AbsoluteX(a)) => asl_absolute_x(cpu, *a),
				Inst::BCC(x) => bcc(cpu, *x),
				Inst::BCS(x) => bcs(cpu, *x),
				Inst::BEQ(x) => beq(cpu, *x),
				Inst::BIT(BIT::ZeroPage(x)) => bit_zero_page(cpu, *x),
				Inst::BIT(BIT::Absolute(a)) => bit_absolute(cpu, *a),
				Inst::BMI(x) => bmi(cpu, *x),
				Inst::BNE(x) => bne(cpu, *x),
				Inst::BPL(x) => bpl(cpu, *x),
				Inst::BRK => brk(cpu),
				Inst::BVC(x) => bvc(cpu, *x),
				Inst::BVS(x) => bvs(cpu, *x),
				Inst::CLC => clc(cpu),
				Inst::CLD => cld(cpu),
				Inst::CLI => cli(cpu),
				Inst::CLV => clv(cpu),
				Inst::CMP(CMP::Immediate(x)) => cmp_immediate(cpu, *x),
				Inst::CMP(CMP::ZeroPage(x)) => cmp_zero_page(cpu, *x),
				Inst::CMP(CMP::ZeroPageX(x)) => cmp_zero_page_x(cpu, *x),
				Inst::CMP(CMP::Absolute(a)) => cmp_absolute(cpu, *a),
				Inst::CMP(CMP::AbsoluteX(a)) => cmp_absolute_x(cpu, *a),
				Inst::CMP(CMP::AbsoluteY(a)) => cmp_absolute_y(cpu, *a),
				Inst::CMP(CMP::IndirectX(x)) => cmp_indirect_x(cpu, *x),
				Inst::CMP(CMP::IndirectY(x)) => cmp_indirect_y(cpu, *x),
				Inst::CPX(CPX::Immediate(x)) => cpx_immediate(cpu, *x),
				Inst::CPX(CPX::ZeroPage(x)) => cpx_zero_page(cpu, *x),
				Inst::CPX(CPX::Absolute(a)) => cpx_absolute(cpu, *a),
				Inst::CPY(CPY::Immediate(x)) => cpy_immediate(cpu, *x),
				Inst::CPY(CPY::ZeroPage(x)) => cpy_zero_page(cpu, *x),
				Inst::CPY(CPY::Absolute(a)) => cpy_absolute(cpu, *a),
				Inst::DEC(DEC::ZeroPage(x)) => dec_zero_page(cpu, *x),
				Inst::DEC(DEC::ZeroPageX(x)) => dec_zero_page_x(cpu, *x),
				Inst::DEC(DEC::Absolute(a)) => dec_absolute(cpu, *a),
				Inst::DEC(DEC::AbsoluteX(a)) => dec_absolute_x(cpu, *a),
				Inst::DEX => dex(cpu),
				Inst::DEY => dey(cpu),
				Inst::EOR(EOR::Immediate(x)) => eor_immediate(cpu, *x),
				Inst::EOR(EOR::ZeroPage(x)) => eor_zero_page(cpu, *x),
				Inst::EOR(EOR::ZeroPageX(x)) => eor_zero_page_x(cpu, *x),
				Inst::EOR(EOR::Absolute(a)) => eor_absolute(cpu, *a),
				Inst::EOR(EOR::AbsoluteX(a)) => eor_absolute_x(cpu, *a),
				Inst::EOR(EOR::AbsoluteY(a)) => eor_absolute_y(cpu, *a),
				Inst::EOR(EOR::IndirectX(x)) => eor_indirect_x(cpu, *x),
				Inst::EOR(EOR::IndirectY(x)) => eor_indirect_y(cpu, *x),
				Inst::INC(INC::ZeroPage(x)) => inc_zero_page(cpu, *x),
				Inst::INC(INC::ZeroPageX(x)) => inc_zero_page_x(cpu, *x),
				Inst::INC(INC::Absolute(a)) => inc_absolute(cpu, *a),
				Inst::INC(INC::AbsoluteX(a)) => inc_absolute_x(cpu, *a),
				Inst::INX => inx(cpu),
				Inst::INY => iny(cpu),
				Inst::JMP(JMP::Absolute(a)) => jmp_absolute(cpu, *a),
				Inst::JMP(JMP::Indirect(x)) => jmp_indirect(cpu, *x),
				Inst::JSR(x) => jsr(cpu, *x),
				Inst::LDA(LDA::Immediate(x)) => lda_immediate(cpu, *x),
				Inst::LDA(LDA::ZeroPage(x)) => lda_zero_page(cpu, *x),
				Inst::LDA(LDA::ZeroPageX(x)) => lda_zero_page_x(cpu, *x),
				Inst::LDA(LDA::Absolute(a)) => lda_absolute(cpu, *a),
				Inst::LDA(LDA::AbsoluteX(a)) => lda_absolute_x(cpu, *a),
				Inst::LDA(LDA::AbsoluteY(a)) => lda_absolute_y(cpu, *a),
				Inst::LDA(LDA::IndirectX(x)) => lda_indirect_x(cpu, *x),
				Inst::LDA(LDA::IndirectY(x)) => lda_indirect_y(cpu, *x),
				Inst::LDX(LDX::Immediate(x)) => ldx_immediate(cpu, *x),
				Inst::LDX(LDX::ZeroPage(x)) => ldx_zero_page(cpu, *x),
				Inst::LDX(LDX::ZeroPageY(x)) => ldx_zero_page_y(cpu, *x),
				Inst::LDX(LDX::Absolute(a)) => ldx_absolute(cpu, *a),
				Inst::LDX(LDX::AbsoluteY(a)) => ldx_absolute_y(cpu, *a),
				Inst::LDY(LDY::Immediate(x)) => ldy_immediate(cpu, *x),
				Inst::LDY(LDY::ZeroPage(x)) => ldy_zero_page(cpu, *x),
				Inst::LDY(LDY::ZeroPageX(x)) => ldy_zero_page_x(cpu, *x),
				Inst::LDY(LDY::Absolute(a)) => ldy_absolute(cpu, *a),
				Inst::LDY(LDY::AbsoluteX(a)) => ldy_absolute_x(cpu, *a),
				Inst::LSR(LSR::Accumulator) => lsr_accumulator(cpu),
				Inst::LSR(LSR::ZeroPage(x)) => lsr_zero_page(cpu, *x),
				Inst::LSR(LSR::ZeroPageX(x)) => lsr_zero_page_x(cpu, *x),
				Inst::LSR(LSR::Absolute(a)) => lsr_absolute(cpu, *a),
				Inst::LSR(LSR::AbsoluteX(a)) => lsr_absolute_x(cpu, *a),
				Inst::NOP => nop(cpu),
				Inst::ORA(ORA::Immediate(x)) => ora_immediate(cpu, *x),
				Inst::ORA(ORA::ZeroPage(x)) => ora_zero_page(cpu, *x),
				Inst::ORA(ORA::ZeroPageX(x)) => ora_zero_page_x(cpu, *x),
				Inst::ORA(ORA::Absolute(a)) => ora_absolute(cpu, *a),
				Inst::ORA(ORA::AbsoluteX(a)) => ora_absolute_x(cpu, *a),
				Inst::ORA(ORA::AbsoluteY(a)) => ora_absolute_y(cpu, *a),
				Inst::ORA(ORA::IndirectX(x)) => ora_indirect_x(cpu, *x),
				Inst::ORA(ORA::IndirectY(x)) => ora_indirect_y(cpu, *x),
				Inst::PHA => pha(cpu),
				Inst::PHP => php(cpu),
				Inst::PLA => pla(cpu),
				Inst::PLP => plp(cpu),
				Inst::ROL(ROL::Accumulator) => rol_accumulator(cpu),
				Inst::ROL(ROL::ZeroPage(x)) => rol_zero_page(cpu, *x),
				Inst::ROL(ROL::ZeroPageX(x)) => rol_zero_page_x(cpu, *x),
				Inst::ROL(ROL::Absolute(a)) => rol_absolute(cpu, *a),
				Inst::ROL(ROL::AbsoluteX(a)) => rol_absolute_x(cpu, *a),
				Inst::ROR(ROR::Accumulator) => ror_accumulator(cpu),
				Inst::ROR(ROR::ZeroPage(x)) => ror_zero_page(cpu, *x),
				Inst::ROR(ROR::ZeroPageX(x)) => ror_zero_page_x(cpu, *x),
				Inst::ROR(ROR::Absolute(a)) => ror_absolute(cpu, *a),
				Inst::ROR(ROR::AbsoluteX(a)) => ror_absolute_x(cpu, *a),
				Inst::RTI => rti(cpu),
				Inst::RTS => rts(cpu),
				Inst::SBC(SBC::Immediate(x)) => sbc_immediate(cpu, *x),
				Inst::SBC(SBC::ZeroPage(x)) => sbc_zero_page(cpu, *x),
				Inst::SBC(SBC::ZeroPageX(x)) => sbc_zero_page_x(cpu, *x),
				Inst::SBC(SBC::Absolute(a)) => sbc_absolute(cpu, *a),
				Inst::SBC(SBC::AbsoluteX(a)) => sbc_absolute_x(cpu, *a),
				Inst::SBC(SBC::AbsoluteY(a)) => sbc_absolute_y(cpu, *a),
				Inst::SBC(SBC::IndirectX(x)) => sbc_indirect_x(cpu, *x),
				Inst::SBC(SBC::IndirectY(x)) => sbc_indirect_y(cpu, *x),
				Inst::SEC => sec(cpu),
				Inst::SED => sed(cpu),
				Inst::SEI => sei(cpu),
				Inst::STA(STA::ZeroPage(x)) => sta_zero_page(cpu, *x),
				Inst::STA(STA::ZeroPageX(x)) => sta_zero_page_x(cpu, *x),
				Inst::STA(STA::Absolute(a)) => sta_absolute(cpu, *a),
				Inst::STA(STA::AbsoluteX(a)) => sta_absolute_x(cpu, *a),
				Inst::STA(STA::AbsoluteY(a)) => sta_absolute_y(cpu, *a),
				Inst::STA(STA::IndirectX(x)) => sta_indirect_x(cpu, *x),
				Inst::STA(STA::IndirectY(x)) => sta_indirect_y(cpu, *x),
				Inst::STX(STX::ZeroPage(x)) => stx_zero_page(cpu, *x),
				Inst::STX(STX::ZeroPageY(x)) => stx_zero_page_y(cpu, *x),
				Inst::STX(STX::Absolute(a)) => stx_absolute(cpu, *a),
				Inst::STY(STY::ZeroPage(x)) => sty_zero_page(cpu, *x),
				Inst::STY(STY::ZeroPageX(x)) => sty_zero_page_x(cpu, *x),
				Inst::STY(STY::Absolute(a)) => sty_absolute(cpu, *a),
				Inst::TAX => tax(cpu),
				Inst::TAY => tay(cpu),
				Inst::TSX => tsx(cpu),
				Inst::TXA => txa(cpu),
				Inst::TXS => txs(cpu),
				Inst::TYA => tya(cpu),
				// Inst::LAX(LAX::ZeroPage(x)) => lax_zero_page(cpu, *x),
				// Inst::LAX(LAX::ZeroPageY(x)) => lax_zero_page_y(cpu, *x),
				// Inst::LAX(LAX::Absolute(a)) => lax_absolute(cpu, *a),
				// Inst::LAX(LAX::AbsoluteY(a)) => lax_absolute_y(cpu, *a),
				// Inst::LAX(LAX::IndirectX(x)) => lax_indirect_x(cpu, *x),
				// Inst::LAX(LAX::IndirectY(x)) => lax_indirect_y(cpu, *x),
				// Inst::SAX(SAX::ZeroPage(x)) => sax_zero_page(cpu, *x),
				// Inst::SAX(SAX::ZeroPageY(x)) => sax_zero_page_y(cpu, *x),
				// Inst::SAX(SAX::Absolute(a)) => sax_absolute(cpu, *a),
				// Inst::SAX(SAX::IndirectX(x)) => sax_indirect_x(cpu, *x),
				// Inst::DCP(DCP::ZeroPage(x)) => dcp_zero_page(cpu, *x),
				// Inst::DCP(DCP::ZeroPageX(x)) => dcp_zero_page_x(cpu, *x),
				// Inst::DCP(DCP::Absolute(a)) => dcp_absolute(cpu, *a),
				// Inst::DCP(DCP::AbsoluteX(a)) => dcp_absolute_x(cpu, *a),
				// Inst::DCP(DCP::AbsoluteY(a)) => dcp_absolute_y(cpu, *a),
				// Inst::DCP(DCP::IndirectX(x)) => dcp_indirect_x(cpu, *x),
				// Inst::DCP(DCP::IndirectY(x)) => dcp_indirect_y(cpu, *x),
				// Inst::ISC(ISC::ZeroPage(x)) => isc_zero_page(cpu, *x),
				// Inst::ISC(ISC::ZeroPageX(x)) => isc_zero_page_x(cpu, *x),
				// Inst::ISC(ISC::Absolute(a)) => isc_absolute(cpu, *a),
				// Inst::ISC(ISC::AbsoluteX(a)) => isc_absolute_x(cpu, *a),
				// Inst::ISC(ISC::AbsoluteY(a)) => isc_absolute_y(cpu, *a),
				// Inst::ISC(ISC::IndirectX(x)) => isc_indirect_x(cpu, *x),
				// Inst::ISC(ISC::IndirectY(x)) => isc_indirect_y(cpu, *x),
				// Inst::RLA(RLA::ZeroPage(x)) => rla_zero_page(cpu, *x),
				// Inst::RLA(RLA::ZeroPageX(x)) => rla_zero_page_x(cpu, *x),
				// Inst::RLA(RLA::Absolute(a)) => rla_absolute(cpu, *a),
				// Inst::RLA(RLA::AbsoluteX(a)) => rla_absolute_x(cpu, *a),
				// Inst::RLA(RLA::AbsoluteY(a)) => rla_absolute_y(cpu, *a),
				// Inst::RLA(RLA::IndirectX(x)) => rla_indirect_x(cpu, *x),
				// Inst::RLA(RLA::IndirectY(x)) => rla_indirect_y(cpu, *x),
				// Inst::RRA(RRA::ZeroPage(x)) => rra_zero_page(cpu, *x),
				// Inst::RRA(RRA::ZeroPageX(x)) => rra_zero_page_x(cpu, *x),
				// Inst::RRA(RRA::Absolute(a)) => rra_absolute(cpu, *a),
				// Inst::RRA(RRA::AbsoluteX(a)) => rra_absolute_x(cpu, *a),
				// Inst::RRA(RRA::AbsoluteY(a)) => rra_absolute_y(cpu, *a),
				// Inst::RRA(RRA::IndirectX(x)) => rra_indirect_x(cpu, *x),
				// Inst::RRA(RRA::IndirectY(x)) => rra_indirect_y(cpu, *x),
				// Inst::SLO(SLO::ZeroPage(x)) => slo_zero_page(cpu, *x),
				// Inst::SLO(SLO::ZeroPageX(x)) => slo_zero_page_x(cpu, *x),
				// Inst::SLO(SLO::Absolute(a)) => slo_absolute(cpu, *a),
				// Inst::SLO(SLO::AbsoluteX(a)) => slo_absolute_x(cpu, *a),
				// Inst::SLO(SLO::AbsoluteY(a)) => slo_absolute_y(cpu, *a),
				// Inst::SLO(SLO::IndirectX(x)) => slo_indirect_x(cpu, *x),
				// Inst::SLO(SLO::IndirectY(x)) => slo_indirect_y(cpu, *x),
				// Inst::SRE(SRE::ZeroPage(x)) => sre_zero_page(cpu, *x),
				// Inst::SRE(SRE::ZeroPageX(x)) => sre_zero_page_x(cpu, *x),
				// Inst::SRE(SRE::Absolute(a)) => sre_absolute(cpu, *a),
				// Inst::SRE(SRE::AbsoluteX(a)) => sre_absolute_x(cpu, *a),
				// Inst::SRE(SRE::AbsoluteY(a)) => sre_absolute_y(cpu, *a),
				// Inst::SRE(SRE::IndirectX(x)) => sre_indirect_x(cpu, *x),
				// Inst::SRE(SRE::IndirectY(x)) => sre_indirect_y(cpu, *x),
				// Inst::ANC(x) => anc(cpu, *x),
				// Inst::ALR(x) => alr(cpu, *x),
				// Inst::ARR(x) => arr(cpu, *x),
				// Inst::AXS(x) => axs(cpu, *x),
				// Inst::LAS(x) => las(cpu, *x),
				// Inst::TAS(x) => tas(cpu, *x),
				// Inst::SHY(x) => shy(cpu, *x),
				// Inst::SHX(x) => shx(cpu, *x),
				// Inst::AHX(AHX::AbsoluteY(a)) => ahx_absolute_y(cpu, *a),
				// Inst::AHX(AHX::IndirectY(x)) => ahx_indirect_y(cpu, *x),
				// Inst::NOPU(..) => {}
				_ => todo!("No support for unofficial instructions yet")
			}
		}
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
		x => bail!("Unknown opcode: {:02x?}", &x),
	}
}
