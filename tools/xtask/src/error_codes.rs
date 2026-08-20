use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};

const REGISTRY_PATH: &str = "docs/ERROR-CODES.md";
const LOCK_PATH: &str = "docs/governance/error-code-lock.toml";
const CONSTANTS_PATH: &str = "crates/ling-diagnostics/src/lib.rs";
const REGISTRY_SCHEMA: &str = "ling.diagnostic-registry/0.1";
const ACTIVE_HEADING: &str = "## Active allocations / 活跃分配";
const RETIRED_HEADING: &str = "## Retired allocations / 退役分配";
const TABLE_HEADER: [&str; 11] = [
    "Code",
    "Phase",
    "Stability",
    "Severity",
    "中文标题",
    "English title",
    "中文模板",
    "English template",
    "Payload schema",
    "Repair schema",
    "Since",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckSummary {
    pub active_count: usize,
    pub retired_count: usize,
    pub domain_count: usize,
    pub rust_constant_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Registry {
    entries: Vec<RegistryEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RegistryEntry {
    code: String,
    phase: String,
    stability: String,
    severity: String,
    title_zh: String,
    title_en: String,
    template_zh: String,
    template_en: String,
    facts: BTreeMap<String, FactSchema>,
    repair: Option<BTreeMap<String, FactSchema>>,
    since: String,
    retired: bool,
    line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FactSchema {
    value_type: String,
    optional: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CompatibilityLock {
    schema_version: u32,
    source: String,
    registry_schema: String,
    #[serde(default)]
    high_water: BTreeMap<String, u16>,
    #[serde(default)]
    code: Vec<LockedCode>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LockedCode {
    id: String,
    core_sha256: String,
    retired: bool,
    #[serde(default)]
    facts: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Section {
    Active,
    Retired,
}

pub fn check_repository(root: &Path) -> Result<CheckSummary, Vec<String>> {
    let registry = load_registry(root)?;
    let mut errors = validate_registry(&registry);
    let constants = validate_implementation(root, &registry, &mut errors);
    validate_lock(root, &registry, &mut errors);
    finish(errors).map(|()| summary(&registry, constants.len()))
}

pub fn render_lock_repository(root: &Path) -> Result<String, Vec<String>> {
    let registry = load_registry(root)?;
    let mut errors = validate_registry(&registry);
    validate_implementation(root, &registry, &mut errors);
    validate_lock_evolution(root, &registry, &mut errors);
    finish(errors).map(|()| render_lock(&registry))
}

fn load_registry(root: &Path) -> Result<Registry, Vec<String>> {
    let text = fs::read_to_string(root.join(REGISTRY_PATH)).map_err(|error| {
        vec![format!(
            "GOV-DIAG-0002: cannot read {REGISTRY_PATH}: {error}"
        )]
    })?;
    parse_registry(&text)
}

fn parse_registry(text: &str) -> Result<Registry, Vec<String>> {
    let mut errors = Vec::new();
    if !text.contains(REGISTRY_SCHEMA) {
        errors.push(format!(
            "GOV-DIAG-0003: {REGISTRY_PATH} does not declare registry schema {REGISTRY_SCHEMA}"
        ));
    }

    let mut section = None;
    let mut in_table = false;
    let mut active_table = false;
    let mut retired_table = false;
    let mut entries = Vec::new();

    for (index, line) in text.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = line.trim();
        if trimmed == ACTIVE_HEADING {
            section = Some(Section::Active);
            in_table = false;
            continue;
        }
        if trimmed == RETIRED_HEADING {
            section = Some(Section::Retired);
            in_table = false;
            continue;
        }
        let Some(current_section) = section else {
            continue;
        };
        if trimmed.starts_with('|') {
            let cells = split_row(trimmed);
            if cells == TABLE_HEADER {
                in_table = true;
                match current_section {
                    Section::Active => active_table = true,
                    Section::Retired => retired_table = true,
                }
                continue;
            }
            if in_table && is_separator(&cells) {
                continue;
            }
            if in_table {
                match parse_entry(&cells, current_section, line_number) {
                    Ok(entry) => entries.push(entry),
                    Err(mut row_errors) => errors.append(&mut row_errors),
                }
            }
        } else if in_table && !trimmed.is_empty() {
            in_table = false;
        }
    }

    if !active_table {
        errors.push(format!(
            "GOV-DIAG-0003: {REGISTRY_PATH} has no active allocation table"
        ));
    }
    if !retired_table {
        errors.push(format!(
            "GOV-DIAG-0003: {REGISTRY_PATH} has no retired allocation table"
        ));
    }
    if entries.is_empty() {
        errors.push(format!(
            "GOV-DIAG-0003: {REGISTRY_PATH} contains no diagnostic allocations"
        ));
    }

    finish(errors).map(|()| Registry { entries })
}

fn parse_entry(
    cells: &[&str],
    section: Section,
    line: usize,
) -> Result<RegistryEntry, Vec<String>> {
    if cells.len() != TABLE_HEADER.len() {
        return Err(vec![format!(
            "GOV-DIAG-0003: {REGISTRY_PATH}:{line} has {} cells; expected {}",
            cells.len(),
            TABLE_HEADER.len()
        )]);
    }

    let code = plain_cell(cells[0]);
    let mut errors = Vec::new();
    let facts = match parse_schema_cell(cells[8], &code, "Payload schema") {
        Ok(facts) => facts,
        Err(error) => {
            errors.push(format!("GOV-DIAG-0005: {REGISTRY_PATH}:{line} {error}"));
            BTreeMap::new()
        }
    };
    let repair = match parse_repair_cell(cells[9], &code) {
        Ok(repair) => repair,
        Err(error) => {
            errors.push(format!("GOV-DIAG-0005: {REGISTRY_PATH}:{line} {error}"));
            None
        }
    };

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(RegistryEntry {
        code,
        phase: plain_cell(cells[1]),
        stability: plain_cell(cells[2]),
        severity: plain_cell(cells[3]),
        title_zh: cells[4].trim().to_owned(),
        title_en: cells[5].trim().to_owned(),
        template_zh: cells[6].trim().to_owned(),
        template_en: cells[7].trim().to_owned(),
        facts,
        repair,
        since: plain_cell(cells[10]),
        retired: section == Section::Retired,
        line,
    })
}

fn validate_registry(registry: &Registry) -> Vec<String> {
    let mut errors = Vec::new();
    let mut codes = BTreeMap::new();
    let mut active_count = 0_usize;
    let mut retired_count = 0_usize;

    for entry in &registry.entries {
        if let Some(previous) = codes.insert(entry.code.clone(), entry.line) {
            errors.push(format!(
                "GOV-DIAG-0001: duplicate code {} at {REGISTRY_PATH}:{} (first at line {previous})",
                display_code(entry),
                entry.line
            ));
        }
        validate_entry(entry, &mut errors);
        if entry.retired {
            retired_count += 1;
        } else {
            active_count += 1;
        }
    }

    if active_count == 0 {
        errors.push("GOV-DIAG-0005: registry contains no active codes".to_owned());
    }
    if retired_count == 0 {
        errors.push(
            "GOV-DIAG-0005: retired allocation list is empty; historical L-IMPL-0001 must remain recorded"
                .to_owned(),
        );
    }
    errors
}

fn validate_entry(entry: &RegistryEntry, errors: &mut Vec<String>) {
    let domain = match code_parts(&entry.code) {
        Some((domain, _)) => domain,
        None => {
            errors.push(format!(
                "GOV-DIAG-0003: {} at {REGISTRY_PATH}:{} is not L-<DOMAIN>-<FOUR DIGITS>",
                display_code(entry),
                entry.line
            ));
            ""
        }
    };
    if !domain.is_empty() && entry.phase != domain {
        errors.push(format!(
            "GOV-DIAG-0005: {} phase {} does not match its root-cause domain {domain}",
            display_code(entry),
            entry.phase
        ));
    }

    let expected_stability = if entry.retired {
        "Deprecated"
    } else {
        "Preview"
    };
    if entry.stability != expected_stability {
        errors.push(format!(
            "GOV-DIAG-0005: {} stability is {}; expected {expected_stability} in this table",
            display_code(entry),
            entry.stability
        ));
    }
    if !matches!(entry.severity.as_str(), "Error" | "Warning") {
        errors.push(format!(
            "GOV-DIAG-0005: {} has unsupported severity {}; expected Error or Warning",
            display_code(entry),
            entry.severity
        ));
    }

    validate_translation(entry, errors);
    validate_templates(entry, errors);

    if !valid_version(&entry.since) {
        errors.push(format!(
            "GOV-DIAG-0005: {} has invalid first version {}",
            display_code(entry),
            entry.since
        ));
    }

    if let Some(repair) = &entry.repair {
        let kind = repair.get("kind");
        let changes_semantics = repair.get("changes_semantics");
        if kind
            != Some(&FactSchema {
                value_type: "string".to_owned(),
                optional: false,
            })
            || changes_semantics
                != Some(&FactSchema {
                    value_type: "boolean".to_owned(),
                    optional: false,
                })
        {
            errors.push(format!(
                "GOV-DIAG-0005: {} repair schema must contain required kind:string and changes_semantics:boolean fields",
                display_code(entry)
            ));
        }
    }
}

fn validate_translation(entry: &RegistryEntry, errors: &mut Vec<String>) {
    for (label, value) in [
        ("Chinese title", entry.title_zh.as_str()),
        ("Chinese template", entry.template_zh.as_str()),
    ] {
        if value.trim().is_empty() || value.is_ascii() {
            errors.push(format!(
                "GOV-DIAG-0004: {} has a missing or non-Chinese {label}",
                display_code(entry)
            ));
        }
    }
    for (label, value) in [
        ("English title", entry.title_en.as_str()),
        ("English template", entry.template_en.as_str()),
    ] {
        if value.trim().is_empty()
            || !value
                .chars()
                .any(|character| character.is_ascii_alphabetic())
        {
            errors.push(format!(
                "GOV-DIAG-0004: {} has a missing or non-English {label}",
                display_code(entry)
            ));
        }
    }
}

fn validate_templates(entry: &RegistryEntry, errors: &mut Vec<String>) {
    let zh = template_parameters(&entry.template_zh);
    let en = template_parameters(&entry.template_en);
    match (zh, en) {
        (Ok(zh), Ok(en)) if zh == en => {}
        (Ok(zh), Ok(en)) => errors.push(format!(
            "GOV-DIAG-0004: {} bilingual template parameters differ: zh={zh:?}, en={en:?}",
            display_code(entry)
        )),
        (Err(error), _) | (_, Err(error)) => errors.push(format!(
            "GOV-DIAG-0004: {} has an invalid template: {error}",
            display_code(entry)
        )),
    }
}

fn validate_implementation(
    root: &Path,
    registry: &Registry,
    errors: &mut Vec<String>,
) -> BTreeSet<String> {
    let constants = load_constants(root, errors);
    let active = registry
        .entries
        .iter()
        .filter(|entry| !entry.retired)
        .map(|entry| entry.code.clone())
        .collect::<BTreeSet<_>>();
    let retired = registry
        .entries
        .iter()
        .filter(|entry| entry.retired)
        .map(|entry| entry.code.clone())
        .collect::<BTreeSet<_>>();
    let occurrences = scan_public_code_literals(root, errors);
    validate_code_sets(&active, &retired, &constants, &occurrences, errors);
    constants
}

fn validate_code_sets(
    active: &BTreeSet<String>,
    retired: &BTreeSet<String>,
    constants: &BTreeSet<String>,
    occurrences: &[(String, String)],
    errors: &mut Vec<String>,
) {
    for code in active.difference(constants) {
        errors.push(format!(
            "GOV-DIAG-0006: active registry code {code} has no canonical Rust constant in {CONSTANTS_PATH}"
        ));
    }
    for code in constants.difference(active) {
        errors.push(format!(
            "GOV-DIAG-0006: Rust constant {code} is not an active allocation in {REGISTRY_PATH}"
        ));
    }
    for code in constants.intersection(retired) {
        errors.push(format!(
            "GOV-DIAG-0006: retired code {code} remains exposed as a canonical Rust constant"
        ));
    }

    let registered = active.union(retired).cloned().collect::<BTreeSet<_>>();
    for (path, code) in occurrences {
        if !registered.contains(code) {
            errors.push(format!(
                "GOV-DIAG-0006: unregistered public diagnostic code {code} occurs in {path}"
            ));
        } else if retired.contains(code) {
            errors.push(format!(
                "GOV-DIAG-0006: retired diagnostic code {code} is still referenced by {path}"
            ));
        }
    }
}

fn load_constants(root: &Path, errors: &mut Vec<String>) -> BTreeSet<String> {
    let text = match fs::read_to_string(root.join(CONSTANTS_PATH)) {
        Ok(text) => text,
        Err(error) => {
            errors.push(format!(
                "GOV-DIAG-0002: cannot read {CONSTANTS_PATH}: {error}"
            ));
            return BTreeSet::new();
        }
    };
    let marker = "DiagnosticCode::new(\"";
    let mut remaining = text.as_str();
    let mut constants = BTreeSet::new();
    while let Some(start) = remaining.find(marker) {
        remaining = &remaining[start + marker.len()..];
        let Some(end) = remaining.find('\"') else {
            errors.push(format!(
                "GOV-DIAG-0006: unterminated DiagnosticCode::new string in {CONSTANTS_PATH}"
            ));
            break;
        };
        let code = &remaining[..end];
        if code_parts(code).is_none() {
            errors.push(format!(
                "GOV-DIAG-0006: invalid canonical Rust diagnostic code {code:?}"
            ));
        } else if !constants.insert(code.to_owned()) {
            errors.push(format!(
                "GOV-DIAG-0001: duplicate canonical Rust diagnostic code {code}"
            ));
        }
        remaining = &remaining[end + 1..];
    }
    if constants.is_empty() {
        errors.push(format!(
            "GOV-DIAG-0006: {CONSTANTS_PATH} declares no DiagnosticCode constants"
        ));
    }
    constants
}

fn scan_public_code_literals(root: &Path, errors: &mut Vec<String>) -> Vec<(String, String)> {
    let mut files = Vec::new();
    for directory in ["crates", "tests"] {
        collect_files(&root.join(directory), &mut files, errors);
    }
    files.sort();
    let mut occurrences = Vec::new();
    for path in files {
        let extension = path.extension().and_then(|value| value.to_str());
        if !matches!(extension, Some("rs" | "toml" | "json" | "snap")) {
            continue;
        }
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) => {
                errors.push(format!(
                    "GOV-DIAG-0002: cannot read {}: {error}",
                    repository_path(root, &path)
                ));
                continue;
            }
        };
        let display_path = repository_path(root, &path);
        for candidate in code_candidates(&text) {
            if code_parts(&candidate).is_none() {
                errors.push(format!(
                    "GOV-DIAG-0006: malformed public diagnostic code {candidate} occurs in {display_path}"
                ));
            } else {
                occurrences.push((display_path.clone(), candidate));
            }
        }
    }
    occurrences
}

fn collect_files(path: &Path, files: &mut Vec<PathBuf>, errors: &mut Vec<String>) {
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(format!(
                "GOV-DIAG-0002: cannot scan {}: {error}",
                path.display()
            ));
            return;
        }
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                errors.push(format!(
                    "GOV-DIAG-0002: cannot scan directory entry: {error}"
                ));
                continue;
            }
        };
        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(error) => {
                errors.push(format!(
                    "GOV-DIAG-0002: cannot inspect {}: {error}",
                    path.display()
                ));
                continue;
            }
        };
        if file_type.is_dir() {
            collect_files(&path, files, errors);
        } else if file_type.is_file() {
            files.push(path);
        }
    }
}

