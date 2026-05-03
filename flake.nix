{
  description = "stdbr";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];

      imports = [
        inputs.devshell.flakeModule
        inputs.treefmt-nix.flakeModule
      ];

      perSystem = { system, lib, ... }:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          _module.args.pkgs = pkgs;

          treefmt = {
            projectRootFile = "flake.nix";
            programs = {
              rustfmt.enable = true;
              nixpkgs-fmt.enable = true;
              buildifier.enable = true;
            };
          };

          devshells.default = {
            name = "stdbr";

            packages = with pkgs; [
              rustToolchain

              bazel_7
              bazel-buildtools

              gcc
              pkg-config
              openssl
              cacert
              git

              jdk

              rust-cbindgen
              python3
              maturin
              wasm-pack
              nodejs
            ] ++ lib.optionals pkgs.stdenv.isDarwin [
              pkgs.libiconv
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            ];

            env = [
              { name = "CC"; value = "${pkgs.gcc}/bin/gcc"; }
              { name = "RUST_SRC_PATH"; value = "${rustToolchain}/lib/rustlib/src/rust/library"; }
            ];

            commands = [
              {
                name = "build";
                command = "bazel build //...";
                help = "Build all targets";
                category = "bazel";
              }
              {
                name = "check";
                command = "bazel test //...";
                help = "Run all tests";
                category = "bazel";
              }
              {
                name = "fmt";
                command = "nix fmt";
                help = "Format all files (rustfmt + nixpkgs-fmt + buildifier)";
                category = "dev";
              }
              {
                name = "clean";
                command = "bazel clean --expunge";
                help = "Clean bazel cache";
                category = "bazel";
              }
            ];
          };
        };
    };
}
