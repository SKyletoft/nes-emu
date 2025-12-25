#![allow(unused, clippy::upper_case_acronyms)]

use std::fmt::{self, Display};

use anyhow::{Result, bail};

use crate::{cpu::Cpu, evaluate_instruction::*, interpret::State, nrom256::NROM256};

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UnalignedU16 {
	lo: u8,
	hi: u8,
}

impl Display for UnalignedU16 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&u16::from(*self), f)
	}
}

impl fmt::UpperHex for UnalignedU16 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::UpperHex::fmt(&u16::from(*self), f)
	}
}

impl std::fmt::Debug for UnalignedU16 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fmt::Debug::fmt(&u16::from(*self), f)
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

impl UnalignedU16 {
	pub fn as_u16(self) -> u16 {
		self.into()
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
	AncImmediate(u8) = 0x0B,
	AncImmediate2(u8) = 0x2B,
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
	DcpAbsolute(UnalignedU16) = 0xCF,
	DcpAbsoluteX(UnalignedU16) = 0xDF,
	DcpAbsoluteY(UnalignedU16) = 0xDB,
	DcpIndirectX(u8) = 0xC3,
	DcpIndirectY(u8) = 0xD3,
	DcpZeroPage(u8) = 0xC7,
	DcpZeroPageX(u8) = 0xD7,
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
	Ign(UnalignedU16) = 0x0C,
	IgnAbsoluteX(UnalignedU16) = 0x1C,
	IgnAbsoluteX2(UnalignedU16) = 0x3C,
	IgnAbsoluteX3(UnalignedU16) = 0x5C,
	IgnAbsoluteX4(UnalignedU16) = 0x7C,
	IgnAbsoluteX5(UnalignedU16) = 0xDC,
	IgnAbsoluteX6(UnalignedU16) = 0xFC,
	IgnDirect(u8) = 0x04,
	IgnDirect2(u8) = 0x44,
	IgnDirect3(u8) = 0x64,
	IgnDirectX(u8) = 0x14,
	IgnDirectX2(u8) = 0x34,
	IgnDirectX3(u8) = 0x54,
	IgnDirectX4(u8) = 0x74,
	IgnDirectX5(u8) = 0xD4,
	IgnDirectX6(u8) = 0xF4,
	IncAbsolute(UnalignedU16) = 0xEE,
	IncAbsoluteX(UnalignedU16) = 0xFE,
	IncZeroPage(u8) = 0xE6,
	IncZeroPageX(u8) = 0xF6,
	Inx = 0xE8,
	Iny = 0xC8,
	IscAbsolute(UnalignedU16) = 0xEF,
	IscAbsoluteX(UnalignedU16) = 0xFF,
	IscAbsoluteY(UnalignedU16) = 0xFB,
	IscIndirectX(u8) = 0xE3,
	IscIndirectY(u8) = 0xF3,
	IscZeroPage(u8) = 0xE7,
	IscZeroPageX(u8) = 0xF7,
	JmpAbsolute(UnalignedU16) = 0x4C,
	JmpIndirect(UnalignedU16) = 0x6C,
	Jsr(UnalignedU16) = 0x20,
	LasAbsoluteY(UnalignedU16) = 0xBB,
	LaxImmediate(u8) = 0xAB,
	LaxAbsolute(UnalignedU16) = 0xAF,
	LaxAbsoluteY(UnalignedU16) = 0xBF,
	LaxIndirectX(u8) = 0xA3,
	LaxIndirectY(u8) = 0xB3,
	LaxZeroPage(u8) = 0xA7,
	LaxZeroPageY(u8) = 0xB7,
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
	Nop = 0x1A,
	Nop2 = 0x3A,
	Nop3 = 0x5A,
	Nop4 = 0x7A,
	Nop5 = 0xDA,
	Nop6 = 0xEA,
	Nop7 = 0xFA,
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
	RlaAbsolute(UnalignedU16) = 0x2F,
	RlaAbsoluteX(UnalignedU16) = 0x3F,
	RlaAbsoluteY(UnalignedU16) = 0x3B,
	RlaIndirectX(u8) = 0x23,
	RlaIndirectY(u8) = 0x33,
	RlaZeroPage(u8) = 0x27,
	RlaZeroPageX(u8) = 0x37,
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
	RraAbsolute(UnalignedU16) = 0x6F,
	RraAbsoluteX(UnalignedU16) = 0x7F,
	RraAbsoluteY(UnalignedU16) = 0x7B,
	RraIndirectX(u8) = 0x63,
	RraIndirectY(u8) = 0x73,
	RraZeroPage(u8) = 0x67,
	RraZeroPageX(u8) = 0x77,
	Rti = 0x40,
	Rts = 0x60,
	ShxAbsoluteY(UnalignedU16) = 0x9E,
	ShyAbsoluteX(UnalignedU16) = 0x9C,
	SaxAbsolute(UnalignedU16) = 0x8F,
	SaxIndirectX(u8) = 0x83,
	SaxZeroPage(u8) = 0x87,
	SaxZeroPageY(u8) = 0x97,
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
	Skb(u8) = 0x80,
	Skb2(u8) = 0x82,
	Skb3(u8) = 0x89,
	Skb4(u8) = 0xC2,
	Skb5(u8) = 0xE2,
	SloAbsolute(UnalignedU16) = 0x0F,
	SloAbsoluteX(UnalignedU16) = 0x1F,
	SloAbsoluteY(UnalignedU16) = 0x1B,
	SloIndirectX(u8) = 0x03,
	SloIndirectY(u8) = 0x13,
	SloZeroPage(u8) = 0x07,
	SloZeroPageX(u8) = 0x17,
	SreAbsolute(UnalignedU16) = 0x4F,
	SreAbsoluteX(UnalignedU16) = 0x5F,
	SreAbsoluteY(UnalignedU16) = 0x5B,
	SreIndirectX(u8) = 0x43,
	SreIndirectY(u8) = 0x53,
	SreZeroPage(u8) = 0x47,
	SreZeroPageX(u8) = 0x57,
	StaAbsolute(UnalignedU16) = 0x8D,
	StaAbsoluteX(UnalignedU16) = 0x9D,
	StaAbsoluteY(UnalignedU16) = 0x99,
	StaIndirectX(u8) = 0x81,
	StaIndirectY(u8) = 0x91,
	StaZeroPage(u8) = 0x85,
	StaZeroPageX(u8) = 0x95,
	Stp = 0x02,
	Stp10 = 0xB2,
	Stp11 = 0xD2,
	Stp12 = 0xF2,
	Stp2 = 0x12,
	Stp3 = 0x22,
	Stp4 = 0x32,
	Stp5 = 0x42,
	Stp6 = 0x52,
	Stp7 = 0x62,
	Stp8 = 0x72,
	Stp9 = 0x92,
	StxAbsolute(UnalignedU16) = 0x8E,
	StxZeroPage(u8) = 0x86,
	StxZeroPageY(u8) = 0x96,
	StyAbsolute(UnalignedU16) = 0x8C,
	StyZeroPage(u8) = 0x84,
	StyZeroPageX(u8) = 0x94,
	TasAbsoluteY(UnalignedU16) = 0x9B,
	Tax = 0xAA,
	Tay = 0xA8,
	Tsx = 0xBA,
	Txa = 0x8A,
	Txs = 0x9A,
	Tya = 0x98,
	XaaImmediate(u8) = 0x8B,
}

const _: () = {
	assert!(1 == align_of::<Inst>());
	assert!(3 == size_of::<Inst>());
};

impl From<[u8; 3]> for Inst {
	fn from(code: [u8; 3]) -> Self {
		// This could be a huge match statement, but I checked that LLVM could optimise that to the
		// same thing and then went with the more readable version:
		// https://godbolt.org/z/eM74c6EEs
		unsafe { std::mem::transmute::<[u8; size_of::<Inst>()], Inst>(code) }
	}
}

impl Inst {
	pub fn ends_bb(&self) -> bool {
		matches!(
			self,
			Inst::Bcc(..)
				| Inst::Bcs(..)
				| Inst::Beq(..)
				| Inst::Bmi(..)
				| Inst::Bne(..)
				| Inst::Bpl(..)
				| Inst::Bvc(..)
				| Inst::Bvs(..)
				| Inst::JmpIndirect(..)
				| Inst::JmpAbsolute(..)
				| Inst::Jsr(..)
				| Inst::Rti | Inst::Rts
				| Inst::Stp | Inst::Stp2
				| Inst::Stp3 | Inst::Stp4
				| Inst::Stp5 | Inst::Stp6
				| Inst::Stp7 | Inst::Stp8
				| Inst::Stp9 | Inst::Stp10
				| Inst::Stp11
				| Inst::Stp12
		)
	}

