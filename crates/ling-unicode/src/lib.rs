//! Unicode 17 identifier validation, normalization, and security metadata for Ling.

mod generated;
mod tables;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use unicode_normalization::UnicodeNormalization;

/// The Unicode version fixed by the v0.0.1 language definition.
pub const UNICODE_VERSION: UnicodeVersion = UnicodeVersion::new(17, 0, 0);

const _: () = assert!(
    unicode_ident::UNICODE_VERSION.0 == UNICODE_VERSION.major
        && unicode_ident::UNICODE_VERSION.1 == UNICODE_VERSION.minor
        && unicode_ident::UNICODE_VERSION.2 == UNICODE_VERSION.patch
);
const _: () = assert!(
    unicode_normalization::UNICODE_VERSION.0 == UNICODE_VERSION.major
        && unicode_normalization::UNICODE_VERSION.1 == UNICODE_VERSION.minor
        && unicode_normalization::UNICODE_VERSION.2 == UNICODE_VERSION.patch
);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnicodeVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

impl UnicodeVersion {
    #[must_use]
    pub const fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    #[must_use]
    pub const fn major(self) -> u8 {
        self.major
    }

    #[must_use]
    pub const fn minor(self) -> u8 {
        self.minor
    }

    #[must_use]
    pub const fn patch(self) -> u8 {
        self.patch
    }
}

impl fmt::Display for UnicodeVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Returns the SHA-256 manifest for the pinned Unicode data used by this crate.
#[must_use]
pub const fn unicode_data_checksums() -> &'static [(&'static str, &'static str)] {
    generated::DATA_SHA256
}

/// Returns whether a scalar can begin a Ling identifier before security filtering.
#[must_use]
pub fn is_identifier_start(character: char) -> bool {
    character == '_' || unicode_ident::is_xid_start(character)
}

/// Returns whether a scalar can continue a Ling identifier before security filtering.
#[must_use]
pub fn is_identifier_continue(character: char) -> bool {
    unicode_ident::is_xid_continue(character)
}

/// Returns whether a scalar has the Unicode 17 `Bidi_Control` property.
#[must_use]
pub fn is_bidi_control(character: char) -> bool {
    tables::contains(generated::BIDI_CONTROL, character)
}

/// Returns whether a scalar has the Unicode 17 `Default_Ignorable_Code_Point` property.
#[must_use]
pub fn is_default_ignorable(character: char) -> bool {
    tables::contains(generated::DEFAULT_IGNORABLE_CODE_POINT, character)
}

/// Returns whether a scalar has the Unicode 17 `White_Space` property.
#[must_use]
pub fn is_white_space(character: char) -> bool {
    tables::contains(generated::WHITE_SPACE, character)
}

/// A validated identifier together with its source spelling and semantic name.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Identifier {
    original: String,
    normalized: String,
}

impl Identifier {
    #[must_use]
    pub fn original(&self) -> &str {
        &self.original
    }

    #[must_use]
    pub fn normalized(&self) -> &str {
        &self.normalized
    }

    #[must_use]
    pub fn was_normalized(&self) -> bool {
        self.original != self.normalized
    }
}

/// A four-letter ISO 15924 script code from Unicode Script/Script_Extensions.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Script(&'static str);

impl Script {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for Script {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

/// A UTS #39 Identifier_Type value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdentifierType(&'static str);

impl IdentifierType {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for IdentifierType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IdentifierStatus {
    Allowed,
    Restricted,
}

/// Security metadata computed from the same normalized identifier used for name equality.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentifierSecurity {
    identifier: Identifier,
    scripts: Box<[Script]>,
    skeleton: String,
    status: IdentifierStatus,
    identifier_types: Box<[IdentifierType]>,
    suspicious_mixed_script: bool,
}

impl IdentifierSecurity {
    #[must_use]
    pub const fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    #[must_use]
    pub fn scripts(&self) -> &[Script] {
        &self.scripts
    }

