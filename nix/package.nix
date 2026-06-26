# buildZeroclaw — callPackage-compatible function for ZeroClaw packages.
# Passed through pkgs.callPackage in flake.nix to provide .override support.
{ pkgs, rustToolchain, zeroclawDefaultFeatures, zeroclawVersion, root, pname
, cargoPkg, features ? zeroclawDefaultFeatures
}:

let
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustToolchain;
    rustc = rustToolchain;
  };
in
rustPlatform.buildRustPackage {
  inherit pname;
  version = zeroclawVersion;
  src = root;
  cargoLock = {
    lockFile = "${root}/Cargo.lock";
    outputHashes = builtins.fromJSON (builtins.readFile "${root}/nix/hashes.json");
  };
  cargoBuildFlags =
    [ "-p" cargoPkg "--no-default-features" ]
    ++ pkgs.lib.optionals (features != [])
      [ "--features" (pkgs.lib.concatStringsSep "," features) ];
  doCheck = false;
  nativeBuildInputs = [ pkgs.pkg-config ]
    ++ pkgs.lib.optionals (builtins.elem "channel-voice-call" features) [ pkgs.autoPatchelfHook ];
  buildInputs = [ pkgs.stdenv.cc.cc ]
    ++ pkgs.lib.optionals (builtins.elem "channel-voice-call" features) [ pkgs.alsa-lib ];
}
