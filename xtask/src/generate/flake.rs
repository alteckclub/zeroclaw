//! Nix flake renderer. The generated zone contains the canonical version
//! (`zeroclawVersion`) and Dist feature list (`zeroclawDefaultFeatures`). The
//! actual package build logic lives in `nix/package.nix` and is referenced via
//! `pkgs.callPackage`.

use super::spec::{self, Selection};
use std::path::Path;

fn begin(zone: &str) -> String {
    format!("        # >>> generated:{zone} by `cargo generate installers` - do not edit <<<")
}
fn end(zone: &str) -> String {
    format!("        # >>> end generated:{zone} <<<")
}

const ZONE: &str = "flake-packages";

/// Render the generated zone body: the canonical version and Dist feature list.
/// Indented to sit inside the per-system `let` block of the flake.
pub fn render_zone(root: &Path) -> anyhow::Result<String> {
    let version = spec::resolve_version(root)?;
    let dist = spec::resolve_feature_list(root, &Selection::Dist)?;
    let feature_list = dist
        .iter()
        .map(|f| format!("\"{f}\""))
        .collect::<Vec<_>>()
        .join(" ");

    let lines = [
        "        # Default feature set: canonical Dist (all channels, no heavyweight).",
        "        # Override with `packages.zeroclaw.override { features = [ ... ]; }`.",
        &format!("        zeroclawVersion = \"{version}\";"),
        &format!("        zeroclawDefaultFeatures = [ {feature_list} ];"),
    ];
    Ok(lines.join("\n"))
}

/// Splice the generated package zone into the flake, preserving hand-written
/// outputs (devShell, nixos modules, checks) outside the sentinels.
pub fn render_file(root: &Path, current: &str) -> anyhow::Result<String> {
    let b = begin(ZONE);
    let e = end(ZONE);
    let begin_at = current.find(&b).ok_or_else(|| {
        anyhow::Error::msg(format!("flake.nix missing generated:{ZONE} BEGIN sentinel"))
    })?;
    let after_begin = begin_at + b.len();
    let end_rel = current[after_begin..].find(&e).ok_or_else(|| {
        anyhow::Error::msg(format!("flake.nix missing generated:{ZONE} END sentinel"))
    })?;
    let end_at = after_begin + end_rel;
    let body = render_zone(root)?;
    let mut out = String::new();
    out.push_str(&current[..after_begin]);
    out.push('\n');
    out.push_str(&body);
    out.push('\n');
    out.push_str(&current[end_at..]);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
    }

    #[test]
    fn zone_exposes_default_features() {
        let z = render_zone(&root()).unwrap();
        assert!(
            z.contains("zeroclawDefaultFeatures"),
            "default feature list present"
        );
    }

    #[test]
    fn zone_exposes_version() {
        let v = spec::resolve_version(&root()).unwrap();
        let z = render_zone(&root()).unwrap();
        assert!(
            z.contains(&format!("zeroclawVersion = \"{v}\"")),
            "version present"
        );
    }

    #[test]
    fn zone_default_is_dist_channels_no_heavyweight() {
        let z = render_zone(&root()).unwrap();
        assert!(z.contains("\"channel-discord\""), "dist ships all channels");
        assert!(!z.contains("\"hardware\""), "dist excludes heavyweight");
    }
}
