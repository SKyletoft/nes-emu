use bitfields::bitfield;
use bytemuck::{Pod, Zeroable};

#[bitfield(u8)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Controller {
	#[bits(1)]
	a: bool,
	#[bits(1)]
	b: bool,
	#[bits(1)]
	select: bool,
	#[bits(1)]
	start: bool,
	#[bits(1)]
	up: bool,
	#[bits(1)]
	down: bool,
	#[bits(1)]
	left: bool,
	#[bits(1)]
	right: bool,
}
