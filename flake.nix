{
  description = "Lox interpreter and compiler";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      rustToolchain = (pkgs.rust-bin.fromRustupToolchainFile "${self}/rust-toolchain.toml").override {
        extensions = [ "rust-src" ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustToolchain
          cargo-nextest
          just
          wild
          clang
          rust-analyzer
          tombi
        ];
      };
    };
}