fn validate_lock(root: &Path, registry: &Registry, errors: &mut Vec<String>) {
    let text = match fs::read_to_string(root.join(LOCK_PATH)) {
        Ok(text) => text,
        Err(error) => {
            errors.push(format!("GOV-DIAG-0002: cannot read {LOCK_PATH}: {error}"));
            return;
        }
    };
    let lock: CompatibilityLock = match toml::from_str(&text) {
        Ok(lock) => lock,
        Err(error) => {
            errors.push(format!(
                "GOV-DIAG-0007: invalid compatibility lock {LOCK_PATH}: {error}"
            ));
            return;
        }
    };
    if lock.schema_version != 1 {
        errors.push(format!(
            "GOV-DIAG-0007: unsupported lock schema_version {}; expected 1",
            lock.schema_version
        ));
    }
    if lock.source != REGISTRY_PATH {
        errors.push(format!(
            "GOV-DIAG-0007: lock source is {}; expected {REGISTRY_PATH}",
            lock.source
        ));
    }
    if lock.registry_schema != REGISTRY_SCHEMA {
        errors.push(format!(
            "GOV-DIAG-0007: lock registry_schema is {}; expected {REGISTRY_SCHEMA}",
            lock.registry_schema
        ));
    }

    let expected_high_water = high_water(registry);
    if lock.high_water != expected_high_water {
        errors.push(format!(
            "GOV-DIAG-0007: {LOCK_PATH} high-water marks differ from the registry"
        ));
    }

    let expected = locked_entries(registry);
    let mut actual = BTreeMap::new();
    for entry in &lock.code {
        if actual.insert(entry.id.clone(), entry).is_some() {
            errors.push(format!(
                "GOV-DIAG-0001: duplicate code {} in {LOCK_PATH}",
                entry.id
            ));
        }
    }
    for (code, expected_entry) in &expected {
        match actual.get(code) {
            Some(actual_entry) => {
                if actual_entry.core_sha256 != expected_entry.core_sha256 {
                    errors.push(format!(
                        "GOV-DIAG-0007: {code} immutable meaning, phase, severity, or first version differs from its compatibility lock"
                    ));
                }
                if actual_entry.retired != expected_entry.retired {
                    errors.push(format!(
                        "GOV-DIAG-0007: {code} active/retired state differs from its compatibility lock"
                    ));
                }
                if actual_entry.facts != expected_entry.facts {
                    errors.push(format!(
                        "GOV-DIAG-0007: {code} payload schema differs from its compatibility lock"
                    ));
                }
            }
            None => errors.push(format!("GOV-DIAG-0007: {code} is missing from {LOCK_PATH}")),
        }
    }
    for code in actual.keys() {
        if !expected.contains_key(code) {
            errors.push(format!(
                "GOV-DIAG-0007: locked code {code} was removed from {REGISTRY_PATH}; retain it in the retired table"
            ));
        }
    }

    if normalize_newlines(&text) != render_lock(registry) {
        errors.push(format!(
            "GOV-DIAG-0008: {LOCK_PATH} is not the deterministic compatibility lock for {REGISTRY_PATH}"
        ));
    }
}

