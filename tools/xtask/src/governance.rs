use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::Deserialize;

const MANIFEST_PATH: &str = "docs/governance/authority.toml";
const REPORT_PATH: &str = "docs/governance/authority.md";

pub(crate) type SpecificationRecord = (String, String, String);
pub(crate) type SpecificationRecords = BTreeMap<String, SpecificationRecord>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DocumentRecord {
    pub kind: String,
    pub status: String,
    pub path: String,
}

pub(crate) type DocumentRecords = BTreeMap<String, DocumentRecord>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub document_count: usize,
    pub accepted_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorityIndex {
    schema_version: u32,
    updated: String,
    #[serde(default)]
    document: Vec<Document>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    id: String,
    title: String,
    kind: String,
    status: String,
    version: String,
    authority: String,
    path: String,
    #[serde(default)]
    covers: Vec<String>,
    #[serde(default)]
    depends_on: Vec<String>,
    #[serde(default)]
    supersedes: Vec<String>,
    #[serde(default)]
    stable_basis: bool,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let index = load_index(root)?;
    let mut errors = validate(root, &index);
    let rendered = render(&index);
    let report_path = root.join(REPORT_PATH);
    match fs::read_to_string(&report_path) {
        Ok(actual) if normalize_newlines(&actual) == rendered => {}
        Ok(_) => errors.push(format!(
            "GOV-AUTH-0008: {} is not the deterministic rendering of {}",
            REPORT_PATH, MANIFEST_PATH
        )),
        Err(error) => errors.push(format!(
            "GOV-AUTH-0002: cannot read {}: {error}",
            REPORT_PATH
        )),
    }

    finish(errors).map(|()| CheckSummary {
        document_count: index.document.len(),
        accepted_count: index
            .document
            .iter()
            .filter(|document| document.status == "Accepted")
            .count(),
    })
}

pub fn render_repository(root: &Path) -> Result<String, Vec<String>> {
    let index = load_index(root)?;
    finish(validate(root, &index)).map(|()| render(&index))
}

pub(crate) fn document_statuses(root: &Path) -> Result<BTreeMap<String, String>, Vec<String>> {
    let index = load_index(root)?;
    finish(validate(root, &index))?;
    Ok(index
        .document
        .into_iter()
        .map(|document| (document.id, document.status))
        .collect())
}

pub(crate) fn document_records(root: &Path) -> Result<DocumentRecords, Vec<String>> {
    let index = load_index(root)?;
    finish(validate(root, &index))?;
    Ok(index
        .document
        .into_iter()
        .map(|document| {
            (
                document.id,
                DocumentRecord {
                    kind: document.kind,
                    status: document.status,
                    path: document.path,
                },
            )
        })
        .collect())
}

pub(crate) fn specification_records(root: &Path) -> Result<SpecificationRecords, Vec<String>> {
    let index = load_index(root)?;
    finish(validate(root, &index))?;
    Ok(index
        .document
        .into_iter()
        .filter(|document| matches!(document.kind.as_str(), "RFC" | "Decision"))
        .map(|document| (document.id, (document.kind, document.status, document.path)))
        .collect())
}

fn load_index(root: &Path) -> Result<AuthorityIndex, Vec<String>> {
    let path = root.join(MANIFEST_PATH);
    let text = fs::read_to_string(&path).map_err(|error| {
        vec![format!(
            "GOV-AUTH-0002: cannot read {}: {error}",
            MANIFEST_PATH
        )]
    })?;
    toml::from_str(&text).map_err(|error| {
        vec![format!(
            "GOV-AUTH-0009: invalid authority manifest {}: {error}",
            MANIFEST_PATH
        )]
    })
}

