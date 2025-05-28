{
  description = "OpenGL window with Rust";
  # Build using `nix build`
  # Testing using:

  inputs = {
    nixpkgs = {
      type = "github";
      owner = "nixos";
      repo = "nixpkgs";
    };
    flake-parts = {
      type = "github";
      owner = "hercules-ci";
      repo = "flake-parts";
    };
    rust-overlay = {
      type = "github";
      owner = "oxalica";
      repo = "rust-overlay";
    };
  };

  outputs = inputs: 
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let
          runtimeDeps = with pkgs; [ alsa-lib speechd xwayland ];
          buildDeps = with pkgs; [ 
            pkg-config 
            rustPlatform.bindgenHook 
            cmake 
            xorg.libX11
            xwayland
          ];
          devDeps = with pkgs; [ 
            gdb
            xorg.libX11
            xorg.libXrandr
            xorg.libXinerama
            xorg.libXcursor
            xorg.libXi
            xwayland
            libGL
          ];

          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          msrv = cargoToml.package.rust-version;

          rustPackage = features:
            (pkgs.makeRustPlatform {
              cargo = pkgs.rust-bin.stable.latest.minimal;
              rustc = pkgs.rust-bin.stable.latest.minimal;
            }).buildRustPackage {
              inherit (cargoToml.package) name version;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
              buildFeatures = features;
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps;
              # Uncomment if your cargo tests require networking or otherwise
              # don't play nicely with the Nix build sandbox:
              # doCheck = false;
            };

          mkDevShell = rustc:
            pkgs.mkShell {
              shellHook = ''
                export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
                export LD_LIBRARY_PATH="''${LD_LIBRARY_PATH}''${LD_LIBRARY_PATH:+:}${pkgs.libglvnd}/lib"
              '';
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps ++ devDeps ++ [ rustc ];
            };
        in {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          packages.default = self'.packages.example;
          devShells.default = self'.devShells.nightly;

          packages.example = (rustPackage "attempt_opengl");
          packages.example-base = (rustPackage "");

          devShells.nightly = (mkDevShell (pkgs.rust-bin.selectLatestNightlyWith
            (toolchain: toolchain.default)));
          devShells.stable = (mkDevShell pkgs.rust-bin.stable.latest.default);
          devShells.msrv = (mkDevShell pkgs.rust-bin.stable.${msrv}.default);
        };
    };
}