fn validate_lock_evolution(root: &Path, registry: &Registry, errors: &mut Vec<String>) {
    let text = match fs::read_to_string(root.join(LOCK_PATH)) {
        Ok(text) => text,
        Err(error) => {
            errors.push(format!(
                "GOV-DIAG-0007: cannot evolve a missing {LOCK_PATH}: {error}"
            ));
            return;
        }
    };
    let lock: CompatibilityLock = match toml::from_str(&text) {
        Ok(lock) => lock,
        Err(error) => {
            errors.push(format!(
                "GOV-DIAG-0007: cannot evolve invalid compatibility lock {LOCK_PATH}: {error}"
            ));
            return;
        }
    };
    if lock.schema_version != 1
        || lock.source != REGISTRY_PATH
        || lock.registry_schema != REGISTRY_SCHEMA
    {
        errors.push(format!(
            "GOV-DIAG-0007: cannot evolve {LOCK_PATH} with an invalid header"
        ));
        return;
    }

    errors.extend(validate_evolution(registry, &lock));
}

fn validate_evolution(registry: &Registry, lock: &CompatibilityLock) -> Vec<String> {
    let mut errors = Vec::new();
    let current = locked_entries(registry);
    let mut previous = BTreeMap::new();
    for entry in &lock.code {
        if previous.insert(entry.id.clone(), entry).is_some() {
            errors.push(format!(
                "GOV-DIAG-0001: duplicate code {} in {LOCK_PATH}",
                entry.id
            ));
        }
    }
    for (code, old) in &previous {
        let Some(new) = current.get(code) else {
            errors.push(format!(
                "GOV-DIAG-0007: cannot remove locked code {code}; move it to the retired table"
            ));
            continue;
        };
        if old.core_sha256 != new.core_sha256 {
            errors.push(format!(
                "GOV-DIAG-0007: cannot change the locked root-cause contract for {code}; allocate a new code"
            ));
        }
        if old.retired && !new.retired {
            errors.push(format!(
                "GOV-DIAG-0007: cannot reactivate retired code {code}; allocate a new code"
            ));
        }
        for fact in &old.facts {
            if !new.facts.contains(fact) {
                errors.push(format!(
                    "GOV-DIAG-0007: cannot remove or change locked payload field {fact} from {code}"
                ));
            }
        }
        for fact in &new.facts {
            if !old.facts.contains(fact) && !fact.ends_with(":optional") {
                errors.push(format!(
                    "GOV-DIAG-0007: new payload field {fact} on {code} must be optional"
                ));
            }
        }
    }

    for code in current.keys() {
        if previous.contains_key(code) {
            continue;
        }
        let Some((domain, number)) = code_parts(code) else {
            continue;
        };
        match lock.high_water.get(domain) {
            Some(high_water) if number <= *high_water => errors.push(format!(
                "GOV-DIAG-0007: new code {code} does not advance {domain} high-water mark {high_water:04}"
            )),
            None
                if number != 1
                    && !current.contains_key(&format!("L-{domain}-0001")) =>
            {
                errors.push(format!(
                    "GOV-DIAG-0007: first allocation in new domain {domain} must be 0001, found {number:04}"
                ));
            }
            _ => {}
        }
    }
    errors
}

