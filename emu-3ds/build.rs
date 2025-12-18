fn main() {
	println!("cargo:rerun-if-changed={}/../nesc/src/libmario.a", env!("CARGO_MANIFEST_DIR"));
	println!("cargo:rustc-link-search=native={}/../nesc/src", env!("CARGO_MANIFEST_DIR"));
	println!("cargo:rustc-link-lib=static=mario");
}
