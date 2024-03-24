{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    artemist-packages = {
      url = "git+https://git.mildlyfunctional.gay/artemist/packages.git";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.utils.follows = "utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "utils";
    };
  };
  outputs = { self, nixpkgs, utils, artemist-packages, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            artemist-packages.overlays.default
            rust-overlay.overlays.default
          ];
        };
      in rec {
        devShells.zephyr =
          artemist-packages.devShells.${system}.zephyr.override {
            toolchains = with pkgs.zephyrPackages.toolchains; [
              arm-zephyr-eabi
              riscv64-zephyr-elf
            ];
            modules = with pkgs.zephyrPackages.modules; [
              mbedtls
              hal_rpi_pico
              hal_atmel
              cmsis
            ];
            extraPackages = with pkgs; [ pyocd ];
          };
        devShells.zephyr-west = devShells.zephyr.override {
          enableWest = true;
          zephyrSrc = null;
          modules = [ ];
        };
        devShells.rust-embedded = let
          rust = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "llvm-tools-preview" "clippy" ];
            # just use all of them, why not
            targets = [
              "thumbv6m-none-eabi"
              "thumbv7m-none-eabi"
              "thumbv7em-none-eabi"
              "thumbv7em-none-eabihf"
              "riscv32imac-unknown-none-elf"
            ];
          };
        in with pkgs;
        mkShell {
          packages = [ rust cargo-binutils probe-rs cargo-generate gdb ];
          RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
        };

        devShells.typst = with pkgs; mkShell { packages = [ typst ]; };
        formatter = pkgs.nixfmt;
      });

}
