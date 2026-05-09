use bitfields::bitfield;
use bytemuck::{Pod, Zeroable};

use crate::const_assert_eq;

const NES_CPU_CLOCKSPEED_HZ: f32 = 1_789_773.;

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
	sweep_unit_enabled: bool,
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

#[bitfield(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Zeroable, Pod)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
pub struct Apu {
	pub pulse1: Pulse,
	pub pulse2: Pulse,
	pub triangle: Triangle,
	pub noise: Noise,
	pub dmc: Dmc,
	pub frame_counter: u8,
	pub status: Status,
	pub _unused: u16,
}

impl Apu {
	pub fn registers_as_raw_bytes_mut(&mut self) -> &mut [u8; 0x14] {
		const_assert_eq!(size_of::<Apu>(), 0x18);
		let full_thing: &mut [u8; 0x18] = bytemuck::cast_mut(self);
		let prefix: &mut [u8; 0x14] = (&mut full_thing[..0x14]).try_into().expect("0x14 < 0x18");
		prefix
	}

	pub fn write_status(&mut self, val: u8) {
		let new_status = Status::from_bits(val & 0b0001_1111);
		if !new_status.noise_active() {
			self.noise.set_length_counter_load(0);
		}
		if !new_status.triangle_active() {
			self.noise.set_length_counter_load(0);
		}
		if !new_status.pulse1_active() {
			self.pulse1.set_length_counter_load(0);
		}
		if !new_status.pulse2_active() {
			self.pulse2.set_length_counter_load(0);
		}

		self.status.0 &= 0b1110_0000;
		self.status.0 |= new_status.into_bits();
	}

	pub fn get_sound(&self, time: f32) -> f32 {
		[
			// if self.status.pulse1_active() {
			self.pulse1.get_sound(time), // } else {
			                             //	0.
			                             // },
			                             // self.pulse2.get_sound(time),
			                             // self.triangle.get_sound(time),
			                             // self.noise.get_sound(time),
			                             // self.dmc.get_sound(time),
		]
		.into_iter()
		.sum()
	}
}

impl Pulse {
	const DUTY_CYCLES: [[bool; 8]; 4] = [
		[false, true, false, false, false, false, false, false],
		[false, true, true, false, false, false, false, false],
		[false, true, true, true, true, false, false, false],
		[true, false, false, true, true, true, true, true],
	];

	fn get_sound(&self, time: f32) -> f32 {
		let period = ((self.timer_high() as u16) << 8) | (self.timer_low() as u16);
		if period < 8 {
			return 0.;
		}

		let mut effective_period = period;
		if self.sweep_unit_enabled() && self.shift() > 0 {
			let sweep_delta = period >> self.shift();
			if self.negate() {
				effective_period = effective_period.saturating_sub(sweep_delta);
			} else {
				effective_period = effective_period.saturating_add(sweep_delta);
			}
		}

		if effective_period < 8 {
			return 0.;
		}

		let apu_cycles = (time * NES_CPU_CLOCKSPEED_HZ / 2.) as usize;
		let duty_index = (apu_cycles / (effective_period as usize + 1)) % 8;

		eprintln!(
			"DEBUG: period={}, effective={}, apu_cycles={}, duty_idx={}, duty_val={}",
			period,
			effective_period,
			apu_cycles,
			duty_index,
			Self::DUTY_CYCLES[self.duty() as usize][duty_index] as i32
		);

		let duty_value = if Self::DUTY_CYCLES[self.duty() as usize][duty_index] {
			1.
		} else {
			-1.
		};

		let volume = self.volume_envelope() as f32;

		duty_value * volume / 15. / 2.
	}
}