fn validate(root: &Path, index: &AuthorityIndex) -> Vec<String> {
    let mut errors = Vec::new();
    if index.schema_version != 1 {
        errors.push(format!(
            "GOV-AUTH-0010: unsupported schema_version {}; expected 1",
            index.schema_version
        ));
    }
    if index.updated.trim().is_empty() {
        errors.push("GOV-AUTH-0010: updated must not be empty".to_owned());
    }
    if index.document.is_empty() {
        errors.push("GOV-AUTH-0010: authority index contains no documents".to_owned());
    }

    let mut by_id = BTreeMap::new();
    for document in &index.document {
        validate_document(root, document, &mut errors);
        if by_id.insert(document.id.clone(), document).is_some() {
            errors.push(format!(
                "GOV-AUTH-0001: duplicate document id {}",
                document.id
            ));
        }
    }

    for document in &index.document {
        for dependency in &document.depends_on {
            if !by_id.contains_key(dependency) {
                errors.push(format!(
                    "GOV-AUTH-0004: {} depends on unknown document {}",
                    document.id, dependency
                ));
            }
        }
        for superseded in &document.supersedes {
            if superseded == &document.id {
                errors.push(format!(
                    "GOV-AUTH-0010: {} cannot supersede itself",
                    document.id
                ));
            } else if !by_id.contains_key(superseded) {
                errors.push(format!(
                    "GOV-AUTH-0004: {} supersedes unknown document {}",
                    document.id, superseded
                ));
            }
        }
    }

    validate_dependency_cycles(&by_id, &mut errors);
    validate_discovered_specifications(root, &by_id, &mut errors);
    errors
}

fn validate_document(root: &Path, document: &Document, errors: &mut Vec<String>) {
    if document.id.trim().is_empty()
        || document.title.trim().is_empty()
        || document.kind.trim().is_empty()
        || document.version.trim().is_empty()
        || document.path.trim().is_empty()
    {
        errors.push(format!(
            "GOV-AUTH-0010: {} has an empty required field",
            display_id(document)
        ));
    }

    const STATUSES: &[&str] = &[
        "Accepted",
        "Active",
        "Completed",
        "Draft",
        "Evidence",
        "Planning",
    ];
    if !STATUSES.contains(&document.status.as_str()) {
        errors.push(format!(
            "GOV-AUTH-0010: {} has unknown status {}",
            display_id(document),
            document.status
        ));
    }

    if authority_rank(&document.authority).is_none() {
        errors.push(format!(
            "GOV-AUTH-0010: {} has unknown authority class {}",
            display_id(document),
            document.authority
        ));
    }

    if (document.status == "Accepted") != (document.authority == "Accepted") {
        errors.push(format!(
            "GOV-AUTH-0010: {} status {} is incompatible with authority class {}",
            display_id(document),
            document.status,
            document.authority
        ));
    }

    if document.stable_basis && document.status != "Accepted" {
        errors.push(format!(
            "GOV-AUTH-0003: {} is {} and cannot be a Stable implementation basis",
            display_id(document),
            document.status
        ));
    }

    let relative = Path::new(&document.path);
    let path_segments_are_invalid = document
        .path
        .split('/')
        .any(|segment| segment.is_empty() || matches!(segment, "." | ".."));
    let has_windows_drive_prefix = document.path.as_bytes().get(1) == Some(&b':');
    let invalid_path = relative.is_absolute()
        || document.path.contains('\\')
        || document.path.starts_with('/')
        || path_segments_are_invalid
        || has_windows_drive_prefix;
    if invalid_path {
        errors.push(format!(
            "GOV-AUTH-0010: {} path must be a forward-slash repository-relative path: {}",
            display_id(document),
            document.path
        ));
    } else if !root.join(relative).exists() {
        errors.push(format!(
            "GOV-AUTH-0002: {} references missing path {}",
            display_id(document),
            document.path
        ));
    }

    let mut seen = BTreeSet::new();
    for dependency in document.depends_on.iter().chain(document.supersedes.iter()) {
        if !seen.insert(dependency) {
            errors.push(format!(
                "GOV-AUTH-0010: {} repeats relation {}",
                display_id(document),
                dependency
            ));
        }
    }
}

fn display_id(document: &Document) -> &str {
    if document.id.is_empty() {
        "<missing-id>"
    } else {
        &document.id
    }
}

