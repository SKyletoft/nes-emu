use bitfields::bitfield;
use bytemuck::{Pod, Zeroable};

pub const NES_CPU_CLOCKSPEED_HZ: u32 = 1_789_773;
const QUARTER_FRAME_CYCLES: u16 = 7458;

const LENGTH_TABLE: [u8; 32] = [
	10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 36, 22,
	56, 24, 52, 26, 104, 28, 208, 30,
];

const NOISE_PERIOD_TABLE: [u16; 16] = [
	4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];

const DMC_RATE_TABLE: [u16; 16] = [
	428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
];

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
	pub length_counter_load: u8,
	#[bits(3)]
	pub timer_high: u8,
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

#[derive(Clone, Copy, Debug)]
pub enum ApuWrite {
	Pulse1Reg0(u8, usize),
	Pulse1Reg1(u8, usize),
	Pulse1Reg2(u8, usize),
	Pulse1Reg3(u8, usize),
	Pulse2Reg0(u8, usize),
	Pulse2Reg1(u8, usize),
	Pulse2Reg2(u8, usize),
	Pulse2Reg3(u8, usize),
	TriangleReg0(u8, usize),
	TriangleReg2(u8, usize),
	TriangleReg3(u8, usize),
	NoiseReg0(u8, usize),
	NoiseReg2(u8, usize),
	NoiseReg3(u8, usize),
	DmcReg0(u8, usize),
	DmcReg1(u8, usize),
	DmcReg2(u8, usize),
	DmcReg3(u8, usize),
	Status(u8, usize),
	FrameCounter(u8, usize),
}

impl ApuWrite {
	pub fn cycle(&self) -> usize {
		match *self {
			Self::Pulse1Reg0(_, c)
			| Self::Pulse1Reg1(_, c)
			| Self::Pulse1Reg2(_, c)
			| Self::Pulse1Reg3(_, c)
			| Self::Pulse2Reg0(_, c)
			| Self::Pulse2Reg1(_, c)
			| Self::Pulse2Reg2(_, c)
			| Self::Pulse2Reg3(_, c)
			| Self::TriangleReg0(_, c)
			| Self::TriangleReg2(_, c)
			| Self::TriangleReg3(_, c)
			| Self::NoiseReg0(_, c)
			| Self::NoiseReg2(_, c)
			| Self::NoiseReg3(_, c)
			| Self::DmcReg0(_, c)
			| Self::DmcReg1(_, c)
			| Self::DmcReg2(_, c)
			| Self::DmcReg3(_, c)
			| Self::Status(_, c)
			| Self::FrameCounter(_, c) => c,
		}
	}
}

#[derive(Debug, Clone)]
pub struct Apu {
	pub pulse1: Pulse,
	pub pulse2: Pulse,
	pub triangle: Triangle,
	pub noise: Noise,
	pub dmc: Dmc,
	pub status: Status,
	pub frame_counter: u8,
	pub prg_rom: &'static [u8],

	frame_divider: u16,
	frame_step: u8,

	pulse1_timer: u16,
	pulse1_sequence: u8,
	pub pulse1_length_counter: u8,
	pulse1_envelope_start: bool,
	pulse1_envelope_divider: u8,
	pulse1_envelope_decay: u8,
	pulse1_sweep_counter: u8,
	pulse1_sweep_reload: bool,
	pulse1_current_period: u16,

	pulse2_timer: u16,
	pulse2_sequence: u8,
	pub pulse2_length_counter: u8,
	pulse2_envelope_start: bool,
	pulse2_envelope_divider: u8,
	pulse2_envelope_decay: u8,
	pulse2_sweep_counter: u8,
	pulse2_sweep_reload: bool,
	pulse2_current_period: u16,

	triangle_timer: u16,
	triangle_period: u16,
	pub triangle_length_counter: u8,
	triangle_linear_counter: u8,
	triangle_linear_counter_reload: bool,
	triangle_sequence: u8,
	triangle_counting_up: bool,

	noise_timer: u16,
	pub noise_length_counter: u8,
	noise_envelope_start: bool,
	noise_envelope_divider: u8,
	noise_envelope_decay: u8,
	noise_lfsr: u16,

	dmc_timer: u16,
	dmc_start_address: u16,
	dmc_remaining_length: u16,
	dmc_sample_buffer: u8,
	dmc_bits_remaining: u8,
	dmc_silence: bool,
	dmc_output_level: u8,
}

