#![allow(unused, clippy::upper_case_acronyms)]

use crate::{cpu::Cpu, evaluate_instruction::*, interpret::State};

use anyhow::{Result, bail};

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UnalignedU16 {
	lo: u8,
	hi: u8,
}

impl std::fmt::Debug for UnalignedU16 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let x: u16 = (*self).into();
		write!(f, "{}", x)
	}
}

impl From<u16> for UnalignedU16 {
	fn from(val: u16) -> Self {
		let [lo, hi] = val.to_le_bytes();
		Self { lo, hi }
	}
}

impl From<UnalignedU16> for u16 {
	fn from(val: UnalignedU16) -> Self {
		u16::from_le_bytes([val.lo, val.hi])
	}
}

impl From<&UnalignedU16> for u16 {
	fn from(val: &UnalignedU16) -> Self {
		(*val).into()
	}
}

// Auto-generated NES CPU instruction set
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Inst {
	ADCAbsolute(UnalignedU16) = 0x6D,
	ADCAbsoluteX(UnalignedU16) = 0x7D,
	ADCAbsoluteY(UnalignedU16) = 0x79,
	ADCImmediate(u8) = 0x69,
	ADCIndirectX(u8) = 0x61,
	ADCIndirectY(u8) = 0x71,
	ADCZeroPage(u8) = 0x65,
	ADCZeroPageX(u8) = 0x75,
	AHXAbsoluteY(UnalignedU16) = 0x9F,
	AHXIndirectY(u8) = 0x93,
	ALRImmediate(u8) = 0x4B,
	ANC2Immediate(u8) = 0x2B,
	ANCImmediate(u8) = 0x0B,
	ANDAbsolute(UnalignedU16) = 0x2D,
	ANDAbsoluteX(UnalignedU16) = 0x3D,
	ANDAbsoluteY(UnalignedU16) = 0x39,
	ANDImmediate(u8) = 0x29,
	ANDIndirectX(u8) = 0x21,
	ANDIndirectY(u8) = 0x31,
	ANDZeroPage(u8) = 0x25,
	ANDZeroPageX(u8) = 0x35,
	ARRImmediate(u8) = 0x6B,
	ASLAbsolute(UnalignedU16) = 0x0E,
	ASLAbsoluteX(UnalignedU16) = 0x1E,
	ASLAccumulator = 0x0A,
	ASLZeroPage(u8) = 0x06,
	ASLZeroPageX(u8) = 0x16,
	AXSImmediate(u8) = 0xCB,
	BCC(i8) = 0x90,
	BCS(i8) = 0xB0,
	BEQ(i8) = 0xF0,
	BITAbsolute(UnalignedU16) = 0x2C,
	BITZeroPage(u8) = 0x24,
	BMI(i8) = 0x30,
	BNE(i8) = 0xD0,
	BPL(i8) = 0x10,
	BRK = 0x00,
	BVC(i8) = 0x50,
	BVS(i8) = 0x70,
	CLC = 0x18,
	CLD = 0xD8,
	CLI = 0x58,
	CLV = 0xB8,
	CMPAbsolute(UnalignedU16) = 0xCD,
	CMPAbsoluteX(UnalignedU16) = 0xDD,
	CMPAbsoluteY(UnalignedU16) = 0xD9,
	CMPImmediate(u8) = 0xC9,
	CMPIndirectX(u8) = 0xC1,
	CMPIndirectY(u8) = 0xD1,
	CMPZeroPage(u8) = 0xC5,
	CMPZeroPageX(u8) = 0xD5,
	CPXAbsolute(UnalignedU16) = 0xEC,
	CPXImmediate(u8) = 0xE0,
	CPXZeroPage(u8) = 0xE4,
	CPYAbsolute(UnalignedU16) = 0xCC,
	CPYImmediate(u8) = 0xC0,
	CPYZeroPage(u8) = 0xC4,
	DCPAbsolute(UnalignedU16) = 0xCF,
	DCPAbsoluteX(UnalignedU16) = 0xDF,
	DCPAbsoluteY(UnalignedU16) = 0xDB,
	DCPIndirectX(u8) = 0xC3,
	DCPIndirectY(u8) = 0xD3,
	DCPZeroPage(u8) = 0xC7,
	DCPZeroPageX(u8) = 0xD7,
	DECAbsolute(UnalignedU16) = 0xCE,
	DECAbsoluteX(UnalignedU16) = 0xDE,
	DECZeroPage(u8) = 0xC6,
	DECZeroPageX(u8) = 0xD6,
	DEX = 0xCA,
	DEY = 0x88,
	EORAbsolute(UnalignedU16) = 0x4D,
	EORAbsoluteX(UnalignedU16) = 0x5D,
	EORAbsoluteY(UnalignedU16) = 0x59,
	EORImmediate(u8) = 0x49,
	EORIndirectX(u8) = 0x41,
	EORIndirectY(u8) = 0x51,
	EORZeroPage(u8) = 0x45,
	EORZeroPageX(u8) = 0x55,
	INCAbsolute(UnalignedU16) = 0xEE,
	INCAbsoluteX(UnalignedU16) = 0xFE,
	INCZeroPage(u8) = 0xE6,
	INCZeroPageX(u8) = 0xF6,
	INX = 0xE8,
	INY = 0xC8,
	ISCAbsolute(UnalignedU16) = 0xEF,
	ISCAbsoluteX(UnalignedU16) = 0xFF,
	ISCAbsoluteY(UnalignedU16) = 0xFB,
	ISCIndirectX(u8) = 0xE3,
	ISCIndirectY(u8) = 0xF3,
	ISCZeroPage(u8) = 0xE7,
	ISCZeroPageX(u8) = 0xF7,
	JMPAbsolute(UnalignedU16) = 0x4C,
	JMPIndirect(UnalignedU16) = 0x6C,
	JSR(UnalignedU16) = 0x20,
	LASAbsoluteY(UnalignedU16) = 0xBB,
	LAXAbsolute(UnalignedU16) = 0xAF,
	LAXAbsoluteY(UnalignedU16) = 0xBF,
	LAXImmediate(u8) = 0xAB,
	LAXIndirectX(u8) = 0xA3,
	LAXIndirectY(u8) = 0xB3,
	LAXZeroPage(u8) = 0xA7,
	LAXZeroPageY(u8) = 0xB7,
	LDAAbsolute(UnalignedU16) = 0xAD,
	LDAAbsoluteX(UnalignedU16) = 0xBD,
	LDAAbsoluteY(UnalignedU16) = 0xB9,
	LDAImmediate(u8) = 0xA9,
	LDAIndirectX(u8) = 0xA1,
	LDAIndirectY(u8) = 0xB1,
	LDAZeroPage(u8) = 0xA5,
	LDAZeroPageX(u8) = 0xB5,
	LDXAbsolute(UnalignedU16) = 0xAE,
	LDXAbsoluteY(UnalignedU16) = 0xBE,
	LDXImmediate(u8) = 0xA2,
	LDXZeroPage(u8) = 0xA6,
	LDXZeroPageY(u8) = 0xB6,
	LDYAbsolute(UnalignedU16) = 0xAC,
	LDYAbsoluteX(UnalignedU16) = 0xBC,
	LDYImmediate(u8) = 0xA0,
	LDYZeroPage(u8) = 0xA4,
	LDYZeroPageX(u8) = 0xB4,
	LSRAbsolute(UnalignedU16) = 0x4E,
	LSRAbsoluteX(UnalignedU16) = 0x5E,
	LSRAccumulator = 0x4A,
	LSRZeroPage(u8) = 0x46,
	LSRZeroPageX(u8) = 0x56,
	NOP10 = 0x5A,
	NOP11 = 0x62,
	NOP12 = 0x72,
	NOP13 = 0x7A,
	NOP14 = 0x82,
	NOP15 = 0x92,
	NOP16 = 0xB2,
	NOP17 = 0xC2,
	NOP18 = 0xD2,
	NOP19 = 0xDA,
	NOP2 = 0x02,
	NOP20 = 0xE2,
	NOP21 = 0xF2,
	NOP22 = 0xFA,
	NOP3 = 0x12,
	NOP4 = 0x1A,
	NOP5 = 0x22,
	NOP6 = 0x32,
	NOP7 = 0x3A,
	NOP8 = 0x42,
	NOP9 = 0x52,
	NOPAbsolute(UnalignedU16) = 0x0C,
	NOPAbsoluteX(UnalignedU16) = 0x1C,
	NOPAbsoluteX2(UnalignedU16) = 0x3C,
	NOPAbsoluteX3(UnalignedU16) = 0x5C,
	NOPAbsoluteX4(UnalignedU16) = 0x7C,
	NOPAbsoluteX5(UnalignedU16) = 0xDC,
	NOPAbsoluteX6(UnalignedU16) = 0xFC,
	NOPImmediate(u8) = 0x80,
	NOPImmediate2(u8) = 0x89,
	NOPImmediate3(u8) = 0xEA,
	NOPZeroPage(u8) = 0x04,
	NOPZeroPage3(u8) = 0x44,
	NOPZeroPage4(u8) = 0x64,
	NOPZeroPageX(u8) = 0x14,
	NOPZeroPageX2(u8) = 0x34,
	NOPZeroPageX3(u8) = 0x54,
	NOPZeroPageX4(u8) = 0x74,
	NOPZeroPageX5(u8) = 0xD4,
	NOPZeroPageX6(u8) = 0xF4,
	ORAAbsolute(UnalignedU16) = 0x0D,
	ORAAbsoluteX(UnalignedU16) = 0x1D,
	ORAAbsoluteY(UnalignedU16) = 0x19,
	ORAImmediate(u8) = 0x09,
	ORAIndirectX(u8) = 0x01,
	ORAIndirectY(u8) = 0x11,
	ORAZeroPage(u8) = 0x05,
	ORAZeroPageX(u8) = 0x15,
	PHA = 0x48,
	PHP = 0x08,
	PLA = 0x68,
	PLP = 0x28,
	RLAAbsolute(UnalignedU16) = 0x2F,
	RLAAbsoluteX(UnalignedU16) = 0x3F,
	RLAAbsoluteY(UnalignedU16) = 0x3B,
	RLAIndirectX(u8) = 0x23,
	RLAIndirectY(u8) = 0x33,
	RLAZeroPage(u8) = 0x27,
	RLAZeroPageX(u8) = 0x37,
	ROLAbsolute(UnalignedU16) = 0x2E,
	ROLAbsoluteX(UnalignedU16) = 0x3E,
	ROLAccumulator = 0x2A,
	ROLZeroPage(u8) = 0x26,
	ROLZeroPageX(u8) = 0x36,
	RORAbsolute(UnalignedU16) = 0x6E,
	RORAbsoluteX(UnalignedU16) = 0x7E,
	RORAccumulator = 0x6A,
	RORZeroPage(u8) = 0x66,
	RORZeroPageX(u8) = 0x76,
	RRAAbsolute(UnalignedU16) = 0x6F,
	RRAAbsoluteX(UnalignedU16) = 0x7F,
	RRAAbsoluteY(UnalignedU16) = 0x7B,
	RRAIndirectX(u8) = 0x63,
	RRAIndirectY(u8) = 0x73,
	RRAZeroPage(u8) = 0x67,
	RRAZeroPageX(u8) = 0x77,
	RTI = 0x40,
	RTS = 0x60,
	SAXAbsolute(UnalignedU16) = 0x8F,
	SAXIndirectX(u8) = 0x83,
	SAXZeroPage(u8) = 0x87,
	SAXZeroPageY(u8) = 0x97,
	SBCAbsolute(UnalignedU16) = 0xED,
	SBCAbsoluteX(UnalignedU16) = 0xFD,
	SBCAbsoluteY(UnalignedU16) = 0xF9,
	SBCImmediate(u8) = 0xE9,
	SBCImmediate2(u8) = 0xEB,
	SBCIndirectX(u8) = 0xE1,
	SBCIndirectY(u8) = 0xF1,
	SBCZeroPage(u8) = 0xE5,
	SBCZeroPageX(u8) = 0xF5,
	SEC = 0x38,
	SED = 0xF8,
	SEI = 0x78,
	SHXAbsoluteY(UnalignedU16) = 0x9E,
	SHYAbsoluteX(UnalignedU16) = 0x9C,
	SLOAbsolute(UnalignedU16) = 0x0F,
	SLOAbsoluteX(UnalignedU16) = 0x1F,
	SLOAbsoluteY(UnalignedU16) = 0x1B,
	SLOIndirectX(u8) = 0x03,
	SLOIndirectY(u8) = 0x13,
	SLOZeroPage(u8) = 0x07,
	SLOZeroPageX(u8) = 0x17,
	SREAbsolute(UnalignedU16) = 0x4F,
	SREAbsoluteX(UnalignedU16) = 0x5F,
	SREAbsoluteY(UnalignedU16) = 0x5B,
	SREIndirectX(u8) = 0x43,
	SREIndirectY(u8) = 0x53,
	SREZeroPage(u8) = 0x47,
	SREZeroPageX(u8) = 0x57,
	STAAbsolute(UnalignedU16) = 0x8D,
	STAAbsoluteX(UnalignedU16) = 0x9D,
	STAAbsoluteY(UnalignedU16) = 0x99,
	STAIndirectX(u8) = 0x81,
	STAIndirectY(u8) = 0x91,
	STAZeroPage(u8) = 0x85,
	STAZeroPageX(u8) = 0x95,
	STXAbsolute(UnalignedU16) = 0x8E,
	STXZeroPage(u8) = 0x86,
	STXZeroPageY(u8) = 0x96,
	STYAbsolute(UnalignedU16) = 0x8C,
	STYZeroPage(u8) = 0x84,
	STYZeroPageX(u8) = 0x94,
	TASAbsoluteY(UnalignedU16) = 0x9B,
	TAX = 0xAA,
	TAY = 0xA8,
	TSX = 0xBA,
	TXA = 0x8A,
	TXS = 0x9A,
	TYA = 0x98,
	XAAImmediate(u8) = 0x8B,
}