fn render_lock(registry: &Registry) -> String {
    let mut output = String::new();
    output.push_str("# Generated from docs/ERROR-CODES.md; do not allocate codes here.\n");
    output.push_str("# Refresh with: cargo xtask governance render-error-code-lock\n");
    output.push_str("schema_version = 1\n");
    output.push_str(&format!("source = {}\n", toml_string(REGISTRY_PATH)));
    output.push_str(&format!(
        "registry_schema = {}\n\n",
        toml_string(REGISTRY_SCHEMA)
    ));
    output.push_str("[high_water]\n");
    for (domain, number) in high_water(registry) {
        output.push_str(&format!("{domain} = {number}\n"));
    }
    for entry in locked_entries(registry).into_values() {
        output.push_str("\n[[code]]\n");
        output.push_str(&format!("id = {}\n", toml_string(&entry.id)));
        output.push_str(&format!(
            "core_sha256 = {}\n",
            toml_string(&entry.core_sha256)
        ));
        output.push_str(&format!("retired = {}\n", entry.retired));
        output.push_str("facts = [");
        for (index, fact) in entry.facts.iter().enumerate() {
            if index > 0 {
                output.push_str(", ");
            }
            output.push_str(&toml_string(fact));
        }
        output.push_str("]\n");
    }
    output
}

