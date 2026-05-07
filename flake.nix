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

          version = "1.7.8";  # update when releasing new version

          src = self; # uses the repo itself as source

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [ pkgs.rust-bin.stable.latest.default pkgs.git ];

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
            pkgs.git
          ];
        };
      }
    )
    // {
      overlays.default = final: prev: {
        try-rs = self.packages.${final.system}.try-rs;
      };

      homeModules.default =
        { config, lib, pkgs, ... }:
        with lib;
        let
          cfg = config.programs.try-rs;
        in
        {
          options.programs.try-rs = {
            enable = mkEnableOption "try-rs - temporary workspace manager with TUI";

            package = mkOption {
              type = types.package;
              default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
              defaultText = literalExpression "inputs.try-rs.packages.\${pkgs.stdenv.hostPlatform.system}.default";
              description = "The try-rs package to use.";
            };

            path = mkOption {
              type = types.nullOr types.str;
              default = null;
              example = "~/src/tries";
              description = ''
                Path where try-rs directories will be stored. When set, writes
                `tries_paths` to `$XDG_CONFIG_HOME/try-rs/config.toml`.
              '';
            };

            enableBashIntegration = mkOption {
              type = types.bool;
              default = true;
              description = "Whether to enable Bash integration.";
            };

            enableZshIntegration = mkOption {
              type = types.bool;
              default = true;
              description = "Whether to enable Zsh integration.";
            };

            enableFishIntegration = mkOption {
              type = types.bool;
              default = true;
              description = "Whether to enable Fish integration.";
            };
          };

          config = mkIf cfg.enable {
            home.packages = [ cfg.package ];

            xdg.configFile."try-rs/config.toml" = mkIf (cfg.path != null) {
              text = ''
                tries_paths = "${cfg.path}"
              '';
            };

            programs.bash.initExtra = mkIf (cfg.enableBashIntegration && config.programs.bash.enable) ''
              eval "$(${cfg.package}/bin/try-rs --setup-stdout bash)"
            '';

            programs.zsh.initContent = mkIf (cfg.enableZshIntegration && config.programs.zsh.enable) ''
              eval "$(${cfg.package}/bin/try-rs --setup-stdout zsh)"
            '';

            programs.fish.shellInit = mkIf (cfg.enableFishIntegration && config.programs.fish.enable) ''
              ${cfg.package}/bin/try-rs --setup-stdout fish | source
            '';
          };
        };
    };
}
