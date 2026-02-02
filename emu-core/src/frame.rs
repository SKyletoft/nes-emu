use crate::{
	mapper::Mapper,
	ppu::{NesColour, Ppu},
};

pub trait NesFramebuffer {
	fn set(&mut self, y: usize, x: usize, col: NesColour);
	fn swap(&mut self);

	fn render<M: Mapper>(&mut self, m: &M, ppu: &Ppu, lines: &[(i16, i16); 240]) {
		let bg = ppu.palettes[0][0];
		for dot in 0..256 {
			for (at, _) in lines.iter().copied().enumerate() {
				self.set(at, dot as usize, bg);
			}
		}
		if ppu.mask.show_spr() {
			for (idx, sprite) in ppu
				.oam
				.iter()
				.enumerate()
				.filter(|(_, s)| s.is_visible() && s.attr.priority())
			{
				let sprite_y = sprite.y as i16 + 1; // Hardware bug
				let sprite_x = sprite.x as i16;
				let sprite_pixels = m.get_sprite_pixels(idx, ppu);
				let y_range = sprite_y..(sprite_y + 8);
				let x_range = sprite_x..(sprite_x + 8);
				for ((line, dot), col) in y_range
					.flat_map(move |l| x_range.clone().map(move |d| (l, d)))
					.zip(sprite_pixels)
					.filter(|((l, d), _)| *l < 240 && *d < 256)
				{
					if let Some(col) = col {
						self.set(line as usize, dot as usize, col);
					}
				}
			}
		}

		if ppu.mask.show_bg() {
			for (at, pos) in lines.iter().enumerate() {
				for dot in 0..256 {
					let tilemap_x = (dot + pos.0) % 512;
					let tilemap_y = pos.1; // This is broken, but I'm preserving behaviour for now
					let palettes = ppu.palettes;
					let Some(col) = m.get_bg_pixel(tilemap_x, tilemap_y, ppu, &palettes) else {
						continue;
					};
					self.set(at, dot as usize, col);
				}
			}
		}

		if ppu.mask.show_spr() {
			for (idx, sprite) in ppu
				.oam
				.iter()
				.enumerate()
				.filter(|(_, s)| s.is_visible() && !s.attr.priority())
			{
				let sprite_y = sprite.y as i16 + 1; // Hardware bug
				let sprite_x = sprite.x as i16;
				let sprite_pixels = m.get_sprite_pixels(idx, ppu);
				let y_range = sprite_y..(sprite_y + 8);
				let x_range = sprite_x..(sprite_x + 8);
				for ((line, dot), col) in y_range
					.flat_map(move |l| x_range.clone().map(move |d| (l, d)))
					.zip(sprite_pixels)
					.filter(|((l, d), _)| *l < 240 && *d < 256)
				{
					if let Some(col) = col {
						self.set(line as usize, dot as usize, col);
					}
				}
			}
		}
		self.swap();
	}
}