fn authority_rank(authority: &str) -> Option<u8> {
    match authority {
        "Accepted" => Some(1),
        "Semantics" => Some(2),
        "Language" => Some(3),
        "Conformance" => Some(4),
        "Roadmap" => Some(5),
        "Planning" => Some(6),
        "Registry" => Some(7),
        "Evidence" => Some(8),
        "Opinion" => Some(9),
        "Implementation" => Some(10),
        "Draft" => Some(11),
        _ => None,
    }
}

fn validate_dependency_cycles(documents: &BTreeMap<String, &Document>, errors: &mut Vec<String>) {
    let mut visited = BTreeSet::new();
    let mut visiting = Vec::new();
    for id in documents.keys() {
        visit(id, documents, &mut visited, &mut visiting, errors);
    }
}

fn visit(
    id: &str,
    documents: &BTreeMap<String, &Document>,
    visited: &mut BTreeSet<String>,
    visiting: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    if visited.contains(id) {
        return;
    }
    if let Some(position) = visiting.iter().position(|candidate| candidate == id) {
        let mut cycle = visiting[position..].to_vec();
        cycle.push(id.to_owned());
        errors.push(format!(
            "GOV-AUTH-0005: authority dependency cycle: {}",
            cycle.join(" -> ")
        ));
        return;
    }

    let Some(document) = documents.get(id) else {
        return;
    };
    visiting.push(id.to_owned());
    for dependency in &document.depends_on {
        if documents.contains_key(dependency) {
            visit(dependency, documents, visited, visiting, errors);
        }
    }
    visiting.pop();
    visited.insert(id.to_owned());
}

fn validate_discovered_specifications(
    root: &Path,
    documents: &BTreeMap<String, &Document>,
    errors: &mut Vec<String>,
) {
    let mut discovered = Vec::new();
    discover_rfcs(root, &mut discovered, errors);
    discover_decisions(root, &mut discovered, errors);
    discovered.sort();

    for (id, status, path) in discovered {
        match documents.get(&id) {
            Some(document) if document.status != status => errors.push(format!(
                "GOV-AUTH-0006: {id} status is {status} in {path} but {} in {}",
                document.status, MANIFEST_PATH
            )),
            Some(document) if document.path != path => errors.push(format!(
                "GOV-AUTH-0006: {id} path is {path} on disk but {} in {}",
                document.path, MANIFEST_PATH
            )),
            Some(_) => {}
            None => errors.push(format!(
                "GOV-AUTH-0007: discovered {status} document {id} at {path}, but it is absent from {}",
                MANIFEST_PATH
            )),
        }
    }
}

fn discover_rfcs(
    root: &Path,
    output: &mut Vec<(String, String, String)>,
    errors: &mut Vec<String>,
) {
    let docs = root.join("docs");
    let Ok(entries) = fs::read_dir(&docs) else {
        errors.push("GOV-AUTH-0002: cannot read docs directory".to_owned());
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !(name.starts_with("RFC-") && name.ends_with(".md")) {
            continue;
        }
        let id = name.trim_end_matches(".md").to_owned();
        let relative = format!("docs/{name}");
        discover_status(&path, &id, &relative, output, errors);
    }
}

fn discover_decisions(
    root: &Path,
    output: &mut Vec<(String, String, String)>,
    errors: &mut Vec<String>,
) {
    let directory = root.join("docs/decisions");
    let Ok(entries) = fs::read_dir(&directory) else {
        errors.push("GOV-AUTH-0002: cannot read docs/decisions directory".to_owned());
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("md") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            errors.push(format!("GOV-AUTH-0002: cannot read {}", path.display()));
            continue;
        };
        let id = text
            .lines()
            .find_map(|line| line.strip_prefix("# DEC-"))
            .and_then(|rest| rest.get(..4))
            .map(|number| format!("DEC-{number}"));
        let Some(id) = id else {
            errors.push(format!(
                "GOV-AUTH-0006: decision file {} has no DEC identifier heading",
                path.display()
            ));
            continue;
        };
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let relative = format!("docs/decisions/{name}");
        discover_status_from_text(&text, &id, &relative, output, errors);
    }
}