fn locked_entries(registry: &Registry) -> BTreeMap<String, LockedCode> {
    registry
        .entries
        .iter()
        .map(|entry| {
            let locked = LockedCode {
                id: entry.code.clone(),
                core_sha256: core_fingerprint(entry),
                retired: entry.retired,
                facts: lock_facts(entry),
            };
            (entry.code.clone(), locked)
        })
        .collect()
}

fn high_water(registry: &Registry) -> BTreeMap<String, u16> {
    let mut marks: BTreeMap<String, u16> = BTreeMap::new();
    for entry in &registry.entries {
        if let Some((domain, number)) = code_parts(&entry.code) {
            marks
                .entry(domain.to_owned())
                .and_modify(|current| *current = (*current).max(number))
                .or_insert(number);
        }
    }
    marks
}

fn core_fingerprint(entry: &RegistryEntry) -> String {
    let mut hasher = Sha256::new();
    hash_part(&mut hasher, "ling.error-code-contract/v1");
    for value in [
        entry.code.as_str(),
        entry.phase.as_str(),
        entry.severity.as_str(),
        normalized_prose(&entry.title_zh).as_str(),
        normalized_prose(&entry.title_en).as_str(),
        entry.since.as_str(),
    ] {
        hash_part(&mut hasher, value);
    }
    let mut fingerprint = String::from("sha256:");
    for byte in hasher.finalize() {
        write!(&mut fingerprint, "{byte:02x}").expect("writing to a String cannot fail");
    }
    fingerprint
}

