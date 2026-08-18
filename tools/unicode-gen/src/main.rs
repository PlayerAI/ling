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
    let output = arguments.next().map_or_else(
        || manifest_directory.join("../../crates/ling-unicode/src/generated.rs"),
        PathBuf::from,
    );
    if arguments.next().is_some() {
        return Err("usage: unicode-gen [data-directory] [output-file]".into());
    }

    let database = load_database(&data_directory)?;
    let generated = render(&database)?;
    fs::write(&output, generated)?;
    let rustfmt_status = Command::new("rustfmt")
        .args(["--edition", "2024"])
        .arg(&output)
        .status()?;
    if !rustfmt_status.success() {
        return Err(format!("rustfmt failed for {}", output.display()).into());
    }
    println!("generated {}", output.display());
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
    fn pinned_database_has_expected_unicode_17_shape() {
        let database = load_database(&Path::new(env!("CARGO_MANIFEST_DIR")).join("data"))
            .expect("pinned Unicode data must load");

        assert_eq!(database.confusables.len(), 6_565);
        assert!(database.scripts.len() > 900);
        assert!(database.script_extensions.len() > 100);
        assert!(!database.identifier_allowed.is_empty());
        assert!(!database.identifier_types.is_empty());
    }
}
