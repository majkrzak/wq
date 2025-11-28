{ inputs, ... }:
{
  perSystem =
    { pkgs, ... }:
    let
      craneLib = inputs.crane.mkLib pkgs;
    in
    {
      packages.wq = craneLib.buildPackage {
        src = craneLib.cleanCargoSource ./..;
      };
    };
}