fn hash_part(hasher: &mut Sha256, value: &str) {
    let length = u64::try_from(value.len()).unwrap_or(u64::MAX);
    hasher.update(length.to_be_bytes());
    hasher.update(value.as_bytes());
}

fn lock_facts(entry: &RegistryEntry) -> Vec<String> {
    entry
        .facts
        .iter()
        .map(|(name, schema)| {
            format!(
                "{name}:{}:{}",
                schema.value_type,
                if schema.optional {
                    "optional"
                } else {
                    "required"
                }
            )
        })
        .collect()
}

fn parse_schema_cell(
    cell: &str,
    code: &str,
    label: &str,
) -> Result<BTreeMap<String, FactSchema>, String> {
    let plain = plain_cell(cell);
    if plain == "—" {
        return Ok(BTreeMap::new());
    }
    if plain.is_empty() {
        return Err(format!("{code} has an empty {label}"));
    }

    let mut fields = BTreeMap::new();
    for item in plain.split(',') {
        let item = item.trim();
        let Some((raw_name, value_type)) = item.split_once(':') else {
            return Err(format!("{code} {label} item {item:?} is not name:type"));
        };
        let optional = raw_name.ends_with('?');
        let name = raw_name.strip_suffix('?').unwrap_or(raw_name).trim();
        let value_type = value_type.trim();
        if !valid_field_name(name) {
            return Err(format!(
                "{code} {label} contains invalid field name {name:?}"
            ));
        }
        if !matches!(value_type, "string" | "integer" | "boolean" | "string[]") {
            return Err(format!(
                "{code} {label} field {name} has unsupported type {value_type:?}"
            ));
        }
        if fields
            .insert(
                name.to_owned(),
                FactSchema {
                    value_type: value_type.to_owned(),
                    optional,
                },
            )
            .is_some()
        {
            return Err(format!("{code} {label} contains duplicate field {name}"));
        }
    }
    Ok(fields)
}

fn parse_repair_cell(
    cell: &str,
    code: &str,
) -> Result<Option<BTreeMap<String, FactSchema>>, String> {
    if plain_cell(cell) == "—" {
        Ok(None)
    } else {
        parse_schema_cell(cell, code, "Repair schema").map(Some)
    }
}

fn template_parameters(template: &str) -> Result<BTreeSet<String>, String> {
    let mut parameters = BTreeSet::new();
    let mut remaining = template;
    while let Some(start) = remaining.find('{') {
        remaining = &remaining[start + 1..];
        let Some(end) = remaining.find('}') else {
            return Err("opening brace has no closing brace".to_owned());
        };
        let parameter = &remaining[..end];
        if !valid_field_name(parameter) {
            return Err(format!("invalid template parameter {{{parameter}}}"));
        }
        parameters.insert(parameter.to_owned());
        remaining = &remaining[end + 1..];
    }
    if remaining.contains('}') {
        return Err("closing brace has no opening brace".to_owned());
    }
    Ok(parameters)
}

fn code_candidates(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut candidates = Vec::new();
    let mut index = 0_usize;
    while index + 2 < bytes.len() {
        if bytes[index] != b'L' || bytes[index + 1] != b'-' {
            index += 1;
            continue;
        }
        if index > 0
            && (bytes[index - 1].is_ascii_alphanumeric() || matches!(bytes[index - 1], b'_' | b'-'))
        {
            index += 2;
            continue;
        }
        let start = index;
        index += 2;
        let domain_start = index;
        while index < bytes.len() && bytes[index].is_ascii_uppercase() {
            index += 1;
        }
        if index == domain_start || bytes.get(index) != Some(&b'-') {
            continue;
        }
        index += 1;
        let digits_start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        if index > digits_start {
            candidates.push(text[start..index].to_owned());
        }
    }
    candidates
}

