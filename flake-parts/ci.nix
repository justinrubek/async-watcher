{inputs, ...}: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    ...
  }: let
    ciPackages = [
      self'.packages.cocogitto
      self'.packages.bomper
    ];

    packages = {
      cocogitto = pkgs.cocogitto;
      bomper = inputs'.bomper.packages.cli;
    };

    devShells = {
      ci = pkgs.mkShell rec {
        packages = ciPackages;

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
      };
    };
  in rec {
    inherit devShells packages;

    legacyPackages = {
      inherit ciPackages;
    };
  };
}
