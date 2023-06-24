{
  inputs,
  self,
  ...
}: {
  perSystem = {self', ...}: let
  in {
    pre-commit = {
      check.enable = true;

      settings = {
        src = ../.;
        hooks = {
          treefmt.enable = true;
        };

        settings.treefmt.package = self'.packages.treefmt;
      };
    };
  };
}
