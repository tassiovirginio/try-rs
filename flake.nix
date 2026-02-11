{
  description = "try-rs: Temporary workspace manager with TUI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
      in
      {
        packages.try-rs = pkgs.rustPlatform.buildRustPackage {
          pname = "try-rs";

          version = "1.1.1";  # update when releasing new version

          src = self; # uses the repo itself as source

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [ pkgs.rust-bin.stable.latest.default ];

          meta = with pkgs.lib; {
            description = "Temporary workspace manager with TUI";
            homepage = "https://github.com/tassiovirginio/try-rs";
            license = licenses.mit;
            mainProgram = "try-rs";
          };
        };

        packages.default = self.packages.${system}.try-rs;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rust-bin.stable.latest.default
            pkgs.cargo
          ];
        };
      }
    )
    // {
      overlays.default = final: prev: {
        try-rs = self.packages.${final.system}.try-rs;
      };
    };
}
