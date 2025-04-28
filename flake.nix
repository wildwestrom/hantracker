{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = nixpkgs.legacyPackages.${system}.extend (
          final: prev: {
            rustPkgs = import nixpkgs {
              inherit system overlays;
            };
          }
        );
        rust-toolchain = pkgs.rustPkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            just
            markdownlint-cli2
            taplo-cli

            rust-toolchain
            pkg-config

            # for reqwest
            openssl

            sqlx-cli

            # GTK
            gtk4
            libadwaita
            gdk-pixbuf
            wrapGAppsHook
            gobject-introspection
          ];
          DATABASE_URL = "sqlite://data.sqlite?mode=rwc";
        };
      }
    );
}
