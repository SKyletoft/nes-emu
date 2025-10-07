use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct Pulse {
	pub sweep: u8,
	pub timer_low: u8,
	pub timer_high: u8,
	pub control: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct Triangle {
	pub timer_low: u8,
	pub timer_high: u8,
	pub control: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct Noise {
	pub timer_low: u8,
	pub timer_high: u8,
	pub control: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct Dmc {
	pub timer_low: u8,
	pub timer_high: u8,
	pub control: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct Apu {
	pub pulse1: Pulse,
	pub pulse2: Pulse,
	pub triangle: Triangle,
	pub noise: Noise,
	pub dmc: Dmc,
	pub frame_counter: u8,
	pub status: u8,
}

impl Apu {
	pub fn new() -> Self {
		Self::default()
	}

}