fn discover_status(
    path: &Path,
    id: &str,
    relative: &str,
    output: &mut Vec<(String, String, String)>,
    errors: &mut Vec<String>,
) {
    match fs::read_to_string(path) {
        Ok(text) => discover_status_from_text(&text, id, relative, output, errors),
        Err(error) => errors.push(format!("GOV-AUTH-0002: cannot read {relative}: {error}")),
    }
}

fn discover_status_from_text(
    text: &str,
    id: &str,
    relative: &str,
    output: &mut Vec<(String, String, String)>,
    errors: &mut Vec<String>,
) {
    let status = text.lines().find_map(|line| {
        line.trim()
            .strip_prefix("> 状态：")
            .map(|value| value.trim().trim_matches('`').to_owned())
    });
    match status {
        Some(status) => output.push((id.to_owned(), status, relative.to_owned())),
        None => errors.push(format!(
            "GOV-AUTH-0006: {id} at {relative} has no status metadata"
        )),
    }
}

fn render(index: &AuthorityIndex) -> String {
    let mut documents = index.document.iter().collect::<Vec<_>>();
    documents.sort_by(|left, right| {
        authority_rank(&left.authority)
            .unwrap_or(u8::MAX)
            .cmp(&authority_rank(&right.authority).unwrap_or(u8::MAX))
            .then_with(|| left.id.cmp(&right.id))
    });

    let mut output = String::new();
    output.push_str("# Ling 规范权威索引 / Specification Authority Index\n\n");
    output.push_str("> 状态：由 `authority.toml` 确定性生成\n");
    output.push_str(&format!("> 更新日期：{}\n", index.updated));
    output.push_str("> 本索引描述现有权威关系，不新增语言语义。\n\n");
    output.push_str("## Authority order\n\n");
    output.push_str("```text\n");
    output.push_str("Accepted RFC\n");
    output.push_str("    > docs/SEMANTICS.md\n");
    output.push_str("    > docs/LANGUAGE.md\n");
    output.push_str("    > tests/conformance/\n");
    output.push_str("    > docs/ROADMAP-1.0.md and engineering plans\n");
    output.push_str("    > Rust implementation\n");
    output.push_str("    > code comments\n");
    output.push_str("```\n\n");
    output.push_str("Accepted decisions are scoped normative records for the questions they close; they cannot override an Accepted RFC. A Draft RFC is indexed for discovery but is not an Accepted implementation basis. If two normative sources conflict, implementation stops and records a specification gap. A lower-authority plan is corrected to match the higher source.\n\n");
    output.push_str("## Documents\n\n");
    output.push_str("| ID | Kind | Status | Version | Authority | Stable basis | Path | Covers | Depends on | Supersedes |\n");
    output.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for document in documents {
        let path = report_link(&document.path);
        output.push_str(&format!(
            "| `{}` | {} | `{}` | `{}` | `{}` | {} | [{}]({}) | {} | {} | {} |\n",
            escape_cell(&document.id),
            escape_cell(&document.kind),
            escape_cell(&document.status),
            escape_cell(&document.version),
            escape_cell(&document.authority),
            if document.stable_basis { "yes" } else { "no" },
            escape_cell(&document.title),
            path,
            list_cell(&document.covers),
            list_cell(&document.depends_on),
            list_cell(&document.supersedes),
        ));
    }
    output.push_str("\n## Conflict and correction workflow\n\n");
    output.push_str("1. Verify the document lifecycle state in the source file and this index.\n");
    output.push_str("2. Stop implementation when higher-authority normative sources conflict or required behavior is unspecified.\n");
    output.push_str("3. Record a spec-gap with observable impact, affected tasks, alternatives, and the required RFC/decision.\n");
    output.push_str(
        "4. Correct lower-authority plans and implementation only after the authority is clear.\n",
    );
    output.push_str(
        "5. Run `cargo xtask governance check-authority` and the relevant conformance suite.\n\n",
    );
    output.push_str("## Machine source\n\n");
    output.push_str("The machine-readable source is [`authority.toml`](authority.toml). The checker rejects duplicate IDs, missing paths, unknown relations, dependency cycles, lifecycle mismatches, Draft documents used as Stable bases, and report drift.\n");
    output
}

