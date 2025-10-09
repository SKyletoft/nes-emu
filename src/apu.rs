use bitfields::bitfield;
use bytemuck::{Pod, Zeroable};

#[bitfield(u32)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Pulse {
	#[bits(2)]
	duty: u8,
	#[bits(1)]
	envelope_loop_length_counter_halt: bool,
	#[bits(1)]
	constant_volume: bool,
	#[bits(4)]
	volume_envelope: u8,
	#[bits(1)]
	sweet_unit_enabled: bool,
	#[bits(3)]
	period: u8,
	#[bits(1)]
	negate: bool,
	#[bits(3)]
	shift: u8,
	#[bits(8)]
	timer_low: u8,
	#[bits(5)]
	length_counter_load: u8,
	#[bits(3)]
	timer_high: u8,
}

#[bitfield(u32)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Triangle {
	#[bits(1)]
	control_flag: bool,
	#[bits(7)]
	linear_counter_reload_value: u8,
	#[skip]
	__: u8,
	#[bits(8)]
	timer_low: u8,
	#[bits(5)]
	length_counter_load: u8,
	#[bits(3)]
	timer_high: u8,
}

#[bitfield(u32)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Noise {
	#[bits(2)]
	_unused: u8,
	#[bits(1)]
	envelope_loop_length_counter_halt: bool,
	#[bits(1)]
	constant_volume: bool,
	#[bits(4)]
	volume_envelope: u8,
	#[bits(8)]
	__: u8,
	#[bits(1)]
	mode: bool,
	#[bits(3)]
	__: u8,
	#[bits(4)]
	period_index: u8,
	#[bits(5)]
	length_counter_load: u8,
	#[bits(3)]
	__: u8,
}

#[bitfield(u32)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Dmc {
	#[bits(1)]
	irq_enable: bool,
	#[bits(1)]
	loop_flag: bool,
	#[bits(2)]
	__: u8,
	#[bits(4)]
	rate_index: u8,
	#[bits(1)]
	__: u8,
	#[bits(7)]
	direct_load: u8,
	#[bits(8)]
	sample_address: u8,
	#[bits(8)]
	sample_length: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Apu {
	pub pulse1: Pulse,
	pub pulse2: Pulse,
	pub triangle: Triangle,
	pub noise: Noise,
	pub dmc: Dmc,
	pub frame_counter: u8,
	pub status: Status,
}

#[bitfield(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Zeroable)]
pub struct Status {
	#[bits(1)]
	dmc_interrupt: bool,
	#[bits(1)]
	frame_interrupt: bool,
	#[bits(1)]
	_unused: bool,
	#[bits(1)]
	dmc_active: bool,
	#[bits(1)]
	noise_active: bool,
	#[bits(1)]
	triangle_active: bool,
	#[bits(1)]
	pulse1_active: bool,
	#[bits(1)]
	pulse2_active: bool,
}

impl Apu {
	fn write_status(&mut self, val: u8) {
		let new_status = Status::from_bits(val & 0b0001_1111);

		// The status register is used to enable and disable individual channels, control the DMC, and can read the status of length counters and APU interrupts.
		// $4015 write	---D NT21	Enable DMC (D), noise (N), triangle (T), and pulse channels (2/1)

		//     Writing a zero to any of the channel enable bits (NT21) will silence that channel and halt its length counter.
		//     If the DMC bit is clear, the DMC bytes remaining will be set to 0 and the DMC will silence when it empties.
		//     If the DMC bit is set, the DMC sample will be restarted only if its bytes remaining is 0. If there are bits remaining in the 1-byte sample buffer, these will finish playing before the next sample is fetched.
		//     Writing to this register clears the DMC interrupt flag.
		//     Power-up and reset have the effect of writing $00, silencing all channels.

		if !new_status.noise_active() {}

		self.status.0 &= 0b1110_0000;
		self.status.0 |= new_status.into_bits();
	}
	fn read_status(&mut self, val: u8) {}
}
