{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

    flakebox.url = "github:rustshop/flakebox";
    flakebox.inputs.nixpkgs.follows = "nixpkgs";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      flakebox,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        projectName = "mindmap";

        pkgs = nixpkgs.legacyPackages.${system};
        openssl = pkgs.openssl;
        libldap = pkgs.openldap;

        flakeboxLib = flakebox.lib.${system} {
          config = {
            github.ci.buildOutputs = [ ".#ci.${projectName}" ];
          };
        };

        buildPaths = [
          "Cargo.toml"
          "Cargo.lock"
          "src"
        ];

        buildSrc = flakeboxLib.filterSubPaths {
          root = builtins.path {
            name = projectName;
            path = ./.;
          };
          paths = buildPaths;
        };

        multiBuild = (flakeboxLib.craneMultiBuild { }) (
          craneLib':
          let
            craneLib = (
              craneLib'.overrideArgs {
                pname = projectName;
                src = buildSrc;
                nativeBuildInputs = [
                  pkgs.pkg-config
                  openssl
                  libldap
                ];
                buildInputs = [
                  openssl
                  libldap
                ];
                cargoBuildOptions = [ "--features=vendored" ];
              }
            );
          in
          {
            mindmap = craneLib.buildPackage { meta.mainProgram = "mindmap"; };
            todo = craneLib.buildPackage { meta.mainProgram = "todo"; };
          }
        );
      in
      {
        packages.default = multiBuild.${projectName};

        legacyPackages = multiBuild;

        devShells = flakeboxLib.mkShells {
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [
            openssl
            libldap
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            openssl
            libldap
          ];
        };
      }
    );
}
