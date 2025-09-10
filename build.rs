fn main() {
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

	// Generate bindings from the header file
	let bindings = bindgen::Builder::default()
		.header("src/evaluate-instruction.c")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	// Write the bindings to the $OUT_DIR/bindings.rs file
	let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("instbindings.rs"))
		.expect("Couldn't write bindings!");
}
