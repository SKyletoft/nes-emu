fn main() {
	let shared_texture = emu_core::graphics::new_bitmap();

	let texture_ptr = shared_texture.clone();
	let game = Box::new(game::MAPPER.clone());
	let mut system_state = emu_core::interpret::State::new(game, texture_ptr);

	for _ in 0..1_000_000_000 {
		game::nes_game(&mut system_state);
		system_state.catch_up_ppu();
	}
}
