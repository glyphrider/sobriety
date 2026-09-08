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
    pkgs.wasm-pack
    pkgs.geckodriver
    pkgs.firefox
  ];

  shellHook = ''
    rustup default stable
    rustup target add wasm32-unknown-unknown
  '';
}