impl Apu {
	const DUTY_CYCLES: [[bool; 8]; 4] = [
		[false, true, false, false, false, false, false, false],
		[false, true, true, false, false, false, false, false],
		[false, true, true, true, true, false, false, false],
		[true, false, false, true, true, true, true, true],
	];

	pub fn write_register(&mut self, write: &ApuWrite) {
		match *write {
			ApuWrite::Pulse1Reg0(val, _) => {
				self.pulse1_reg0(val);
			}
			ApuWrite::Pulse1Reg1(val, _) => {
				self.pulse1_reg1(val);
			}
			ApuWrite::Pulse1Reg2(val, _) => {
				self.pulse1.set_timer_low(val);
			}
			ApuWrite::Pulse1Reg3(val, _) => {
				self.pulse1_reg3(val);
			}
			ApuWrite::Pulse2Reg0(val, _) => {
				self.pulse2_reg0(val);
			}
			ApuWrite::Pulse2Reg1(val, _) => {
				self.pulse2_reg1(val);
			}
			ApuWrite::Pulse2Reg2(val, _) => {
				self.pulse2.set_timer_low(val);
			}
			ApuWrite::Pulse2Reg3(val, _) => {
				self.pulse2_reg3(val);
			}
			ApuWrite::TriangleReg0(val, _) => {
				self.triangle_reg0(val);
			}
			ApuWrite::TriangleReg2(val, _) => {
				self.triangle.set_timer_low(val);
			}
			ApuWrite::TriangleReg3(val, _) => {
				self.triangle_reg3(val);
			}
			ApuWrite::NoiseReg0(val, _) => {
				self.noise_reg0(val);
			}
			ApuWrite::NoiseReg2(val, _) => {
				self.noise_reg2(val);
			}
			ApuWrite::NoiseReg3(val, _) => {
				self.noise_reg3(val);
			}
			ApuWrite::DmcReg0(val, _) => {
				self.dmc_reg0(val);
			}
			ApuWrite::DmcReg1(val, _) => {
				self.dmc_reg1(val);
			}
			ApuWrite::DmcReg2(val, _) => {
				self.dmc_reg2(val);
			}
			ApuWrite::DmcReg3(val, _) => {
				self.dmc_reg3(val);
			}
			ApuWrite::Status(val, _) => self.write_apu_status(val),
			ApuWrite::FrameCounter(val, _) => {
				self.frame_counter = val;
				self.frame_divider = 0;
				self.frame_step = 0;
				if (val >> 7) & 1 != 0 {
					self.status.set_frame_interrupt(false);
				}
			}
		}
	}

	fn pulse1_reg0(&mut self, val: u8) {
		let duty = val >> 6;
		let loop_halt = (val >> 5) & 1 != 0;
		let const_vol = (val >> 4) & 1 != 0;
		let vol_env = val & 0x0F;
		self.pulse1.set_duty(duty);
		self.pulse1.set_envelope_loop_length_counter_halt(loop_halt);
		self.pulse1.set_constant_volume(const_vol);
		self.pulse1.set_volume_envelope(vol_env);
		self.pulse1_envelope_start = true;
	}

	fn pulse1_reg1(&mut self, val: u8) {
		let enabled = (val >> 7) & 1 != 0;
		let period = (val >> 4) & 7;
		let negate = (val >> 3) & 1 != 0;
		let shift = val & 7;
		self.pulse1.set_sweep_unit_enabled(enabled);
		self.pulse1.set_period(period);
		self.pulse1.set_negate(negate);
		self.pulse1.set_shift(shift);
		self.pulse1_sweep_reload = true;
	}

	fn pulse1_reg3(&mut self, val: u8) {
		let length_idx = (val >> 3) as usize;
		let timer_high = val & 7;
		self.pulse1.set_length_counter_load(length_idx as u8);
		self.pulse1.set_timer_high(timer_high);
		self.pulse1_length_counter = LENGTH_TABLE[length_idx];
		let period = (self.pulse1.timer_low() as u16) | ((timer_high as u16) << 8);
		self.pulse1_current_period = period;
		self.pulse1_timer = period;
		self.pulse1_sequence = 0;
	}

	fn pulse2_reg0(&mut self, val: u8) {
		let duty = val >> 6;
		let loop_halt = (val >> 5) & 1 != 0;
		let const_vol = (val >> 4) & 1 != 0;
		let vol_env = val & 0x0F;
		self.pulse2.set_duty(duty);
		self.pulse2.set_envelope_loop_length_counter_halt(loop_halt);
		self.pulse2.set_constant_volume(const_vol);
		self.pulse2.set_volume_envelope(vol_env);
		self.pulse2_envelope_start = true;
	}

