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
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	// Write the bindings to the $OUT_DIR/bindings.rs file
	let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("cpubindings.rs"))
		.expect("Couldn't write bindings!");
}

fn evaluate_instruction_c() {
	// Generate bindings from the header file
	let bindings = bindgen::Builder::default()
		.header("src/evaluate_instruction.c")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	// Write the bindings to the $OUT_DIR/bindings.rs file
	let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("instbindings.rs"))
		.expect("Couldn't write bindings!");

	// Compile the evaluate_instruction.c file
	cc::Build::new()
		.file("src/evaluate_instruction.c")
		.compile("evaluate_instruction");
}
