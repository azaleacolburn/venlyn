{
  description = "A Compiler";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
    }:
    let
      inherit (nixpkgs) lib;
      forEachPkgs = f: lib.genAttrs (import systems) (system: f nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forEachPkgs (pkgs: {
        default = pkgs.mkShell {
          inputsFrom = [ self.packages.${pkgs.stdenv.hostPlatform.system}.default ];
          buildInputs = with pkgs; [
            clippy
            bacon
          ];
        };
      });

      packages = forEachPkgs (pkgs: {
        default =
          let
            p = (lib.importTOML ./Cargo.toml).package;
          in
          pkgs.rustPlatform.buildRustPackage {
            pname = p.name;
            inherit (p) version;

            src = ./.;

            cargoLock.lockFile = ./Cargo.lock;

            meta = {
              mainProgram = "venlyn";
              license = lib.licenses.mit;
            };
          };
      });
    };
}
