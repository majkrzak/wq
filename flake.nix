{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
    import-tree.url = "github:vic/import-tree";
  };
  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } (
      inputs.nixpkgs.lib.pipe inputs.import-tree [
        (i: i.filterNot (inputs.nixpkgs.lib.hasSuffix "/flake.nix"))
        (i: i ./.)
      ]
    );
}
