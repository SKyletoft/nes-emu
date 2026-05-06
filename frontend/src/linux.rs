use std::time::{Duration, Instant};

use emu_core::{
	controller::ControllerState, frame::NesFramebuffer, interpret::State, mapper::Mapper,
	nrom::NROM256,
};
use sdl2::{audio::AudioSpecDesired, controller::Button, event::Event, keyboard::Keycode};

use crate::{
	debug_mode::{BackgroundView, DebugMode},
	sdl_framebuffer::{SdlFramebuffer, SoundSample},
};

pub fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let controller_subsystem = sdl_context.game_controller().unwrap();
	let audio_subsystem = sdl_context.audio().unwrap();

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

	let mut audio_device = audio_subsystem
		.open_playback(
			None,
			&AudioSpecDesired {
				freq: None,
				channels: Some(1),
				samples: None,
			},
			|spec| SoundSample {
				apu_log: [Default::default()].into(),
				actual_spec: spec,
				time_in_seconds: 0.,
			},
		)
		.unwrap();

	let framebuffer =
		SdlFramebuffer::new(&texture_creator, &mut canvas, &mut audio_device).unwrap();

	let mut event_pump = sdl_context.event_pump().unwrap();

	for controller in 0..controller_subsystem.num_joysticks().unwrap() {
		if controller_subsystem.is_game_controller(controller) {
			std::mem::forget(controller_subsystem.open(controller));
		}
	}
	let mut controller_state = ControllerState::new();

	#[cfg(feature = "compiled-game")]
	let game = game::MAPPER.clone().with_framebuffer(framebuffer);

	#[cfg(not(feature = "compiled-game"))]
	let game = NROM256::parse_ines(include_bytes!("../../non-free/SMB1.nes"))
		.unwrap()
		.with_framebuffer(framebuffer);

	let mut system_state = State::new(game);

	const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / 60);

	system_state.rest.rom.framebuffer.audio_device.resume();
	'running: loop {
		let start_of_frame = Instant::now();
		let frame = system_state.rest.ppu.frame;

		while system_state.rest.ppu.frame == frame {
			if !handle_events(&mut event_pump, &mut controller_state, &mut system_state) {
				break 'running;
			}

			*system_state.rest.controller1.state_mut() = controller_state.into_bits();

			emu_core::perf_stats::start_cpu();
			while system_state.rest.ppu_runahead <= 341 {
				#[cfg(feature = "compiled-game")]
				game::nes_game(&mut system_state);

				#[cfg(not(feature = "compiled-game"))]
				system_state.next();
			}
			emu_core::perf_stats::stop_cpu();
			system_state.catch_up_ppu();
			emu_core::perf_stats::start_apu();
			system_state
				.rest
				.rom
				.framebuffer
				.render_audio(&system_state.rest.apu);
			emu_core::perf_stats::stop_apu();
		}

		println!("CPU: {}", system_state.rest.cycles);
		let end_of_frame = Instant::now();
		let to_sleep = FRAME_DURATION.saturating_sub(end_of_frame - start_of_frame);
		std::thread::sleep(to_sleep);
	}
}

fn handle_events(
	event_pump: &mut sdl2::EventPump,
	controller_state: &mut ControllerState,
	system_state: &mut State<NROM256<SdlFramebuffer<'_>>>,
) -> bool {
	let fb = system_state.rest.rom.framebuffer();
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
				if !fb.debug_mode_enabled {
					controller_state.set_left(true);
				} else {
					match fb.debug_mode {
						DebugMode::Backgrounds(view) => {
							fb.debug_mode = DebugMode::Backgrounds(view.prev());
						}
						DebugMode::Sprites(idx) => {
							fb.debug_mode = DebugMode::Sprites(if idx == 0 { 63 } else { idx - 1 });
						}
					}
				}
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
				if !fb.debug_mode_enabled {
					controller_state.set_right(true);
				} else {
					match fb.debug_mode {
						DebugMode::Backgrounds(view) => {
							fb.debug_mode = DebugMode::Backgrounds(view.next());
						}
						DebugMode::Sprites(idx) => {
							fb.debug_mode = DebugMode::Sprites(if idx == 63 { 0 } else { idx + 1 });
						}
					}
				}
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
				if !fb.debug_mode_enabled {
					controller_state.set_up(true);
				} else {
					match fb.debug_mode {
						DebugMode::Backgrounds(_) => {
							fb.debug_mode = DebugMode::Sprites(0);
						}
						DebugMode::Sprites(_) => {
							fb.debug_mode = DebugMode::Backgrounds(BackgroundView::Both);
						}
					}
				}
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
				if !fb.debug_mode_enabled {
					controller_state.set_down(true);
				} else {
					match fb.debug_mode {
						DebugMode::Backgrounds(_) => {
							fb.debug_mode = DebugMode::Sprites(0);
						}
						DebugMode::Sprites(_) => {
							fb.debug_mode = DebugMode::Backgrounds(BackgroundView::Both);
						}
					}
				}
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
				if !fb.debug_mode_enabled {
					controller_state.set_a(true);
				} else {
					fb.debug_background_mode = fb.debug_background_mode.next();
				}
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
			} if !fb.debug_mode_enabled => {
				fb.hide_left = !fb.hide_left;
			}
			Event::ControllerButtonDown {
				button: Button::RightShoulder,
				..
			} if !fb.debug_mode_enabled => {
				fb.hide_right = !fb.hide_right;
			}
			Event::KeyDown {
				keycode: Some(Keycode::D),
				..
			}
			| Event::ControllerButtonDown {
				button: Button::Y, ..
			} => {
				fb.debug_mode_enabled = !fb.debug_mode_enabled;
			}
			_ => {}
		}
	}
	true
}
