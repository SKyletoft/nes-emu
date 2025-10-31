{
	inputs = {
		nixpkgs.url     = "github:nixos/nixpkgs/nixpkgs-unstable";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem(system:
			let
				pkgs = nixpkgs.legacyPackages.${system};
				shellInputs = with pkgs; [
					rustc
					cargo
					clippy
					rustfmt
					rust-analyzer

					python3

					llvmPackages_21.clang-tools
					valgrind

					fceux # For comparison
					mesen

					kdePackages.kcachegrind
				];
				nativeBuildInputs = with pkgs; [
					clang
					pkg-config
				];
				buildInputs = with pkgs; [
					clang
					SDL2
				];
			in {
				devShells.default = pkgs.mkShell {
					packages = buildInputs ++ nativeBuildInputs ++ shellInputs;
					LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
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
