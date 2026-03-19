use std::{
	sync::atomic::{AtomicU32, Ordering},
	time::Instant,
};

pub struct PerfStats {
	pub cpu_time_us: AtomicU32,
	pub ppu_time_us: AtomicU32,
	pub gpu_time_us: AtomicU32,
	pub frame_start_us: AtomicU32,
}

struct TimerState {
	cpu_start: AtomicU32,
	ppu_start: AtomicU32,
	gpu_start: AtomicU32,
}

static PERF_STATS: PerfStats = PerfStats {
	cpu_time_us: AtomicU32::new(0),
	ppu_time_us: AtomicU32::new(0),
	gpu_time_us: AtomicU32::new(0),
	frame_start_us: AtomicU32::new(0),
};

static TIMER_STATE: TimerState = TimerState {
	cpu_start: AtomicU32::new(0),
	ppu_start: AtomicU32::new(0),
	gpu_start: AtomicU32::new(0),
};

static EPOCH: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

fn get_epoch() -> &'static Instant {
	EPOCH.get_or_init(Instant::now)
}

fn get_time_us() -> u32 {
	get_epoch().elapsed().as_micros() as u32
}

pub fn start_cpu() {
	if TIMER_STATE.cpu_start.load(Ordering::Relaxed) == 0 {
		TIMER_STATE
			.cpu_start
			.store(get_time_us(), Ordering::Relaxed);
	}
}

pub fn stop_cpu() {
	let start_us = TIMER_STATE.cpu_start.swap(0, Ordering::Relaxed);
	if start_us != 0 {
		PERF_STATS
			.cpu_time_us
			.fetch_add(get_time_us().saturating_sub(start_us), Ordering::Relaxed);
	}
}

pub fn start_ppu() {
	if TIMER_STATE.ppu_start.load(Ordering::Relaxed) == 0 {
		TIMER_STATE
			.ppu_start
			.store(get_time_us(), Ordering::Relaxed);
	}
}

pub fn stop_ppu() {
	let start_us = TIMER_STATE.ppu_start.swap(0, Ordering::Relaxed);
	if start_us != 0 {
		PERF_STATS
			.ppu_time_us
			.fetch_add(get_time_us().saturating_sub(start_us), Ordering::Relaxed);
	}
}

pub fn start_gpu() {
	if TIMER_STATE.gpu_start.load(Ordering::Relaxed) == 0 {
		TIMER_STATE
			.gpu_start
			.store(get_time_us(), Ordering::Relaxed);
	}
}

pub fn stop_gpu() {
	let start_us = TIMER_STATE.gpu_start.swap(0, Ordering::Relaxed);
	if start_us != 0 {
		PERF_STATS
			.gpu_time_us
			.fetch_add(get_time_us().saturating_sub(start_us), Ordering::Relaxed);
	}
}

#[derive(Debug, Copy, Clone)]
pub struct FrameStats {
	pub cpu_ns: u64,
	pub ppu_ns: u64,
	pub gpu_ns: u64,
	pub total_ns: u64,
}

impl std::fmt::Display for FrameStats {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let cpu_time = self.cpu_ms();
		let ppu_time = self.ppu_ms();
		let gpu_time = self.gpu_ms();
		let fps = self.fps();
		let frame_time = self.total_ms();
		write!(
			f,
			"{cpu_time:3}ms {ppu_time:3}ms {gpu_time:3}ms {fps:.02} {frame_time:3}ms"
		)
	}
}

impl FrameStats {
	pub fn cpu_ms(&self) -> u64 {
		self.cpu_ns / 1_000_000
	}

	pub fn ppu_ms(&self) -> u64 {
		self.ppu_ns / 1_000_000
	}

	pub fn gpu_ms(&self) -> u64 {
		self.gpu_ns / 1_000_000
	}

	pub fn total_ms(&self) -> u64 {
		self.total_ns / 1_000_000
	}

	pub fn fps(&self) -> f32 {
		let emulation_ns = self.cpu_ns + self.ppu_ns + self.gpu_ns;
		if emulation_ns == 0 {
			0.0
		} else {
			1_000_000_000.0 / emulation_ns as f32
		}
	}
}

pub fn get_and_reset_frame_stats() -> FrameStats {
	let now_us = get_time_us();
	let cpu_us = PERF_STATS.cpu_time_us.swap(0, Ordering::Relaxed);
	let ppu_us = PERF_STATS.ppu_time_us.swap(0, Ordering::Relaxed);
	let gpu_us = PERF_STATS.gpu_time_us.swap(0, Ordering::Relaxed);
	let frame_start = PERF_STATS
		.frame_start_us
		.swap(get_time_us(), Ordering::Relaxed);
	let total_us = now_us.saturating_sub(frame_start);

	FrameStats {
		cpu_ns: cpu_us as u64 * 1000,
		ppu_ns: ppu_us as u64 * 1000,
		gpu_ns: gpu_us as u64 * 1000,
		total_ns: total_us as u64 * 1000,
	}
}
