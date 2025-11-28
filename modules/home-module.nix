{
  moduleWithSystem,
  inputs,
  ...
}:
{
  imports = [
    inputs.home-manager.flakeModules.home-manager
  ];
  flake.homeModules.wg = moduleWithSystem (
    { self', ... }:
    {
      lib,
      pkgs,
      config,
    }:
    let
      cfg = config.programs.wg;
    in
    {
      options.programs.wg = {
        enable = lib.mkEnableOption "wg";
        package = lib.mkPackageOption self'.package "wg" { };
      };
      config = lib.mkIf cfg.enabled {
        home.packages = [ cfg.packages ];
      };
    }
  );
}
