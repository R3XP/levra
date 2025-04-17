{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
		flake-parts.url = "github:hercules-ci/flake-parts";
		systems.url = "github:nix-systems/default";
		rust-overlay.url = "github:oxalica/rust-overlay";
	};

	outputs = inputs:
		inputs.flake-parts.lib.mkFlake { inherit inputs; } {
			systems = import inputs.systems;

			perSystem = { config, self', pkgs, lib, system, ... }:
				let
					rustToolchain = pkgs.rust-bin.stable.latest.default.override {
						extensions = [
							"rust-src"
							"rust-analyzer"
							"clippy"
						];
						targets = [
							"wasm32-unknown-unknown"
						];
					};

					rustBuildInputs = [
						pkgs.openssl
						pkgs.libiconv
						pkgs.pkg-config
					] ++ lib.optionals pkgs.stdenv.isLinux [
							pkgs.glib
							pkgs.gtk3
							pkgs.libsoup_3
							pkgs.webkitgtk_4_1
							pkgs.xdotool
						] ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
							IOKit
							Carbon
							WebKit
							Security
							Cocoa
						]);

				in {
					_module.args.pkgs = import inputs.nixpkgs {
						inherit system;
						overlays = [ inputs.rust-overlay.overlays.default ];
					};

					devShells.default = pkgs.mkShell {
						name = "dioxus-dev";
						buildInputs = rustBuildInputs;
						nativeBuildInputs = [ rustToolchain ];

						shellHook = ''
			  # Für rust-analyzer Tooltips
			  export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
			  export PATH="$HOME/.cargo/bin:$PATH"

			  if ! command -v dx &>/dev/null; then
			  echo "Installing dioxus-cli..."
			  cargo install dioxus-cli
			  fi
			  '';
					};
				};
		};
}