	fn pulse2_reg1(&mut self, val: u8) {
		let enabled = (val >> 7) & 1 != 0;
		let period = (val >> 4) & 7;
		let negate = (val >> 3) & 1 != 0;
		let shift = val & 7;
		self.pulse2.set_sweep_unit_enabled(enabled);
		self.pulse2.set_period(period);
		self.pulse2.set_negate(negate);
		self.pulse2.set_shift(shift);
		self.pulse2_sweep_reload = true;
	}

	fn pulse2_reg3(&mut self, val: u8) {
		let length_idx = (val >> 3) as usize;
		let timer_high = val & 7;
		self.pulse2.set_length_counter_load(length_idx as u8);
		self.pulse2.set_timer_high(timer_high);
		self.pulse2_length_counter = LENGTH_TABLE[length_idx];
		let period = (self.pulse2.timer_low() as u16) | ((timer_high as u16) << 8);
		self.pulse2_current_period = period;
		self.pulse2_timer = period;
		self.pulse2_sequence = 0;
	}

	fn triangle_reg0(&mut self, val: u8) {
		let control = (val >> 7) & 1 != 0;
		let linear_reload = val & 0x7F;
		self.triangle.set_control_flag(control);
		self.triangle.set_linear_counter_reload_value(linear_reload);
	}

	fn triangle_reg3(&mut self, val: u8) {
		let length_idx = (val >> 3) as usize;
		let timer_high = val & 7;
		self.triangle.set_length_counter_load(length_idx as u8);
		self.triangle.set_timer_high(timer_high);
		self.triangle_length_counter = LENGTH_TABLE[length_idx];
		self.triangle_linear_counter_reload = true;
		let period = (self.triangle.timer_low() as u16) | ((timer_high as u16) << 8);
		self.triangle_period = period;
		self.triangle_timer = period;
	}

	fn noise_reg0(&mut self, val: u8) {
		let loop_halt = (val >> 5) & 1 != 0;
		let const_vol = (val >> 4) & 1 != 0;
		let vol_env = val & 0x0F;
		self.noise.set_envelope_loop_length_counter_halt(loop_halt);
		self.noise.set_constant_volume(const_vol);
		self.noise.set_volume_envelope(vol_env);
		self.noise_envelope_start = true;
	}

	fn noise_reg2(&mut self, val: u8) {
		let mode = (val >> 7) & 1 != 0;
		let period_index = val & 0x0F;
		self.noise.set_mode(mode);
		self.noise.set_period_index(period_index);
	}

	fn noise_reg3(&mut self, val: u8) {
		let length_idx = (val >> 3) as usize;
		self.noise.set_length_counter_load(length_idx as u8);
		self.noise_length_counter = LENGTH_TABLE[length_idx];
	}

	fn dmc_reg0(&mut self, val: u8) {
		let irq_enable = (val >> 7) & 1 != 0;
		let loop_flag = (val >> 6) & 1 != 0;
		let rate_index = val & 0x0F;
		self.dmc.set_irq_enable(irq_enable);
		self.dmc.set_loop_flag(loop_flag);
		self.dmc.set_rate_index(rate_index);
	}

	fn dmc_reg1(&mut self, val: u8) {
		self.dmc.set_direct_load(val & 0x7F);
		self.dmc_output_level = val & 0x7F;
	}

	fn dmc_reg2(&mut self, val: u8) {
		self.dmc.set_sample_address(val);
	}

	fn dmc_reg3(&mut self, val: u8) {
		self.dmc.set_sample_length(val);
	}

	pub fn peek_status(&self) -> u8 {
		let mut status = 0u8;
		if self.pulse1_length_counter > 0 {
			status |= 0x01;
		}
		if self.pulse2_length_counter > 0 {
			status |= 0x02;
		}
		if self.triangle_length_counter > 0 {
			status |= 0x04;
		}
		if self.noise_length_counter > 0 {
			status |= 0x08;
		}
		if !self.dmc_silence || self.dmc_remaining_length > 0 {
			status |= 0x10;
		}
		if self.status.frame_interrupt() {
			status |= 0x40;
		}
		if self.status.dmc_interrupt() {
			status |= 0x80;
		}
		status
	}

	pub fn read_status(&mut self) -> u8 {
		let status = self.peek_status();
		self.status.set_frame_interrupt(false);
		status
	}

