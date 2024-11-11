{
  description = "Entretien Web 2024 - 42Angouleme";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    nixpkgs-dioxus-cli.url = "github:nixos/nixpkgs?ref=e0464e47880a69896f0fb1810f00e0de469f770a";
    systems.url = "github:nix-systems/x86_64-linux";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
      rust-overlay,
      ...
    }@inputs:

    let
      forEachSystem =
        f:
        (nixpkgs.lib.genAttrs (import systems) (
          system:
          f (
            import nixpkgs {
              inherit system;
              overlays = [ (import rust-overlay) ];
            }
          )
        ));
    in
    {
      formatter = forEachSystem (pkgs: pkgs.nixfmt-rfc-style);

      packages = forEachSystem (pkgs: {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "entretien-web-2024-42a";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            openssl
            curl
          ];

          buildInputs = with pkgs; [
            openssl
            curl
          ];

          meta = {
            homepage = "https://github.com/xtrm-en/ew2024-42a";
            description = "Entretien Web 2024 - 42Angouleme";
          };
        };
      });

      devShell = forEachSystem (
        pkgs:
        pkgs.mkShell (
          let
            rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
            pkgs-pinned = (import inputs.nixpkgs-dioxus-cli) { system = pkgs.stdenv.hostPlatform.system; };
          in
          {
            nativeBuildInputs = with pkgs; [
              cargo-watch
              # Depend on the cli instead of using npx
              tailwindcss
            ] ++ [
              # Pin dioxus-cli to v0.5.6
              pkgs-pinned.dioxus-cli
            ];

            buildInputs = with pkgs; [
              rustToolchain
              pkg-config
              openssl
              curl
            ];
          }
        )
      );

    };
}
