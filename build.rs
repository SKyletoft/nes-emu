fn main() {
	// Tell cargo to invalidate the built crate whenever the header changes
	println!("cargo:rerun-if-changed=src/evaluate_instruction.c");

	let mut build = cc::Build::new();
	build.file("src/evaluate_instruction.c");

	build.flag("-w");
	// build.flag("-Wall").flag("-Wextra");
	build.flag("-I.");

	// Check the optimization level
	let opt_level = std::env::var("OPT_LEVEL").unwrap_or_default();
	match opt_level.as_str() {
		"0" => {
			// Debug build
			build.flag("-Og").flag("-g3");
		}
		"1" | "2" | "3" | "s" | "z" => {
			// Release build
			build.flag("-O3");
		}
		_ => {
			panic!("Unknown opt_level!");
		}
	}

	build.compile("evaluate_instruction");
}
