{
	inputs = {
		nixpkgs.url     = "github:nixos/nixpkgs/nixpkgs-unstable";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem(system:
			let
				pkgs = nixpkgs.legacyPackages.${system};
				nativeBuildInputs = with pkgs; [
					rustc
					cargo
					clippy
					rustfmt
					rust-analyzer

					python3
				];
				buildInputs = with pkgs; [
					gcc
				];
			in {
				devShells.default = pkgs.mkShell {
					inherit nativeBuildInputs buildInputs;
				};
				packages.default = pkgs.rustPlatform.buildRustPackage {
					pname = "nes-emu";
					version = "0.0.1";
					src = ./.;
					cargoLock.lockFile = ./Cargo.lock;

					inherit buildInputs;
				};
			}
		);
}
