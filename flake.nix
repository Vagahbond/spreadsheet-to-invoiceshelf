{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system} = {
      default = pkgs.mkShell {
        nativeBuildInputs = [pkgs.pkg-config];
        buildInputs = [pkgs.cargo pkgs.rustc pkgs.openssl];
      };
    };
  };
}