	fn write_apu_status(&mut self, val: u8) {
		if (val & 0x01) == 0 {
			self.pulse1_length_counter = 0;
		}
		if (val & 0x02) == 0 {
			self.pulse2_length_counter = 0;
		}
		if (val & 0x04) == 0 {
			self.triangle_length_counter = 0;
		}
		if (val & 0x08) == 0 {
			self.noise_length_counter = 0;
		}
		if (val & 0x10) == 0 {
			self.dmc_remaining_length = 0;
			self.dmc_silence = true;
			self.status.set_dmc_interrupt(false);
		} else if self.dmc_remaining_length == 0 {
			self.start_dmc();
		}
	}

	fn start_dmc(&mut self) {
		let addr = 0xC000u16.wrapping_add((self.dmc.sample_address() as u16) << 6);
		self.dmc_start_address = if addr < 0x8000 { addr | 0x8000 } else { addr };
		let len = (self.dmc.sample_length() as u16) << 4;
		self.dmc_remaining_length = if len == 0 { 256 } else { len + 1 };
		self.dmc_silence = true;
		self.dmc_bits_remaining = 0;
	}

	fn clock_quarter_frame(&mut self) {
		clock_envelope(
			&mut self.pulse1_envelope_start,
			&mut self.pulse1_envelope_divider,
			&mut self.pulse1_envelope_decay,
			self.pulse1.volume_envelope(),
			self.pulse1.envelope_loop_length_counter_halt(),
		);
		clock_envelope(
			&mut self.pulse2_envelope_start,
			&mut self.pulse2_envelope_divider,
			&mut self.pulse2_envelope_decay,
			self.pulse2.volume_envelope(),
			self.pulse2.envelope_loop_length_counter_halt(),
		);
		clock_envelope(
			&mut self.noise_envelope_start,
			&mut self.noise_envelope_divider,
			&mut self.noise_envelope_decay,
			self.noise.volume_envelope(),
			self.noise.envelope_loop_length_counter_halt(),
		);

		if self.triangle_linear_counter_reload {
			self.triangle_linear_counter = self.triangle.linear_counter_reload_value();
			if !self.triangle.control_flag() {
				self.triangle_linear_counter_reload = false;
			}
		} else if self.triangle_linear_counter > 0 {
			self.triangle_linear_counter -= 1;
		}
	}

	fn clock_half_frame(&mut self) {
		clock_sweep(
			self.pulse1.sweep_unit_enabled(),
			self.pulse1.shift(),
			self.pulse1.negate(),
			&mut self.pulse1_sweep_counter,
			&mut self.pulse1_sweep_reload,
			self.pulse1.period(),
			&mut self.pulse1_current_period,
		);
		clock_sweep(
			self.pulse2.sweep_unit_enabled(),
			self.pulse2.shift(),
			self.pulse2.negate(),
			&mut self.pulse2_sweep_counter,
			&mut self.pulse2_sweep_reload,
			self.pulse2.period(),
			&mut self.pulse2_current_period,
		);
		clock_length_counter(
			self.pulse1.envelope_loop_length_counter_halt(),
			&mut self.pulse1_length_counter,
		);
		clock_length_counter(
			self.pulse2.envelope_loop_length_counter_halt(),
			&mut self.pulse2_length_counter,
		);
		clock_length_counter(
			self.triangle.control_flag(),
			&mut self.triangle_length_counter,
		);
		clock_length_counter(
			self.noise.envelope_loop_length_counter_halt(),
			&mut self.noise_length_counter,
		);
	}

	#[allow(clippy::too_many_arguments)]
	fn get_pulse_volume(
		length_counter: u8,
		sequence: u8,
		current_period: u16,
		duty: u8,
		sweep_enabled: bool,
		shift: u8,
		negate: bool,
		constant_volume: bool,
		volume_envelope: u8,
		envelope_decay: u8,
	) -> u8 {
		if length_counter == 0 {
			return 0;
		}
		if current_period < 8 {
			return 0;
		}
		if sweep_enabled && shift > 0 {
			let period = current_period;
			let delta = period >> shift;
			if negate {
				let target = period.wrapping_sub(delta).wrapping_sub(1);
				if target > 0x7FF {
					return 0;
				}
			} else {
				let target = period + delta;
				if target > 0x7FF {
					return 0;
				}
			}
		}
		if !Self::DUTY_CYCLES[duty as usize][sequence as usize] {
			return 0;
		}
		if constant_volume {
			volume_envelope
		} else {
			envelope_decay
		}
	}