	pub fn len(&self) -> u8 {
		match self {
			Inst::AslAccumulator
			| Inst::Brk
			| Inst::Clc
			| Inst::Cld
			| Inst::Cli
			| Inst::Clv
			| Inst::Dex
			| Inst::Dey
			| Inst::Inx
			| Inst::Iny
			| Inst::LsrAccumulator
			| Inst::Nop
			| Inst::Nop2
			| Inst::Nop3
			| Inst::Nop4
			| Inst::Nop5
			| Inst::Nop6
			| Inst::Nop7
			| Inst::Pha
			| Inst::Php
			| Inst::Pla
			| Inst::Plp
			| Inst::RolAccumulator
			| Inst::RorAccumulator
			| Inst::Rti
			| Inst::Rts
			| Inst::Sec
			| Inst::Sed
			| Inst::Sei
			| Inst::Stp
			| Inst::Stp10
			| Inst::Stp11
			| Inst::Stp12
			| Inst::Stp2
			| Inst::Stp3
			| Inst::Stp4
			| Inst::Stp5
			| Inst::Stp6
			| Inst::Stp7
			| Inst::Stp8
			| Inst::Stp9
			| Inst::Tax
			| Inst::Tay
			| Inst::Tsx
			| Inst::Txa
			| Inst::Txs
			| Inst::Tya => 1,

			Inst::AdcImmediate(..)
			| Inst::AdcIndirectX(..)
			| Inst::AdcIndirectY(..)
			| Inst::AdcZeroPage(..)
			| Inst::AdcZeroPageX(..)
			| Inst::AhxIndirectY(..)
			| Inst::AlrImmediate(..)
			| Inst::AncImmediate(..)
			| Inst::AncImmediate2(..)
			| Inst::AndImmediate(..)
			| Inst::AndIndirectX(..)
			| Inst::AndIndirectY(..)
			| Inst::AndZeroPage(..)
			| Inst::AndZeroPageX(..)
			| Inst::ArrImmediate(..)
			| Inst::AslZeroPage(..)
			| Inst::AslZeroPageX(..)
			| Inst::AxsImmediate(..)
			| Inst::Bcc(..)
			| Inst::Bcs(..)
			| Inst::Beq(..)
			| Inst::BitZeroPage(..)
			| Inst::Bmi(..)
			| Inst::Bne(..)
			| Inst::Bpl(..)
			| Inst::Bvc(..)
			| Inst::Bvs(..)
			| Inst::CmpImmediate(..)
			| Inst::CmpIndirectX(..)
			| Inst::CmpIndirectY(..)
			| Inst::CmpZeroPage(..)
			| Inst::CmpZeroPageX(..)
			| Inst::CpxImmediate(..)
			| Inst::CpxZeroPage(..)
			| Inst::CpyImmediate(..)
			| Inst::CpyZeroPage(..)
			| Inst::DcpIndirectX(..)
			| Inst::DcpIndirectY(..)
			| Inst::DcpZeroPage(..)
			| Inst::DcpZeroPageX(..)
			| Inst::DecZeroPage(..)
			| Inst::DecZeroPageX(..)
			| Inst::EorImmediate(..)
			| Inst::EorIndirectX(..)
			| Inst::EorIndirectY(..)
			| Inst::EorZeroPage(..)
			| Inst::EorZeroPageX(..)
			| Inst::IgnDirect(..)
			| Inst::IgnDirect2(..)
			| Inst::IgnDirect3(..)
			| Inst::IgnDirectX(..)
			| Inst::IgnDirectX2(..)
			| Inst::IgnDirectX3(..)
			| Inst::IgnDirectX4(..)
			| Inst::IgnDirectX5(..)
			| Inst::IgnDirectX6(..)
			| Inst::IncZeroPage(..)
			| Inst::IncZeroPageX(..)
			| Inst::IscIndirectX(..)
			| Inst::IscIndirectY(..)
			| Inst::IscZeroPage(..)
			| Inst::IscZeroPageX(..)
			| Inst::LaxImmediate(..)
			| Inst::LaxIndirectX(..)
			| Inst::LaxIndirectY(..)
			| Inst::LaxZeroPage(..)
			| Inst::LaxZeroPageY(..)
			| Inst::LdaImmediate(..)
			| Inst::LdaIndirectX(..)
			| Inst::LdaIndirectY(..)
			| Inst::LdaZeroPage(..)
			| Inst::LdaZeroPageX(..)
			| Inst::LdxImmediate(..)
			| Inst::LdxZeroPage(..)
			| Inst::LdxZeroPageY(..)
			| Inst::LdyImmediate(..)
			| Inst::LdyZeroPage(..)
			| Inst::LdyZeroPageX(..)
			| Inst::LsrZeroPage(..)
			| Inst::LsrZeroPageX(..)
			| Inst::OraImmediate(..)
			| Inst::OraIndirectX(..)
			| Inst::OraIndirectY(..)
			| Inst::OraZeroPage(..)
			| Inst::OraZeroPageX(..)
			| Inst::RlaIndirectX(..)
			| Inst::RlaIndirectY(..)
			| Inst::RlaZeroPage(..)
			| Inst::RlaZeroPageX(..)
			| Inst::RolZeroPage(..)
			| Inst::RolZeroPageX(..)
			| Inst::RorZeroPage(..)
			| Inst::RorZeroPageX(..)
			| Inst::RraIndirectX(..)
			| Inst::RraIndirectY(..)
			| Inst::RraZeroPage(..)
			| Inst::RraZeroPageX(..)
			| Inst::SaxIndirectX(..)
			| Inst::SaxZeroPage(..)
			| Inst::SaxZeroPageY(..)
			| Inst::SbcImmediate(..)
			| Inst::SbcImmediate2(..)
			| Inst::SbcIndirectX(..)
			| Inst::SbcIndirectY(..)
			| Inst::SbcZeroPage(..)
			| Inst::SbcZeroPageX(..)
			| Inst::Skb(..)
			| Inst::Skb2(..)
			| Inst::Skb3(..)
			| Inst::Skb4(..)
			| Inst::Skb5(..)
			| Inst::SloIndirectX(..)
			| Inst::SloIndirectY(..)
			| Inst::SloZeroPage(..)
			| Inst::SloZeroPageX(..)
			| Inst::SreIndirectX(..)
			| Inst::SreIndirectY(..)
			| Inst::SreZeroPage(..)
			| Inst::SreZeroPageX(..)
			| Inst::StaIndirectX(..)
			| Inst::StaIndirectY(..)
			| Inst::StaZeroPage(..)
			| Inst::StaZeroPageX(..)
			| Inst::StxZeroPage(..)
			| Inst::StxZeroPageY(..)
			| Inst::StyZeroPage(..)
			| Inst::StyZeroPageX(..)
			| Inst::XaaImmediate(..) => 2,

			Inst::AdcAbsolute(..)
			| Inst::AdcAbsoluteX(..)
			| Inst::AdcAbsoluteY(..)
			| Inst::AhxAbsoluteY(..)
			| Inst::AndAbsolute(..)
			| Inst::AndAbsoluteX(..)
			| Inst::AndAbsoluteY(..)
			| Inst::AslAbsolute(..)
			| Inst::AslAbsoluteX(..)
			| Inst::BitAbsolute(..)
			| Inst::CmpAbsolute(..)
			| Inst::CmpAbsoluteX(..)
			| Inst::CmpAbsoluteY(..)
			| Inst::CpxAbsolute(..)
			| Inst::CpyAbsolute(..)
			| Inst::DcpAbsolute(..)
			| Inst::DcpAbsoluteX(..)
			| Inst::DcpAbsoluteY(..)
			| Inst::DecAbsolute(..)
			| Inst::DecAbsoluteX(..)
			| Inst::EorAbsolute(..)
			| Inst::EorAbsoluteX(..)
			| Inst::EorAbsoluteY(..)
			| Inst::Ign(..)
			| Inst::IgnAbsoluteX(..)
			| Inst::IgnAbsoluteX2(..)
			| Inst::IgnAbsoluteX3(..)
			| Inst::IgnAbsoluteX4(..)
			| Inst::IgnAbsoluteX5(..)
			| Inst::IgnAbsoluteX6(..)
			| Inst::IncAbsolute(..)
			| Inst::IncAbsoluteX(..)
			| Inst::IscAbsolute(..)
			| Inst::IscAbsoluteX(..)
			| Inst::IscAbsoluteY(..)
			| Inst::JmpAbsolute(..)
			| Inst::JmpIndirect(..)
			| Inst::Jsr(..)
			| Inst::LasAbsoluteY(..)
			| Inst::LaxAbsolute(..)
			| Inst::LaxAbsoluteY(..)
			| Inst::LdaAbsolute(..)
			| Inst::LdaAbsoluteX(..)
			| Inst::LdaAbsoluteY(..)
			| Inst::LdxAbsolute(..)
			| Inst::LdxAbsoluteY(..)
			| Inst::LdyAbsolute(..)
			| Inst::LdyAbsoluteX(..)
			| Inst::LsrAbsolute(..)
			| Inst::LsrAbsoluteX(..)
			| Inst::OraAbsolute(..)
			| Inst::OraAbsoluteX(..)
			| Inst::OraAbsoluteY(..)
			| Inst::RlaAbsolute(..)
			| Inst::RlaAbsoluteX(..)
			| Inst::RlaAbsoluteY(..)
			| Inst::RolAbsolute(..)
			| Inst::RolAbsoluteX(..)
			| Inst::RorAbsolute(..)
			| Inst::RorAbsoluteX(..)
			| Inst::RraAbsolute(..)
			| Inst::RraAbsoluteX(..)
			| Inst::RraAbsoluteY(..)
			| Inst::SaxAbsolute(..)
			| Inst::SbcAbsolute(..)
			| Inst::SbcAbsoluteX(..)
			| Inst::SbcAbsoluteY(..)
			| Inst::ShxAbsoluteY(..)
			| Inst::ShyAbsoluteX(..)
			| Inst::SloAbsolute(..)
			| Inst::SloAbsoluteX(..)
			| Inst::SloAbsoluteY(..)
			| Inst::SreAbsolute(..)
			| Inst::SreAbsoluteX(..)
			| Inst::SreAbsoluteY(..)
			| Inst::StaAbsolute(..)
			| Inst::StaAbsoluteX(..)
			| Inst::StaAbsoluteY(..)
			| Inst::StxAbsolute(..)
			| Inst::StyAbsolute(..)
			| Inst::TasAbsoluteY(..) => 3,
		}
	}

