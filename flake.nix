{
  description = "Entretien Web 2024 - 42Angouleme (rsty)";

  # Declare the input flakes
  inputs = {
    nixpkgs.url = "github:xtrm-en/nixpkgs?ref=nixos-unstable";
    nixpkgs-dioxus-cli.url = "github:xtrm-en/nixpkgs?ref=e0464e47880a69896f0fb1810f00e0de469f770a";
    systems.url = "github:nix-systems/x86_64-linux";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  # Declare the outputs function
  outputs =
    {
      self,
      nixpkgs,
      systems,
      rust-overlay,
      ...
    }@inputs:

    let
      name = "rsty";

      # Helper function
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

      packages = forEachSystem (pkgs: (
        let
          # Application build
          backend = pkgs.rustPlatform.buildRustPackage {
            pname = name;
            version = "0.2.0";

            src = ./axum_backend;

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
          };

          # Docker image package build
          # dockerImage = pkgs.dockerTools.buildImage {
          #   inherit name;
          #   tag = "latest";
          #   copyToRoot = [ bin ];
          #   config = {
          #     Cmd = [ "${bin}/bin/rsty" ];
          #   };
          # };
        in
        {
          inherit backend; #dockerImage;
          default = backend;
        }));

      # Declare the development shell
      devShell = forEachSystem (
        pkgs:
        pkgs.mkShell (
          let
            rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
            pkgs-pinned = (import inputs.nixpkgs-dioxus-cli) { system = pkgs.stdenv.hostPlatform.system; };
          in
          {
            nativeBuildInputs = with pkgs; [
              pkg-config
              rustToolchain
              cargo-watch
              # Depend on the cli instead of using npx, saves us space and dealing with javascript
              tailwindcss
            ] ++ [
              # Pin dioxus-cli to v0.5.6
              pkgs-pinned.dioxus-cli
            ];

            buildInputs = with pkgs; [
              openssl
            ];
          }
        )
      );

    };
}
