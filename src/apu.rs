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

	pub fn read(&self, addr: u16) -> u8 {
		match addr {
			0x4000 => self.pulse1.sweep,
			0x4001 => self.pulse1.timer_low,
			0x4002 => self.pulse1.timer_high,
			0x4003 => self.pulse1.control,
			0x4004 => self.pulse2.sweep,
			0x4005 => self.pulse2.timer_low,
			0x4006 => self.pulse2.timer_high,
			0x4007 => self.pulse2.control,
			0x4008 => self.triangle.timer_low,
			0x4009 => self.triangle.timer_high,
			0x400A => self.triangle.control,
			0x400B => 0, // unused
			0x400C => self.noise.timer_low,
			0x400D => self.noise.timer_high,
			0x400E => self.noise.control,
			0x400F => 0, // unused
			0x4010 => self.dmc.timer_low,
			0x4011 => self.dmc.timer_high,
			0x4012 => self.dmc.control,
			0x4013 => 0, // unused
			0x4015 => self.status,
			0x4017 => self.frame_counter,
			_ => 0,
		}
	}

	pub fn write(&mut self, addr: u16, val: u8) {
		match addr {
			0x4000 => self.pulse1.sweep = val,
			0x4001 => self.pulse1.timer_low = val,
			0x4002 => self.pulse1.timer_high = val,
			0x4003 => self.pulse1.control = val,
			0x4004 => self.pulse2.sweep = val,
			0x4005 => self.pulse2.timer_low = val,
			0x4006 => self.pulse2.timer_high = val,
			0x4007 => self.pulse2.control = val,
			0x4008 => self.triangle.timer_low = val,
			0x4009 => self.triangle.timer_high = val,
			0x400A => self.triangle.control = val,
			0x400C => self.noise.timer_low = val,
			0x400D => self.noise.timer_high = val,
			0x400E => self.noise.control = val,
			0x4010 => self.dmc.timer_low = val,
			0x4011 => self.dmc.timer_high = val,
			0x4012 => self.dmc.control = val,
			0x4015 => self.status = val,
			0x4017 => self.frame_counter = val,
			_ => {}
		}
	}
}
