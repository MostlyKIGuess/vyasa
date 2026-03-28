{
  description = "Vyasa - AI text pattern analyzer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            rustup
            wasm-pack
            openssl
            pkg-config
          ];

          shellHook = ''
            echo "Vyasa "
            echo "Run 'cargo build' to build"
            echo "Run 'cargo run' to start the server on port 6767"
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "vyasa";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          buildInputs = with pkgs; [ openssl pkg-config ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
        };
      }
    );
}
