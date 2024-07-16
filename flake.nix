{
  description = "Simple devshell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          # cargo
          rustup
          lua5_4_compat
          pkg-config
          rust-analyzer
          clippy
          rustfmt
          vscode-extensions.vadimcn.vscode-lldb.adapter
        ];
      };

      formatter = pkgs.alejandra;
    });
}
