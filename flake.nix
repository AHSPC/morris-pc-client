{
  description = "morris pc client";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    # build platforms
    systems = [ "x86_64-linux" "aarch64-linux" ];

    eachSystem = fn: # fn takes in pkgs as a param
      nixpkgs.lib.genAttrs systems (system: fn nixpkgs.legacyPackages.${system});
  in {
    devShells = eachSystem (pkgs: {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc
          rustfmt
          rust-analyzer
          clippy
          cargo
          openssl pkg-config # Required for compiling rust-openssl
        ];
      };
    });

    packages = let
      rustBuild = pkgs: pkgs.rustPlatform.buildRustPackage {
        name = "morris-pc-client";
        cargoLock.lockFile = ./Cargo.lock;
        src = pkgs.lib.cleanSource ./.;

        # Required for compiling rust-openssl on *nix
        nativeBuildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux [ pkgs.pkg-config ];
        OPENSSL_NO_VENDOR = 1; # Forces openssl-sys to use pkg-config
        buildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux [ pkgs.openssl ];
      };
    in
      eachSystem (pkgs: rec {
        morris-pc-client = rustBuild pkgs;
        morris-pc-client-windows = rustBuild pkgs.pkgsCross.mingwW64;

        default = morris-pc-client;
      });
  };
}