const _: () = {
	assert!(1 == align_of::<Inst>());
	assert!(3 == size_of::<Inst>());
};

impl Inst {
	pub fn ends_bb(&self) -> bool {
		match self {
			Inst::BCC(..) => true,
			Inst::BCS(..) => true,
			Inst::BEQ(..) => true,
			Inst::BMI(..) => true,
			Inst::BNE(..) => true,
			Inst::BPL(..) => true,
			Inst::BVC(..) => true,
			Inst::BVS(..) => true,
			Inst::JMPIndirect(..) => true,
			Inst::JMPAbsolute(..) => true,
			Inst::JSR(..) => true,
			Inst::RTI => true,
			Inst::RTS => true,
			_ => false,
		}
	}

	pub fn len(&self) -> u8 {
		match self {
			Inst::BRK
			| Inst::CLC
			| Inst::CLD
			| Inst::CLI
			| Inst::CLV
			| Inst::DEX
			| Inst::DEY
			| Inst::INX
			| Inst::INY
			| Inst::NOP2
			| Inst::NOP3
			| Inst::NOP4
			| Inst::NOP5
			| Inst::NOP6
			| Inst::NOP7
			| Inst::NOP8
			| Inst::NOP9
			| Inst::NOP10
			| Inst::NOP11
			| Inst::NOP12
			| Inst::NOP13
			| Inst::NOP14
			| Inst::NOP15
			| Inst::NOP16
			| Inst::NOP17
			| Inst::NOP18
			| Inst::NOP19
			| Inst::NOP20
			| Inst::NOP21
			| Inst::NOP22
			| Inst::PHA
			| Inst::PHP
			| Inst::PLA
			| Inst::PLP
			| Inst::RTI
			| Inst::RTS
			| Inst::SEC
			| Inst::SED
			| Inst::SEI
			| Inst::TAX
			| Inst::TAY
			| Inst::TSX
			| Inst::TXA
			| Inst::TXS
			| Inst::TYA
			| Inst::ASLAccumulator
			| Inst::LSRAccumulator
			| Inst::ROLAccumulator
			| Inst::RORAccumulator => 1,
			Inst::ORAImmediate(..)
			| Inst::ORAZeroPage(..)
			| Inst::ORAZeroPageX(..)
			| Inst::ORAIndirectX(..)
			| Inst::ORAIndirectY(..)
			| Inst::ANDImmediate(..)
			| Inst::ANDZeroPage(..)
			| Inst::ANDZeroPageX(..)
			| Inst::ANDIndirectX(..)
			| Inst::ANDIndirectY(..)
			| Inst::EORImmediate(..)
			| Inst::EORZeroPage(..)
			| Inst::EORZeroPageX(..)
			| Inst::EORIndirectX(..)
			| Inst::EORIndirectY(..)
			| Inst::ADCImmediate(..)
			| Inst::ADCZeroPage(..)
			| Inst::ADCZeroPageX(..)
			| Inst::ADCIndirectX(..)
			| Inst::ADCIndirectY(..)
			| Inst::LDAImmediate(..)
			| Inst::LDAZeroPage(..)
			| Inst::LDAZeroPageX(..)
			| Inst::LDAIndirectX(..)
			| Inst::LDAIndirectY(..)
			| Inst::LDXImmediate(..)
			| Inst::LDXZeroPage(..)
			| Inst::LDXZeroPageY(..)
			| Inst::LDYImmediate(..)
			| Inst::LDYZeroPage(..)
			| Inst::LDYZeroPageX(..)
			| Inst::STAZeroPage(..)
			| Inst::STAZeroPageX(..)
			| Inst::STAIndirectX(..)
			| Inst::STAIndirectY(..)
			| Inst::STXZeroPage(..)
			| Inst::STXZeroPageY(..)
			| Inst::STYZeroPage(..)
			| Inst::STYZeroPageX(..)
			| Inst::CMPImmediate(..)
			| Inst::CMPZeroPage(..)
			| Inst::CMPZeroPageX(..)
			| Inst::CMPIndirectX(..)
			| Inst::CMPIndirectY(..)
			| Inst::CPXImmediate(..)
			| Inst::CPXZeroPage(..)
			| Inst::CPYImmediate(..)
			| Inst::CPYZeroPage(..)
			| Inst::SBCImmediate(..)
			| Inst::SBCZeroPage(..)
			| Inst::SBCZeroPageX(..)
			| Inst::SBCIndirectX(..)
			| Inst::SBCIndirectY(..)
			| Inst::BITZeroPage(..)
			| Inst::ASLZeroPage(..)
			| Inst::ASLZeroPageX(..)
			| Inst::LSRZeroPage(..)
			| Inst::LSRZeroPageX(..)
			| Inst::ROLZeroPage(..)
			| Inst::ROLZeroPageX(..)
			| Inst::RORZeroPage(..)
			| Inst::RORZeroPageX(..)
			| Inst::DECZeroPage(..)
			| Inst::DECZeroPageX(..)
			| Inst::INCZeroPage(..)
			| Inst::INCZeroPageX(..)
			| Inst::BPL(..)
			| Inst::BMI(..)
			| Inst::BVC(..)
			| Inst::BVS(..)
			| Inst::BCC(..)
			| Inst::BCS(..)
			| Inst::BNE(..)
			| Inst::BEQ(..)
			| Inst::ANCImmediate(..)
			| Inst::ANC2Immediate(..)
			| Inst::ALRImmediate(..)
			| Inst::ARRImmediate(..)
			| Inst::AXSImmediate(..)
			| Inst::LAXZeroPage(..)
			| Inst::LAXZeroPageY(..)
			| Inst::LAXIndirectX(..)
			| Inst::LAXIndirectY(..)
			| Inst::SAXZeroPage(..)
			| Inst::SAXZeroPageY(..)
			| Inst::SAXIndirectX(..)
			| Inst::DCPZeroPage(..)
			| Inst::DCPZeroPageX(..)
			| Inst::DCPIndirectX(..)
			| Inst::DCPIndirectY(..)
			| Inst::ISCZeroPage(..)
			| Inst::ISCZeroPageX(..)
			| Inst::ISCIndirectX(..)
			| Inst::ISCIndirectY(..)
			| Inst::RLAZeroPage(..)
			| Inst::RLAZeroPageX(..)
			| Inst::RLAIndirectX(..)
			| Inst::RLAIndirectY(..)
			| Inst::RRAZeroPage(..)
			| Inst::RRAZeroPageX(..)
			| Inst::RRAIndirectX(..)
			| Inst::RRAIndirectY(..)
			| Inst::SLOZeroPage(..)
			| Inst::SLOZeroPageX(..)
			| Inst::SLOIndirectX(..)
			| Inst::SLOIndirectY(..)
			| Inst::SREZeroPage(..)
			| Inst::SREZeroPageX(..)
			| Inst::SREIndirectX(..)
			| Inst::SREIndirectY(..)
			| Inst::AHXIndirectY(..)
			| Inst::NOPImmediate(..)
			| Inst::NOPImmediate2(..)
			| Inst::NOPImmediate3(..) => 2,
			Inst::ORAAbsolute(..)
			| Inst::ORAAbsoluteX(..)
			| Inst::ORAAbsoluteY(..)
			| Inst::ANDAbsolute(..)
			| Inst::ANDAbsoluteX(..)
			| Inst::ANDAbsoluteY(..)
			| Inst::EORAbsolute(..)
			| Inst::EORAbsoluteX(..)
			| Inst::EORAbsoluteY(..)
			| Inst::ADCAbsolute(..)
			| Inst::ADCAbsoluteX(..)
			| Inst::ADCAbsoluteY(..)
			| Inst::LDAAbsolute(..)
			| Inst::LDAAbsoluteX(..)
			| Inst::LDAAbsoluteY(..)
			| Inst::LDXAbsolute(..)
			| Inst::LDXAbsoluteY(..)
			| Inst::LDYAbsolute(..)
			| Inst::LDYAbsoluteX(..)
			| Inst::STAAbsolute(..)
			| Inst::STAAbsoluteX(..)
			| Inst::STAAbsoluteY(..)
			| Inst::STXAbsolute(..)
			| Inst::STYAbsolute(..)
			| Inst::CMPAbsolute(..)
			| Inst::CMPAbsoluteX(..)
			| Inst::CMPAbsoluteY(..)
			| Inst::CPXAbsolute(..)
			| Inst::CPYAbsolute(..)
			| Inst::SBCAbsolute(..)
			| Inst::SBCAbsoluteX(..)
			| Inst::SBCAbsoluteY(..)
			| Inst::BITAbsolute(..)
			| Inst::ASLAbsolute(..)
			| Inst::ASLAbsoluteX(..)
			| Inst::LSRAbsolute(..)
			| Inst::LSRAbsoluteX(..)
			| Inst::ROLAbsolute(..)
			| Inst::ROLAbsoluteX(..)
			| Inst::RORAbsolute(..)
			| Inst::RORAbsoluteX(..)
			| Inst::DECAbsolute(..)
			| Inst::DECAbsoluteX(..)
			| Inst::INCAbsolute(..)
			| Inst::INCAbsoluteX(..)
			| Inst::JMPAbsolute(..)
			| Inst::JMPIndirect(..)
			| Inst::JSR(..)
			| Inst::LAXAbsolute(..)
			| Inst::LAXAbsoluteY(..)
			| Inst::SAXAbsolute(..)
			| Inst::DCPAbsolute(..)
			| Inst::DCPAbsoluteX(..)
			| Inst::DCPAbsoluteY(..)
			| Inst::ISCAbsolute(..)
			| Inst::ISCAbsoluteX(..)
			| Inst::ISCAbsoluteY(..)
			| Inst::RLAAbsolute(..)
			| Inst::RLAAbsoluteX(..)
			| Inst::RLAAbsoluteY(..)
			| Inst::RRAAbsolute(..)
			| Inst::RRAAbsoluteX(..)
			| Inst::RRAAbsoluteY(..)
			| Inst::SLOAbsolute(..)
			| Inst::SLOAbsoluteX(..)
			| Inst::SLOAbsoluteY(..)
			| Inst::SREAbsolute(..)
			| Inst::SREAbsoluteX(..)
			| Inst::SREAbsoluteY(..)
			| Inst::LASAbsoluteY(..)
			| Inst::TASAbsoluteY(..)
			| Inst::SHYAbsoluteX(..)
			| Inst::SHXAbsoluteY(..)
			| Inst::AHXAbsoluteY(..)
			| Inst::NOPAbsolute(..)
			| Inst::NOPAbsoluteX(..)
			| Inst::NOPAbsoluteX2(..)
			| Inst::NOPAbsoluteX3(..)
			| Inst::NOPAbsoluteX4(..)
			| Inst::NOPAbsoluteX5(..)
			| Inst::NOPAbsoluteX6(..) => 3,

			Inst::LAXImmediate(_) => todo!(),
			Inst::NOPZeroPage(_) => todo!(),
			Inst::NOPZeroPage3(_) => todo!(),
			Inst::NOPZeroPage4(_) => todo!(),
			Inst::NOPZeroPageX(_) => todo!(),
			Inst::NOPZeroPageX2(_) => todo!(),
			Inst::NOPZeroPageX3(_) => todo!(),
			Inst::NOPZeroPageX4(_) => todo!(),
			Inst::NOPZeroPageX5(_) => todo!(),
			Inst::NOPZeroPageX6(_) => todo!(),
			Inst::SBCImmediate2(_) => todo!(),
			Inst::XAAImmediate(_) => todo!(),
		}
	}

