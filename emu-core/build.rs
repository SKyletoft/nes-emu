fn main() {
	// Tell cargo to invalidate the built crate whenever the header changes
	println!("cargo:rerun-if-changed=inc/interface.h");
	println!("cargo:rerun-if-changed=src/evaluate_instruction.c");

	let mut build = cc::Build::new();
	build.file("src/evaluate_instruction.c");

	let target = std::env::var("TARGET").unwrap_or_default();

	match target.as_str() {
		"armv6k-nintendo-3ds" => {
			build.compiler("arm-none-eabi-gcc");
			build.flags([
				"-mfloat-abi=hard",
				"-mtune=mpcore",
				"-mtp=soft",
				"-march=armv6k",
			]);
		}
		_ => {
			build.compiler("clang");
		}
	};

	// build.flag("-w");
	build.flag("-Wall").flag("-Wextra").flag("-Wconversion");
	build.flag("-I.").flag("-Iinc").flag("-std=c23");

	// Check the optimization level
	let opt_level = std::env::var("OPT_LEVEL").unwrap_or_default();
	match opt_level.as_str() {
		"0" => {
			// Debug build
			build.flag("-Og").flag("-g3");
		}
		o @ ("1" | "2" | "3" | "s" | "z") => {
			build.flag(format!("-O{o}"));
		}
		_ => {
			panic!("Unknown opt_level!");
		}
	}

	build.compile("evaluate_instruction");
}