    #[must_use]
    pub fn skeleton(&self) -> &str {
        &self.skeleton
    }

    #[must_use]
    pub const fn status(&self) -> IdentifierStatus {
        self.status
    }

    #[must_use]
    pub fn identifier_types(&self) -> &[IdentifierType] {
        &self.identifier_types
    }

    #[must_use]
    pub const fn has_suspicious_mixed_script(&self) -> bool {
        self.suspicious_mixed_script
    }
}

/// Validates an identifier according to the v0.0.1 XID and forbidden-character rules.
pub fn validate_identifier(raw: &str) -> Result<Identifier, IdentifierError> {
    let mut characters = raw.char_indices();
    let Some((first_offset, first)) = characters.next() else {
        return Err(IdentifierError::Empty);
    };

    validate_character(first, first_offset)?;
    if !is_identifier_start(first) {
        return Err(IdentifierError::InvalidStart {
            character: first,
            byte_offset: first_offset,
        });
    }

    for (byte_offset, character) in characters {
        validate_character(character, byte_offset)?;
        if !is_identifier_continue(character) {
            return Err(IdentifierError::InvalidContinue {
                character,
                byte_offset,
            });
        }
    }

    let normalized: String = raw.nfc().collect();
    debug_assert!(is_valid_xid(&normalized));

    Ok(Identifier {
        original: raw.to_owned(),
        normalized,
    })
}

/// Computes Script Set, UTS #39 skeleton, Identifier_Status, and Identifier_Type.
pub fn inspect_identifier(raw: &str) -> Result<IdentifierSecurity, IdentifierError> {
    let identifier = validate_identifier(raw)?;
    let mut scripts = BTreeSet::new();
    let mut identifier_types = BTreeSet::new();
    let mut all_latin_is_ascii = true;
    let mut status = IdentifierStatus::Allowed;

    for character in identifier.normalized.chars() {
        let character_scripts = tables::scripts(character);
        for script in character_scripts {
            if !matches!(*script, "Zyyy" | "Zinh" | "Zzzz") {
                scripts.insert(*script);
                if *script == "Latn" && !character.is_ascii() {
                    all_latin_is_ascii = false;
                }
            }
        }
        for identifier_type in tables::identifier_types(character) {
            identifier_types.insert(*identifier_type);
        }
        if !tables::identifier_status_allowed(character) {
            status = IdentifierStatus::Restricted;
        }
    }

    let suspicious_mixed_script = suspicious_script_set(&scripts, all_latin_is_ascii);
    let scripts = scripts
        .into_iter()
        .map(Script)
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let identifier_types = identifier_types
        .into_iter()
        .map(IdentifierType)
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let skeleton = confusable_skeleton(&identifier.normalized);

    Ok(IdentifierSecurity {
        identifier,
        scripts,
        skeleton,
        status,
        identifier_types,
        suspicious_mixed_script,
    })
}

/// Computes the Unicode 17 UTS #39 confusable skeleton for diagnostic use.
#[must_use]
pub fn confusable_skeleton(input: &str) -> String {
    let mut mapped = String::new();
    for character in input.nfd() {
        if let Some(target) = tables::confusable_target(character) {
            for codepoint in target {
                mapped.push(
                    char::from_u32(*codepoint)
                        .expect("pinned UTS #39 data contains valid Unicode scalars"),
                );
            }
        } else {
            mapped.push(character);
        }
    }
    mapped.nfd().collect()
}

/// Compares valid identifiers by their case-sensitive NFC names.
pub fn equal_name(left: &str, right: &str) -> Result<bool, IdentifierError> {
    let left = validate_identifier(left)?;
    let right = validate_identifier(right)?;
    Ok(left.normalized == right.normalized)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ForbiddenProperty {
    JoinControl,
    BidiControl,
    VariationSelector,
    DefaultIgnorable,
    PrivateUse,
    Noncharacter,
    Unassigned,
    Deprecated,
    PatternSyntax,
    PatternWhiteSpace,
}

impl fmt::Display for ForbiddenProperty {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::JoinControl => "Join_Control",
            Self::BidiControl => "Bidi_Control",
            Self::VariationSelector => "Variation_Selector",
            Self::DefaultIgnorable => "Default_Ignorable_Code_Point",
            Self::PrivateUse => "Private_Use",
            Self::Noncharacter => "Noncharacter_Code_Point",
            Self::Unassigned => "Unassigned",
            Self::Deprecated => "Deprecated",
            Self::PatternSyntax => "Pattern_Syntax",
            Self::PatternWhiteSpace => "Pattern_White_Space",
        };
        formatter.write_str(name)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentifierError {
    Empty,
    InvalidStart {
        character: char,
        byte_offset: usize,
    },
    InvalidContinue {
        character: char,
        byte_offset: usize,
    },
    Forbidden {
        character: char,
        byte_offset: usize,
        property: ForbiddenProperty,
    },
}

impl fmt::Display for IdentifierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("identifier is empty"),
            Self::InvalidStart {
                character,
                byte_offset,
            } => write!(
                formatter,
                "character U+{:04X} `{character}` at byte {byte_offset} is not XID_Start",
                u32::from(*character)
            ),
            Self::InvalidContinue {
                character,
                byte_offset,
            } => write!(
                formatter,
                "character U+{:04X} `{character}` at byte {byte_offset} is not XID_Continue",
                u32::from(*character)
            ),
            Self::Forbidden {
                character,
                byte_offset,
                property,
            } => write!(
                formatter,
                "character U+{:04X} at byte {byte_offset} has forbidden property {property}",
                u32::from(*character)
            ),
        }
    }
}

