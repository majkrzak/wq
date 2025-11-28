{ inputs, ... }:
{
  perSystem =
    { pkgs, ... }:
    let
      craneLib = inputs.crane.mkLib pkgs;
    in
    {
      packages.wg = craneLib.buildPackage {
        src = craneLib.cleanCargoSource ./..;
      };
    };
}
