{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let 
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      rustSrc = pkgs.lib.cleanSourceWith {
        src = ./.;
        filter = name: type: !builtins.elem ( baseNameOf name ) [ "./target" ".git" ];
      };

      manifestPath = "${toString rustSrc}/Cargo.toml";
      manifest = builtins.fromTOML ( builtins.readFile manifestPath );

      cargoNew = pkgs.writeShellScriptBin "cargo-init" ''
        export CARGO_TARGET_DIR=./target
        exec ${pkgs.cargo}/bin/cargo init "$@"
      '';

      cargoRun = pkgs.writeShellScriptBin "cargo-run" ''
        export CARGO_TARGET_DIR=./target
        exec ${pkgs.cargo}/bin/cargo run --manifest-path ./Cargo.toml "$@"
      '';

      cargoCheck = pkgs.writeShellScriptBin "cargo-check" ''
        export CARGO_TARGET_DIR=./target
        exec ${pkgs.cargo}/bin/cargo check --manifest-path ./Cargo.toml "$@"
      '';

      cargoClippy = pkgs.writeShellScriptBin "cargo-clippy" ''
     export CARGO_TARGET_DIR=./target
        exec ${pkgs.cargo}/bin/cargo clippy --manifest-path ./Cargo.toml "$@"
      '';

    in {
      packages."${system}".default = pkgs.rustPlatform.buildRustPackage rec {
        pname = manifest.package.name;
        version = manifest.package.version;
        src = rustSrc;
        cargoLock.lockFile = "${src}/Cargo.lock";

        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ pkgs.openssl ];

        OPENSSL_DIR = "${pkgs.openssl.dev}";
        OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      };

      devShells."${system}".default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc 
          cargo 
          rustfmt
          clippy 
          fish
          openssl
          pkg-config
        ];

        shellHook = ''
          export SHELL="${pkgs.fish}/bin/fish"
          export OPENSSL_DIR=${pkgs.openssl.dev}
          export OPENSSL_LIB_DIR=${pkgs.openssl.out}/lib
          export PKG_CONFIG_PATH=${pkgs.openssl.dev}/lib/pkgconfig
        '';
      };

      apps."${system}" = {
        init = {
          type = "app";
          program = (cargoNew) + "/bin/cargo-init";
        };

        default = {
          type = "app";
          program = (cargoRun) + "/bin/cargo-run";
        };

        check = {
          type = "app";
          program = (cargoCheck) + "/bin/cargo-check";
        };

        clippy = {
          type = "app";
          program = (cargoClippy) + "/bin/cargo-clippy";
        };
      };

    };
}
