{
  description = "A minimal command to control obs via obs-websocket";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        # Extract version from Cargo.toml
        version = (pkgs.lib.importTOML ./Cargo.toml).package.version;
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "obs-cmd";
          inherit version;
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = true;
          };
          
          # Build dependencies
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ openssl ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ 
              pkgs.darwin.apple_sdk.frameworks.Security 
            ];
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [ 
            rustc 
            cargo 
            clippy 
            rustfmt 
            pkg-config 
            openssl 
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ 
            pkgs.darwin.apple_sdk.frameworks.Security 
          ];
          
          RUST_BACKTRACE = "1";
        };
      });
}