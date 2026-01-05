pub mod apu;
pub mod controller;
pub mod cpu;
pub mod evaluate_instruction;
pub mod frame;
pub mod graphics;
pub mod inst;
pub mod interpret;
pub mod mapper;
pub mod mmc3;
pub mod nrom128;
pub mod nrom256;
pub mod ppu;

#[cfg(test)]
mod tests;

/// An assertion that is checked in debug mode and UB to violate in release mode.
#[macro_export]
macro_rules! unsafe_assert {
	($t:expr $(, $ts:expr)*) => {
		std::hint::assert_unchecked(true); // To silence unnecessary unsafe warning in debug builds
		#[cfg(debug_assertions)]
		assert!($t $(, $ts)*);
		#[cfg(not(debug_assertions))]
		::core::hint::assert_unchecked($t);
	};
}

#[macro_export]
macro_rules! unsafe_assert_eq {
	($t:expr, $t2:expr $(, $ts:expr)*) => {
		std::hint::assert_unchecked(true); // To silence unnecessary unsafe warning in debug builds
		#[cfg(debug_assertions)]
		assert_eq!($t, $t2 $(, $ts)*);
		#[cfg(not(debug_assertions))]
		::core::hint::assert_unchecked($t == $t2);
	};
}
