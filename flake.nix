{
  description = "Simple but blazingly fast screenshot utility";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";

    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/*.tar.gz";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self,  nixpkgs, rust-overlay, flake-utils, crane }:
  flake-utils.lib.eachDefaultSystem (system:
      let
        craneLib = crane.lib.${system};

        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };

        libPath =  with pkgs; lib.makeLibraryPath [
          xorg.libX11
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [ xorg.libX11 ];

        cargoArtifacts = craneLib.buildDepsOnly ({
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          inherit buildInputs nativeBuildInputs;
          pname = "lqth";
        });
      in with pkgs; {
        packages = rec {
          lqth = craneLib.buildPackage {
            src = craneLib.path ./.;

            inherit buildInputs nativeBuildInputs cargoArtifacts;

            postInstall = ''
            '';

            GIT_HASH = self.rev or self.dirtyRev;
          };

          devShell = mkShell {
            inherit buildInputs nativeBuildInputs;

            packages = with pkgs; [
              (rust-bin.stable.latest.default.override {
                extensions = [ "rust-analyzer" ];
              })
              cargo-bloat
              cargo-outdated
              cargo-udeps
              rust-analyzer
              cargo-deny
              just
              typos
              codespell
              # committed
              grcov
              cargo-readme
              cargo-depgraph
              graphviz
            ];
            LD_LIBRARY_PATH = "${libPath}";
          };
        }
      ) // {
      overlay = final: prev: {
        inherit (self.packages.${final.system}) lqth;
      };
}