	pub fn evaluate(&self, state: &mut State) {
		match self {
			Inst::ADCImmediate(x) => adc_immediate(state, *x),
			Inst::ADCZeroPage(x) => adc_zero_page(state, *x),
			Inst::ADCZeroPageX(x) => adc_zero_page_x(state, *x),
			Inst::ADCAbsolute(a) => adc_absolute(state, a.into()),
			Inst::ADCAbsoluteX(a) => adc_absolute_x(state, a.into()),
			Inst::ADCAbsoluteY(a) => adc_absolute_y(state, a.into()),
			Inst::ADCIndirectX(x) => adc_indirect_x(state, *x),
			Inst::ADCIndirectY(x) => adc_indirect_y(state, *x),
			Inst::ANDImmediate(x) => and_immediate(state, *x),
			Inst::ANDZeroPage(x) => and_zero_page(state, *x),
			Inst::ANDZeroPageX(x) => and_zero_page_x(state, *x),
			Inst::ANDAbsolute(a) => and_absolute(state, a.into()),
			Inst::ANDAbsoluteX(a) => and_absolute_x(state, a.into()),
			Inst::ANDAbsoluteY(a) => and_absolute_y(state, a.into()),
			Inst::ANDIndirectX(x) => and_indirect_x(state, *x),
			Inst::ANDIndirectY(x) => and_indirect_y(state, *x),
			Inst::ASLAccumulator => asl_accumulator(state),
			Inst::ASLZeroPage(x) => asl_zero_page(state, *x),
			Inst::ASLZeroPageX(x) => asl_zero_page_x(state, *x),
			Inst::ASLAbsolute(a) => asl_absolute(state, a.into()),
			Inst::ASLAbsoluteX(a) => asl_absolute_x(state, a.into()),
			Inst::BCC(x) => bcc(state, *x),
			Inst::BCS(x) => bcs(state, *x),
			Inst::BEQ(x) => beq(state, *x),
			Inst::BITZeroPage(x) => bit_zero_page(state, *x),
			Inst::BITAbsolute(a) => bit_absolute(state, a.into()),
			Inst::BMI(x) => bmi(state, *x),
			Inst::BNE(x) => bne(state, *x),
			Inst::BPL(x) => bpl(state, *x),
			Inst::BRK => brk(state),
			Inst::BVC(x) => bvc(state, *x),
			Inst::BVS(x) => bvs(state, *x),
			Inst::CLC => clc(state),
			Inst::CLD => cld(state),
			Inst::CLI => cli(state),
			Inst::CLV => clv(state),
			Inst::CMPImmediate(x) => cmp_immediate(state, *x),
			Inst::CMPZeroPage(x) => cmp_zero_page(state, *x),
			Inst::CMPZeroPageX(x) => cmp_zero_page_x(state, *x),
			Inst::CMPAbsolute(a) => cmp_absolute(state, a.into()),
			Inst::CMPAbsoluteX(a) => cmp_absolute_x(state, a.into()),
			Inst::CMPAbsoluteY(a) => cmp_absolute_y(state, a.into()),
			Inst::CMPIndirectX(x) => cmp_indirect_x(state, *x),
			Inst::CMPIndirectY(x) => cmp_indirect_y(state, *x),
			Inst::CPXImmediate(x) => cpx_immediate(state, *x),
			Inst::CPXZeroPage(x) => cpx_zero_page(state, *x),
			Inst::CPXAbsolute(a) => cpx_absolute(state, a.into()),
			Inst::CPYImmediate(x) => cpy_immediate(state, *x),
			Inst::CPYZeroPage(x) => cpy_zero_page(state, *x),
			Inst::CPYAbsolute(a) => cpy_absolute(state, a.into()),
			Inst::DECZeroPage(x) => dec_zero_page(state, *x),
			Inst::DECZeroPageX(x) => dec_zero_page_x(state, *x),
			Inst::DECAbsolute(a) => dec_absolute(state, a.into()),
			Inst::DECAbsoluteX(a) => dec_absolute_x(state, a.into()),
			Inst::DEX => dex(state),
			Inst::DEY => dey(state),
			Inst::EORImmediate(x) => eor_immediate(state, *x),
			Inst::EORZeroPage(x) => eor_zero_page(state, *x),
			Inst::EORZeroPageX(x) => eor_zero_page_x(state, *x),
			Inst::EORAbsolute(a) => eor_absolute(state, a.into()),
			Inst::EORAbsoluteX(a) => eor_absolute_x(state, a.into()),
			Inst::EORAbsoluteY(a) => eor_absolute_y(state, a.into()),
			Inst::EORIndirectX(x) => eor_indirect_x(state, *x),
			Inst::EORIndirectY(x) => eor_indirect_y(state, *x),
			Inst::INCZeroPage(x) => inc_zero_page(state, *x),
			Inst::INCZeroPageX(x) => inc_zero_page_x(state, *x),
			Inst::INCAbsolute(a) => inc_absolute(state, a.into()),
			Inst::INCAbsoluteX(a) => inc_absolute_x(state, a.into()),
			Inst::INX => inx(state),
			Inst::INY => iny(state),
			Inst::JMPAbsolute(a) => jmp_absolute(state, a.into()),
			Inst::JMPIndirect(x) => jmp_indirect(state, x.into()),
			Inst::JSR(x) => jsr(state, x.into()),
			Inst::LDAImmediate(x) => lda_immediate(state, *x),
			Inst::LDAZeroPage(x) => lda_zero_page(state, *x),
			Inst::LDAZeroPageX(x) => lda_zero_page_x(state, *x),
			Inst::LDAAbsolute(a) => lda_absolute(state, a.into()),
			Inst::LDAAbsoluteX(a) => lda_absolute_x(state, a.into()),
			Inst::LDAAbsoluteY(a) => lda_absolute_y(state, a.into()),
			Inst::LDAIndirectX(x) => lda_indirect_x(state, *x),
			Inst::LDAIndirectY(x) => lda_indirect_y(state, *x),
			Inst::LDXImmediate(x) => ldx_immediate(state, *x),
			Inst::LDXZeroPage(x) => ldx_zero_page(state, *x),
			Inst::LDXZeroPageY(x) => ldx_zero_page_y(state, *x),
			Inst::LDXAbsolute(a) => ldx_absolute(state, a.into()),
			Inst::LDXAbsoluteY(a) => ldx_absolute_y(state, a.into()),
			Inst::LDYImmediate(x) => ldy_immediate(state, *x),
			Inst::LDYZeroPage(x) => ldy_zero_page(state, *x),
			Inst::LDYZeroPageX(x) => ldy_zero_page_x(state, *x),
			Inst::LDYAbsolute(a) => ldy_absolute(state, a.into()),
			Inst::LDYAbsoluteX(a) => ldy_absolute_x(state, a.into()),
			Inst::LSRAccumulator => lsr_accumulator(state),
			Inst::LSRZeroPage(x) => lsr_zero_page(state, *x),
			Inst::LSRZeroPageX(x) => lsr_zero_page_x(state, *x),
			Inst::LSRAbsolute(a) => lsr_absolute(state, a.into()),
			Inst::LSRAbsoluteX(a) => lsr_absolute_x(state, a.into()),
			Inst::NOP2 => nop(state),
			Inst::ORAImmediate(x) => ora_immediate(state, *x),
			Inst::ORAZeroPage(x) => ora_zero_page(state, *x),
			Inst::ORAZeroPageX(x) => ora_zero_page_x(state, *x),
			Inst::ORAAbsolute(a) => ora_absolute(state, a.into()),
			Inst::ORAAbsoluteX(a) => ora_absolute_x(state, a.into()),
			Inst::ORAAbsoluteY(a) => ora_absolute_y(state, a.into()),
			Inst::ORAIndirectX(x) => ora_indirect_x(state, *x),
			Inst::ORAIndirectY(x) => ora_indirect_y(state, *x),
			Inst::PHA => pha(state),
			Inst::PHP => php(state),
			Inst::PLA => pla(state),
			Inst::PLP => plp(state),
			Inst::ROLAccumulator => rol_accumulator(state),
			Inst::ROLZeroPage(x) => rol_zero_page(state, *x),
			Inst::ROLZeroPageX(x) => rol_zero_page_x(state, *x),
			Inst::ROLAbsolute(a) => rol_absolute(state, a.into()),
			Inst::ROLAbsoluteX(a) => rol_absolute_x(state, a.into()),
			Inst::RORAccumulator => ror_accumulator(state),
			Inst::RORZeroPage(x) => ror_zero_page(state, *x),
			Inst::RORZeroPageX(x) => ror_zero_page_x(state, *x),
			Inst::RORAbsolute(a) => ror_absolute(state, a.into()),
			Inst::RORAbsoluteX(a) => ror_absolute_x(state, a.into()),
			Inst::RTI => rti(state),
			Inst::RTS => rts(state),
			Inst::SBCImmediate(x) => sbc_immediate(state, *x),
			Inst::SBCZeroPage(x) => sbc_zero_page(state, *x),
			Inst::SBCZeroPageX(x) => sbc_zero_page_x(state, *x),
			Inst::SBCAbsolute(a) => sbc_absolute(state, a.into()),
			Inst::SBCAbsoluteX(a) => sbc_absolute_x(state, a.into()),
			Inst::SBCAbsoluteY(a) => sbc_absolute_y(state, a.into()),
			Inst::SBCIndirectX(x) => sbc_indirect_x(state, *x),
			Inst::SBCIndirectY(x) => sbc_indirect_y(state, *x),
			Inst::SEC => sec(state),
			Inst::SED => sed(state),
			Inst::SEI => sei(state),
			Inst::STAZeroPage(x) => sta_zero_page(state, *x),
			Inst::STAZeroPageX(x) => sta_zero_page_x(state, *x),
			Inst::STAAbsolute(a) => sta_absolute(state, a.into()),
			Inst::STAAbsoluteX(a) => sta_absolute_x(state, a.into()),
			Inst::STAAbsoluteY(a) => sta_absolute_y(state, a.into()),
			Inst::STAIndirectX(x) => sta_indirect_x(state, *x),
			Inst::STAIndirectY(x) => sta_indirect_y(state, *x),
			Inst::STXZeroPage(x) => stx_zero_page(state, *x),
			Inst::STXZeroPageY(x) => stx_zero_page_y(state, *x),
			Inst::STXAbsolute(a) => stx_absolute(state, a.into()),
			Inst::STYZeroPage(x) => sty_zero_page(state, *x),
			Inst::STYZeroPageX(x) => sty_zero_page_x(state, *x),
			Inst::STYAbsolute(a) => sty_absolute(state, a.into()),
			Inst::TAX => tax(state),
			Inst::TAY => tay(state),
			Inst::TSX => tsx(state),
			Inst::TXA => txa(state),
			Inst::TXS => txs(state),
			Inst::TYA => tya(state),
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
			_ => todo!("No support for unofficial instructions yet"),
		}
	}
}

pub fn parse_instruction(code: &mut &[u8]) -> Result<Inst> {
	if code.len() < size_of::<Inst>() {
		bail!("Not enough memory to read an instruction");
	}
	let mut copy = [0u8; size_of::<Inst>()];
	copy.copy_from_slice(&code[..size_of::<Inst>()]);
	Ok(unsafe { std::mem::transmute(copy) })
}

#[cfg(test)]
mod test {
	use super::*;
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
