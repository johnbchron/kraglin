{
  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.2311.556873.tar.gz";
    rust-overlay = {
      inputs.nixpkgs.follows = "nixpkgs";
      url = "https://flakehub.com/f/oxalica/rust-overlay/0.1.1327.tar.gz";
    };
    crane = {
      url = "https://flakehub.com/f/ipetkov/crane/0.16.3.tar.gz";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, crane, flake-utils }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        });

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        src = ./.;

        common_args = {
          inherit src;
          doCheck = false;

        };

        deps_only = craneLib.buildDepsOnly common_args;
        crate = craneLib.buildPackage (common_args // {
          cargoArtifacts = deps_only;
        });

      in {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            bacon cargo-nextest cargo-deny
            toolchain
          ];
        };
        packages = {
          default = crate;
        };
        checks = {
          clippy-check = craneLib.cargoClippy (common_args // {
            cargoArtifacts = deps_only;
          });
          docs-check = craneLib.cargoDoc (common_args // {
            cargoArtifacts = deps_only;
          });
          fmt-check = craneLib.cargoFmt {
            pname = common_args.pname;
            version = common_args.version;
            
            inherit src;
          };
          nextest-check = craneLib.cargoNextest (common_args // {
            cargoArtifacts = deps_only;
            partitions = 1;
            partitionType = "count";
          });
        };
      });
}
