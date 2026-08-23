use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::{Component, Path};

use serde::Deserialize;
use sha2::{Digest, Sha256};

const MANIFEST_PATH: &str = "docs/governance/seed-corpus-freeze.toml";
const REPORT_PATH: &str = "docs/governance/seed-corpus-freeze.md";
const EXPECTED_AUTHORITY: &str = "DEC-0230";
const EXPECTED_RELEASE: &str = "v0.0.1";
const EXPECTED_UNICODE: &str = "17.0.0";
const EXPECTED_CORPUS_ROOT: &str = "tests/conformance";

const EXPECTED_SURFACES: &[(&str, &str)] = &[
    ("source programs", "SeedFrozen"),
    ("parser trees", "NotFrozen"),
    ("diagnostics", "SeedFrozen"),
    ("Semantic Graph", "SeparateProtocol"),
    ("Audit", "SeparateProtocol"),
    ("bytecode", "SeparateProtocol"),
    ("package/lock", "SeparateProtocol"),
    ("replay", "Unavailable"),
    ("evidence", "Unavailable"),
    ("Zed/LSP fixtures", "Unavailable"),
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpusManifest {
    schema_version: u32,
    authority: String,
    release: String,
    unicode_version: String,
    corpus_root: String,
    case_count: usize,
    file_count: usize,
    sha256: String,
    report: String,
    surface: Vec<Surface>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Surface {
    name: String,
    state: String,
    evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub case_count: usize,
    pub file_count: usize,
    pub surface_count: usize,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CorpusSnapshot {
    case_count: usize,
    entries: Vec<(String, Vec<u8>)>,
    sha256: String,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let manifest = load_manifest(root)?;
    let snapshot = collect_snapshot(root, &manifest.corpus_root)?;
    let mut errors = validate(root, &manifest, &snapshot);
    let report = render(&manifest, &snapshot);
    match fs::read_to_string(root.join(REPORT_PATH)) {
        Ok(actual) if normalize_newlines(&actual) == report => {}
        Ok(_) => errors.push(format!(
            "GOV-CORPUS-0008: {REPORT_PATH} is stale; run cargo xtask corpus render"
        )),
        Err(error) => errors.push(format!(
            "GOV-CORPUS-0001: cannot read {REPORT_PATH}: {error}"
        )),
    }
    finish(errors).map(|()| CheckSummary {
        case_count: snapshot.case_count,
        file_count: snapshot.entries.len(),
        surface_count: manifest.surface.len(),
        sha256: snapshot.sha256,
    })
}

pub fn render_repository(root: &Path) -> Result<String, Vec<String>> {
    let manifest = load_manifest(root)?;
    let snapshot = collect_snapshot(root, &manifest.corpus_root)?;
    finish(validate(root, &manifest, &snapshot)).map(|()| render(&manifest, &snapshot))
}

fn load_manifest(root: &Path) -> Result<CorpusManifest, Vec<String>> {
    let text = fs::read_to_string(root.join(MANIFEST_PATH)).map_err(|error| {
        vec![format!(
            "GOV-CORPUS-0001: cannot read {MANIFEST_PATH}: {error}"
        )]
    })?;
    toml::from_str(&text).map_err(|error| {
        vec![format!(
            "GOV-CORPUS-0002: invalid corpus manifest {MANIFEST_PATH}: {error}"
        )]
    })
}

fn collect_snapshot(root: &Path, corpus_root: &str) -> Result<CorpusSnapshot, Vec<String>> {
    let directory = root.join(corpus_root);
    let mut errors = Vec::new();
    let mut cases = match read_directory(&directory) {
        Ok(entries) => entries,
        Err(error) => return Err(vec![error]),
    };
    cases.sort_by_key(|entry| entry.file_name());

    let mut entries = Vec::new();
    let mut case_count = 0;
    for case in cases {
        let case_path = case.path();
        let file_type = match case.file_type() {
            Ok(file_type) => file_type,
            Err(error) => {
                errors.push(format!(
                    "GOV-CORPUS-0001: cannot inspect {}: {error}",
                    display_path(root, &case_path)
                ));
                continue;
            }
        };
        if !file_type.is_dir() || file_type.is_symlink() {
            errors.push(format!(
                "GOV-CORPUS-0003: corpus root contains a non-directory or symlink {}",
                display_path(root, &case_path)
            ));
            continue;
        }
        case_count += 1;
        let mut files = match read_directory(&case_path) {
            Ok(files) => files,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };
        files.sort_by_key(|entry| entry.file_name());
        let names = files
            .iter()
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect::<BTreeSet<_>>();
        let expected = BTreeSet::from(["case.ling".to_owned(), "expect.toml".to_owned()]);
        if names != expected {
            errors.push(format!(
                "GOV-CORPUS-0004: {} must contain exactly case.ling and expect.toml; found {names:?}",
                display_path(root, &case_path)
            ));
            continue;
        }
        for file in files {
            let path = file.path();
            let file_type = match file.file_type() {
                Ok(file_type) => file_type,
                Err(error) => {
                    errors.push(format!(
                        "GOV-CORPUS-0001: cannot inspect {}: {error}",
                        display_path(root, &path)
                    ));
                    continue;
                }
            };
            if !file_type.is_file() || file_type.is_symlink() {
                errors.push(format!(
                    "GOV-CORPUS-0003: corpus case contains a non-file or symlink {}",
                    display_path(root, &path)
                ));
                continue;
            }
            match fs::read(&path) {
                Ok(bytes) => entries.push((display_path(&directory, &path), bytes)),
                Err(error) => errors.push(format!(
                    "GOV-CORPUS-0001: cannot read {}: {error}",
                    display_path(root, &path)
                )),
            }
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    let sha256 = snapshot_digest(&entries);
    Ok(CorpusSnapshot {
        case_count,
        entries,
        sha256,
    })
}

fn validate(root: &Path, manifest: &CorpusManifest, snapshot: &CorpusSnapshot) -> Vec<String> {
    let mut errors = Vec::new();
    if manifest.schema_version != 1
        || manifest.authority != EXPECTED_AUTHORITY
        || manifest.release != EXPECTED_RELEASE
        || manifest.unicode_version != EXPECTED_UNICODE
        || manifest.corpus_root != EXPECTED_CORPUS_ROOT
        || manifest.report != REPORT_PATH
    {
        errors.push(format!(
            "GOV-CORPUS-0005: manifest markers must be schema 1, authority {EXPECTED_AUTHORITY}, release {EXPECTED_RELEASE}, Unicode {EXPECTED_UNICODE}, root {EXPECTED_CORPUS_ROOT}, and report {REPORT_PATH}"
        ));
    }
    if manifest.case_count != snapshot.case_count || manifest.file_count != snapshot.entries.len() {
        errors.push(format!(
            "GOV-CORPUS-0006: frozen counts are {}/{} but current counts are {}/{}",
            manifest.case_count,
            manifest.file_count,
            snapshot.case_count,
            snapshot.entries.len()
        ));
    }
    if manifest.sha256 != snapshot.sha256 {
        errors.push(format!(
            "GOV-CORPUS-0007: frozen SHA-256 {} differs from current {}",
            manifest.sha256, snapshot.sha256
        ));
    }
    let actual = manifest
        .surface
        .iter()
        .map(|surface| (surface.name.as_str(), surface.state.as_str()))
        .collect::<Vec<_>>();
    if actual != EXPECTED_SURFACES {
        errors.push(format!(
            "GOV-CORPUS-0009: surface classification differs; expected {EXPECTED_SURFACES:?}, found {actual:?}"
        ));
    }
    let mut names = BTreeSet::new();
    for surface in &manifest.surface {
        if !names.insert(surface.name.as_str()) || surface.evidence.is_empty() {
            errors.push(format!(
                "GOV-CORPUS-0010: surface {:?} is duplicate or has no evidence",
                surface.name
            ));
        }
        for evidence in &surface.evidence {
            let path = Path::new(evidence);
            if !is_relative_path(path) || !root.join(path).exists() {
                errors.push(format!(
                    "GOV-CORPUS-0011: surface {:?} has invalid or missing evidence {evidence:?}",
                    surface.name
                ));
            }
        }
    }
    errors
}

fn render(manifest: &CorpusManifest, snapshot: &CorpusSnapshot) -> String {
    let mut output = String::new();
    output.push_str("# Seed Corpus Freeze\n\n");
    output.push_str("This generated report freezes only the accepted v0.0.1 conformance corpus. It is not a v0.1-v0.5 history or a compatibility promise.\n\n");
    output.push_str(&format!("- Authority: `{}`\n", manifest.authority));
    output.push_str(&format!("- Release: `{}`\n", manifest.release));
    output.push_str(&format!("- Unicode: `{}`\n", manifest.unicode_version));
    output.push_str(&format!("- Cases: `{}`\n", snapshot.case_count));
    output.push_str(&format!("- Files: `{}`\n", snapshot.entries.len()));
    output.push_str(&format!("- Canonical SHA-256: `{}`\n\n", snapshot.sha256));
    output.push_str("| Requested surface | State | Evidence |\n| --- | --- | --- |\n");
    for surface in &manifest.surface {
        output.push_str(&format!(
            "| {} | `{}` | {} |\n",
            surface.name,
            surface.state,
            surface
                .evidence
                .iter()
                .map(|path| format!("`{path}`"))
                .collect::<Vec<_>>()
                .join("<br>")
        ));
    }
    output.push_str("\n`SeedFrozen` covers exact checked-in v0.0.1 bytes only. `SeparateProtocol` retains its own authority and versioning. `NotFrozen` and `Unavailable` are explicit non-claims.\n");
    output
}

fn snapshot_digest(entries: &[(String, Vec<u8>)]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"ling.seed-corpus-freeze/0");
    hasher.update((entries.len() as u64).to_le_bytes());
    for (path, bytes) in entries {
        hasher.update((path.len() as u64).to_le_bytes());
        hasher.update(path.as_bytes());
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    let digest = hasher.finalize();
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(&mut output, "{byte:02x}").expect("writing to a String cannot fail");
    }
    output
}

fn read_directory(path: &Path) -> Result<Vec<fs::DirEntry>, String> {
    fs::read_dir(path)
        .map_err(|error| format!("GOV-CORPUS-0001: cannot read {}: {error}", path.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            format!(
                "GOV-CORPUS-0001: cannot enumerate {}: {error}",
                path.display()
            )
        })
}

fn display_path(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => value.to_str(),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn is_relative_path(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}

fn finish(mut errors: Vec<String>) -> Result<(), Vec<String>> {
    errors.sort();
    errors.dedup();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_digest_depends_on_paths_lengths_and_bytes() {
        let baseline = vec![("a/case.ling".to_owned(), b"let main () = ()\n".to_vec())];
        let changed_path = vec![("b/case.ling".to_owned(), baseline[0].1.clone())];
        let changed_bytes = vec![(baseline[0].0.clone(), b"let main () = 1\n".to_vec())];
        assert_ne!(snapshot_digest(&baseline), snapshot_digest(&changed_path));
        assert_ne!(snapshot_digest(&baseline), snapshot_digest(&changed_bytes));
        assert_eq!(snapshot_digest(&baseline), snapshot_digest(&baseline));
    }

    #[test]
    fn repository_seed_corpus_freeze_is_current() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("Seed corpus freeze is valid");
        assert_eq!(summary.case_count, 42);
        assert_eq!(summary.file_count, 84);
        assert_eq!(summary.surface_count, 10);
        assert_eq!(summary.sha256.len(), 64);
    }
}