impl Error for IdentifierError {}

fn validate_character(character: char, byte_offset: usize) -> Result<(), IdentifierError> {
    if let Some(property) = forbidden_property(character) {
        return Err(IdentifierError::Forbidden {
            character,
            byte_offset,
            property,
        });
    }
    Ok(())
}

fn is_valid_xid(identifier: &str) -> bool {
    let mut characters = identifier.chars();
    characters.next().is_some_and(is_identifier_start) && characters.all(is_identifier_continue)
}

fn forbidden_property(character: char) -> Option<ForbiddenProperty> {
    use generated as data;

    [
        (data::JOIN_CONTROL, ForbiddenProperty::JoinControl),
        (data::BIDI_CONTROL, ForbiddenProperty::BidiControl),
        (
            data::VARIATION_SELECTOR,
            ForbiddenProperty::VariationSelector,
        ),
        (
            data::DEFAULT_IGNORABLE_CODE_POINT,
            ForbiddenProperty::DefaultIgnorable,
        ),
        (data::PRIVATE_USE, ForbiddenProperty::PrivateUse),
        (
            data::NONCHARACTER_CODE_POINT,
            ForbiddenProperty::Noncharacter,
        ),
        (data::UNASSIGNED, ForbiddenProperty::Unassigned),
        (data::DEPRECATED, ForbiddenProperty::Deprecated),
        (data::PATTERN_SYNTAX, ForbiddenProperty::PatternSyntax),
        (
            data::PATTERN_WHITE_SPACE,
            ForbiddenProperty::PatternWhiteSpace,
        ),
    ]
    .into_iter()
    .find_map(|(ranges, property)| tables::contains(ranges, character).then_some(property))
}

