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
	AdcAbsolute(UnalignedU16) = 0x6D,
	AdcAbsoluteX(UnalignedU16) = 0x7D,
	AdcAbsoluteY(UnalignedU16) = 0x79,
	AdcImmediate(u8) = 0x69,
	AdcIndirectX(u8) = 0x61,
	AdcIndirectY(u8) = 0x71,
	AdcZeroPage(u8) = 0x65,
	AdcZeroPageX(u8) = 0x75,
	AhxAbsoluteY(UnalignedU16) = 0x9F,
	AhxIndirectY(u8) = 0x93,
	AlrImmediate(u8) = 0x4B,
	AncImmediate2(u8) = 0x2B,
	AncImmediate(u8) = 0x0B,
	AndAbsolute(UnalignedU16) = 0x2D,
	AndAbsoluteX(UnalignedU16) = 0x3D,
	AndAbsoluteY(UnalignedU16) = 0x39,
	AndImmediate(u8) = 0x29,
	AndIndirectX(u8) = 0x21,
	AndIndirectY(u8) = 0x31,
	AndZeroPage(u8) = 0x25,
	AndZeroPageX(u8) = 0x35,
	ArrImmediate(u8) = 0x6B,
	AslAbsolute(UnalignedU16) = 0x0E,
	AslAbsoluteX(UnalignedU16) = 0x1E,
	AslAccumulator = 0x0A,
	AslZeroPage(u8) = 0x06,
	AslZeroPageX(u8) = 0x16,
	AxsImmediate(u8) = 0xCB,
	Bcc(i8) = 0x90,
	Bcs(i8) = 0xB0,
	Beq(i8) = 0xF0,
	BitAbsolute(UnalignedU16) = 0x2C,
	BitZeroPage(u8) = 0x24,
	Bmi(i8) = 0x30,
	Bne(i8) = 0xD0,
	Bpl(i8) = 0x10,
	Brk = 0x00,
	Bvc(i8) = 0x50,
	Bvs(i8) = 0x70,
	Clc = 0x18,
	Cld = 0xD8,
	Cli = 0x58,
	Clv = 0xB8,
	CmpAbsolute(UnalignedU16) = 0xCD,
	CmpAbsoluteX(UnalignedU16) = 0xDD,
	CmpAbsoluteY(UnalignedU16) = 0xD9,
	CmpImmediate(u8) = 0xC9,
	CmpIndirectX(u8) = 0xC1,
	CmpIndirectY(u8) = 0xD1,
	CmpZeroPage(u8) = 0xC5,
	CmpZeroPageX(u8) = 0xD5,
	CpxAbsolute(UnalignedU16) = 0xEC,
	CpxImmediate(u8) = 0xE0,
	CpxZeroPage(u8) = 0xE4,
	CpyAbsolute(UnalignedU16) = 0xCC,
	CpyImmediate(u8) = 0xC0,
	CpyZeroPage(u8) = 0xC4,
	DCPAbsolute(UnalignedU16) = 0xCF,
	DCPAbsoluteX(UnalignedU16) = 0xDF,
	DCPAbsoluteY(UnalignedU16) = 0xDB,
	DCPIndirectX(u8) = 0xC3,
	DCPIndirectY(u8) = 0xD3,
	DCPZeroPage(u8) = 0xC7,
	DCPZeroPageX(u8) = 0xD7,
	DecAbsolute(UnalignedU16) = 0xCE,
	DecAbsoluteX(UnalignedU16) = 0xDE,
	DecZeroPage(u8) = 0xC6,
	DecZeroPageX(u8) = 0xD6,
	Dex = 0xCA,
	Dey = 0x88,
	EorAbsolute(UnalignedU16) = 0x4D,
	EorAbsoluteX(UnalignedU16) = 0x5D,
	EorAbsoluteY(UnalignedU16) = 0x59,
	EorImmediate(u8) = 0x49,
	EorIndirectX(u8) = 0x41,
	EorIndirectY(u8) = 0x51,
	EorZeroPage(u8) = 0x45,
	EorZeroPageX(u8) = 0x55,
	IncAbsolute(UnalignedU16) = 0xEE,
	IncAbsoluteX(UnalignedU16) = 0xFE,
	IncZeroPage(u8) = 0xE6,
	IncZeroPageX(u8) = 0xF6,
	Inx = 0xE8,
	Iny = 0xC8,
	ISCAbsolute(UnalignedU16) = 0xEF,
	ISCAbsoluteX(UnalignedU16) = 0xFF,
	ISCAbsoluteY(UnalignedU16) = 0xFB,
	ISCIndirectX(u8) = 0xE3,
	ISCIndirectY(u8) = 0xF3,
	ISCZeroPage(u8) = 0xE7,
	ISCZeroPageX(u8) = 0xF7,
	JmpAbsolute(UnalignedU16) = 0x4C,
	JmpIndirect(UnalignedU16) = 0x6C,
	Jsr(UnalignedU16) = 0x20,
	LASAbsoluteY(UnalignedU16) = 0xBB,
	LAXAbsolute(UnalignedU16) = 0xAF,
	LAXAbsoluteY(UnalignedU16) = 0xBF,
	LAXImmediate(u8) = 0xAB,
	LAXIndirectX(u8) = 0xA3,
	LAXIndirectY(u8) = 0xB3,
	LAXZeroPage(u8) = 0xA7,
	LAXZeroPageY(u8) = 0xB7,
	LdaAbsolute(UnalignedU16) = 0xAD,
	LdaAbsoluteX(UnalignedU16) = 0xBD,
	LdaAbsoluteY(UnalignedU16) = 0xB9,
	LdaImmediate(u8) = 0xA9,
	LdaIndirectX(u8) = 0xA1,
	LdaIndirectY(u8) = 0xB1,
	LdaZeroPage(u8) = 0xA5,
	LdaZeroPageX(u8) = 0xB5,
	LdxAbsolute(UnalignedU16) = 0xAE,
	LdxAbsoluteY(UnalignedU16) = 0xBE,
	LdxImmediate(u8) = 0xA2,
	LdxZeroPage(u8) = 0xA6,
	LdxZeroPageY(u8) = 0xB6,
	LdyAbsolute(UnalignedU16) = 0xAC,
	LdyAbsoluteX(UnalignedU16) = 0xBC,
	LdyImmediate(u8) = 0xA0,
	LdyZeroPage(u8) = 0xA4,
	LdyZeroPageX(u8) = 0xB4,
	LsrAbsolute(UnalignedU16) = 0x4E,
	LsrAbsoluteX(UnalignedU16) = 0x5E,
	LsrAccumulator = 0x4A,
	LsrZeroPage(u8) = 0x46,
	LsrZeroPageX(u8) = 0x56,
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
	Nop2 = 0x02,
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
	OraAbsolute(UnalignedU16) = 0x0D,
	OraAbsoluteX(UnalignedU16) = 0x1D,
	OraAbsoluteY(UnalignedU16) = 0x19,
	OraImmediate(u8) = 0x09,
	OraIndirectX(u8) = 0x01,
	OraIndirectY(u8) = 0x11,
	OraZeroPage(u8) = 0x05,
	OraZeroPageX(u8) = 0x15,
	Pha = 0x48,
	Php = 0x08,
	Pla = 0x68,
	Plp = 0x28,
	RLAAbsolute(UnalignedU16) = 0x2F,
	RLAAbsoluteX(UnalignedU16) = 0x3F,
	RLAAbsoluteY(UnalignedU16) = 0x3B,
	RLAIndirectX(u8) = 0x23,
	RLAIndirectY(u8) = 0x33,
	RLAZeroPage(u8) = 0x27,
	RLAZeroPageX(u8) = 0x37,
	RolAbsolute(UnalignedU16) = 0x2E,
	RolAbsoluteX(UnalignedU16) = 0x3E,
	RolAccumulator = 0x2A,
	RolZeroPage(u8) = 0x26,
	RolZeroPageX(u8) = 0x36,
	RorAbsolute(UnalignedU16) = 0x6E,
	RorAbsoluteX(UnalignedU16) = 0x7E,
	RorAccumulator = 0x6A,
	RorZeroPage(u8) = 0x66,
	RorZeroPageX(u8) = 0x76,
	RRAAbsolute(UnalignedU16) = 0x6F,
	RRAAbsoluteX(UnalignedU16) = 0x7F,
	RRAAbsoluteY(UnalignedU16) = 0x7B,
	RRAIndirectX(u8) = 0x63,
	RRAIndirectY(u8) = 0x73,
	RRAZeroPage(u8) = 0x67,
	RRAZeroPageX(u8) = 0x77,
	Rti = 0x40,
	Rts = 0x60,
	SAXAbsolute(UnalignedU16) = 0x8F,
	SAXIndirectX(u8) = 0x83,
	SAXZeroPage(u8) = 0x87,
	SAXZeroPageY(u8) = 0x97,
	SbcAbsolute(UnalignedU16) = 0xED,
	SbcAbsoluteX(UnalignedU16) = 0xFD,
	SbcAbsoluteY(UnalignedU16) = 0xF9,
	SbcImmediate(u8) = 0xE9,
	SbcImmediate2(u8) = 0xEB,
	SbcIndirectX(u8) = 0xE1,
	SbcIndirectY(u8) = 0xF1,
	SbcZeroPage(u8) = 0xE5,
	SbcZeroPageX(u8) = 0xF5,
	Sec = 0x38,
	Sed = 0xF8,
	Sei = 0x78,
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
	StaAbsolute(UnalignedU16) = 0x8D,
	StaAbsoluteX(UnalignedU16) = 0x9D,
	StaAbsoluteY(UnalignedU16) = 0x99,
	StaIndirectX(u8) = 0x81,
	StaIndirectY(u8) = 0x91,
	StaZeroPage(u8) = 0x85,
	StaZeroPageX(u8) = 0x95,
	StxAbsolute(UnalignedU16) = 0x8E,
	StxZeroPage(u8) = 0x86,
	StxZeroPageY(u8) = 0x96,
	StyAbsolute(UnalignedU16) = 0x8C,
	StyZeroPage(u8) = 0x84,
	StyZeroPageX(u8) = 0x94,
	TASAbsoluteY(UnalignedU16) = 0x9B,
	Tax = 0xAA,
	Tay = 0xA8,
	Tsx = 0xBA,
	Txa = 0x8A,
	Txs = 0x9A,
	Tya = 0x98,
	XAAImmediate(u8) = 0x8B,
}

