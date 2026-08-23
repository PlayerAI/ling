use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::governance;

const MANIFEST_PATH: &str = "docs/governance/protocol-inventory.toml";
const REPORT_PATH: &str = "docs/governance/protocol-inventory.md";
const REQUIRED_IDS: &[&str] = &[
    "PROTO-CLI",
    "PROTO-CLI-EXIT",
    "PROTO-HUMAN-OUTPUT",
    "PROTO-DIAGNOSTIC-JSON",
    "PROTO-SEMANTIC-GRAPH-JSON",
    "PROTO-PACKAGE-SEMANTIC-GRAPH-JSON",
    "PROTO-CANONICAL-BYTES",
    "PROTO-SEMANTIC-ID",
    "PROTO-AUDIT-SOURCE",
    "PROTO-REPL-JSON",
    "PROTO-INTERNAL-INCIDENT",
    "PROTO-SEMANTIC-TRANSACTION",
    "PROTO-PACKAGE-MANIFEST",
    "PROTO-LOCKFILE",
    "PROTO-BUILD-METADATA",
    "PROTO-BYTECODE",
    "PROTO-VM-CONTROL",
    "PROTO-REPLAY",
    "PROTO-ABI",
    "PROTO-EVIDENCE",
];
const CATEGORIES: &[&str] = &[
    "CLI",
    "LSP",
    "Human output",
    "JSON",
    "Canonical identity",
    "Text protocol",
    "Incident",
    "Transaction",
    "Package metadata",
    "Bytecode",
    "Runtime control",
    "Replay",
    "ABI",
    "Evidence",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub protocol_count: usize,
    pub public_count: usize,
    pub preview_count: usize,
    pub experimental_count: usize,
    pub stable_count: usize,
    pub internal_count: usize,
    pub future_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProtocolRecord {
    pub category: String,
    pub visibility: String,
    pub current_version: String,
    pub stability: String,
    pub implemented: bool,
    pub public_schema: bool,
    pub canonical: bool,
}

pub(crate) type ProtocolRecords = BTreeMap<String, ProtocolRecord>;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProtocolInventory {
    schema_version: u32,
    updated: String,
    #[serde(default)]
    protocol: Vec<Protocol>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Protocol {
    id: String,
    title: String,
    category: String,
    visibility: String,
    current_version: String,
    stability: String,
    implemented: bool,
    public_schema: bool,
    canonical: bool,
    #[serde(default)]
    producer: Vec<String>,
    #[serde(default)]
    consumer: Vec<String>,
    reader_policy: String,
    writer_policy: String,
    unknown_field_policy: String,
    migration_tool: String,
    #[serde(default)]
    fixtures: Vec<String>,
    #[serde(default)]
    sources: Vec<String>,
    #[serde(default)]
    version_markers: Vec<String>,
    #[serde(default)]
    authority: Vec<String>,
    #[serde(default)]
    notes: Vec<String>,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let authority_statuses = governance::document_statuses(root)?;
    let inventory = load_inventory(root)?;
    let mut errors = validate(root, &inventory, &authority_statuses, REQUIRED_IDS);
    let rendered = render(&inventory);
    match fs::read_to_string(root.join(REPORT_PATH)) {
        Ok(actual) if normalize_newlines(&actual) == rendered => {}
        Ok(_) => errors.push(format!(
            "GOV-PROTO-0008: {REPORT_PATH} is not the deterministic rendering of {MANIFEST_PATH}"
        )),
        Err(error) => errors.push(format!(
            "GOV-PROTO-0002: cannot read {REPORT_PATH}: {error}"
        )),
    }

    finish(errors).map(|()| summary(&inventory))
}

pub fn render_repository(root: &Path) -> Result<String, Vec<String>> {
    let authority_statuses = governance::document_statuses(root)?;
    let inventory = load_inventory(root)?;
    finish(validate(
        root,
        &inventory,
        &authority_statuses,
        REQUIRED_IDS,
    ))
    .map(|()| render(&inventory))
}

pub(crate) fn protocol_records(root: &Path) -> Result<ProtocolRecords, Vec<String>> {
    let authority_statuses = governance::document_statuses(root)?;
    let inventory = load_inventory(root)?;
    finish(validate(
        root,
        &inventory,
        &authority_statuses,
        REQUIRED_IDS,
    ))?;
    Ok(inventory
        .protocol
        .into_iter()
        .map(|protocol| {
            (
                protocol.id,
                ProtocolRecord {
                    category: protocol.category,
                    visibility: protocol.visibility,
                    current_version: protocol.current_version,
                    stability: protocol.stability,
                    implemented: protocol.implemented,
                    public_schema: protocol.public_schema,
                    canonical: protocol.canonical,
                },
            )
        })
        .collect())
}

fn load_inventory(root: &Path) -> Result<ProtocolInventory, Vec<String>> {
    let text = fs::read_to_string(root.join(MANIFEST_PATH)).map_err(|error| {
        vec![format!(
            "GOV-PROTO-0002: cannot read {MANIFEST_PATH}: {error}"
        )]
    })?;
    toml::from_str(&text).map_err(|error| {
        vec![format!(
            "GOV-PROTO-0009: invalid protocol inventory {MANIFEST_PATH}: {error}"
        )]
    })
}

fn validate(
    root: &Path,
    inventory: &ProtocolInventory,
    authority_statuses: &BTreeMap<String, String>,
    required_ids: &[&str],
) -> Vec<String> {
    let mut errors = Vec::new();
    if inventory.schema_version != 1 {
        errors.push(format!(
            "GOV-PROTO-0010: unsupported schema_version {}; expected 1",
            inventory.schema_version
        ));
    }
    if !is_date(&inventory.updated) {
        errors.push("GOV-PROTO-0010: updated must be a YYYY-MM-DD date".to_owned());
    }
    if inventory.protocol.is_empty() {
        errors.push("GOV-PROTO-0010: protocol inventory contains no records".to_owned());
    }

    let mut records = BTreeMap::new();
    for protocol in &inventory.protocol {
        validate_protocol(root, protocol, authority_statuses, &mut errors);
        if records.insert(protocol.id.clone(), protocol).is_some() {
            errors.push(format!(
                "GOV-PROTO-0001: duplicate protocol id {}",
                display_id(protocol)
            ));
        }
    }
    for required_id in required_ids {
        if !records.contains_key(*required_id) {
            errors.push(format!(
                "GOV-PROTO-0001: required protocol {required_id} is absent from {MANIFEST_PATH}"
            ));
        }
    }
    errors
}

fn validate_protocol(
    root: &Path,
    protocol: &Protocol,
    authority_statuses: &BTreeMap<String, String>,
    errors: &mut Vec<String>,
) {
    if !valid_id(&protocol.id) {
        errors.push(format!(
            "GOV-PROTO-0010: {} is not a valid PROTO-* identifier",
            display_id(protocol)
        ));
    }
    for (field, value) in [
        ("title", protocol.title.as_str()),
        ("category", protocol.category.as_str()),
        ("visibility", protocol.visibility.as_str()),
        ("stability", protocol.stability.as_str()),
        ("reader_policy", protocol.reader_policy.as_str()),
        ("writer_policy", protocol.writer_policy.as_str()),
        (
            "unknown_field_policy",
            protocol.unknown_field_policy.as_str(),
        ),
        ("migration_tool", protocol.migration_tool.as_str()),
    ] {
        if value.trim().is_empty() {
            errors.push(format!(
                "GOV-PROTO-0010: {} has an empty {field}",
                display_id(protocol)
            ));
        }
    }
    if !CATEGORIES.contains(&protocol.category.as_str()) {
        errors.push(format!(
            "GOV-PROTO-0010: {} has unknown category {}",
            display_id(protocol),
            protocol.category
        ));
    }

    let lifecycle_is_valid = matches!(
        (
            protocol.visibility.as_str(),
            protocol.stability.as_str(),
            protocol.implemented,
        ),
        ("Public", "Experimental" | "Preview" | "Stable", true)
            | ("Internal", "Internal", true)
            | ("Planned public", "Future", false)
    );
    if !lifecycle_is_valid {
        errors.push(format!(
            "GOV-PROTO-0005: {} has inconsistent visibility {}, stability {}, and implemented={}",
            display_id(protocol),
            protocol.visibility,
            protocol.stability,
            protocol.implemented
        ));
    }
    if protocol.public_schema && protocol.visibility != "Public" {
        errors.push(format!(
            "GOV-PROTO-0005: {} marks a non-public record as a public schema",
            display_id(protocol)
        ));
    }
    if protocol.implemented && protocol.current_version.trim().is_empty() {
        errors.push(format!(
            "GOV-PROTO-0003: implemented protocol {} has no current version",
            display_id(protocol)
        ));
    }
    if protocol.public_schema && protocol.implemented && protocol.version_markers.is_empty() {
        errors.push(format!(
            "GOV-PROTO-0003: public schema {} has no implementation version marker",
            display_id(protocol)
        ));
    }
    if protocol.stability == "Stable" && protocol.fixtures.is_empty() {
        errors.push(format!(
            "GOV-PROTO-0004: Stable protocol {} has no fixture",
            display_id(protocol)
        ));
    }
    if protocol.stability == "Future"
        && (!protocol.current_version.is_empty()
            || !protocol.version_markers.is_empty()
            || !protocol.fixtures.is_empty()
            || protocol.canonical
            || protocol.public_schema)
    {
        errors.push(format!(
            "GOV-PROTO-0005: Future protocol {} claims a version, fixture, canonical form, or current public schema",
            display_id(protocol)
        ));
    }

    for (field, values, required) in [
        ("producer", protocol.producer.as_slice(), true),
        ("consumer", protocol.consumer.as_slice(), true),
        ("sources", protocol.sources.as_slice(), true),
        ("authority", protocol.authority.as_slice(), true),
        ("fixtures", protocol.fixtures.as_slice(), false),
        (
            "version_markers",
            protocol.version_markers.as_slice(),
            false,
        ),
        ("notes", protocol.notes.as_slice(), false),
    ] {
        validate_values(protocol, field, values, required, errors);
    }

    let mut source_texts = Vec::new();
    for source in &protocol.sources {
        match validate_path(root, protocol, "source", source, true) {
            Ok(()) => match fs::read_to_string(root.join(source)) {
                Ok(text) => source_texts.push(text),
                Err(error) => errors.push(format!(
                    "GOV-PROTO-0002: {} cannot read source {source}: {error}",
                    display_id(protocol)
                )),
            },
            Err(error) => errors.push(error),
        }
    }
    for fixture in &protocol.fixtures {
        if let Err(error) = validate_path(root, protocol, "fixture", fixture, false) {
            errors.push(error);
        }
    }
    for marker in &protocol.version_markers {
        if !source_texts.iter().any(|text| text.contains(marker)) {
            errors.push(format!(
                "GOV-PROTO-0007: {} version marker {marker:?} is absent from its sources",
                display_id(protocol)
            ));
        }
    }

    let mut has_accepted_authority = false;
    for authority in &protocol.authority {
        match authority_statuses.get(authority) {
            Some(status) if status == "Accepted" => has_accepted_authority = true,
            Some(_) => {}
            None => errors.push(format!(
                "GOV-PROTO-0011: {} references unknown authority {authority}",
                display_id(protocol)
            )),
        }
    }
    if matches!(protocol.stability.as_str(), "Preview" | "Stable") && !has_accepted_authority {
        errors.push(format!(
            "GOV-PROTO-0006: {} {} protocol has no Accepted authority",
            display_id(protocol),
            protocol.stability
        ));
    }
}

fn validate_values(
    protocol: &Protocol,
    field: &str,
    values: &[String],
    required: bool,
    errors: &mut Vec<String>,
) {
    if required && values.is_empty() {
        errors.push(format!(
            "GOV-PROTO-0010: {} has no {field} entries",
            display_id(protocol)
        ));
    }
    let mut seen = BTreeSet::new();
    for value in values {
        if value.trim().is_empty() {
            errors.push(format!(
                "GOV-PROTO-0010: {} has an empty {field} entry",
                display_id(protocol)
            ));
        } else if !seen.insert(value) {
            errors.push(format!(
                "GOV-PROTO-0010: {} repeats {field} entry {value}",
                display_id(protocol)
            ));
        }
    }
}

fn validate_path(
    root: &Path,
    protocol: &Protocol,
    field: &str,
    value: &str,
    require_file: bool,
) -> Result<(), String> {
    if !is_relative_path(value) {
        return Err(format!(
            "GOV-PROTO-0002: {} has invalid {field} path {value}",
            display_id(protocol)
        ));
    }
    let path = root.join(value);
    let exists = if require_file {
        path.is_file()
    } else {
        path.exists()
    };
    if !exists {
        return Err(format!(
            "GOV-PROTO-0002: {} references missing {field} path {value}",
            display_id(protocol)
        ));
    }
    Ok(())
}

fn render(inventory: &ProtocolInventory) -> String {
    let mut protocols = inventory.protocol.iter().collect::<Vec<_>>();
    protocols.sort_by(|left, right| {
        visibility_rank(&left.visibility)
            .cmp(&visibility_rank(&right.visibility))
            .then_with(|| category_rank(&left.category).cmp(&category_rank(&right.category)))
            .then_with(|| left.id.cmp(&right.id))
    });
    let counts = summary(inventory);

    let mut output = String::new();
    output.push_str("# Ling 公开接口与协议清单 / Public Protocol Inventory\n\n");
    output.push_str("> 状态：由 `protocol-inventory.toml` 确定性生成\n");
    output.push_str(&format!("> 更新日期：{}\n", inventory.updated));
    output.push_str("> 本清单记录当前兼容边界，不新增语言语义或协议承诺。\n\n");
    output.push_str("## Summary\n\n");
    output.push_str(&format!(
        "- {} records: {} current public, {} internal, {} Future.\n",
        counts.protocol_count, counts.public_count, counts.internal_count, counts.future_count
    ));
    output.push_str(&format!(
        "- Current public stability: {} Experimental, {} Preview, {} Stable.\n",
        counts.experimental_count, counts.preview_count, counts.stable_count
    ));
    output.push_str("- `Stable` means the ROADMAP-1.0 1.x commitment. No current Seed protocol has passed that gate; stable diagnostic codes remain a documented compatibility subset inside the Preview Diagnostic protocol.\n\n");
    output.push_str("## Inventory\n\n");
    output.push_str("| ID | Visibility | Category | Current version | Stability | Public schema | Canonical | Fixtures |\n");
    output.push_str("| --- | --- | --- | --- | --- | --- | --- | ---: |\n");
    for protocol in &protocols {
        output.push_str(&format!(
            "| `{}` | {} | {} | {} | `{}` | {} | {} | {} |\n",
            escape_cell(&protocol.id),
            escape_cell(&protocol.visibility),
            escape_cell(&protocol.category),
            if protocol.current_version.is_empty() {
                "—".to_owned()
            } else {
                format!("`{}`", escape_cell(&protocol.current_version))
            },
            escape_cell(&protocol.stability),
            yes_no(protocol.public_schema),
            yes_no(protocol.canonical),
            protocol.fixtures.len(),
        ));
    }
    output.push_str("\n## Reader, writer, and migration policies\n\n");
    for protocol in protocols {
        output.push_str(&format!(
            "### `{}` — {}\n\n",
            escape_heading(&protocol.id),
            escape_heading(&protocol.title)
        ));
        output.push_str(&format!("- Producer: {}\n", list_text(&protocol.producer)));
        output.push_str(&format!("- Consumer: {}\n", list_text(&protocol.consumer)));
        output.push_str(&format!(
            "- Reader policy: {}\n",
            protocol.reader_policy.trim()
        ));
        output.push_str(&format!(
            "- Writer policy: {}\n",
            protocol.writer_policy.trim()
        ));
        output.push_str(&format!(
            "- Unknown-field policy: {}\n",
            protocol.unknown_field_policy.trim()
        ));
        output.push_str(&format!(
            "- Migration tool: {}\n",
            protocol.migration_tool.trim()
        ));
        output.push_str(&format!(
            "- Authority: {}\n",
            code_list(&protocol.authority)
        ));
        output.push_str(&format!("- Sources: {}\n", path_list(&protocol.sources)));
        output.push_str(&format!(
            "- Fixtures: {}\n",
            if protocol.fixtures.is_empty() {
                "—".to_owned()
            } else {
                path_list(&protocol.fixtures)
            }
        ));
        if !protocol.notes.is_empty() {
            output.push_str(&format!("- Notes: {}\n", list_text(&protocol.notes)));
        }
        output.push('\n');
    }
    output.push_str("## Machine source\n\n");
    output.push_str("The machine-readable source is [`protocol-inventory.toml`](protocol-inventory.toml). Run `cargo xtask governance check-protocols` to reject duplicate or missing required records, unversioned implemented/public schemas, invalid stability claims, Preview/Stable protocols without Accepted authority, Stable protocols without fixtures, missing paths/version markers, and generated-report drift.\n");
    output
}

fn summary(inventory: &ProtocolInventory) -> CheckSummary {
    CheckSummary {
        protocol_count: inventory.protocol.len(),
        public_count: inventory
            .protocol
            .iter()
            .filter(|protocol| protocol.visibility == "Public")
            .count(),
        preview_count: count_stability(inventory, "Preview"),
        experimental_count: count_stability(inventory, "Experimental"),
        stable_count: count_stability(inventory, "Stable"),
        internal_count: count_stability(inventory, "Internal"),
        future_count: count_stability(inventory, "Future"),
    }
}

fn count_stability(inventory: &ProtocolInventory, stability: &str) -> usize {
    inventory
        .protocol
        .iter()
        .filter(|protocol| protocol.stability == stability)
        .count()
}

fn visibility_rank(visibility: &str) -> u8 {
    match visibility {
        "Public" => 1,
        "Internal" => 2,
        "Planned public" => 3,
        _ => u8::MAX,
    }
}

fn category_rank(category: &str) -> u8 {
    CATEGORIES
        .iter()
        .position(|candidate| candidate == &category)
        .and_then(|index| u8::try_from(index).ok())
        .unwrap_or(u8::MAX)
}

fn path_list(paths: &[String]) -> String {
    paths
        .iter()
        .map(|path| format!("[`{}`]({})", escape_cell(path), report_link(path)))
        .collect::<Vec<_>>()
        .join(", ")
}

fn report_link(path: &str) -> String {
    match path.strip_prefix("docs/") {
        Some(relative) => format!("../{relative}"),
        None => format!("../../{path}"),
    }
}

fn list_text(values: &[String]) -> String {
    values
        .iter()
        .map(|value| value.trim())
        .collect::<Vec<_>>()
        .join("; ")
}

fn code_list(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("`{}`", escape_cell(value)))
        .collect::<Vec<_>>()
        .join(", ")
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn display_id(protocol: &Protocol) -> &str {
    if protocol.id.is_empty() {
        "<missing-id>"
    } else {
        &protocol.id
    }
}

fn valid_id(value: &str) -> bool {
    value.starts_with("PROTO-")
        && !value.ends_with('-')
        && !value.contains("--")
        && value.chars().all(|character| {
            character.is_ascii_uppercase() || character.is_ascii_digit() || character == '-'
        })
}

fn is_date(value: &str) -> bool {
    value.len() == 10
        && value.as_bytes()[4] == b'-'
        && value.as_bytes()[7] == b'-'
        && value
            .chars()
            .enumerate()
            .all(|(index, character)| matches!(index, 4 | 7) || character.is_ascii_digit())
}

fn is_relative_path(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains('\\')
        && value.as_bytes().get(1) != Some(&b':')
        && value
            .split('/')
            .all(|segment| !segment.is_empty() && !matches!(segment, "." | ".."))
}

fn escape_cell(value: &str) -> String {
    value.replace(['\r', '\n'], " ").replace('|', "\\|")
}

fn escape_heading(value: &str) -> String {
    value.replace(['\r', '\n'], " ").replace('`', "\\`")
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
    use super::{
        MANIFEST_PATH, ProtocolInventory, REPORT_PATH, check_repository, render, validate,
    };
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

    struct TempRepository(PathBuf);

    impl TempRepository {
        fn new() -> Self {
            let sequence = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir()
                .join(format!("ling-protocols-{}-{sequence}", std::process::id()));
            fs::create_dir_all(path.join("src")).expect("create source fixture directory");
            fs::create_dir_all(path.join("tests")).expect("create test fixture directory");
            fs::write(
                path.join("src/protocol.rs"),
                "const SCHEMA: &str = \"ling.test/0.1\";\n",
            )
            .expect("write version marker");
            fs::write(path.join("tests/protocol.fixture"), "fixture\n")
                .expect("write protocol fixture");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempRepository {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn record() -> String {
        r#"
[[protocol]]
id = "PROTO-DIAGNOSTIC-JSON"
title = "Test protocol"
category = "JSON"
visibility = "Public"
current_version = "ling.test/0.1"
stability = "Preview"
implemented = true
public_schema = true
canonical = false
producer = ["test writer"]
consumer = ["test reader"]
reader_policy = "Require the current version."
writer_policy = "Write the current version."
unknown_field_policy = "Reject unknown core fields."
migration_tool = "None."
fixtures = ["tests/protocol.fixture"]
sources = ["src/protocol.rs"]
version_markers = ["ling.test/0.1"]
authority = ["DEC-0001"]
notes = []
"#
        .to_owned()
    }

    fn parse(body: &str) -> ProtocolInventory {
        toml::from_str(&format!(
            "schema_version = 1\nupdated = \"2026-08-20\"\n{body}"
        ))
        .expect("valid protocol inventory fixture")
    }

    fn authorities(status: &str) -> BTreeMap<String, String> {
        [("DEC-0001".to_owned(), status.to_owned())]
            .into_iter()
            .collect()
    }

    fn errors(body: &str) -> Vec<String> {
        let repository = TempRepository::new();
        validate(
            repository.path(),
            &parse(body),
            &authorities("Accepted"),
            &["PROTO-DIAGNOSTIC-JSON"],
        )
    }

    #[test]
    fn accepts_versioned_preview_schema_with_fixture_and_accepted_authority() {
        assert!(errors(&record()).is_empty());
    }

    #[test]
    fn rejects_unversioned_public_schema() {
        let body = record()
            .replace(
                "current_version = \"ling.test/0.1\"",
                "current_version = \"\"",
            )
            .replace(
                "version_markers = [\"ling.test/0.1\"]",
                "version_markers = []",
            );
        let errors = errors(&body);
        assert!(errors.iter().any(|error| error.contains("GOV-PROTO-0003")));
    }

    #[test]
    fn rejects_stable_protocol_without_fixture() {
        let body = record()
            .replace("stability = \"Preview\"", "stability = \"Stable\"")
            .replace("fixtures = [\"tests/protocol.fixture\"]", "fixtures = []");
        let errors = errors(&body);
        assert!(errors.iter().any(|error| error.contains("GOV-PROTO-0004")));
    }

    #[test]
    fn rejects_future_record_that_claims_an_implementation_version() {
        let body = record()
            .replace("visibility = \"Public\"", "visibility = \"Planned public\"")
            .replace("stability = \"Preview\"", "stability = \"Future\"");
        let errors = errors(&body);
        assert!(errors.iter().any(|error| error.contains("GOV-PROTO-0005")));
    }

    #[test]
    fn rejects_preview_without_accepted_authority() {
        let repository = TempRepository::new();
        let errors = validate(
            repository.path(),
            &parse(&record()),
            &authorities("Draft"),
            &["PROTO-DIAGNOSTIC-JSON"],
        );
        assert!(errors.iter().any(|error| error.contains("GOV-PROTO-0006")));
    }

    #[test]
    fn rejects_version_marker_absent_from_sources() {
        let body = record().replace(
            "version_markers = [\"ling.test/0.1\"]",
            "version_markers = [\"ling.test/9.9\"]",
        );
        let errors = errors(&body);
        assert!(errors.iter().any(|error| error.contains("GOV-PROTO-0007")));
    }

    #[test]
    fn rejects_duplicate_ids() {
        let duplicate = format!("{}{}", record(), record());
        let errors = errors(&duplicate);
        assert!(errors.iter().any(|error| error.contains("GOV-PROTO-0001")));
    }

    #[test]
    fn rendering_is_deterministic() {
        let inventory = parse(&record());
        assert_eq!(render(&inventory), render(&inventory));
    }

    #[test]
    fn repository_inventory_covers_every_required_surface() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("repository protocol inventory is valid");
        assert_eq!(summary.protocol_count, 33);
        assert_eq!(summary.public_count, 29);
        assert_eq!(summary.preview_count, 14);
        assert_eq!(summary.experimental_count, 15);
        assert_eq!(summary.stable_count, 0);
        assert_eq!(summary.internal_count, 1);
        assert_eq!(summary.future_count, 3);
    }

    #[test]
    fn repository_has_one_protocol_inventory_source_of_truth() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        assert!(root.join(MANIFEST_PATH).is_file());
        assert!(root.join(REPORT_PATH).is_file());
        assert!(
            !root.join("docs/protocols/registry.toml").exists(),
            "the non-normative G6 path must not become a second protocol registry"
        );
    }
}
