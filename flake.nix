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

          rustToolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
            extensions = [ "llvm-tools-preview" ];
          };
          fuzzToolchain = pkgs.rust-bin.nightly.latest.minimal;
          msrvToolchain = pkgs.rust-bin.stable."1.85.0".minimal;
          semverToolchain = pkgs.rust-bin.stable."1.91.0".minimal;
          nativeToolchain = pkgs.symlinkJoin {
            name = "stdbr-native-toolchain";
            paths = [ pkgs.gcc pkgs.binutils ];
          };
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

          checks.quality-static = pkgs.runCommand "stdbr-quality-static"
            {
              nativeBuildInputs = with pkgs; [ actionlint shellcheck ];
            } ''
            shellcheck ${./tools/quality/quality.sh}
            actionlint \
              ${./.github/workflows/ci.yml} \
              ${./.github/workflows/ibge-sync.yml} \
              ${./.github/workflows/publish-test.yml} \
              ${./.github/workflows/release.yml}
            touch $out
          '';

          devshells.default = {
            name = "stdbr";

            packages = with pkgs; [
              rustToolchain

              bazel_7
              bazel-buildtools

              nativeToolchain
              pkg-config
              openssl
              cacert
              git

              jdk

              python3
              maturin
              wasm-pack
              nodejs

              cargo-audit
              cargo-fuzz
              cargo-llvm-cov
              cargo-semver-checks
              actionlint
              shellcheck
            ] ++ lib.optionals pkgs.stdenv.isDarwin [
              pkgs.libiconv
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            ];

            env = [
              {
                name = "PATH";
                prefix = "${nativeToolchain}/bin";
              }
              { name = "CC"; value = "${nativeToolchain}/bin/gcc"; }
              { name = "AR"; value = "${nativeToolchain}/bin/ar"; }
              { name = "CBINDGEN"; value = lib.getExe pkgs.rust-cbindgen; }
              { name = "FUZZ_CARGO"; value = "${fuzzToolchain}/bin/cargo"; }
              { name = "FUZZ_RUSTC"; value = "${fuzzToolchain}/bin/rustc"; }
              { name = "MSRV_CARGO"; value = "${msrvToolchain}/bin/cargo"; }
              { name = "MSRV_RUSTC"; value = "${msrvToolchain}/bin/rustc"; }
              { name = "SEMVER_TOOLCHAIN_BIN"; value = "${semverToolchain}/bin"; }
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
                command = "bazel test //:all_tests";
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
                name = "quality";
                command = ''"$PRJ_ROOT/tools/quality/quality.sh" all "$@"'';
                help = "Run all local quality gates with bounded resources";
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