fn suspicious_script_set(scripts: &BTreeSet<&str>, all_latin_is_ascii: bool) -> bool {
    if scripts.len() <= 1 {
        return false;
    }

    let mut effective = scripts.clone();
    if all_latin_is_ascii {
        effective.remove("Latn");
    }
    if effective.is_empty() {
        return false;
    }

    let japanese = effective
        .iter()
        .all(|script| matches!(*script, "Hani" | "Hira" | "Kana"));
    let korean = effective
        .iter()
        .all(|script| matches!(*script, "Hani" | "Hang"));
    let may_ignore_ascii_latin = effective
        .iter()
        .any(|script| matches!(*script, "Hani" | "Hira" | "Kana" | "Hang"));

    !(japanese || korean) || (scripts.contains("Latn") && !may_ignore_ascii_latin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_identifier_data_uses_unicode_17() {
        assert_eq!(unicode_ident::UNICODE_VERSION, (17, 0, 0));
        assert_eq!(unicode_normalization::UNICODE_VERSION, (17, 0, 0));
        assert_eq!(UNICODE_VERSION.to_string(), "17.0.0");
        assert_eq!(unicode_data_checksums().len(), 11);
    }

    #[test]
    fn exposes_pinned_display_name_security_properties() {
        assert!(is_bidi_control('\u{202e}'));
        assert!(!is_bidi_control('中'));
        assert!(is_default_ignorable('\u{200b}'));
        assert!(!is_default_ignorable('文'));
        assert!(is_white_space('\u{3000}'));
        assert!(!is_white_space('文'));
    }

    #[test]
    fn accepts_chinese_and_ascii_suffixes() {
        let security = inspect_identifier("人物ID").unwrap();

        assert_eq!(security.identifier().original(), "人物ID");
        assert_eq!(security.identifier().normalized(), "人物ID");
        assert_eq!(
            security
                .scripts()
                .iter()
                .map(|script| script.as_str())
                .collect::<Vec<_>>(),
            ["Hani", "Latn"]
        );
        assert!(!security.has_suspicious_mixed_script());
    }

    #[test]
    fn normalizes_names_to_nfc() {
        let identifier = validate_identifier("e\u{301}").unwrap();

        assert_eq!(identifier.normalized(), "é");
        assert!(identifier.was_normalized());
        assert!(equal_name("e\u{301}", "é").unwrap());
    }

    #[test]
    fn permits_a_single_underscore() {
        assert_eq!(validate_identifier("_").unwrap().normalized(), "_");
    }

    #[test]
    fn rejects_non_xid_characters() {
        assert!(matches!(
            validate_identifier("😀"),
            Err(IdentifierError::InvalidStart { .. })
        ));
    }

    #[test]
    fn rejects_join_controls_before_xid_classification() {
        assert_eq!(
            validate_identifier("a\u{200d}b").unwrap_err(),
            IdentifierError::Forbidden {
                character: '\u{200d}',
                byte_offset: 1,
                property: ForbiddenProperty::JoinControl
            }
        );
    }

    #[test]
    fn rejects_generated_default_ignorables() {
        assert_eq!(
            validate_identifier("a\u{034f}").unwrap_err(),
            IdentifierError::Forbidden {
                character: '\u{034f}',
                byte_offset: 1,
                property: ForbiddenProperty::DefaultIgnorable
            }
        );
    }

    #[test]
    fn confusable_skeleton_detects_latin_cyrillic_spoofing() {
        let latin = inspect_identifier("paypal").unwrap();
        let spoofed = inspect_identifier("pаypal").unwrap();

        assert_eq!(latin.skeleton(), spoofed.skeleton());
        assert!(spoofed.has_suspicious_mixed_script());
        assert_eq!(spoofed.status(), IdentifierStatus::Allowed);
    }

    #[test]
    fn japanese_and_korean_script_combinations_are_allowed() {
        assert!(
            !inspect_identifier("日本かなカナID")
                .unwrap()
                .has_suspicious_mixed_script()
        );
        assert!(
            !inspect_identifier("韓國한글ID")
                .unwrap()
                .has_suspicious_mixed_script()
        );
    }

    #[test]
    fn name_equality_is_case_sensitive() {
        assert!(!equal_name("Ling", "ling").unwrap());
    }
}