	fn get_triangle_volume(&self) -> u8 {
		if self.triangle_length_counter == 0 || self.triangle_linear_counter == 0 {
			return 0;
		}
		self.triangle_sequence
	}

	fn get_noise_volume(&self) -> u8 {
		if self.noise_length_counter == 0 {
			return 0;
		}
		let lfsr_bit0 = self.noise_lfsr & 1;
		let vol = if self.noise.constant_volume() {
			self.noise.volume_envelope()
		} else {
			self.noise_envelope_decay
		};
		if lfsr_bit0 == 0 { vol } else { 0 }
	}

	pub fn get_sound(&mut self, cpu_cycles: usize) -> f32 {
		let mut total_pulse: u32 = 0;
		let mut total_triangle: u32 = 0;
		let mut total_noise: u32 = 0;
		let mut total_dmc: u32 = 0;

		for _ in 0..cpu_cycles {
			self.frame_divider += 1;
			if self.frame_divider >= QUARTER_FRAME_CYCLES {
				self.frame_divider = 0;
				self.clock_quarter_frame();
				if self.frame_step == 1 || self.frame_step == 3 {
					self.clock_half_frame();
				}
				self.frame_step = (self.frame_step + 1) & 3;
			}

			clock_pulse_timer(
				&mut self.pulse1_timer,
				&mut self.pulse1_sequence,
				self.pulse1_current_period,
			);
			clock_pulse_timer(
				&mut self.pulse2_timer,
				&mut self.pulse2_sequence,
				self.pulse2_current_period,
			);

			if self.triangle_timer == 0 {
				self.triangle_timer = self.triangle_period;
				if self.triangle_counting_up {
					if self.triangle_sequence == 15 {
						self.triangle_counting_up = false;
						self.triangle_sequence = 14;
					} else {
						self.triangle_sequence += 1;
					}
				} else if self.triangle_sequence == 0 {
					self.triangle_counting_up = true;
					self.triangle_sequence = 1;
				} else {
					self.triangle_sequence -= 1;
				}
			} else {
				self.triangle_timer -= 1;
			}

			if self.noise_timer == 0 {
				let period_index = self.noise.period_index() as usize;
				self.noise_timer = NOISE_PERIOD_TABLE[period_index];
				let feedback = (self.noise_lfsr & 1)
					^ ((self.noise_lfsr >> if self.noise.mode() { 6 } else { 1 }) & 1);
				self.noise_lfsr = (self.noise_lfsr >> 1) | (feedback << 14);
			} else {
				self.noise_timer -= 1;
			}

			if self.dmc_timer == 0 {
				let rate_idx = self.dmc.rate_index() as usize;
				self.dmc_timer = DMC_RATE_TABLE[rate_idx];

				if self.dmc_bits_remaining == 0 {
					if self.dmc_remaining_length > 0 && !self.prg_rom.is_empty() {
						let offset = (self.dmc_start_address.wrapping_sub(0x8000)) as usize;
						self.dmc_sample_buffer = self
							.prg_rom
							.get(offset % self.prg_rom.len())
							.copied()
							.unwrap_or(0);
						self.dmc_start_address = self.dmc_start_address.wrapping_add(1);
						if self.dmc_start_address < 0x8000 {
							self.dmc_start_address |= 0x8000;
						}
						self.dmc_remaining_length -= 1;
						self.dmc_bits_remaining = 8;
						self.dmc_silence = false;

						if self.dmc_remaining_length == 0 {
							if self.dmc.loop_flag() {
								self.start_dmc();
							} else if self.dmc.irq_enable() {
								self.status.set_dmc_interrupt(true);
							}
							self.dmc_silence = true;
						}
					} else {
						self.dmc_silence = true;
					}
				}

				if !self.dmc_silence {
					let bit = (self.dmc_sample_buffer >> 7) & 1;
					self.dmc_sample_buffer <<= 1;
					self.dmc_bits_remaining -= 1;
					self.dmc_output_level = if bit != 0 { self.dmc.direct_load() } else { 0 };
				}
			} else {
				self.dmc_timer -= 1;
			}

			total_pulse += Self::get_pulse_volume(
				self.pulse1_length_counter,
				self.pulse1_sequence,
				self.pulse1_current_period,
				self.pulse1.duty(),
				self.pulse1.sweep_unit_enabled(),
				self.pulse1.shift(),
				self.pulse1.negate(),
				self.pulse1.constant_volume(),
				self.pulse1.volume_envelope(),
				self.pulse1_envelope_decay,
			) as u32;
			total_pulse += Self::get_pulse_volume(
				self.pulse2_length_counter,
				self.pulse2_sequence,
				self.pulse2_current_period,
				self.pulse2.duty(),
				self.pulse2.sweep_unit_enabled(),
				self.pulse2.shift(),
				self.pulse2.negate(),
				self.pulse2.constant_volume(),
				self.pulse2.volume_envelope(),
				self.pulse2_envelope_decay,
			) as u32;
			total_triangle += self.get_triangle_volume() as u32;
			total_noise += self.get_noise_volume() as u32;
			total_dmc += (self.dmc_output_level as u32 * 15) / 127;
		}

		let avg_pulse = total_pulse as f32 / cpu_cycles as f32;
		let avg_triangle = total_triangle as f32 / cpu_cycles as f32;
		let avg_noise = total_noise as f32 / cpu_cycles as f32;
		let avg_dmc = total_dmc as f32 / cpu_cycles as f32;

		let mixed = avg_pulse + avg_triangle + avg_noise + avg_dmc;
		mixed / (15.0 * 4.0) * 2.0 - 1.0
	}
}