	pub fn evaluate(&self, state: &mut State<NROM256>) {
		match self {
			Inst::AdcAbsolute(a) => adc_absolute(state, a.into()),
			Inst::AdcAbsoluteX(a) => adc_absolute_x(state, a.into()),
			Inst::AdcAbsoluteY(a) => adc_absolute_y(state, a.into()),
			Inst::AdcImmediate(x) => adc_immediate(state, *x),
			Inst::AdcIndirectX(x) => adc_indirect_x(state, *x),
			Inst::AdcIndirectY(x) => adc_indirect_y(state, *x),
			Inst::AdcZeroPage(x) => adc_zero_page(state, *x),
			Inst::AdcZeroPageX(x) => adc_zero_page_x(state, *x),
			Inst::AndAbsolute(a) => and_absolute(state, a.into()),
			Inst::AndAbsoluteX(a) => and_absolute_x(state, a.into()),
			Inst::AndAbsoluteY(a) => and_absolute_y(state, a.into()),
			Inst::AndImmediate(x) => and_immediate(state, *x),
			Inst::AndIndirectX(x) => and_indirect_x(state, *x),
			Inst::AndIndirectY(x) => and_indirect_y(state, *x),
			Inst::AndZeroPage(x) => and_zero_page(state, *x),
			Inst::AndZeroPageX(x) => and_zero_page_x(state, *x),
			Inst::AslAbsolute(a) => asl_absolute(state, a.into()),
			Inst::AslAbsoluteX(a) => asl_absolute_x(state, a.into()),
			Inst::AslAccumulator => asl_accumulator(state),
			Inst::AslZeroPage(x) => asl_zero_page(state, *x),
			Inst::AslZeroPageX(x) => asl_zero_page_x(state, *x),
			Inst::Bcc(x) => bcc(state, *x),
			Inst::Bcs(x) => bcs(state, *x),
			Inst::Beq(x) => beq(state, *x),
			Inst::BitAbsolute(a) => bit_absolute(state, a.into()),
			Inst::BitZeroPage(x) => bit_zero_page(state, *x),
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
			Inst::CmpAbsolute(a) => cmp_absolute(state, a.into()),
			Inst::CmpAbsoluteX(a) => cmp_absolute_x(state, a.into()),
			Inst::CmpAbsoluteY(a) => cmp_absolute_y(state, a.into()),
			Inst::CmpImmediate(x) => cmp_immediate(state, *x),
			Inst::CmpIndirectX(x) => cmp_indirect_x(state, *x),
			Inst::CmpIndirectY(x) => cmp_indirect_y(state, *x),
			Inst::CmpZeroPage(x) => cmp_zero_page(state, *x),
			Inst::CmpZeroPageX(x) => cmp_zero_page_x(state, *x),
			Inst::CpxAbsolute(a) => cpx_absolute(state, a.into()),
			Inst::CpxImmediate(x) => cpx_immediate(state, *x),
			Inst::CpxZeroPage(x) => cpx_zero_page(state, *x),
			Inst::CpyAbsolute(a) => cpy_absolute(state, a.into()),
			Inst::CpyImmediate(x) => cpy_immediate(state, *x),
			Inst::CpyZeroPage(x) => cpy_zero_page(state, *x),
			Inst::DcpAbsolute(x) => dcp_absolute(state, x.as_u16()),
			Inst::DcpAbsoluteX(x) => dcp_absolute_x(state, x.as_u16()),
			Inst::DcpAbsoluteY(x) => dcp_absolute_y(state, x.as_u16()),
			Inst::DcpIndirectX(x) => dcp_indirect_x(state, *x),
			Inst::DcpIndirectY(x) => dcp_indirect_y(state, *x),
			Inst::DcpZeroPage(x) => dcp_zero_page(state, *x),
			Inst::DcpZeroPageX(x) => dcp_zero_page_x(state, *x),
			Inst::DecAbsolute(a) => dec_absolute(state, a.into()),
			Inst::DecAbsoluteX(a) => dec_absolute_x(state, a.into()),
			Inst::DecZeroPage(x) => dec_zero_page(state, *x),
			Inst::DecZeroPageX(x) => dec_zero_page_x(state, *x),
			Inst::Dex => dex(state),
			Inst::Dey => dey(state),
			Inst::EorAbsolute(a) => eor_absolute(state, a.into()),
			Inst::EorAbsoluteX(a) => eor_absolute_x(state, a.into()),
			Inst::EorAbsoluteY(a) => eor_absolute_y(state, a.into()),
			Inst::EorImmediate(x) => eor_immediate(state, *x),
			Inst::EorIndirectX(x) => eor_indirect_x(state, *x),
			Inst::EorIndirectY(x) => eor_indirect_y(state, *x),
			Inst::EorZeroPage(x) => eor_zero_page(state, *x),
			Inst::EorZeroPageX(x) => eor_zero_page_x(state, *x),
			Inst::Ign(_) => ign(state),
			Inst::IgnAbsoluteX(x) => ign_absolute_x(state, x.as_u16()),
			Inst::IgnAbsoluteX2(x) => ign_absolute_x(state, x.as_u16()),
			Inst::IgnAbsoluteX3(x) => ign_absolute_x(state, x.as_u16()),
			Inst::IgnAbsoluteX4(x) => ign_absolute_x(state, x.as_u16()),
			Inst::IgnAbsoluteX5(x) => ign_absolute_x(state, x.as_u16()),
			Inst::IgnAbsoluteX6(x) => ign_absolute_x(state, x.as_u16()),
			Inst::IncAbsolute(a) => inc_absolute(state, a.into()),
			Inst::IncAbsoluteX(a) => inc_absolute_x(state, a.into()),
			Inst::IncZeroPage(x) => inc_zero_page(state, *x),
			Inst::IncZeroPageX(x) => inc_zero_page_x(state, *x),
			Inst::Inx => inx(state),
			Inst::Iny => iny(state),
			Inst::IscAbsolute(x) => isc_absolute(state, x.as_u16()),
			Inst::IscAbsoluteX(x) => isc_absolute_x(state, x.as_u16()),
			Inst::IscAbsoluteY(x) => isc_absolute_y(state, x.as_u16()),
			Inst::IscIndirectX(x) => isc_indirect_x(state, *x),
			Inst::IscIndirectY(x) => isc_indirect_y(state, *x),
			Inst::IscZeroPage(x) => isc_zero_page(state, *x),
			Inst::IscZeroPageX(x) => isc_zero_page_x(state, *x),
			Inst::JmpAbsolute(a) => jmp_absolute(state, a.into()),
			Inst::JmpIndirect(x) => jmp_indirect(state, x.into()),
			Inst::Jsr(x) => jsr(state, x.into()),
			Inst::LaxAbsolute(x) => lax_absolute(state, x.as_u16()),
			Inst::LaxAbsoluteY(x) => lax_absolute_y(state, x.as_u16()),
			Inst::LaxIndirectX(x) => lax_indirect_x(state, *x),
			Inst::LaxIndirectY(x) => lax_indirect_y(state, *x),
			Inst::LaxZeroPage(x) => lax_zero_page(state, *x),
			Inst::LaxZeroPageY(x) => lax_zero_page_y(state, *x),
			Inst::LdaAbsolute(a) => lda_absolute(state, a.into()),
			Inst::LdaAbsoluteX(a) => lda_absolute_x(state, a.into()),
			Inst::LdaAbsoluteY(a) => lda_absolute_y(state, a.into()),
			Inst::LdaImmediate(x) => lda_immediate(state, *x),
			Inst::LdaIndirectX(x) => lda_indirect_x(state, *x),
			Inst::LdaIndirectY(x) => lda_indirect_y(state, *x),
			Inst::LdaZeroPage(x) => lda_zero_page(state, *x),
			Inst::LdaZeroPageX(x) => lda_zero_page_x(state, *x),
			Inst::LdxAbsolute(a) => ldx_absolute(state, a.into()),
			Inst::LdxAbsoluteY(a) => ldx_absolute_y(state, a.into()),
			Inst::LdxImmediate(x) => ldx_immediate(state, *x),
			Inst::LdxZeroPage(x) => ldx_zero_page(state, *x),
			Inst::LdxZeroPageY(x) => ldx_zero_page_y(state, *x),
			Inst::LdyAbsolute(a) => ldy_absolute(state, a.into()),
			Inst::LdyAbsoluteX(a) => ldy_absolute_x(state, a.into()),
			Inst::LdyImmediate(x) => ldy_immediate(state, *x),
			Inst::LdyZeroPage(x) => ldy_zero_page(state, *x),
			Inst::LdyZeroPageX(x) => ldy_zero_page_x(state, *x),
			Inst::LsrAbsolute(a) => lsr_absolute(state, a.into()),
			Inst::LsrAbsoluteX(a) => lsr_absolute_x(state, a.into()),
			Inst::LsrAccumulator => lsr_accumulator(state),
			Inst::LsrZeroPage(x) => lsr_zero_page(state, *x),
			Inst::LsrZeroPageX(x) => lsr_zero_page_x(state, *x),
			Inst::Nop => nop(state),
			Inst::Nop2 => nop(state),
			Inst::Nop3 => nop(state),
			Inst::Nop4 => nop(state),
			Inst::Nop5 => nop(state),
			Inst::Nop6 => nop(state),
			Inst::Nop7 => nop(state),
			Inst::OraAbsolute(a) => ora_absolute(state, a.into()),
			Inst::OraAbsoluteX(a) => ora_absolute_x(state, a.into()),
			Inst::OraAbsoluteY(a) => ora_absolute_y(state, a.into()),
			Inst::OraImmediate(x) => ora_immediate(state, *x),
			Inst::OraIndirectX(x) => ora_indirect_x(state, *x),
			Inst::OraIndirectY(x) => ora_indirect_y(state, *x),
			Inst::OraZeroPage(x) => ora_zero_page(state, *x),
			Inst::OraZeroPageX(x) => ora_zero_page_x(state, *x),
			Inst::Pha => pha(state),
			Inst::Php => php(state),
			Inst::Pla => pla(state),
			Inst::Plp => plp(state),
			Inst::RlaAbsolute(x) => rla_absolute(state, x.as_u16()),
			Inst::RlaAbsoluteX(x) => rla_absolute_x(state, x.as_u16()),
			Inst::RlaAbsoluteY(x) => rla_absolute_y(state, x.as_u16()),
			Inst::RlaIndirectX(x) => rla_indirect_x(state, *x),
			Inst::RlaIndirectY(x) => rla_indirect_y(state, *x),
			Inst::RlaZeroPage(x) => rla_zero_page(state, *x),
			Inst::RlaZeroPageX(x) => rla_zero_page_x(state, *x),
			Inst::RolAbsolute(a) => rol_absolute(state, a.into()),
			Inst::RolAbsoluteX(a) => rol_absolute_x(state, a.into()),
			Inst::RolAccumulator => rol_accumulator(state),
			Inst::RolZeroPage(x) => rol_zero_page(state, *x),
			Inst::RolZeroPageX(x) => rol_zero_page_x(state, *x),
			Inst::RorAbsolute(a) => ror_absolute(state, a.into()),
			Inst::RorAbsoluteX(a) => ror_absolute_x(state, a.into()),
			Inst::RorAccumulator => ror_accumulator(state),
			Inst::RorZeroPage(x) => ror_zero_page(state, *x),
			Inst::RorZeroPageX(x) => ror_zero_page_x(state, *x),
			Inst::RraAbsolute(x) => rra_absolute(state, x.as_u16()),
			Inst::RraAbsoluteX(x) => rra_absolute_x(state, x.as_u16()),
			Inst::RraAbsoluteY(x) => rra_absolute_y(state, x.as_u16()),
			Inst::RraIndirectX(x) => rra_indirect_x(state, *x),
			Inst::RraIndirectY(x) => rra_indirect_y(state, *x),
			Inst::RraZeroPage(x) => rra_zero_page(state, *x),
			Inst::RraZeroPageX(x) => rra_zero_page_x(state, *x),
			Inst::Rti => rti(state),
			Inst::Rts => rts(state),
			Inst::SaxAbsolute(x) => sax_absolute(state, x.as_u16()),
			Inst::SaxIndirectX(x) => sax_indirect_x(state, *x),
			Inst::SaxZeroPage(x) => sax_zero_page(state, *x),
			Inst::SaxZeroPageY(x) => sax_zero_page_y(state, *x),
			Inst::SbcAbsolute(a) => sbc_absolute(state, a.into()),
			Inst::SbcAbsoluteX(a) => sbc_absolute_x(state, a.into()),
			Inst::SbcAbsoluteY(a) => sbc_absolute_y(state, a.into()),
			Inst::SbcImmediate(x) => sbc_immediate(state, *x),
			Inst::SbcIndirectX(x) => sbc_indirect_x(state, *x),
			Inst::SbcIndirectY(x) => sbc_indirect_y(state, *x),
			Inst::SbcZeroPage(x) => sbc_zero_page(state, *x),
			Inst::SbcZeroPageX(x) => sbc_zero_page_x(state, *x),
			Inst::Sec => sec(state),
			Inst::Sed => sed(state),
			Inst::Sei => sei(state),
			Inst::Skb(_) => skb(state),
			Inst::Skb2(_) => skb(state),
			Inst::Skb3(_) => skb(state),
			Inst::Skb4(_) => skb(state),
			Inst::Skb5(_) => skb(state),
			Inst::SloAbsolute(x) => slo_absolute(state, x.as_u16()),
			Inst::SloAbsoluteX(x) => slo_absolute_x(state, x.as_u16()),
			Inst::SloAbsoluteY(x) => slo_absolute_y(state, x.as_u16()),
			Inst::SloIndirectX(x) => slo_indirect_x(state, *x),
			Inst::SloIndirectY(x) => slo_indirect_y(state, *x),
			Inst::SloZeroPage(x) => slo_zero_page(state, *x),
			Inst::SloZeroPageX(x) => slo_zero_page_x(state, *x),
			Inst::SreAbsolute(x) => sre_absolute(state, x.as_u16()),
			Inst::SreAbsoluteX(x) => sre_absolute_x(state, x.as_u16()),
			Inst::SreAbsoluteY(x) => sre_absolute_y(state, x.as_u16()),
			Inst::SreIndirectX(x) => sre_indirect_x(state, *x),
			Inst::SreIndirectY(x) => sre_indirect_y(state, *x),
			Inst::SreZeroPage(x) => sre_zero_page(state, *x),
			Inst::SreZeroPageX(x) => sre_zero_page_x(state, *x),
			Inst::StaAbsolute(a) => sta_absolute(state, a.into()),
			Inst::StaAbsoluteX(a) => sta_absolute_x(state, a.into()),
			Inst::StaAbsoluteY(a) => sta_absolute_y(state, a.into()),
			Inst::StaIndirectX(x) => sta_indirect_x(state, *x),
			Inst::StaIndirectY(x) => sta_indirect_y(state, *x),
			Inst::StaZeroPage(x) => sta_zero_page(state, *x),
			Inst::StaZeroPageX(x) => sta_zero_page_x(state, *x),
			Inst::StxAbsolute(a) => stx_absolute(state, a.into()),
			Inst::StxZeroPage(x) => stx_zero_page(state, *x),
			Inst::StxZeroPageY(x) => stx_zero_page_y(state, *x),
			Inst::StyAbsolute(a) => sty_absolute(state, a.into()),
			Inst::StyZeroPage(x) => sty_zero_page(state, *x),
			Inst::StyZeroPageX(x) => sty_zero_page_x(state, *x),
			Inst::Tax => tax(state),
			Inst::Tay => tay(state),
			Inst::Tsx => tsx(state),
			Inst::Txa => txa(state),
			Inst::Txs => txs(state),
			Inst::Tya => tya(state),

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
			_ => {
				todo!(
					"No support for unofficial instructions yet ({self:?}, {:02X?})",
					unsafe { std::mem::transmute::<Inst, [u8; 3]>(*self) }
				)
			}
		}
	}