const _: () = {
	assert!(1 == align_of::<Inst>());
	assert!(3 == size_of::<Inst>());
};

impl Inst {
	pub fn ends_bb(&self) -> bool {
		match self {
			Inst::Bcc(..) => true,
			Inst::Bcs(..) => true,
			Inst::Beq(..) => true,
			Inst::Bmi(..) => true,
			Inst::Bne(..) => true,
			Inst::Bpl(..) => true,
			Inst::Bvc(..) => true,
			Inst::Bvs(..) => true,
			Inst::JmpIndirect(..) => true,
			Inst::JmpAbsolute(..) => true,
			Inst::Jsr(..) => true,
			Inst::Rti => true,
			Inst::Rts => true,
			_ => false,
		}
	}

	pub fn len(&self) -> u8 {
		match self {
			Inst::Brk
			| Inst::Clc
			| Inst::Cld
			| Inst::Cli
			| Inst::Clv
			| Inst::Dex
			| Inst::Dey
			| Inst::Inx
			| Inst::Iny
			| Inst::Nop2
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
			| Inst::Pha
			| Inst::Php
			| Inst::Pla
			| Inst::Plp
			| Inst::Rti
			| Inst::Rts
			| Inst::Sec
			| Inst::Sed
			| Inst::Sei
			| Inst::Tax
			| Inst::Tay
			| Inst::Tsx
			| Inst::Txa
			| Inst::Txs
			| Inst::Tya
			| Inst::AslAccumulator
			| Inst::LsrAccumulator
			| Inst::RolAccumulator
			| Inst::RorAccumulator => 1,
			Inst::OraImmediate(..)
			| Inst::OraZeroPage(..)
			| Inst::OraZeroPageX(..)
			| Inst::OraIndirectX(..)
			| Inst::OraIndirectY(..)
			| Inst::AndImmediate(..)
			| Inst::AndZeroPage(..)
			| Inst::AndZeroPageX(..)
			| Inst::AndIndirectX(..)
			| Inst::AndIndirectY(..)
			| Inst::EorImmediate(..)
			| Inst::EorZeroPage(..)
			| Inst::EorZeroPageX(..)
			| Inst::EorIndirectX(..)
			| Inst::EorIndirectY(..)
			| Inst::AdcImmediate(..)
			| Inst::AdcZeroPage(..)
			| Inst::AdcZeroPageX(..)
			| Inst::AdcIndirectX(..)
			| Inst::AdcIndirectY(..)
			| Inst::LdaImmediate(..)
			| Inst::LdaZeroPage(..)
			| Inst::LdaZeroPageX(..)
			| Inst::LdaIndirectX(..)
			| Inst::LdaIndirectY(..)
			| Inst::LdxImmediate(..)
			| Inst::LdxZeroPage(..)
			| Inst::LdxZeroPageY(..)
			| Inst::LdyImmediate(..)
			| Inst::LdyZeroPage(..)
			| Inst::LdyZeroPageX(..)
			| Inst::StaZeroPage(..)
			| Inst::StaZeroPageX(..)
			| Inst::StaIndirectX(..)
			| Inst::StaIndirectY(..)
			| Inst::StxZeroPage(..)
			| Inst::StxZeroPageY(..)
			| Inst::StyZeroPage(..)
			| Inst::StyZeroPageX(..)
			| Inst::CmpImmediate(..)
			| Inst::CmpZeroPage(..)
			| Inst::CmpZeroPageX(..)
			| Inst::CmpIndirectX(..)
			| Inst::CmpIndirectY(..)
			| Inst::CpxImmediate(..)
			| Inst::CpxZeroPage(..)
			| Inst::CpyImmediate(..)
			| Inst::CpyZeroPage(..)
			| Inst::SbcImmediate(..)
			| Inst::SbcZeroPage(..)
			| Inst::SbcZeroPageX(..)
			| Inst::SbcIndirectX(..)
			| Inst::SbcIndirectY(..)
			| Inst::BitZeroPage(..)
			| Inst::AslZeroPage(..)
			| Inst::AslZeroPageX(..)
			| Inst::LsrZeroPage(..)
			| Inst::LsrZeroPageX(..)
			| Inst::RolZeroPage(..)
			| Inst::RolZeroPageX(..)
			| Inst::RorZeroPage(..)
			| Inst::RorZeroPageX(..)
			| Inst::DecZeroPage(..)
			| Inst::DecZeroPageX(..)
			| Inst::IncZeroPage(..)
			| Inst::IncZeroPageX(..)
			| Inst::Bpl(..)
			| Inst::Bmi(..)
			| Inst::Bvc(..)
			| Inst::Bvs(..)
			| Inst::Bcc(..)
			| Inst::Bcs(..)
			| Inst::Bne(..)
			| Inst::Beq(..)
			| Inst::AncImmediate(..)
			| Inst::AncImmediate2(..)
			| Inst::AlrImmediate(..)
			| Inst::ArrImmediate(..)
			| Inst::AxsImmediate(..)
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
			| Inst::AhxIndirectY(..)
			| Inst::NOPImmediate(..)
			| Inst::NOPImmediate2(..)
			| Inst::NOPImmediate3(..) => 2,
			Inst::OraAbsolute(..)
			| Inst::OraAbsoluteX(..)
			| Inst::OraAbsoluteY(..)
			| Inst::AndAbsolute(..)
			| Inst::AndAbsoluteX(..)
			| Inst::AndAbsoluteY(..)
			| Inst::EorAbsolute(..)
			| Inst::EorAbsoluteX(..)
			| Inst::EorAbsoluteY(..)
			| Inst::AdcAbsolute(..)
			| Inst::AdcAbsoluteX(..)
			| Inst::AdcAbsoluteY(..)
			| Inst::LdaAbsolute(..)
			| Inst::LdaAbsoluteX(..)
			| Inst::LdaAbsoluteY(..)
			| Inst::LdxAbsolute(..)
			| Inst::LdxAbsoluteY(..)
			| Inst::LdyAbsolute(..)
			| Inst::LdyAbsoluteX(..)
			| Inst::StaAbsolute(..)
			| Inst::StaAbsoluteX(..)
			| Inst::StaAbsoluteY(..)
			| Inst::StxAbsolute(..)
			| Inst::StyAbsolute(..)
			| Inst::CmpAbsolute(..)
			| Inst::CmpAbsoluteX(..)
			| Inst::CmpAbsoluteY(..)
			| Inst::CpxAbsolute(..)
			| Inst::CpyAbsolute(..)
			| Inst::SbcAbsolute(..)
			| Inst::SbcAbsoluteX(..)
			| Inst::SbcAbsoluteY(..)
			| Inst::BitAbsolute(..)
			| Inst::AslAbsolute(..)
			| Inst::AslAbsoluteX(..)
			| Inst::LsrAbsolute(..)
			| Inst::LsrAbsoluteX(..)
			| Inst::RolAbsolute(..)
			| Inst::RolAbsoluteX(..)
			| Inst::RorAbsolute(..)
			| Inst::RorAbsoluteX(..)
			| Inst::DecAbsolute(..)
			| Inst::DecAbsoluteX(..)
			| Inst::IncAbsolute(..)
			| Inst::IncAbsoluteX(..)
			| Inst::JmpAbsolute(..)
			| Inst::JmpIndirect(..)
			| Inst::Jsr(..)
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
			| Inst::AhxAbsoluteY(..)
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
			Inst::SbcImmediate2(_) => todo!(),
			Inst::XAAImmediate(_) => todo!(),
		}
	}

