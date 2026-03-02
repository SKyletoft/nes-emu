use std::time::{Duration, Instant};

use emu_core::{controller::ControllerState, mapper::Mapper};
use sdl2::{controller::Button, event::Event, keyboard::Keycode};

use crate::sdl_framebuffer::SdlFramebuffer;

pub fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let controller_subsystem = sdl_context.game_controller().unwrap();

	let window = video_subsystem
		.window("NES Emulator", 800, 600)
		.resizable()
		.position_centered()
		.build()
		.map_err(|e| e.to_string())
		.unwrap();

	let mut canvas = window
		.into_canvas()
		.build()
		.map_err(|e| e.to_string())
		.unwrap();
	let texture_creator = canvas.texture_creator();

	let framebuffer = SdlFramebuffer::new(&texture_creator, &mut canvas).unwrap();

	let mut event_pump = sdl_context.event_pump().unwrap();

	for controller in 0..controller_subsystem.num_joysticks().unwrap() {
		if controller_subsystem.is_game_controller(controller) {
			std::mem::forget(controller_subsystem.open(controller));
		}
	}
	let mut controller_state = ControllerState::new();

	let game = emu_core::nrom256::NROM256::parse_ines(include_bytes!("../../non-free/SMB1.nes"))
		.map_err(|e| e.to_string())
		.unwrap()
		.with_framebuffer(framebuffer);
	let mut system_state = emu_core::interpret::State::new(game);

	const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / 60);

	'running: loop {
		let start_of_frame = Instant::now();
		let frame = system_state.rest.ppu.frame;

		while system_state.rest.ppu.frame == frame {
			if !handle_events(&mut event_pump, &mut controller_state, &mut system_state) {
				break 'running;
			}

			*system_state.rest.controller1.state_mut() = controller_state.into_bits();

			while system_state.rest.ppu_runahead <= 341 {
				system_state.next();
			}
			system_state.catch_up_ppu();
		}

		let end_of_frame = Instant::now();
		let to_sleep = FRAME_DURATION.saturating_sub(end_of_frame - start_of_frame);
		std::thread::sleep(to_sleep);
	}
}

fn handle_events(
	event_pump: &mut sdl2::EventPump,
	controller_state: &mut ControllerState,
	system_state: &mut emu_core::interpret::State<emu_core::nrom256::NROM256<SdlFramebuffer<'_>>>,
) -> bool {
	for event in event_pump.poll_iter() {
		match event {
			Event::Quit { .. }
			| Event::KeyDown {
				keycode: Some(Keycode::Escape | Keycode::Q),
				..
			} => return false,
			Event::KeyDown {
				keycode: Some(Keycode::Left),
				..
			}
			| Event::ControllerButtonDown {
				button: Button::DPadLeft,
				..
			} => {
				controller_state.set_left(true);
			}
			Event::KeyUp {
				keycode: Some(Keycode::Left),
				..
			}
			| Event::ControllerButtonUp {
				button: Button::DPadLeft,
				..
			} => {
				controller_state.set_left(false);
			}
			Event::KeyDown {
				keycode: Some(Keycode::Right),
				..
			}
			| Event::ControllerButtonDown {
				button: Button::DPadRight,
				..
			} => {
				controller_state.set_right(true);
			}
			Event::KeyUp {
				keycode: Some(Keycode::Right),
				..
			}
			| Event::ControllerButtonUp {
				button: Button::DPadRight,
				..
			} => {
				controller_state.set_right(false);
			}
			Event::KeyDown {
				keycode: Some(Keycode::Up),
				..
			}
			| Event::ControllerButtonDown {
				button: Button::DPadUp,
				..
			} => {
				controller_state.set_up(true);
			}
			Event::KeyUp {
				keycode: Some(Keycode::Up),
				..
			}
			| Event::ControllerButtonUp {
				button: Button::DPadUp,
				..
			} => {
				controller_state.set_up(false);
			}
			Event::KeyDown {
				keycode: Some(Keycode::Down),
				..
			}
			| Event::ControllerButtonDown {
				button: Button::DPadDown,
				..
			} => {
				controller_state.set_down(true);
			}
			Event::KeyUp {
				keycode: Some(Keycode::Down),
				..
			}
			| Event::ControllerButtonUp {
				button: Button::DPadDown,
				..
			} => {
				controller_state.set_down(false);
			}
			Event::KeyDown {
				keycode: Some(Keycode::Z),
				..
			}
			| Event::ControllerButtonDown {
				button: Button::A, ..
			} => {
				controller_state.set_a(true);
			}
			Event::KeyUp {
				keycode: Some(Keycode::Z),
				..
			}
			| Event::ControllerButtonUp {
				button: Button::A, ..
			} => {
				controller_state.set_a(false);
			}
			Event::KeyDown {
				keycode: Some(Keycode::X),
				..
			}
			| Event::ControllerButtonDown {
				button: Button::B | Button::X,
				..
			} => {
				controller_state.set_b(true);
			}
			Event::KeyUp {
				keycode: Some(Keycode::X),
				..
			}
			| Event::ControllerButtonUp {
				button: Button::B | Button::X,
				..
			} => {
				controller_state.set_b(false);
			}
			Event::KeyDown {
				keycode: Some(Keycode::Return),
				..
			}
			| Event::ControllerButtonDown {
				button: Button::Start,
				..
			} => {
				controller_state.set_start(true);
			}
			Event::KeyUp {
				keycode: Some(Keycode::Return),
				..
			}
			| Event::ControllerButtonUp {
				button: Button::Start,
				..
			} => {
				controller_state.set_start(false);
			}
			Event::KeyDown {
				keycode: Some(Keycode::RShift),
				..
			}
			| Event::ControllerButtonDown {
				button: Button::Back,
				..
			} => {
				controller_state.set_select(true);
			}
			Event::KeyUp {
				keycode: Some(Keycode::RShift),
				..
			}
			| Event::ControllerButtonUp {
				button: Button::Back,
				..
			} => {
				controller_state.set_select(false);
			}
			Event::ControllerButtonDown {
				button: Button::LeftShoulder,
				..
			} => {
				system_state.rest.rom.framebuffer().hide_left =
					!system_state.rest.rom.framebuffer().hide_left;
			}
			Event::ControllerButtonDown {
				button: Button::RightShoulder,
				..
			} => {
				system_state.rest.rom.framebuffer().hide_right =
					!system_state.rest.rom.framebuffer().hide_right;
			}
			_ => {}
		}
	}
	true
}
