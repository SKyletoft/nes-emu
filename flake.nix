{
	inputs = {
		nixpkgs.url     = "github:nixos/nixpkgs/nixpkgs-unstable";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem(system:
			let
				pkgs = nixpkgs.legacyPackages.${system};
			in {
				devShells.default = pkgs.mkShell {
					nativeBuildInputs = with pkgs; [
						rustc
						cargo
						clippy
						rustfmt
						rust-analyzer
					];
					buildInputs = with pkgs; [
						gcc
					];
				};
			}
		);
}
