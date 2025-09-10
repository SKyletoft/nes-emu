fn main() {
	cpu_h();
	evaluate_instruction_c();
}

fn cpu_h() {
	// Tell cargo to invalidate the built crate whenever the header changes
	println!("cargo:rerun-if-changed=inc/cpu.h");

	// Generate bindings from the header file
	let bindings = bindgen::Builder::default()
		.header("inc/cpu.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.generate()
		.expect("Unable to generate bindings");

	// Write the bindings to the $OUT_DIR/bindings.rs file
	let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("cpubindings.rs"))
		.expect("Couldn't write bindings!");
}

fn evaluate_instruction_c() {
	// Tell cargo to invalidate the built crate whenever the header changes
	println!("cargo:rerun-if-changed=src/evaluate_instruction.c");

	let mut build = cc::Build::new();
	build.file("src/evaluate_instruction.c");

	// Check the optimization level
	let opt_level = std::env::var("OPT_LEVEL").unwrap_or_default();
	match opt_level.as_str() {
		"0" => {
			// Debug build
			build.flag("-Wall").flag("-Wextra").flag("-Og").flag("-g3");
		}
		"1" | "2" | "3" | "s" | "z" => {
			// Release build
			build.flag("-w").flag("-O3");
		}
		_ => {
			// Default to release flags if unknown
			build.flag("-Wall").flag("-Wextra").flag("-O3");
		}
	}

	build.compile("evaluate_instruction");
}
