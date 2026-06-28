# zeroclaw-web — NPM build for the embedded web dashboard.
#
# Adapted from nixpkgs' zeroclaw-web derivation.  The dashboard is built in
# isolation and then copied into the Rust package source when the
# `embedded-web` feature is enabled.
{ buildNpmPackage
, src
, zeroclawVersion
, ...
}:
buildNpmPackage {
  pname = "zeroclaw-web";
  inherit src;
  version = zeroclawVersion;

  # Hash of the npm dependency lockfile.  Update with:
  #   nix-prefetch-npm-deps web/package-lock.json
  npmDepsHash = "sha256-SKltlDJm39ZzVaEt1bbnoiXy+wlbq+fC3bO4mW5V15o=";

  # api-generated.ts is produced by `cargo web gen-api`, which requires the
  # compiled Rust gateway binary — unavailable during the web build.  The
  # exported types are compile-time only (erased by Vite), so a minimal stub
  # is sufficient.  Re-check on every bump: if upstream starts committing
  # api-generated.ts or removes the import, this hook can be dropped.
  postPatch = ''
    mkdir -p src/lib
    cat > src/lib/api-generated.ts <<'EOF'
// Stub for Nix build: api-generated.ts is normally produced by
// `cargo web gen-api` which runs the Rust gateway binary to emit an
// OpenAPI spec and pipes it through openapi-typescript.  Since we
// cannot run the gateway during the web build, provide minimal type
// exports.  These are compile-time only and do not affect the bundle.
export type paths = Record<string, never>;
export type components = {
  schemas: {
    ConfigApiCode: string;
  };
};
EOF
  '';

  installPhase = ''
    runHook preInstall

    cp -r dist $out

    runHook postInstall
  '';
}
