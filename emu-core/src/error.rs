use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum EmuError {
	#[error("missing iNES header")]
	MissingHeader,
	#[error("truncated ROM: expected {expected} bytes, found {found}")]
	Truncated { expected: usize, found: usize },
	#[error("wrong amount of PRG ROM: found {found} banks, expected {expected}")]
	WrongPrgSize { found: u8, expected: u8 },
	#[error("unknown mapper type {mapper}")]
	UnknownMapper { mapper: u8 },
	#[error("invalid colour id: 0x{value:X}")]
	InvalidColour { value: u8 },
}

pub type Result<T> = core::result::Result<T, EmuError>;