fn report_link(path: &str) -> String {
    match path.strip_prefix("docs/") {
        Some(relative) => format!("../{relative}"),
        None => format!("../../{path}"),
    }
}

fn list_cell(values: &[String]) -> String {
    if values.is_empty() {
        "—".to_owned()
    } else {
        values
            .iter()
            .map(|value| format!("`{}`", escape_cell(value)))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn escape_cell(value: &str) -> String {
    value.replace('|', "\\|").replace(['\r', '\n'], " ")
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
    use super::{AuthorityIndex, check_repository, render, validate};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

    struct TempRepository(PathBuf);

    impl TempRepository {
        fn new() -> Self {
            let sequence = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("ling-governance-{}-{sequence}", std::process::id()));
            fs::create_dir_all(path.join("docs/decisions")).expect("create fixture directories");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn write(&self, relative: &str, text: &str) {
            let path = self.0.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("create fixture parent");
            }
            fs::write(path, text).expect("write fixture");
        }
    }

    impl Drop for TempRepository {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn parse(text: &str) -> AuthorityIndex {
        toml::from_str(text).expect("valid fixture manifest")
    }

    fn document(id: &str, status: &str, path: &str, stable: bool) -> String {
        let authority = if status == "Accepted" {
            "Accepted"
        } else {
            "Draft"
        };
        format!(
            r#"
[[document]]
id = "{id}"
title = "Fixture"
kind = "RFC"
status = "{status}"
version = "fixture"
authority = "{authority}"
path = "{path}"
covers = ["fixture"]
depends_on = []
supersedes = []
stable_basis = {stable}
"#
        )
    }

    #[test]
    fn rejects_duplicate_ids() {
        let repository = TempRepository::new();
        repository.write("docs/RFC-0001.md", "# RFC-0001\n\n> 状态：Draft\n");
        let manifest = format!(
            "schema_version = 1\nupdated = \"2026-08-20\"\n{}{}",
            document("RFC-0001", "Draft", "docs/RFC-0001.md", false),
            document("RFC-0001", "Draft", "docs/RFC-0001.md", false)
        );
        let errors = validate(repository.path(), &parse(&manifest));
        assert!(errors.iter().any(|error| error.contains("GOV-AUTH-0001")));
    }

    #[test]
    fn rejects_missing_accepted_document() {
        let repository = TempRepository::new();
        let manifest = format!(
            "schema_version = 1\nupdated = \"2026-08-20\"\n{}",
            document("RFC-0001", "Accepted", "docs/RFC-0001.md", true)
        );
        let errors = validate(repository.path(), &parse(&manifest));
        assert!(errors.iter().any(|error| error.contains("GOV-AUTH-0002")));
    }

    #[test]
    fn rejects_draft_as_stable_basis() {
        let repository = TempRepository::new();
        repository.write("docs/RFC-0001.md", "# RFC-0001\n\n> 状态：Draft\n");
        let manifest = format!(
            "schema_version = 1\nupdated = \"2026-08-20\"\n{}",
            document("RFC-0001", "Draft", "docs/RFC-0001.md", true)
        );
        let errors = validate(repository.path(), &parse(&manifest));
        assert!(errors.iter().any(|error| error.contains("GOV-AUTH-0003")));
    }

    #[test]
    fn rejects_unknown_dependencies() {
        let repository = TempRepository::new();
        repository.write("docs/RFC-0001.md", "# RFC-0001\n\n> 状态：Draft\n");
        let manifest = format!(
            "schema_version = 1\nupdated = \"2026-08-20\"\n{}",
            document("RFC-0001", "Draft", "docs/RFC-0001.md", false)
                .replace("depends_on = []", "depends_on = [\"RFC-9999\"]")
        );
        let errors = validate(repository.path(), &parse(&manifest));
        assert!(errors.iter().any(|error| error.contains("GOV-AUTH-0004")));
    }

    #[test]
    fn rejects_dependency_cycles() {
        let repository = TempRepository::new();
        repository.write("docs/spec-a.md", "fixture");
        repository.write("docs/spec-b.md", "fixture");
        let manifest = format!(
            "schema_version = 1\nupdated = \"2026-08-20\"\n{}{}",
            document("SPEC-A", "Draft", "docs/spec-a.md", false)
                .replace("depends_on = []", "depends_on = [\"SPEC-B\"]"),
            document("SPEC-B", "Draft", "docs/spec-b.md", false)
                .replace("depends_on = []", "depends_on = [\"SPEC-A\"]")
        );
        let errors = validate(repository.path(), &parse(&manifest));
        assert!(errors.iter().any(|error| error.contains("GOV-AUTH-0005")));
    }

    #[test]
    fn rejects_source_and_manifest_lifecycle_mismatch() {
        let repository = TempRepository::new();
        repository.write("docs/RFC-0001.md", "# RFC-0001\n\n> 状态：Draft\n");
        let manifest = format!(
            "schema_version = 1\nupdated = \"2026-08-20\"\n{}",
            document("RFC-0001", "Accepted", "docs/RFC-0001.md", true)
        );
        let errors = validate(repository.path(), &parse(&manifest));
        assert!(errors.iter().any(|error| error.contains("GOV-AUTH-0006")));
    }

    #[test]
    fn accepts_valid_superseded_chain() {
        let repository = TempRepository::new();
        repository.write("docs/RFC-0001.md", "# RFC-0001\n\n> 状态：Accepted\n");
        repository.write("docs/RFC-0002.md", "# RFC-0002\n\n> 状态：Accepted\n");
        repository.write("docs/RFC-0003.md", "# RFC-0003\n\n> 状态：Accepted\n");
        let manifest = format!(
            "schema_version = 1\nupdated = \"2026-08-20\"\n{}{}{}",
            document("RFC-0001", "Accepted", "docs/RFC-0001.md", true),
            document("RFC-0002", "Accepted", "docs/RFC-0002.md", true)
                .replace("supersedes = []", "supersedes = [\"RFC-0001\"]"),
            document("RFC-0003", "Accepted", "docs/RFC-0003.md", true)
                .replace("supersedes = []", "supersedes = [\"RFC-0002\"]")
        );
        assert!(validate(repository.path(), &parse(&manifest)).is_empty());
    }

    #[test]
    fn accepts_repository_relative_path_with_chinese_characters() {
        let repository = TempRepository::new();
        repository.write("docs/规范/RFC-0001.md", "fixture");
        let manifest = format!(
            "schema_version = 1\nupdated = \"2026-08-20\"\n{}",
            document("SPEC-ZH", "Draft", "docs/规范/RFC-0001.md", false)
        );
        assert!(validate(repository.path(), &parse(&manifest)).is_empty());
    }

    #[test]
    fn rendering_is_deterministic() {
        let repository = TempRepository::new();
        repository.write("docs/RFC-0001.md", "# RFC-0001\n\n> 状态：Draft\n");
        let manifest = format!(
            "schema_version = 1\nupdated = \"2026-08-20\"\n{}",
            document("RFC-0001", "Draft", "docs/RFC-0001.md", false)
        );
        let index = parse(&manifest);
        assert_eq!(render(&index), render(&index));
    }

    #[test]
    fn repository_authority_index_is_valid_and_current() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("repository authority index is valid");
        assert!(summary.document_count >= 20);
        assert_eq!(summary.accepted_count, 191);

        let task_status = fs::read_to_string(root.join("docs/status/implementation-status.toml"))
            .expect("read implementation task status");
        toml::from_str::<toml::Value>(&task_status).expect("implementation task status is TOML");
    }
}
