{ inputs, ... }:
{
  imports = [
  ];
  perSystem =
    { pkgs, ... }:
    let
      craneLib = inputs.crane.mkLib pkgs;
    in
    {
      packages.default = craneLib.buildPackage {
        src = craneLib.cleanCargoSource ./..;
      };
    };
}
