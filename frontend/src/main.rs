#![feature(const_array, const_trait_impl)]

#[cfg(not(target_os = "horizon"))]
pub mod sdl_framebuffer;
#[cfg(not(target_os = "horizon"))]
pub mod linux;

#[cfg(target_os = "horizon")]
pub mod citro2d_framebuffer;
#[cfg(target_os = "horizon")]
pub mod console;

fn main() {
	#[cfg(not(target_os = "horizon"))]
	linux::main();

	#[cfg(target_os = "horizon")]
	console::main();
}