fn code_parts(code: &str) -> Option<(&str, u16)> {
    let mut parts = code.split('-');
    if parts.next()? != "L" {
        return None;
    }
    let domain = parts.next()?;
    let number = parts.next()?;
    if parts.next().is_some()
        || domain.is_empty()
        || !domain
            .chars()
            .all(|character| character.is_ascii_uppercase())
        || number.len() != 4
        || !number.chars().all(|character| character.is_ascii_digit())
    {
        return None;
    }
    let number = number.parse().ok()?;
    (number != 0).then_some((domain, number))
}

fn split_row(line: &str) -> Vec<&str> {
    line.trim_matches('|').split('|').map(str::trim).collect()
}

fn is_separator(cells: &[&str]) -> bool {
    cells.len() == TABLE_HEADER.len()
        && cells.iter().all(|cell| {
            let trimmed = cell.trim_matches(':');
            trimmed.len() >= 3 && trimmed.chars().all(|character| character == '-')
        })
}

fn plain_cell(cell: &str) -> String {
    cell.trim().replace('`', "")
}

fn normalized_prose(value: &str) -> String {
    value
        .replace('`', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn valid_field_name(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    (first.is_ascii_lowercase() || first == '_')
        && characters.all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        })
}

fn valid_version(value: &str) -> bool {
    !value.is_empty()
        && value.chars().any(|character| character.is_ascii_digit())
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '+')
        })
}

