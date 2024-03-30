{inputs, ...}: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    ...
  }: let
    # packages required for building the rust packages
    extraPackages = [
      pkgs.pkg-config
    ];
    withExtraPackages = base: base ++ extraPackages;

    craneLib = inputs.crane.lib.${system}.overrideToolchain self'.packages.rust-toolchain;

    common-build-args = rec {
      src = inputs.nix-filter.lib {
        root = ../.;
        include = [
          "crates"
          "examples"
          "Cargo.toml"
          "Cargo.lock"
        ];
      };

      pname = "async-watch";

      nativeBuildInputs = withExtraPackages [];
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
    };

    cargoArtifacts = craneLib.buildDepsOnly ({} // common-build-args);

    packages = {
      default = packages.awatch;
      awatch = craneLib.buildPackage ({
          pname = "awatch";
          inherit cargoArtifacts;
          cargoExtraArgs = "--bin awatch";
          meta.mainProgram = "awatch";
        }
        // common-build-args);

      cargo-doc = craneLib.cargoDoc ({
          inherit cargoArtifacts;
        }
        // common-build-args);
    };

    checks = {
      clippy = craneLib.cargoClippy ({
          cargoArtifacts = cargoArtifacts;
          cargoClippyExtraArgs = "--all-features -- --deny warnings";
        }
        // common-build-args);

      rust-fmt = craneLib.cargoFmt ({
          inherit (common-build-args) src;
        }
        // common-build-args);

      rust-tests = craneLib.cargoNextest ({
          cargoArtifacts = cargoArtifacts;
          partitions = 1;
          partitionType = "count";
        }
        // common-build-args);
    };
  in rec {
    inherit packages checks;

    legacyPackages = {
      cargoExtraPackages = extraPackages;
    };
  };
}
