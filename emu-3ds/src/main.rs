use ctru::{
	prelude::*,
	services::{
		gfx::{Screen, Swap},
		gspgpu::FramebufferFormat,
	},
};

fn main() {
	let apt = Apt::new().unwrap();
	let mut hid = Hid::new().unwrap();
	let gfx = Gfx::with_formats_shared(FramebufferFormat::Bgr8, FramebufferFormat::Bgr8).unwrap();
	let _console = Console::new(gfx.bottom_screen.borrow_mut());

	println!("Hello, World!");
	println!("\x1b[29;16HPress Start to exit");

	let mut offset = 0;
	while apt.main_loop() {
		gfx.wait_for_vblank();
		offset += 1;
		{
			let mut top_screen = gfx.top_screen.borrow_mut();
			let frame_buf = top_screen.raw_framebuffer();
			let pixels = unsafe {
				std::slice::from_raw_parts_mut(
					frame_buf.ptr,
					frame_buf.height * frame_buf.width * 4,
				)
			};
			pixels
				.chunks_exact_mut(3 * 240)
				.enumerate()
				.for_each(|(idx, line)| {
					line.chunks_exact_mut(3).for_each(|pixel| {
						let chan = ((idx + offset) / 10) % 3;
						pixel.fill(0);
						pixel[chan] = 0xFF;
					})
				});
			top_screen.swap_buffers();
		}
		hid.scan_input();
		if hid.keys_down().contains(KeyPad::START) {
			break;
		}
	}
}
