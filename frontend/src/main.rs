#![feature(const_array, const_trait_impl)]
#![cfg_attr(target_arch = "xtensa", no_std)]
#![cfg_attr(target_arch = "xtensa", no_main)]

pub mod debug_mode;
pub mod helpers;

#[cfg(not(target_arch = "xtensa"))]
pub mod badge;

#[cfg(not(target_os = "horizon"))]
pub mod linux;
#[cfg(not(target_os = "horizon"))]
pub mod sdl_framebuffer;

#[cfg(target_os = "horizon")]
pub mod citro2d_framebuffer;
#[cfg(target_os = "horizon")]
pub mod console;

#[cfg_attr(target_arch = "xtensa", esp_hal::main)]
fn main() {
	#[cfg(not(target_os = "horizon"))]
	linux::main();

	#[cfg(target_os = "horizon")]
	console::main();

	#[cfg(target_arch = "xtensa")]
	badge::main();
}
