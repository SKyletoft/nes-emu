fn main() {
	let shared_texture = emu_core::graphics::new_bitmap();

	let texture_ptr = shared_texture.clone();
	let game = Box::new(game::MAPPER.clone());
	let mut system_state = emu_core::interpret::State::new(game, texture_ptr);

	while system_state.rest.ppu.frame < 10000 {
		while system_state.rest.ppu_runahead <= 341 {
			game::nes_game(&mut system_state);
		}
		system_state.catch_up_ppu();
	}
}
