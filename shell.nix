{ fenix, mkShell }:

let
  toolchain = fenix.fromToolchainFile {
    file = ./statusng/rust-toolchain.toml;
    sha256 = "sha256-lMLAupxng4Fd9F1oDw8gx+qA0RuF7ou7xhNU8wgs0PU=";
  };
in

mkShell {
  packages = [ toolchain ];
}
