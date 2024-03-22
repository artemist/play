{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    artemist-packages = {
      url = "git+https://git.mildlyfunctional.gay/artemist/packages.git";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.utils.follows = "utils";
    };
  };
  outputs = { self, nixpkgs, utils, artemist-packages }:
    utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in rec {
        devShells.zephyr =
          artemist-packages.devShells.${system}.zephyr.override {
            toolchains =
              with artemist-packages.packages.${system}.zephyrPackages.toolchains; [
                arm-zephyr-eabi
                riscv64-zephyr-elf
              ];
            modules =
              with artemist-packages.packages.${system}.zephyrPackages.modules; [
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
        devShells.typst = with pkgs; mkShell { packages = [ typst ]; };
        formatter = pkgs.nixfmt;
      });

}
