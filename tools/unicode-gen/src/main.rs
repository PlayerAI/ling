use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use sha2::{Digest, Sha256};

const INPUTS: &[Input] = &[
    Input::new(
        "17.0.0/security/confusables.txt",
        "091c7f82fc39ef208faf8f94d29c244de99254675e09de163160c810d13ef22a",
    ),
    Input::new(
        "17.0.0/security/IdentifierStatus.txt",
        "617228a16da13850bf8af28b6cd08f5e9b6595d2eb60404fe6eee2c85b4e4a35",
    ),
    Input::new(
        "17.0.0/security/IdentifierType.txt",
        "924ac63faa97ed73420d6ac48d08279d90968c7da0502ab701e08bfbb9683c22",
    ),
    Input::new(
        "17.0.0/ucd/DerivedCoreProperties.txt",
        "24c7fed1195c482faaefd5c1e7eb821c5ee1fb6de07ecdbaa64b56a99da22c08",
    ),
    Input::new(
        "17.0.0/ucd/DerivedGeneralCategory.txt",
        "d62e5bab70ca74f099343f71224fa051cb1fdd61a1ab45c0488c44cfc0b6102e",
    ),
    Input::new(
        "17.0.0/ucd/NormalizationTest.txt",
        "5019ffd530751a741900c849c0e010332f142a3612234639bd200b82138a87db",
    ),
    Input::new(
        "17.0.0/ucd/PropertyValueAliases.txt",
        "64e9a5f76f7a1e8b5a47d6a1f9a26522a251208f5276bdfa1559dac7cf2e827a",
    ),
    Input::new(
        "17.0.0/ucd/PropList.txt",
        "130dcddcaadaf071008bdfce1e7743e04fdfbc910886f017d9f9ac931d8c64dd",
    ),
    Input::new(
        "17.0.0/ucd/ScriptExtensions.txt",
        "ec2107e58825a1586acee8e0911ce18260394ac8b87e535ca325f1ccbeb06bc6",
    ),
    Input::new(
        "17.0.0/ucd/Scripts.txt",
        "9f5e50d3abaee7d6ce09480f325c706f485ae3240912527e651954d2d6b035bf",
    ),
    Input::new(
        "LICENSE-UNICODE.txt",
        "e7a93b009565cfce55919a381437ac4db883e9da2126fa28b91d12732bc53d96",
    ),
];

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("unicode-gen: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut arguments = std::env::args_os().skip(1);
    let data_directory = arguments
        .next()
        .map_or_else(|| manifest_directory.join("data"), PathBuf::from);
    let rust_output = arguments.next().map_or_else(
        || manifest_directory.join("../../crates/ling-unicode/src/generated.rs"),
        PathBuf::from,
    );
    let tree_sitter_output = arguments.next().map_or_else(
        || {
            manifest_directory
                .join("../../editors/tree-sitter-ling/src/unicode-identifiers.generated.js")
        },
        PathBuf::from,
    );
    if arguments.next().is_some() {
        return Err(
            "usage: unicode-gen [data-directory] [rust-output-file] [tree-sitter-output-file]"
                .into(),
        );
    }

    let database = load_database(&data_directory)?;
    let rust_generated = render(&database)?;
    let tree_sitter_generated = render_tree_sitter(&database)?;
    fs::write(&rust_output, rust_generated)?;
    fs::write(&tree_sitter_output, tree_sitter_generated)?;
    let rustfmt_status = Command::new("rustfmt")
        .args(["--edition", "2024"])
        .arg(&rust_output)
        .status()?;
    if !rustfmt_status.success() {
        return Err(format!("rustfmt failed for {}", rust_output.display()).into());
    }
    println!("generated {}", rust_output.display());
    println!("generated {}", tree_sitter_output.display());
    Ok(())
}

#[derive(Clone, Copy)]
struct Input {
    relative_path: &'static str,
    sha256: &'static str,
}

