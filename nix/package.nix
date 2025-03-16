{rustPlatform}:
rustPlatform.buildRustPackage {
  pname = "searcli";
  version = "0.1.0";

  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
}
