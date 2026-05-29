{
  description = "Rust development environment with ALSA support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        
        nativeBuildInputs = with pkgs; [
          pkg-config
          cargo
          rustc
        ];

        buildInputs = with pkgs; [
          alsa-lib
        ];
        
      };
    };
}