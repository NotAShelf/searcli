{
  mkShell,
  rust-analyzer,
  rustfmt,
  clippy,
  cargo,
  rustPlatform,
  openssl,
  pkg-config,
}:
mkShell {
  name = "rust";
  packages = [
    rust-analyzer
    rustfmt
    clippy
    cargo

    openssl
    pkg-config
  ];

  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}
