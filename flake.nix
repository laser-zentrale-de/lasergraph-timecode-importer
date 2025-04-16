{
  description = "lasergraph-timecode-importer development";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable."1.85.1".default;
        cargo = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              rust
              pkg-config
              pre-commit
              nodejs
              nodePackages.npm
              goreleaser
            ];

            shellHook = ''
              HOOK_PATH=$(git rev-parse --git-path hooks/pre-commit)
              if [ ! -f "$HOOK_PATH" ]; then
                echo "Setting up pre-commit hooks..."
                ${pkgs.pre-commit}/bin/pre-commit install
              fi
            '';
          };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = cargo.package.name;
          version = cargo.package.version;
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      }
    );
}
