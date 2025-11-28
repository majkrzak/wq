{
  moduleWithSystem,
  inputs,
  ...
}:
{
  imports = [
    inputs.home-manager.flakeModules.home-manager
  ];
  flake.homeModules.wq = moduleWithSystem (
    { self', ... }:
    {
      lib,
      pkgs,
      config,
      ...
    }:
    let
      cfg = config.programs.wq;
    in
    {
      options.programs.wq = {
        enable = lib.mkEnableOption "wq";
        package = lib.mkPackageOption self'.package "wq" { };
      };
      config = lib.mkIf cfg.enable {
        home.packages = [ cfg.package ];
      };
    }
  );
}
