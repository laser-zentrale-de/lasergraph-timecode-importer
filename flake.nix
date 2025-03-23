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
      in {
        devShells.default = with pkgs;
          mkShell rec {
            buildInputs = [
              pkg-config
              rust-bin.nightly.latest.default
              pre-commit
              nodejs
              nodePackages.npm
            ];

            shellHook = ''
              HOOK_PATH=$(git rev-parse --git-path hooks/pre-commit)
              if [ ! -f "$HOOK_PATH" ]; then
                echo "Setting up pre-commit hooks..."
                ${pkgs.pre-commit}/bin/pre-commit install
              fi
            '';
          };
      }
    );
}
