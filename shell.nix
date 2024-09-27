{ fenix, mkShell }:

let
  toolchain = fenix.fromToolchainFile {
    file = ./statusng/rust-toolchain.toml;
    sha256 = "sha256-Ngiz76YP4HTY75GGdH2P+APE/DEIx2R/Dn+BwwOyzZU=";
  };
in

mkShell {
  packages = [ toolchain ];
}
