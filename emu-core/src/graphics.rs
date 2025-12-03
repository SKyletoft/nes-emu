use std::sync::{Arc, Mutex};

use bytemuck::{Pod, Zeroable};

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 240;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Pod, Zeroable)]
pub struct Colour {
	pub blue: u8,
	pub green: u8,
	pub red: u8,
	pub alpha: u8,
}

pub type Bitmap = [[Colour; WIDTH]; HEIGHT];

pub const fn empty_bitmap() -> Bitmap {
	[[Colour {
		blue: 0,
		green: 0,
		red: 0,
		alpha: 0,
	}; _]; _]
}

pub fn new_bitmap() -> Arc<Mutex<Box<Bitmap>>> {
	Arc::new(Mutex::new(Box::new(empty_bitmap())))
}