	pub fn instruction_representation(&self) -> String {
		match self {
			Inst::AdcAbsolute(a) => format!("adc_absolute(state, {a});\n"),
			Inst::AdcAbsoluteX(a) => format!("adc_absolute_x(state, {a});\n"),
			Inst::AdcAbsoluteY(a) => format!("adc_absolute_y(state, {a});\n"),
			Inst::AdcImmediate(x) => format!("adc_immediate(state, {x});\n"),
			Inst::AdcIndirectX(x) => format!("adc_indirect_x(state, {x});\n"),
			Inst::AdcIndirectY(x) => format!("adc_indirect_y(state, {x});\n"),
			Inst::AdcZeroPage(x) => format!("adc_zero_page(state, {x});\n"),
			Inst::AdcZeroPageX(x) => format!("adc_zero_page_x(state, {x});\n"),
			Inst::AhxAbsoluteY(x) => format!("ahx_absolute_y(state, {x});\n"),
			Inst::AhxIndirectY(x) => format!("ahx_indirect_y(state, {x});\n"),
			Inst::AlrImmediate(x) => format!("alr_immediate(state, {x});\n"),
			Inst::AncImmediate(x) | Inst::AncImmediate2(x) => {
				format!("anc_immediate(state, {x});\n")
			}
			Inst::AndAbsolute(a) => format!("and_absolute(state, {a});\n"),
			Inst::AndAbsoluteX(a) => format!("and_absolute_x(state, {a});\n"),
			Inst::AndAbsoluteY(a) => format!("and_absolute_y(state, {a});\n"),
			Inst::AndImmediate(x) => format!("and_immediate(state, {x});\n"),
			Inst::AndIndirectX(x) => format!("and_indirect_x(state, {x});\n"),
			Inst::AndIndirectY(x) => format!("and_indirect_y(state, {x});\n"),
			Inst::AndZeroPage(x) => format!("and_zero_page(state, {x});\n"),
			Inst::AndZeroPageX(x) => format!("and_zero_page_x(state, {x});\n"),
			Inst::ArrImmediate(x) => format!("arr_immediate(state, {x});\n"),
			Inst::AslAbsolute(a) => format!("asl_absolute(state, {a});\n"),
			Inst::AslAbsoluteX(a) => format!("asl_absolute_x(state, {a});\n"),
			Inst::AslAccumulator => format!("asl_accumulator(state);\n"),
			Inst::AslZeroPage(x) => format!("asl_zero_page(state, {x});\n"),
			Inst::AslZeroPageX(x) => format!("asl_zero_page_x(state, {x});\n"),
			Inst::AxsImmediate(x) => format!("axs_immediate(state, {x});\n"),
			Inst::Bcc(x) => format!("bcc(state, {x});\n"),
			Inst::Bcs(x) => format!("bcs(state, {x});\n"),
			Inst::Beq(x) => format!("beq(state, {x});\n"),
			Inst::BitAbsolute(a) => format!("bit_absolute(state, {a});\n"),
			Inst::BitZeroPage(x) => format!("bit_zero_page(state, {x});\n"),
			Inst::Bmi(x) => format!("bmi(state, {x});\n"),
			Inst::Bne(x) => format!("bne(state, {x});\n"),
			Inst::Bpl(x) => format!("bpl(state, {x});\n"),
			Inst::Brk => format!("brk(state);\n"),
			Inst::Bvc(x) => format!("bvc(state, {x});\n"),
			Inst::Bvs(x) => format!("bvs(state, {x});\n"),
			Inst::Clc => format!("clc(state);\n"),
			Inst::Cld => format!("cld(state);\n"),
			Inst::Cli => format!("cli(state);\n"),
			Inst::Clv => format!("clv(state);\n"),
			Inst::CmpAbsolute(a) => format!("cmp_absolute(state, {a});\n"),
			Inst::CmpAbsoluteX(a) => format!("cmp_absolute_x(state, {a});\n"),
			Inst::CmpAbsoluteY(a) => format!("cmp_absolute_y(state, {a});\n"),
			Inst::CmpImmediate(x) => format!("cmp_immediate(state, {x});\n"),
			Inst::CmpIndirectX(x) => format!("cmp_indirect_x(state, {x});\n"),
			Inst::CmpIndirectY(x) => format!("cmp_indirect_y(state, {x});\n"),
			Inst::CmpZeroPage(x) => format!("cmp_zero_page(state, {x});\n"),
			Inst::CmpZeroPageX(x) => format!("cmp_zero_page_x(state, {x});\n"),
			Inst::CpxAbsolute(a) => format!("cpx_absolute(state, {a});\n"),
			Inst::CpxImmediate(x) => format!("cpx_immediate(state, {x});\n"),
			Inst::CpxZeroPage(x) => format!("cpx_zero_page(state, {x});\n"),
			Inst::CpyAbsolute(a) => format!("cpy_absolute(state, {a});\n"),
			Inst::CpyImmediate(x) => format!("cpy_immediate(state, {x});\n"),
			Inst::CpyZeroPage(x) => format!("cpy_zero_page(state, {x});\n"),
			Inst::DcpAbsolute(x) => format!("dcp_absolute(state, {x});\n"),
			Inst::DcpAbsoluteX(x) => format!("dcp_absolute_x(state, {x});\n"),
			Inst::DcpAbsoluteY(x) => format!("dcp_absolute_y(state, {x});\n"),
			Inst::DcpIndirectX(x) => format!("dcp_indirect_x(state, {x});\n"),
			Inst::DcpIndirectY(x) => format!("dcp_indirect_y(state, {x});\n"),
			Inst::DcpZeroPage(x) => format!("dcp_zero_page(state, {x});\n"),
			Inst::DcpZeroPageX(x) => format!("dcp_zero_page_x(state, {x});\n"),
			Inst::DecAbsolute(a) => format!("dec_absolute(state, {a});\n"),
			Inst::DecAbsoluteX(a) => format!("dec_absolute_x(state, {a});\n"),
			Inst::DecZeroPage(x) => format!("dec_zero_page(state, {x});\n"),
			Inst::DecZeroPageX(x) => format!("dec_zero_page_x(state, {x});\n"),
			Inst::Dex => format!("dex(state);\n"),
			Inst::Dey => format!("dey(state);\n"),
			Inst::EorAbsolute(a) => format!("eor_absolute(state, {a});\n"),
			Inst::EorAbsoluteX(a) => format!("eor_absolute_x(state, {a});\n"),
			Inst::EorAbsoluteY(a) => format!("eor_absolute_y(state, {a});\n"),
			Inst::EorImmediate(x) => format!("eor_immediate(state, {x});\n"),
			Inst::EorIndirectX(x) => format!("eor_indirect_x(state, {x});\n"),
			Inst::EorIndirectY(x) => format!("eor_indirect_y(state, {x});\n"),
			Inst::EorZeroPage(x) => format!("eor_zero_page(state, {x});\n"),
			Inst::EorZeroPageX(x) => format!("eor_zero_page_x(state, {x});\n"),
			Inst::Ign(x) => format!("ign(state, {x});\n"),
			Inst::IgnAbsoluteX(x) => format!("ign_absolute_x(state, {x});\n"),
			Inst::IgnAbsoluteX2(x) => format!("ign_absolute_x(state, {x});\n"),
			Inst::IgnAbsoluteX3(x) => format!("ign_absolute_x(state, {x});\n"),
			Inst::IgnAbsoluteX4(x) => format!("ign_absolute_x(state, {x});\n"),
			Inst::IgnAbsoluteX5(x) => format!("ign_absolute_x(state, {x});\n"),
			Inst::IgnAbsoluteX6(x) => format!("ign_absolute_x(state, {x});\n"),
			Inst::IgnDirect(x) | Inst::IgnDirect2(x) | Inst::IgnDirect3(x) => {
				format!("ign_direct(state, {x});\n")
			}
			Inst::IgnDirectX(x)
			| Inst::IgnDirectX2(x)
			| Inst::IgnDirectX3(x)
			| Inst::IgnDirectX4(x)
			| Inst::IgnDirectX5(x)
			| Inst::IgnDirectX6(x) => format!("ign_direct_x(state, {x});\n"),
			Inst::IncAbsolute(a) => format!("inc_absolute(state, {a});\n"),
			Inst::IncAbsoluteX(a) => format!("inc_absolute_x(state, {a});\n"),
			Inst::IncZeroPage(x) => format!("inc_zero_page(state, {x});\n"),
			Inst::IncZeroPageX(x) => format!("inc_zero_page_x(state, {x});\n"),
			Inst::Inx => format!("inx(state);\n"),
			Inst::Iny => format!("iny(state);\n"),
			Inst::IscAbsolute(x) => format!("isc_absolute(state, {x});\n"),
			Inst::IscAbsoluteX(x) => format!("isc_absolute_x(state, {x});\n"),
			Inst::IscAbsoluteY(x) => format!("isc_absolute_y(state, {x});\n"),
			Inst::IscIndirectX(x) => format!("isc_indirect_x(state, {x});\n"),
			Inst::IscIndirectY(x) => format!("isc_indirect_y(state, {x});\n"),
			Inst::IscZeroPage(x) => format!("isc_zero_page(state, {x});\n"),
			Inst::IscZeroPageX(x) => format!("isc_zero_page_x(state, {x});\n"),
			Inst::JmpAbsolute(a) => format!("jmp_absolute(state, 0x{a:X});\n"),
			Inst::JmpIndirect(x) => format!("jmp_indirect(state, 0x{x:X});\n"),
			Inst::Jsr(x) => format!("jsr(state, 0x{x:X});\n"),
			Inst::LasAbsoluteY(x) => format!("las_absolute_y(state, {x});\n"),
			Inst::LaxAbsolute(x) => format!("lax_absolute(state, {x});\n"),
			Inst::LaxAbsoluteY(x) => format!("lax_absolute_y(state, {x});\n"),
			Inst::LaxImmediate(x) => format!("lax_immediate(state, {x});\n"),
			Inst::LaxIndirectX(x) => format!("lax_indirect_x(state, {x});\n"),
			Inst::LaxIndirectY(x) => format!("lax_indirect_y(state, {x});\n"),
			Inst::LaxZeroPage(x) => format!("lax_zero_page(state, {x});\n"),
			Inst::LaxZeroPageY(x) => format!("lax_zero_page_y(state, {x});\n"),
			Inst::LdaAbsolute(a) => format!("lda_absolute(state, {a});\n"),
			Inst::LdaAbsoluteX(a) => format!("lda_absolute_x(state, {a});\n"),
			Inst::LdaAbsoluteY(a) => format!("lda_absolute_y(state, {a});\n"),
			Inst::LdaImmediate(x) => format!("lda_immediate(state, {x});\n"),
			Inst::LdaIndirectX(x) => format!("lda_indirect_x(state, {x});\n"),
			Inst::LdaIndirectY(x) => format!("lda_indirect_y(state, {x});\n"),
			Inst::LdaZeroPage(x) => format!("lda_zero_page(state, {x});\n"),
			Inst::LdaZeroPageX(x) => format!("lda_zero_page_x(state, {x});\n"),
			Inst::LdxAbsolute(a) => format!("ldx_absolute(state, {a});\n"),
			Inst::LdxAbsoluteY(a) => format!("ldx_absolute_y(state, {a});\n"),
			Inst::LdxImmediate(x) => format!("ldx_immediate(state, {x});\n"),
			Inst::LdxZeroPage(x) => format!("ldx_zero_page(state, {x});\n"),
			Inst::LdxZeroPageY(x) => format!("ldx_zero_page_y(state, {x});\n"),
			Inst::LdyAbsolute(a) => format!("ldy_absolute(state, {a});\n"),
			Inst::LdyAbsoluteX(a) => format!("ldy_absolute_x(state, {a});\n"),
			Inst::LdyImmediate(x) => format!("ldy_immediate(state, {x});\n"),
			Inst::LdyZeroPage(x) => format!("ldy_zero_page(state, {x});\n"),
			Inst::LdyZeroPageX(x) => format!("ldy_zero_page_x(state, {x});\n"),
			Inst::LsrAbsolute(a) => format!("lsr_absolute(state, {a});\n"),
			Inst::LsrAbsoluteX(a) => format!("lsr_absolute_x(state, {a});\n"),
			Inst::LsrAccumulator => format!("lsr_accumulator(state);\n"),
			Inst::LsrZeroPage(x) => format!("lsr_zero_page(state, {x});\n"),
			Inst::LsrZeroPageX(x) => format!("lsr_zero_page_x(state, {x});\n"),
			Inst::Nop => format!("nop(state);\n"),
			Inst::Nop2 => format!("nop(state);\n"),
			Inst::Nop3 => format!("nop(state);\n"),
			Inst::Nop4 => format!("nop(state);\n"),
			Inst::Nop5 => format!("nop(state);\n"),
			Inst::Nop6 => format!("nop(state);\n"),
			Inst::Nop7 => format!("nop(state);\n"),
			Inst::OraAbsolute(a) => format!("ora_absolute(state, {a});\n"),
			Inst::OraAbsoluteX(a) => format!("ora_absolute_x(state, {a});\n"),
			Inst::OraAbsoluteY(a) => format!("ora_absolute_y(state, {a});\n"),
			Inst::OraImmediate(x) => format!("ora_immediate(state, {x});\n"),
			Inst::OraIndirectX(x) => format!("ora_indirect_x(state, {x});\n"),
			Inst::OraIndirectY(x) => format!("ora_indirect_y(state, {x});\n"),
			Inst::OraZeroPage(x) => format!("ora_zero_page(state, {x});\n"),
			Inst::OraZeroPageX(x) => format!("ora_zero_page_x(state, {x});\n"),
			Inst::Pha => format!("pha(state);\n"),
			Inst::Php => format!("php(state);\n"),
			Inst::Pla => format!("pla(state);\n"),
			Inst::Plp => format!("plp(state);\n"),
			Inst::RlaAbsolute(x) => format!("rla_absolute(state, {x});\n"),
			Inst::RlaAbsoluteX(x) => format!("rla_absolute_x(state, {x});\n"),
			Inst::RlaAbsoluteY(x) => format!("rla_absolute_y(state, {x});\n"),
			Inst::RlaIndirectX(x) => format!("rla_indirect_x(state, {x});\n"),
			Inst::RlaIndirectY(x) => format!("rla_indirect_y(state, {x});\n"),
			Inst::RlaZeroPage(x) => format!("rla_zero_page(state, {x});\n"),
			Inst::RlaZeroPageX(x) => format!("rla_zero_page_x(state, {x});\n"),
			Inst::RolAbsolute(a) => format!("rol_absolute(state, {a});\n"),
			Inst::RolAbsoluteX(a) => format!("rol_absolute_x(state, {a});\n"),
			Inst::RolAccumulator => format!("rol_accumulator(state);\n"),
			Inst::RolZeroPage(x) => format!("rol_zero_page(state, {x});\n"),
			Inst::RolZeroPageX(x) => format!("rol_zero_page_x(state, {x});\n"),
			Inst::RorAbsolute(a) => format!("ror_absolute(state, {a});\n"),
			Inst::RorAbsoluteX(a) => format!("ror_absolute_x(state, {a});\n"),
			Inst::RorAccumulator => format!("ror_accumulator(state);\n"),
			Inst::RorZeroPage(x) => format!("ror_zero_page(state, {x});\n"),
			Inst::RorZeroPageX(x) => format!("ror_zero_page_x(state, {x});\n"),
			Inst::RraAbsolute(x) => format!("rra_absolute(state, {x});\n"),
			Inst::RraAbsoluteX(x) => format!("rra_absolute_x(state, {x});\n"),
			Inst::RraAbsoluteY(x) => format!("rra_absolute_y(state, {x});\n"),
			Inst::RraIndirectX(x) => format!("rra_indirect_x(state, {x});\n"),
			Inst::RraIndirectY(x) => format!("rra_indirect_y(state, {x});\n"),
			Inst::RraZeroPage(x) => format!("rra_zero_page(state, {x});\n"),
			Inst::RraZeroPageX(x) => format!("rra_zero_page_x(state, {x});\n"),
			Inst::Rti => format!("rti(state);\n"),
			Inst::Rts => format!("rts(state);\n"),
			Inst::SaxAbsolute(x) => format!("sax_absolute(state, {x});\n"),
			Inst::SaxIndirectX(x) => format!("sax_indirect_x(state, {x});\n"),
			Inst::SaxZeroPage(x) => format!("sax_zero_page(state, {x});\n"),
			Inst::SaxZeroPageY(x) => format!("sax_zero_page_y(state, {x});\n"),
			Inst::SbcAbsolute(a) => format!("sbc_absolute(state, {a});\n"),
			Inst::SbcAbsoluteX(a) => format!("sbc_absolute_x(state, {a});\n"),
			Inst::SbcAbsoluteY(a) => format!("sbc_absolute_y(state, {a});\n"),
			Inst::SbcImmediate(x) => format!("sbc_immediate(state, {x});\n"),
			Inst::SbcImmediate(x) | Inst::SbcImmediate2(x) => {
				format!("sbc_immediate(state, {x});\n")
			}
			Inst::SbcIndirectX(x) => format!("sbc_indirect_x(state, {x});\n"),
			Inst::SbcIndirectY(x) => format!("sbc_indirect_y(state, {x});\n"),
			Inst::SbcZeroPage(x) => format!("sbc_zero_page(state, {x});\n"),
			Inst::SbcZeroPageX(x) => format!("sbc_zero_page_x(state, {x});\n"),
			Inst::Sec => format!("sec(state);\n"),
			Inst::Sed => format!("sed(state);\n"),
			Inst::Sei => format!("sei(state);\n"),
			Inst::ShxAbsoluteY(x) => format!("shx_absolute_y(state, {x});\n"),
			Inst::ShyAbsoluteX(x) => format!("shy_absolute_x(state, {x});\n"),
			Inst::Skb(_) => format!("skb(state);\n"),
			Inst::Skb2(_) => format!("skb(state);\n"),
			Inst::Skb3(_) => format!("skb(state);\n"),
			Inst::Skb4(_) => format!("skb(state);\n"),
			Inst::Skb5(_) => format!("skb(state);\n"),
			Inst::SloAbsolute(x) => format!("slo_absolute(state, {x});\n"),
			Inst::SloAbsoluteX(x) => format!("slo_absolute_x(state, {x});\n"),
			Inst::SloAbsoluteY(x) => format!("slo_absolute_y(state, {x});\n"),
			Inst::SloIndirectX(x) => format!("slo_indirect_x(state, {x});\n"),
			Inst::SloIndirectY(x) => format!("slo_indirect_y(state, {x});\n"),
			Inst::SloZeroPage(x) => format!("slo_zero_page(state, {x});\n"),
			Inst::SloZeroPageX(x) => format!("slo_zero_page_x(state, {x});\n"),
			Inst::SreAbsolute(x) => format!("sre_absolute(state, {x});\n"),
			Inst::SreAbsoluteX(x) => format!("sre_absolute_x(state, {x});\n"),
			Inst::SreAbsoluteY(x) => format!("sre_absolute_y(state, {x});\n"),
			Inst::SreIndirectX(x) => format!("sre_indirect_x(state, {x});\n"),
			Inst::SreIndirectY(x) => format!("sre_indirect_y(state, {x});\n"),
			Inst::SreZeroPage(x) => format!("sre_zero_page(state, {x});\n"),
			Inst::SreZeroPageX(x) => format!("sre_zero_page_x(state, {x});\n"),
			Inst::StaAbsolute(a) => format!("sta_absolute(state, {a});\n"),
			Inst::StaAbsoluteX(a) => format!("sta_absolute_x(state, {a});\n"),
			Inst::StaAbsoluteY(a) => format!("sta_absolute_y(state, {a});\n"),
			Inst::StaIndirectX(x) => format!("sta_indirect_x(state, {x});\n"),
			Inst::StaIndirectY(x) => format!("sta_indirect_y(state, {x});\n"),
			Inst::StaZeroPage(x) => format!("sta_zero_page(state, {x});\n"),
			Inst::StaZeroPageX(x) => format!("sta_zero_page_x(state, {x});\n"),
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
			| Inst::Stp12 => format!("stp(state);\n"),
			Inst::StxAbsolute(a) => format!("stx_absolute(state, {a});\n"),
			Inst::StxZeroPage(x) => format!("stx_zero_page(state, {x});\n"),
			Inst::StxZeroPageY(x) => format!("stx_zero_page_y(state, {x});\n"),
			Inst::StyAbsolute(a) => format!("sty_absolute(state, {a});\n"),
			Inst::StyZeroPage(x) => format!("sty_zero_page(state, {x});\n"),
			Inst::StyZeroPageX(x) => format!("sty_zero_page_x(state, {x});\n"),
			Inst::TasAbsoluteY(x) => format!("tas_absolute_y(state, {x});\n"),
			Inst::Tax => format!("tax(state);\n"),
			Inst::Tay => format!("tay(state);\n"),
			Inst::Tsx => format!("tsx(state);\n"),
			Inst::Txa => format!("txa(state);\n"),
			Inst::Txs => format!("txs(state);\n"),
			Inst::Tya => format!("tya(state);\n"),
			Inst::XaaImmediate(x) => format!("xaa_immediate(state, {x});\n"),
			Inst::Stp => "\n".into(),
		}
	}
}