	pub fn evaluate(&self, state: &mut State) {
		match self {
			Inst::AdcImmediate(x) => adc_immediate(state, *x),
			Inst::AdcZeroPage(x) => adc_zero_page(state, *x),
			Inst::AdcZeroPageX(x) => adc_zero_page_x(state, *x),
			Inst::AdcAbsolute(a) => adc_absolute(state, a.into()),
			Inst::AdcAbsoluteX(a) => adc_absolute_x(state, a.into()),
			Inst::AdcAbsoluteY(a) => adc_absolute_y(state, a.into()),
			Inst::AdcIndirectX(x) => adc_indirect_x(state, *x),
			Inst::AdcIndirectY(x) => adc_indirect_y(state, *x),
			Inst::AndImmediate(x) => and_immediate(state, *x),
			Inst::AndZeroPage(x) => and_zero_page(state, *x),
			Inst::AndZeroPageX(x) => and_zero_page_x(state, *x),
			Inst::AndAbsolute(a) => and_absolute(state, a.into()),
			Inst::AndAbsoluteX(a) => and_absolute_x(state, a.into()),
			Inst::AndAbsoluteY(a) => and_absolute_y(state, a.into()),
			Inst::AndIndirectX(x) => and_indirect_x(state, *x),
			Inst::AndIndirectY(x) => and_indirect_y(state, *x),
			Inst::AslAccumulator => asl_accumulator(state),
			Inst::AslZeroPage(x) => asl_zero_page(state, *x),
			Inst::AslZeroPageX(x) => asl_zero_page_x(state, *x),
			Inst::AslAbsolute(a) => asl_absolute(state, a.into()),
			Inst::AslAbsoluteX(a) => asl_absolute_x(state, a.into()),
			Inst::Bcc(x) => bcc(state, *x),
			Inst::Bcs(x) => bcs(state, *x),
			Inst::Beq(x) => beq(state, *x),
			Inst::BitZeroPage(x) => bit_zero_page(state, *x),
			Inst::BitAbsolute(a) => bit_absolute(state, a.into()),
			Inst::Bmi(x) => bmi(state, *x),
			Inst::Bne(x) => bne(state, *x),
			Inst::Bpl(x) => bpl(state, *x),
			Inst::Brk => brk(state),
			Inst::Bvc(x) => bvc(state, *x),
			Inst::Bvs(x) => bvs(state, *x),
			Inst::Clc => clc(state),
			Inst::Cld => cld(state),
			Inst::Cli => cli(state),
			Inst::Clv => clv(state),
			Inst::CmpImmediate(x) => cmp_immediate(state, *x),
			Inst::CmpZeroPage(x) => cmp_zero_page(state, *x),
			Inst::CmpZeroPageX(x) => cmp_zero_page_x(state, *x),
			Inst::CmpAbsolute(a) => cmp_absolute(state, a.into()),
			Inst::CmpAbsoluteX(a) => cmp_absolute_x(state, a.into()),
			Inst::CmpAbsoluteY(a) => cmp_absolute_y(state, a.into()),
			Inst::CmpIndirectX(x) => cmp_indirect_x(state, *x),
			Inst::CmpIndirectY(x) => cmp_indirect_y(state, *x),
			Inst::CpxImmediate(x) => cpx_immediate(state, *x),
			Inst::CpxZeroPage(x) => cpx_zero_page(state, *x),
			Inst::CpxAbsolute(a) => cpx_absolute(state, a.into()),
			Inst::CpyImmediate(x) => cpy_immediate(state, *x),
			Inst::CpyZeroPage(x) => cpy_zero_page(state, *x),
			Inst::CpyAbsolute(a) => cpy_absolute(state, a.into()),
			Inst::DecZeroPage(x) => dec_zero_page(state, *x),
			Inst::DecZeroPageX(x) => dec_zero_page_x(state, *x),
			Inst::DecAbsolute(a) => dec_absolute(state, a.into()),
			Inst::DecAbsoluteX(a) => dec_absolute_x(state, a.into()),
			Inst::Dex => dex(state),
			Inst::Dey => dey(state),
			Inst::EorImmediate(x) => eor_immediate(state, *x),
			Inst::EorZeroPage(x) => eor_zero_page(state, *x),
			Inst::EorZeroPageX(x) => eor_zero_page_x(state, *x),
			Inst::EorAbsolute(a) => eor_absolute(state, a.into()),
			Inst::EorAbsoluteX(a) => eor_absolute_x(state, a.into()),
			Inst::EorAbsoluteY(a) => eor_absolute_y(state, a.into()),
			Inst::EorIndirectX(x) => eor_indirect_x(state, *x),
			Inst::EorIndirectY(x) => eor_indirect_y(state, *x),
			Inst::IncZeroPage(x) => inc_zero_page(state, *x),
			Inst::IncZeroPageX(x) => inc_zero_page_x(state, *x),
			Inst::IncAbsolute(a) => inc_absolute(state, a.into()),
			Inst::IncAbsoluteX(a) => inc_absolute_x(state, a.into()),
			Inst::Inx => inx(state),
			Inst::Iny => iny(state),
			Inst::JmpAbsolute(a) => jmp_absolute(state, a.into()),
			Inst::JmpIndirect(x) => jmp_indirect(state, x.into()),
			Inst::Jsr(x) => jsr(state, x.into()),
			Inst::LdaImmediate(x) => lda_immediate(state, *x),
			Inst::LdaZeroPage(x) => lda_zero_page(state, *x),
			Inst::LdaZeroPageX(x) => lda_zero_page_x(state, *x),
			Inst::LdaAbsolute(a) => lda_absolute(state, a.into()),
			Inst::LdaAbsoluteX(a) => lda_absolute_x(state, a.into()),
			Inst::LdaAbsoluteY(a) => lda_absolute_y(state, a.into()),
			Inst::LdaIndirectX(x) => lda_indirect_x(state, *x),
			Inst::LdaIndirectY(x) => lda_indirect_y(state, *x),
			Inst::LdxImmediate(x) => ldx_immediate(state, *x),
			Inst::LdxZeroPage(x) => ldx_zero_page(state, *x),
			Inst::LdxZeroPageY(x) => ldx_zero_page_y(state, *x),
			Inst::LdxAbsolute(a) => ldx_absolute(state, a.into()),
			Inst::LdxAbsoluteY(a) => ldx_absolute_y(state, a.into()),
			Inst::LdyImmediate(x) => ldy_immediate(state, *x),
			Inst::LdyZeroPage(x) => ldy_zero_page(state, *x),
			Inst::LdyZeroPageX(x) => ldy_zero_page_x(state, *x),
			Inst::LdyAbsolute(a) => ldy_absolute(state, a.into()),
			Inst::LdyAbsoluteX(a) => ldy_absolute_x(state, a.into()),
			Inst::LsrAccumulator => lsr_accumulator(state),
			Inst::LsrZeroPage(x) => lsr_zero_page(state, *x),
			Inst::LsrZeroPageX(x) => lsr_zero_page_x(state, *x),
			Inst::LsrAbsolute(a) => lsr_absolute(state, a.into()),
			Inst::LsrAbsoluteX(a) => lsr_absolute_x(state, a.into()),
			Inst::Nop2 => nop(state),
			Inst::OraImmediate(x) => ora_immediate(state, *x),
			Inst::OraZeroPage(x) => ora_zero_page(state, *x),
			Inst::OraZeroPageX(x) => ora_zero_page_x(state, *x),
			Inst::OraAbsolute(a) => ora_absolute(state, a.into()),
			Inst::OraAbsoluteX(a) => ora_absolute_x(state, a.into()),
			Inst::OraAbsoluteY(a) => ora_absolute_y(state, a.into()),
			Inst::OraIndirectX(x) => ora_indirect_x(state, *x),
			Inst::OraIndirectY(x) => ora_indirect_y(state, *x),
			Inst::Pha => pha(state),
			Inst::Php => php(state),
			Inst::Pla => pla(state),
			Inst::Plp => plp(state),
			Inst::RolAccumulator => rol_accumulator(state),
			Inst::RolZeroPage(x) => rol_zero_page(state, *x),
			Inst::RolZeroPageX(x) => rol_zero_page_x(state, *x),
			Inst::RolAbsolute(a) => rol_absolute(state, a.into()),
			Inst::RolAbsoluteX(a) => rol_absolute_x(state, a.into()),
			Inst::RorAccumulator => ror_accumulator(state),
			Inst::RorZeroPage(x) => ror_zero_page(state, *x),
			Inst::RorZeroPageX(x) => ror_zero_page_x(state, *x),
			Inst::RorAbsolute(a) => ror_absolute(state, a.into()),
			Inst::RorAbsoluteX(a) => ror_absolute_x(state, a.into()),
			Inst::Rti => rti(state),
			Inst::Rts => rts(state),
			Inst::SbcImmediate(x) => sbc_immediate(state, *x),
			Inst::SbcZeroPage(x) => sbc_zero_page(state, *x),
			Inst::SbcZeroPageX(x) => sbc_zero_page_x(state, *x),
			Inst::SbcAbsolute(a) => sbc_absolute(state, a.into()),
			Inst::SbcAbsoluteX(a) => sbc_absolute_x(state, a.into()),
			Inst::SbcAbsoluteY(a) => sbc_absolute_y(state, a.into()),
			Inst::SbcIndirectX(x) => sbc_indirect_x(state, *x),
			Inst::SbcIndirectY(x) => sbc_indirect_y(state, *x),
			Inst::Sec => sec(state),
			Inst::Sed => sed(state),
			Inst::Sei => sei(state),
			Inst::StaZeroPage(x) => sta_zero_page(state, *x),
			Inst::StaZeroPageX(x) => sta_zero_page_x(state, *x),
			Inst::StaAbsolute(a) => sta_absolute(state, a.into()),
			Inst::StaAbsoluteX(a) => sta_absolute_x(state, a.into()),
			Inst::StaAbsoluteY(a) => sta_absolute_y(state, a.into()),
			Inst::StaIndirectX(x) => sta_indirect_x(state, *x),
			Inst::StaIndirectY(x) => sta_indirect_y(state, *x),
			Inst::StxZeroPage(x) => stx_zero_page(state, *x),
			Inst::StxZeroPageY(x) => stx_zero_page_y(state, *x),
			Inst::StxAbsolute(a) => stx_absolute(state, a.into()),
			Inst::StyZeroPage(x) => sty_zero_page(state, *x),
			Inst::StyZeroPageX(x) => sty_zero_page_x(state, *x),
			Inst::StyAbsolute(a) => sty_absolute(state, a.into()),
			Inst::Tax => tax(state),
			Inst::Tay => tay(state),
			Inst::Tsx => tsx(state),
			Inst::Txa => txa(state),
			Inst::Txs => txs(state),
			Inst::Tya => tya(state),
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
			// Inst::Alr(x) => alr(cpu, *x),
			// Inst::ARR(x) => arr(cpu, *x),
			// Inst::Axs(x) => axs(cpu, *x),
			// Inst::LAS(x) => las(cpu, *x),
			// Inst::TAS(x) => tas(cpu, *x),
			// Inst::SHY(x) => shy(cpu, *x),
			// Inst::SHX(x) => shx(cpu, *x),
			// Inst::Ahx(Ahx::AbsoluteY(a)) => ahx_absolute_y(cpu, *a),
			// Inst::Ahx(Ahx::IndirectY(x)) => ahx_indirect_y(cpu, *x),
			// Inst::NOPU(..) => {}
			_ => todo!("No support for unofficial instructions yet"),
		}
	}
}

pub fn parse_instruction(code: [u8; 3]) -> Inst {
	// This could be a huge match statement, but I checked that LLVM could optimise that to the
	// same thing and then went with the more readable version:
	// https://godbolt.org/z/eM74c6EEs
	unsafe { std::mem::transmute::<[u8; size_of::<Inst>()], Inst>(code) }
}
