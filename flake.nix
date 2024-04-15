{

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { flake-utils, nixpkgs, naersk, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config = {
            permittedInsecurePackages = [
              "freeimage-unstable-2021-11-01"
            ];
          };
        };

        trenchbroom-working = pkgs.runCommand "trenchbroom"
          { buildInputs = [ pkgs.makeWrapper pkgs.tree pkgs.gnused ]; } ''
          makeWrapper ${pkgs.trenchbroom}/bin/trenchbroom $out/bin/trenchbroom --set QT_QPA_PLATFORM xcb
        '';
        toolchain = with fenix.packages.${system};  combine [
          minimal.cargo
          minimal.rustc
          latest.clippy
          latest.rust-src
          latest.rustfmt
        ];
        min-pkgs = with pkgs; [
          pkg-config
          openssl
          gcc
          mosquitto
          cmake
          gnumake
        ];
      in
      {
        defaultPackage = (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }).buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [ ] ++ min-pkgs;
        };

        devShell = (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }).buildPackage {
          src = ./.;
          dontPatchELF = true;
          nativeBuildInputs = with pkgs; [ ] ++ min-pkgs;
        };
      });
}
