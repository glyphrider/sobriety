{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.trunk
    pkgs.wasm-bindgen-cli
    pkgs.pkg-config
    pkgs.openssl
    pkgs.git
    pkgs.cacert
  ];

  shellHook = ''
    rustup default stable
    rustup target add wasm32-unknown-unknown
  '';
}
