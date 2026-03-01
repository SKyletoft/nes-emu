{
	inputs = {
		nixpkgs.url     = "github:nixos/nixpkgs/nixpkgs-unstable";
		flake-utils.url = "github:numtide/flake-utils";
		devkitnix.url   = "github:SKyletoft/devkitnix";
		rust-overlay = {
			url = "github:oxalica/rust-overlay";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = { self, nixpkgs, devkitnix, rust-overlay, flake-utils }:
		flake-utils.lib.eachDefaultSystem(system:
			let
				pkgs = import nixpkgs {
					inherit system;
					overlays = [( import rust-overlay )];
				};
				rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
					extensions = [ "rust-src" "rust-analyzer" ];
					targets = [ "x86_64-unknown-linux-gnu" ];
				});
				devkitARM = devkitnix.packages.${system}.devkitARM;
				shellInputs = with pkgs; [
					rustToolchain
					cargo-3ds
					cargo-expand
					cargo-show-asm
					cargo-flamegraph

					devkitARM

					python3

					llvmPackages_21.clang-tools
					valgrind
					perf
					gdb
					gf

					fceux # For comparison
					mesen

					azahar

					kdePackages.kcachegrind
				];
				nativeBuildInputs = with pkgs; [
					llvmPackages_21.clang-unwrapped
					pkg-config
				];
				buildInputs = with pkgs; [
					llvmPackages_21.clang-unwrapped
					SDL2
				];
			in {
				devShells.default = pkgs.mkShell {
					packages = buildInputs ++ nativeBuildInputs ++ shellInputs;
					LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
					DEVKITARM = "${devkitARM}/devkitARM";
					DEVKITPRO = "${devkitARM}";
				};
				packages.default = pkgs.rustPlatform.buildRustPackage {
					pname = "nes-emu";
					version = "0.0.1";
					src = ./.;
					cargoLock.lockFile = ./Cargo.lock;
					doCheck = false; # All tests rely on non-free ROMs

					inherit nativeBuildInputs buildInputs;
				};
			}
		);
}
