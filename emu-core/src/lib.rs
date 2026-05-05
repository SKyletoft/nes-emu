pub mod apu;
pub mod controller;
pub mod cpu;
pub mod evaluate_instruction;
pub mod frame;
pub mod inst;
pub mod interpret;
pub mod mapper;
// pub mod mmc3;
pub mod nrom;
pub mod perf_stats;
pub mod ppu;

#[cfg(test)]
mod nestest;

#[cfg(test)]
mod tests;

/// An assertion that is checked in debug mode and UB to violate in release mode.
#[macro_export]
macro_rules! unsafe_assert {
	($t:expr $(, $ts:expr)*) => {{
		std::hint::assert_unchecked(true); // To silence unnecessary unsafe warning in debug builds
		#[cfg(debug_assertions)]
		assert!($t $(, $ts)*);
		#[cfg(not(debug_assertions))]
		::core::hint::assert_unchecked($t);
	}};
}

/// An assertion that is checked in debug mode and UB to violate in release mode.
#[macro_export]
macro_rules! unsafe_assert_eq {
	($t:expr, $t2:expr $(, $ts:expr)*) => {{
		std::hint::assert_unchecked(true); // To silence unnecessary unsafe warning in debug builds
		#[cfg(debug_assertions)]
		assert_eq!($t, $t2 $(, $ts)*);
		#[cfg(not(debug_assertions))]
		::core::hint::assert_unchecked($t == $t2);
	}};
}

/// An assertion that is checked in debug mode and UB to violate in release mode.
#[macro_export]
macro_rules! unsafe_unreachable {
	() => {{
		std::hint::assert_unchecked(true); // To silence unnecessary unsafe warning in debug builds
		#[cfg(debug_assertions)]
		unreachable!();
		#[cfg(not(debug_assertions))]
		::core::hint::unreachable_unchecked();
	}};
	($($ts:tt)*) => {{
		std::hint::assert_unchecked(true); // To silence unnecessary unsafe warning in debug builds
		#[cfg(debug_assertions)]
		unreachable!($($ts)*);
		#[cfg(not(debug_assertions))]
		::core::hint::unreachable_unchecked();
	}};
}

#[macro_export]
macro_rules! const_assert {
	($($arg:tt)*) => {
		const _: () = {
			assert!($($arg)*);
		};
	};
}

#[macro_export]
macro_rules! const_assert_eq {
	($left:expr, $right:expr $(, $($arg:tt)+)?) => {
		const _: () = {
			assert!($left == $right $(, $($arg)+)?);
		};
	};
}
