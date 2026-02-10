use std::time::{Duration, Instant};

use ctru::prelude::*;
use emu_core::{controller::ControllerState, interpret::State};

use crate::citro2d_framebuffer::Citro2DFramebuffer;

pub fn main() {
	let apt = Apt::new().unwrap();
	let mut hid = Hid::new().unwrap();
	let gfx = Gfx::new().unwrap();
	let _console = Console::new(gfx.bottom_screen.borrow_mut());
	println!(" FRAME   CPU   PPU  FPS  ACTUAL");

	// let game = game::MAPPER.clone();
	let game =
		emu_core::nrom256::NROM256::parse_ines(include_bytes!("../../non-free/SMB1.nes")).unwrap();
	let framebuffer = Citro2DFramebuffer::new(&gfx).unwrap();
	let mut system_state = State::new(game, framebuffer);

	let mut last_frame = 0;

	let mut cpu_dur = Duration::new(0, 0);
	let mut ppu_dur = Duration::new(0, 0);
	let mut frame_time = Instant::now();
	while apt.main_loop() {
		while last_frame == system_state.rest.ppu.frame {
			let before = Instant::now();
			while system_state.rest.ppu_runahead <= 341 {
				// game::nes_game(&mut system_state);
				system_state.next();
			}
			let after = Instant::now();
			cpu_dur += after - before;

			let before_ppu = Instant::now();
			system_state.catch_up_ppu();
			let after_ppu = Instant::now();
			ppu_dur += after_ppu - before_ppu;
		}
		last_frame = system_state.rest.ppu.frame;

		let frame_count = system_state.rest.ppu.frame;

		let done = Instant::now();
		let cpu_time = cpu_dur.as_millis();
		let ppu_time = ppu_dur.as_millis();
		let fps = 1.0 / (cpu_dur + ppu_dur).as_secs_f32();
		let frame_time_dur = (done - frame_time).as_millis();
		println!("{frame_count:5}: {cpu_time:3}ms {ppu_time:3}ms {fps:.02} {frame_time_dur:3}ms");
		frame_time = done;
		cpu_dur = Duration::new(0, 0);
		ppu_dur = Duration::new(0, 0);

		hid.scan_input();

		let mut c = ControllerState::new();
		c.set_a(hid.keys_held().contains(KeyPad::A));
		c.set_b(hid.keys_held().contains(KeyPad::B) || hid.keys_held().contains(KeyPad::X));
		c.set_start(hid.keys_held().contains(KeyPad::START));
		c.set_select(hid.keys_held().contains(KeyPad::SELECT));
		c.set_up(hid.keys_held().contains(KeyPad::DPAD_UP));
		c.set_down(hid.keys_held().contains(KeyPad::DPAD_DOWN));
		c.set_left(hid.keys_held().contains(KeyPad::DPAD_LEFT));
		c.set_right(hid.keys_held().contains(KeyPad::DPAD_RIGHT));
		*system_state.rest.controller1.state_mut() = c.into_bits();

		if hid.keys_down().contains(KeyPad::SELECT) {
			break;
		}
	}
}