impl Input {
    const fn new(relative_path: &'static str, sha256: &'static str) -> Self {
        Self {
            relative_path,
            sha256,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CodepointRange {
    start: u32,
    end: u32,
}

#[derive(Debug)]
struct PropertyEntry {
    range: CodepointRange,
    value: String,
}

#[derive(Debug)]
struct ScriptEntry {
    range: CodepointRange,
    scripts: Vec<String>,
}

#[derive(Debug)]
struct Confusable {
    source: u32,
    target: Vec<u32>,
}

#[derive(Debug)]
struct Database {
    properties: BTreeMap<String, Vec<CodepointRange>>,
    xid_start: Vec<CodepointRange>,
    xid_continue: Vec<CodepointRange>,
    scripts: Vec<ScriptEntry>,
    script_extensions: Vec<ScriptEntry>,
    confusables: Vec<Confusable>,
    identifier_allowed: Vec<CodepointRange>,
    identifier_types: Vec<ScriptEntry>,
}

fn load_database(root: &Path) -> Result<Database, Box<dyn Error>> {
    verify_inputs(root)?;

    let prop_list = parse_property_file(&read(root, "17.0.0/ucd/PropList.txt")?)?;
    let core_properties =
        parse_property_file(&read(root, "17.0.0/ucd/DerivedCoreProperties.txt")?)?;
    let general_categories =
        parse_property_file(&read(root, "17.0.0/ucd/DerivedGeneralCategory.txt")?)?;
    let aliases = parse_script_aliases(&read(root, "17.0.0/ucd/PropertyValueAliases.txt")?);

    let mut properties = BTreeMap::new();
    for property in [
        "Bidi_Control",
        "Join_Control",
        "Variation_Selector",
        "White_Space",
        "Pattern_Syntax",
        "Pattern_White_Space",
        "Deprecated",
        "Noncharacter_Code_Point",
    ] {
        properties.insert(property.to_owned(), select(&prop_list, property));
    }
    properties.insert(
        "Default_Ignorable_Code_Point".to_owned(),
        select(&core_properties, "Default_Ignorable_Code_Point"),
    );
    properties.insert("Private_Use".to_owned(), select(&general_categories, "Co"));
    properties.insert("Unassigned".to_owned(), select(&general_categories, "Cn"));

    let xid_start = select(&core_properties, "XID_Start");
    let xid_continue = select(&core_properties, "XID_Continue");
    validate_xid_ranges(&xid_start, &xid_continue)?;

    let mut scripts = parse_scripts(&read(root, "17.0.0/ucd/Scripts.txt")?, &aliases, false)?;
    scripts.sort_by_key(|entry| entry.range.start);
    let mut script_extensions = parse_scripts(
        &read(root, "17.0.0/ucd/ScriptExtensions.txt")?,
        &aliases,
        true,
    )?;
    script_extensions.sort_by_key(|entry| entry.range.start);
    let mut confusables = parse_confusables(&read(root, "17.0.0/security/confusables.txt")?)?;
    confusables.sort_by_key(|entry| entry.source);
    ensure_unique_confusable_sources(&confusables)?;

    let identifier_status =
        parse_property_file(&read(root, "17.0.0/security/IdentifierStatus.txt")?)?;
    let identifier_allowed = select(&identifier_status, "Allowed");
    let mut identifier_types =
        parse_named_ranges(&read(root, "17.0.0/security/IdentifierType.txt")?)?;
    identifier_types.sort_by_key(|entry| entry.range.start);

    Ok(Database {
        properties,
        xid_start,
        xid_continue,
        scripts,
        script_extensions,
        confusables,
        identifier_allowed,
        identifier_types,
    })
}

fn verify_inputs(root: &Path) -> Result<(), Box<dyn Error>> {
    for input in INPUTS {
        let bytes = fs::read(root.join(input.relative_path))?;
        let digest = Sha256::digest(bytes);
        let mut actual = String::with_capacity(digest.len() * 2);
        for byte in digest {
            write!(&mut actual, "{byte:02x}")?;
        }
        if actual != input.sha256 {
            return Err(format!(
                "SHA-256 mismatch for {}: expected {}, got {actual}",
                input.relative_path, input.sha256
            )
            .into());
        }
    }
    Ok(())
}

fn read(root: &Path, relative_path: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(root.join(relative_path))?)
}

fn parse_property_file(input: &str) -> Result<Vec<PropertyEntry>, Box<dyn Error>> {
    data_lines(input)
        .map(|line| {
            let mut fields = line.split(';').map(str::trim);
            let range = parse_range(required_field(&mut fields, "codepoint range")?)?;
            let value = required_field(&mut fields, "property value")?.to_owned();
            Ok(PropertyEntry { range, value })
        })
        .collect()
}

fn parse_script_aliases(input: &str) -> BTreeMap<String, String> {
    data_lines(input)
        .filter_map(|line| {
            let fields = line.split(';').map(str::trim).collect::<Vec<_>>();
            (fields.len() >= 3 && fields[0] == "sc")
                .then(|| (fields[2].to_owned(), fields[1].to_owned()))
        })
        .collect()
}

fn parse_scripts(
    input: &str,
    aliases: &BTreeMap<String, String>,
    values_are_short: bool,
) -> Result<Vec<ScriptEntry>, Box<dyn Error>> {
    data_lines(input)
        .map(|line| {
            let mut fields = line.split(';').map(str::trim);
            let range = parse_range(required_field(&mut fields, "script range")?)?;
            let values = required_field(&mut fields, "script value")?;
            let scripts = values
                .split_whitespace()
                .map(|value| {
                    if values_are_short {
                        Ok(value.to_owned())
                    } else {
                        aliases
                            .get(value)
                            .cloned()
                            .ok_or_else(|| format!("missing short script alias for {value}"))
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ScriptEntry { range, scripts })
        })
        .collect()
}

fn parse_confusables(input: &str) -> Result<Vec<Confusable>, Box<dyn Error>> {
    data_lines(input)
        .map(|line| {
            let mut fields = line.split(';').map(str::trim);
            let source = parse_codepoint(required_field(&mut fields, "confusable source")?)?;
            let target = required_field(&mut fields, "confusable target")?
                .split_whitespace()
                .map(parse_codepoint)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Confusable { source, target })
        })
        .collect()
}

fn parse_named_ranges(input: &str) -> Result<Vec<ScriptEntry>, Box<dyn Error>> {
    data_lines(input)
        .map(|line| {
            let mut fields = line.split(';').map(str::trim);
            let range = parse_range(required_field(&mut fields, "identifier type range")?)?;
            let scripts = required_field(&mut fields, "identifier type")?
                .split_whitespace()
                .map(str::to_owned)
                .collect();
            Ok(ScriptEntry { range, scripts })
        })
        .collect()
}

fn data_lines(input: &str) -> impl Iterator<Item = &str> {
    input.lines().filter_map(|line| {
        let line = line.split('#').next().unwrap_or_default().trim();
        (!line.is_empty()).then_some(line)
    })
}

fn required_field<'a>(
    fields: &mut impl Iterator<Item = &'a str>,
    name: &str,
) -> Result<&'a str, Box<dyn Error>> {
    fields
        .next()
        .filter(|field| !field.is_empty())
        .ok_or_else(|| format!("missing {name}").into())
}

fn parse_range(input: &str) -> Result<CodepointRange, Box<dyn Error>> {
    let mut bounds = input.split("..");
    let start = parse_codepoint(required_field(&mut bounds, "range start")?)?;
    let end = bounds.next().map_or(Ok(start), parse_codepoint)?;
    if bounds.next().is_some() || start > end {
        return Err(format!("invalid codepoint range {input}").into());
    }
    Ok(CodepointRange { start, end })
}

fn parse_codepoint(input: &str) -> Result<u32, Box<dyn Error>> {
    Ok(u32::from_str_radix(input, 16)?)
}

fn select(entries: &[PropertyEntry], value: &str) -> Vec<CodepointRange> {
    entries
        .iter()
        .filter(|entry| entry.value == value)
        .map(|entry| entry.range)
        .collect()
}

fn validate_xid_ranges(
    xid_start: &[CodepointRange],
    xid_continue: &[CodepointRange],
) -> Result<(), Box<dyn Error>> {
    validate_ordered_ranges("XID_Start", xid_start)?;
    validate_ordered_ranges("XID_Continue", xid_continue)?;
    if xid_start.is_empty() || xid_continue.is_empty() {
        return Err("pinned XID properties must not be empty".into());
    }
    if contains(xid_start, u32::from('_')) || !contains(xid_continue, u32::from('_')) {
        return Err("Unicode 17 XID underscore assumptions changed".into());
    }
    for range in xid_start {
        if !range_is_covered(xid_continue, *range) {
            return Err(format!(
                "XID_Start U+{:04X}..U+{:04X} is not covered by XID_Continue",
                range.start, range.end
            )
            .into());
        }
    }
    Ok(())
}

fn validate_ordered_ranges(
    property: &str,
    ranges: &[CodepointRange],
) -> Result<(), Box<dyn Error>> {
    for range in ranges {
        if range.start > range.end
            || char::from_u32(range.start).is_none()
            || char::from_u32(range.end).is_none()
            || (range.start <= 0xDFFF && range.end >= 0xD800)
        {
            return Err(format!(
                "{property} contains an invalid scalar range U+{:04X}..U+{:04X}",
                range.start, range.end
            )
            .into());
        }
    }
    for pair in ranges.windows(2) {
        if pair[0].end >= pair[1].start {
            return Err(format!(
                "{property} ranges overlap or are unordered at U+{:04X}",
                pair[1].start
            )
            .into());
        }
    }
    Ok(())
}

fn contains(ranges: &[CodepointRange], codepoint: u32) -> bool {
    ranges
        .binary_search_by(|range| {
            if codepoint < range.start {
                std::cmp::Ordering::Greater
            } else if codepoint > range.end {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

fn range_is_covered(ranges: &[CodepointRange], expected: CodepointRange) -> bool {
    ranges
        .iter()
        .any(|range| range.start <= expected.start && range.end >= expected.end)
}

fn ensure_unique_confusable_sources(entries: &[Confusable]) -> Result<(), Box<dyn Error>> {
    for pair in entries.windows(2) {
        if pair[0].source == pair[1].source {
            return Err(format!("duplicate confusable source U+{:04X}", pair[0].source).into());
        }
    }
    Ok(())
}

fn render(database: &Database) -> Result<String, std::fmt::Error> {
    let mut output = String::new();
    writeln!(
        output,
        "// @generated by tools/unicode-gen from pinned Unicode 17.0.0 data."
    )?;
    writeln!(output, "// Do not edit manually.\n")?;
    writeln!(
        output,
        "use super::tables::{{CodepointRange, ConfusableMapping, NamedRange}};\n"
    )?;

    writeln!(output, "pub(crate) const DATA_SHA256: &[(&str, &str)] = &[")?;
    for input in INPUTS {
        writeln!(
            output,
            "    ({:?}, {:?}),",
            input.relative_path, input.sha256
        )?;
    }
    writeln!(output, "];\n")?;

    for (property, ranges) in &database.properties {
        render_ranges(&mut output, &constant_name(property), ranges)?;
    }
    render_named_ranges(&mut output, "SCRIPTS", &database.scripts)?;
    render_named_ranges(
        &mut output,
        "SCRIPT_EXTENSIONS",
        &database.script_extensions,
    )?;
    render_ranges(
        &mut output,
        "IDENTIFIER_STATUS_ALLOWED",
        &database.identifier_allowed,
    )?;
    render_named_ranges(&mut output, "IDENTIFIER_TYPES", &database.identifier_types)?;

    writeln!(
        output,
        "pub(crate) const CONFUSABLES: &[ConfusableMapping] = &["
    )?;
    for entry in &database.confusables {
        write!(
            output,
            "    ConfusableMapping::new(0x{:X}, &[",
            entry.source
        )?;
        for target in &entry.target {
            write!(output, "0x{target:X}, ")?;
        }
        writeln!(output, "]),")?;
    }
    writeln!(output, "];\n")?;
    Ok(output)
}

fn render_tree_sitter(database: &Database) -> Result<String, std::fmt::Error> {
    let mut output = String::new();
    writeln!(
        output,
        "// @generated by tools/unicode-gen from pinned Unicode 17.0.0 data."
    )?;
    writeln!(output, "// Do not edit manually.\n")?;
    writeln!(output, "\"use strict\";\n")?;
    writeln!(output, "const UNICODE_VERSION = \"17.0.0\";")?;
    let derived_core_properties = INPUTS
        .iter()
        .find(|input| input.relative_path.ends_with("DerivedCoreProperties.txt"))
        .expect("DerivedCoreProperties.txt is a pinned input");
    writeln!(
        output,
        "const DERIVED_CORE_PROPERTIES_SHA256 = {:?};\n",
        derived_core_properties.sha256
    )?;
    render_javascript_ranges(&mut output, "XID_START_RANGES", &database.xid_start)?;
    render_javascript_ranges(&mut output, "XID_CONTINUE_RANGES", &database.xid_continue)?;
    writeln!(output, "module.exports = Object.freeze({{")?;
    writeln!(output, "  UNICODE_VERSION,")?;
    writeln!(output, "  DERIVED_CORE_PROPERTIES_SHA256,")?;
    writeln!(output, "  XID_START_RANGES,")?;
    writeln!(output, "  XID_CONTINUE_RANGES,")?;
    writeln!(output, "}});")?;
    Ok(output)
}

fn render_javascript_ranges(
    output: &mut String,
    name: &str,
    ranges: &[CodepointRange],
) -> Result<(), std::fmt::Error> {
    writeln!(output, "const {name} = Object.freeze([")?;
    for range in ranges {
        writeln!(output, "  [0x{:X}, 0x{:X}],", range.start, range.end)?;
    }
    writeln!(output, "]);\n")
}

fn render_ranges(
    output: &mut String,
    name: &str,
    ranges: &[CodepointRange],
) -> Result<(), std::fmt::Error> {
    writeln!(output, "pub(crate) const {name}: &[CodepointRange] = &[")?;
    for range in ranges {
        writeln!(
            output,
            "    CodepointRange::new(0x{:X}, 0x{:X}),",
            range.start, range.end
        )?;
    }
    writeln!(output, "];\n")
}

fn render_named_ranges(
    output: &mut String,
    name: &str,
    entries: &[ScriptEntry],
) -> Result<(), std::fmt::Error> {
    writeln!(output, "pub(crate) const {name}: &[NamedRange] = &[")?;
    for entry in entries {
        write!(
            output,
            "    NamedRange::new(0x{:X}, 0x{:X}, &[",
            entry.range.start, entry.range.end
        )?;
        for value in &entry.scripts {
            write!(output, "{value:?}, ")?;
        }
        writeln!(output, "]),")?;
    }
    writeln!(output, "];\n")
}

fn constant_name(property: &str) -> String {
    property.to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_singletons_and_ranges() {
        assert_eq!(
            parse_range("0041").unwrap(),
            CodepointRange {
                start: 0x41,
                end: 0x41
            }
        );
        assert_eq!(
            parse_range("0041..005A").unwrap(),
            CodepointRange {
                start: 0x41,
                end: 0x5a
            }
        );
    }

    #[test]
    fn rejects_ranges_that_cross_non_scalar_codepoints() {
        let ranges = [CodepointRange {
            start: 0xD7FF,
            end: 0xE000,
        }];

        assert!(validate_ordered_ranges("test", &ranges).is_err());
    }

    #[test]
    fn pinned_database_has_expected_unicode_17_shape() {
        let database = load_database(&Path::new(env!("CARGO_MANIFEST_DIR")).join("data"))
            .expect("pinned Unicode data must load");

        assert_eq!(database.confusables.len(), 6_565);
        assert_eq!(database.xid_start.len(), 779);
        assert_eq!(database.xid_continue.len(), 1_422);
        assert!(contains(&database.xid_start, u32::from('人')));
        assert!(contains(&database.xid_continue, u32::from('\u{301}')));
        assert!(!contains(&database.xid_start, u32::from('_')));
        assert!(contains(&database.xid_continue, u32::from('_')));
        assert!(database.scripts.len() > 900);
        assert!(database.script_extensions.len() > 100);
        assert!(!database.identifier_allowed.is_empty());
        assert!(!database.identifier_types.is_empty());
    }

    #[test]
    fn renders_tree_sitter_data_without_host_unicode_properties() {
        let database = load_database(&Path::new(env!("CARGO_MANIFEST_DIR")).join("data"))
            .expect("pinned Unicode data must load");
        let output = render_tree_sitter(&database).expect("JavaScript rendering succeeds");

        assert!(output.contains("const UNICODE_VERSION = \"17.0.0\";"));
        assert!(output.contains("const XID_START_RANGES = Object.freeze(["));
        assert!(output.contains("[0x31350, 0x33479]"));
        assert!(!output.contains("\\p{"));
    }
}
