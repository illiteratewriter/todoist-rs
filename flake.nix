{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    crane,
    fenix,
    flake-utils,
    nixpkgs,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        lib = pkgs.lib;

        inherit (pkgs) writeShellScriptBin;
        inherit (lib) makeLibraryPath getExe';
        inherit (flake-utils.lib) mkApp;

        fenixProfile = fenix.packages.${system}.stable;
        rustToolchain = fenixProfile.withComponents [
          "rustc"
          "cargo"
          "clippy"
          "rustfmt"
        ];

        craneLib = (crane.mkLib pkgs).overrideToolchain (_: rustToolchain);

        inherit (craneLib) buildDepsOnly buildPackage cargoDoc cleanCargoSource devShell;

        src = cleanCargoSource ./.;

        buildInputs = with pkgs; [
          openssl
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        cargoArtifacts = buildDepsOnly {
          inherit src buildInputs nativeBuildInputs;
        };

        commonArgs = {
          inherit src buildInputs nativeBuildInputs cargoArtifacts;
        };

        shell = devShell (commonArgs
          // {
            LD_LIBRARY_PATH = makeLibraryPath [pkgs.openssl];

            packages = [fenixProfile.rust-analyzer];
          });

        defaultPackage = buildPackage commonArgs;
        defaultApp = mkApp {
          drv = defaultPackage;
        };

        docPackage = cargoDoc commonArgs;
        docApp = mkApp {
          drv = let
            mimeopen = getExe' pkgs.perl540Packages.FileMimeInfo "mimeopen";
            docIndex = "${docPackage}/share/doc/todoist/index.html";
          in
            writeShellScriptBin "todoist-rs-open-docs" ''
              ${mimeopen} ${docIndex}
            '';
        };
      in {
        devShell = shell;

        apps = {
          default = defaultApp;
          doc = docApp;
        };

        packages = {
          default = defaultPackage;
          doc = docPackage;
        };
      }
    );
}