fn clock_envelope(
	start: &mut bool,
	divider: &mut u8,
	decay: &mut u8,
	volume_envelope: u8,
	loop_flag: bool,
) {
	if *start {
		*start = false;
		*decay = 15;
		*divider = volume_envelope;
	} else if *divider == 0 {
		*divider = volume_envelope;
		if *decay > 0 {
			*decay -= 1;
		} else if loop_flag {
			*decay = 15;
		}
	} else {
		*divider -= 1;
	}
}

fn clock_sweep(
	enabled: bool,
	shift: u8,
	negate: bool,
	counter: &mut u8,
	reload: &mut bool,
	period: u8,
	current_period: &mut u16,
) {
	if *counter == 0 || *reload {
		*counter = period;
		*reload = false;
		if enabled && shift > 0 {
			let p = *current_period;
			let delta = p >> shift;
			let target = if negate {
				p.wrapping_sub(delta).wrapping_sub(1)
			} else {
				p + delta
			};
			if (8..=0x7FF).contains(&target) {
				*current_period = target;
			}
		}
	} else {
		*counter -= 1;
	}
}

fn clock_length_counter(halt: bool, counter: &mut u8) {
	if !halt && *counter > 0 {
		*counter -= 1;
	}
}

fn clock_pulse_timer(timer: &mut u16, sequence: &mut u8, period: u16) {
	if *timer == 0 {
		*timer = period;
		*sequence = (*sequence + 1) & 7;
	} else {
		*timer -= 1;
	}
}

impl Default for Apu {
	fn default() -> Self {
		Self {
			pulse1: Pulse::from_bits(0),
			pulse2: Pulse::from_bits(0),
			triangle: Triangle::from_bits(0),
			noise: Noise::from_bits(0),
			dmc: Dmc::from_bits(0),
			status: Status::from_bits(0),
			frame_counter: 0,
			prg_rom: &[],
			frame_divider: 0,
			frame_step: 0,
			pulse1_timer: 0,
			pulse1_sequence: 0,
			pulse1_length_counter: 0,
			pulse1_envelope_start: false,
			pulse1_envelope_divider: 0,
			pulse1_envelope_decay: 0,
			pulse1_sweep_counter: 0,
			pulse1_sweep_reload: false,
			pulse1_current_period: 0,
			pulse2_timer: 0,
			pulse2_sequence: 0,
			pulse2_length_counter: 0,
			pulse2_envelope_start: false,
			pulse2_envelope_divider: 0,
			pulse2_envelope_decay: 0,
			pulse2_sweep_counter: 0,
			pulse2_sweep_reload: false,
			pulse2_current_period: 0,
			triangle_timer: 0,
			triangle_period: 0,
			triangle_length_counter: 0,
			triangle_linear_counter: 0,
			triangle_linear_counter_reload: false,
			triangle_sequence: 0,
			triangle_counting_up: true,
			noise_timer: 0,
			noise_length_counter: 0,
			noise_envelope_start: false,
			noise_envelope_divider: 0,
			noise_envelope_decay: 0,
			noise_lfsr: 0x7FFF,
			dmc_timer: 0,
			dmc_start_address: 0,
			dmc_remaining_length: 0,
			dmc_sample_buffer: 0,
			dmc_bits_remaining: 0,
			dmc_silence: true,
			dmc_output_level: 0,
		}
	}
}