fn repository_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn toml_string(value: &str) -> String {
    format!(
        "\"{}\"",
        value
            .replace('\\', "\\\\")
            .replace('\"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    )
}

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}

fn display_code(entry: &RegistryEntry) -> &str {
    if entry.code.is_empty() {
        "<missing-code>"
    } else {
        &entry.code
    }
}

fn summary(registry: &Registry, rust_constant_count: usize) -> CheckSummary {
    let domains = registry
        .entries
        .iter()
        .filter_map(|entry| code_parts(&entry.code).map(|(domain, _)| domain))
        .collect::<BTreeSet<_>>();
    CheckSummary {
        active_count: registry
            .entries
            .iter()
            .filter(|entry| !entry.retired)
            .count(),
        retired_count: registry
            .entries
            .iter()
            .filter(|entry| entry.retired)
            .count(),
        domain_count: domains.len(),
        rust_constant_count,
    }
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
        ACTIVE_HEADING, REGISTRY_SCHEMA, RETIRED_HEADING, Registry, check_repository,
        code_candidates, core_fingerprint, parse_registry, render_lock, validate_code_sets,
        validate_evolution, validate_registry,
    };
    use std::collections::BTreeSet;
    use std::path::Path;

    fn registry(active_rows: &[&str], retired_rows: &[&str]) -> String {
        let header = "| Code | Phase | Stability | Severity | 中文标题 | English title | 中文模板 | English template | Payload schema | Repair schema | Since |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n";
        format!(
            "# Registry\n\n> Registry schema: `{REGISTRY_SCHEMA}`\n\n{ACTIVE_HEADING}\n\n{header}{}\n{RETIRED_HEADING}\n\n{header}{}",
            active_rows.join("\n"),
            retired_rows.join("\n")
        )
    }

    fn active(code: &str) -> String {
        let phase = code.split('-').nth(1).expect("domain");
        format!(
            "| `{code}` | `{phase}` | `Preview` | `Error` | 中文标题 | English title | 中文 {{name}} | English {{name}} | `name:string` | — | `0.0.1-dev` |"
        )
    }

    fn retired() -> String {
        "| `L-IMPL-0001` | `IMPL` | `Deprecated` | `Error` | 已退役诊断 | Retired diagnostic | 已退役 | retired | — | — | `0.0.1-dev` |".to_owned()
    }

    fn parse(active_rows: &[&str]) -> Registry {
        let retired = retired();
        parse_registry(&registry(active_rows, &[&retired])).expect("valid registry fixture")
    }

    #[test]
    fn accepts_bilingual_registry_with_structured_payload() {
        let row = active("L-LEX-0001");
        assert!(validate_registry(&parse(&[&row])).is_empty());
    }

    #[test]
    fn rejects_duplicate_codes() {
        let row = active("L-LEX-0001");
        let errors = validate_registry(&parse(&[&row, &row]));
        assert!(errors.iter().any(|error| error.contains("GOV-DIAG-0001")));
    }

    #[test]
    fn rejects_mismatched_bilingual_parameters() {
        let row = active("L-LEX-0001").replace("English {name}", "English {other}");
        let errors = validate_registry(&parse(&[&row]));
        assert!(errors.iter().any(|error| error.contains("GOV-DIAG-0004")));
    }

    #[test]
    fn rejects_missing_translation() {
        let row = active("L-LEX-0001").replace("中文标题", "Chinese title");
        let errors = validate_registry(&parse(&[&row]));
        assert!(errors.iter().any(|error| error.contains("GOV-DIAG-0004")));
    }

    #[test]
    fn rejects_unstructured_repair_schema() {
        let row =
            active("L-LEX-0001").replace("| — | `0.0.1-dev` |", "| `kind:string` | `0.0.1-dev` |");
        let errors = validate_registry(&parse(&[&row]));
        assert!(errors.iter().any(|error| error.contains("GOV-DIAG-0005")));
    }

    #[test]
    fn rejects_phase_that_disagrees_with_code_domain() {
        let row = active("L-LEX-0001").replace("| `LEX` |", "| `TYPE` |");
        let errors = validate_registry(&parse(&[&row]));
        assert!(errors.iter().any(|error| error.contains("GOV-DIAG-0005")));
    }

    #[test]
    fn immutable_fingerprint_ignores_template_wording_but_not_title_meaning() {
        let row = active("L-LEX-0001");
        let original = parse(&[&row]);
        let wording = row.replace("English {name}", "Improved English {name}");
        let wording = parse(&[&wording]);
        assert_eq!(
            core_fingerprint(&original.entries[0]),
            core_fingerprint(&wording.entries[0])
        );

        let meaning = row.replace("English title", "Different root cause");
        let meaning = parse(&[&meaning]);
        assert_ne!(
            core_fingerprint(&original.entries[0]),
            core_fingerprint(&meaning.entries[0])
        );
    }

    #[test]
    fn lock_evolution_rejects_changed_meaning_and_number_backfill() {
        let original_row = active("L-LEX-0002");
        let original = parse(&[&original_row]);
        let lock = toml::from_str(&render_lock(&original)).expect("valid generated lock");

        let changed_row = original_row.replace("English title", "Different root cause");
        let changed = parse(&[&changed_row]);
        let errors = validate_evolution(&changed, &lock);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("cannot change the locked root-cause contract"))
        );

        let backfill = active("L-LEX-0001");
        let backfilled = parse(&[&original_row, &backfill]);
        let errors = validate_evolution(&backfilled, &lock);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("does not advance LEX high-water mark"))
        );
    }

    #[test]
    fn lock_evolution_allows_only_new_optional_payload_fields() {
        let original_row = active("L-LEX-0001");
        let original = parse(&[&original_row]);
        let lock = toml::from_str(&render_lock(&original)).expect("valid generated lock");

        let optional = original_row.replace("name:string", "name:string, detail?:string");
        assert!(validate_evolution(&parse(&[&optional]), &lock).is_empty());

        let required = original_row.replace("name:string", "name:string, detail:string");
        let errors = validate_evolution(&parse(&[&required]), &lock);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("must be optional"))
        );
    }

    #[test]
    fn lock_evolution_allows_multiple_codes_in_a_new_domain() {
        let original_row = active("L-LEX-0001");
        let original = parse(&[&original_row]);
        let lock = toml::from_str(&render_lock(&original)).expect("valid generated lock");

        let first = active("L-PROJECT-0001");
        let second = active("L-PROJECT-0002");
        let evolved = parse(&[&original_row, &first, &second]);

        assert!(validate_evolution(&evolved, &lock).is_empty());
    }

    #[test]
    fn lock_rendering_is_sorted_and_deterministic() {
        let first = active("L-TYPE-0001");
        let second = active("L-LEX-0001");
        let left = parse(&[&first, &second]);
        let right = parse(&[&second, &first]);
        assert_eq!(render_lock(&left), render_lock(&right));
        assert!(render_lock(&left).find("L-LEX-0001") < render_lock(&left).find("L-TYPE-0001"));
    }

    #[test]
    fn extracts_valid_and_malformed_public_code_candidates() {
        assert_eq!(
            code_candidates("L-LEX-0001 L-TYPE-12 ignore L-*"),
            ["L-LEX-0001", "L-TYPE-12"]
        );
        assert!(code_candidates("GAP-KERNEL-DEVICE-001").is_empty());
    }

    #[test]
    fn rejects_unregistered_public_error_and_retired_use() {
        let active = BTreeSet::from(["L-LEX-0001".to_owned()]);
        let retired = BTreeSet::from(["L-IMPL-0001".to_owned()]);
        let constants = BTreeSet::from(["L-LEX-0001".to_owned(), "L-TYPE-9999".to_owned()]);
        let occurrences = vec![
            ("tests/example.toml".to_owned(), "L-TYPE-9999".to_owned()),
            ("tests/retired.toml".to_owned(), "L-IMPL-0001".to_owned()),
        ];
        let mut errors = Vec::new();
        validate_code_sets(&active, &retired, &constants, &occurrences, &mut errors);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("unregistered public diagnostic code"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.contains("retired diagnostic code"))
        );
    }

    #[test]
    fn repository_registry_matches_constants_and_compatibility_lock() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_repository(root).expect("repository diagnostic registry is valid");
        assert_eq!(summary.active_count, 70);
        assert_eq!(summary.retired_count, 1);
        assert_eq!(summary.domain_count, 14);
        assert_eq!(summary.rust_constant_count, 70);
    }
}
