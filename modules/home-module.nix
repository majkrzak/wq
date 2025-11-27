{
  moduleWithSystem,
  inputs,
  ...
}:
{
  imports = [
    inputs.home-manager.flakeModules.home-manager
  ];
  flake.homeModules.default =
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
        package = lib.mkPackageOption self'.package "default" { };
      };
      config = lib.mkIf cfg.enabled {
        home.packages = [ cfg.packages ];
      };
    };
}
