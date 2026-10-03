#![feature(const_array, const_trait_impl)]
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

pub mod debug_mode;
pub mod helpers;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub mod sdl_framebuffer;

#[cfg(target_os = "horizon")]
pub mod citro2d_framebuffer;
#[cfg(target_os = "horizon")]
pub mod console;

#[cfg(target_os = "none")]
pub mod badge;

#[cfg_attr(target_os = "none", expect(dead_code))]
fn main() {
	#[cfg(target_os = "linux")]
	linux::main();

	#[cfg(target_os = "horizon")]
	console::main();
}
