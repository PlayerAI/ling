/// A read-only Unicode observation for a possible future rename candidate.
///
/// This value records the same identifier facts used by the lexer and name
/// resolver. It is deliberately not a rename validator: keyword, target,
/// visibility, collision, snapshot, and edit policy remain outside this
/// internal observation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RenameIdentifierStatus {
    Allowed,
    Restricted,
}

impl RenameIdentifierStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Restricted => "restricted",
        }
    }
}

/// Unicode 17.0.0 facts for one raw identifier spelling.
///
/// The original spelling is retained for auditability while `normalized` is
/// the NFC name used for equality. Script, Identifier_Type, skeleton, and
/// mixed-script facts are observations only; none of them authorizes a rename
/// or becomes a diagnostic or protocol field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenameIdentifierObservation {
    original: String,
    normalized: String,
    skeleton: String,
    scripts: Box<[String]>,
    identifier_types: Box<[String]>,
    status: RenameIdentifierStatus,
    suspicious_mixed_script: bool,
}

impl RenameIdentifierObservation {
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

    #[must_use]
    pub fn skeleton(&self) -> &str {
        &self.skeleton
    }

    #[must_use]
    pub fn scripts(&self) -> &[String] {
        &self.scripts
    }

    #[must_use]
    pub fn identifier_types(&self) -> &[String] {
        &self.identifier_types
    }

    #[must_use]
    pub const fn status(&self) -> RenameIdentifierStatus {
        self.status
    }

    #[must_use]
    pub const fn has_suspicious_mixed_script(&self) -> bool {
        self.suspicious_mixed_script
    }
}

/// Observes a raw rename candidate using the authoritative Unicode rules.
///
/// Invalid XID or forbidden-character input is returned as the existing
/// `ling-unicode` error. A successful result must not be interpreted as an
/// accepted new name: the unresolved prepare-rename contract still owns
/// keyword, target, alias, collision, visibility, snapshot, and edit policy.
pub fn observe_rename_identifier(
    raw: &str,
) -> Result<RenameIdentifierObservation, ling_unicode::IdentifierError> {
    let security = ling_unicode::inspect_identifier(raw)?;
    Ok(RenameIdentifierObservation {
        original: security.identifier().original().to_owned(),
        normalized: security.identifier().normalized().to_owned(),
        skeleton: security.skeleton().to_owned(),
        scripts: security
            .scripts()
            .iter()
            .map(|script| script.as_str().to_owned())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        identifier_types: security
            .identifier_types()
            .iter()
            .map(|identifier_type| identifier_type.as_str().to_owned())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        status: match security.status() {
            ling_unicode::IdentifierStatus::Allowed => RenameIdentifierStatus::Allowed,
            ling_unicode::IdentifierStatus::Restricted => RenameIdentifierStatus::Restricted,
        },
        suspicious_mixed_script: security.has_suspicious_mixed_script(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_original_and_nfc_name() {
        let observation = observe_rename_identifier("e\u{301}").unwrap();

        assert_eq!(observation.original(), "e\u{301}");
        assert_eq!(observation.normalized(), "é");
        assert!(observation.was_normalized());
        assert_eq!(observation.status(), RenameIdentifierStatus::Allowed);
    }

    #[test]
    fn records_security_facts_without_rejecting_them() {
        let observation = observe_rename_identifier("pаypal").unwrap();

        assert_eq!(observation.original(), "pаypal");
        assert_eq!(observation.normalized(), "pаypal");
        assert_eq!(observation.skeleton(), "paypal");
        assert!(observation.has_suspicious_mixed_script());
        assert_eq!(observation.status(), RenameIdentifierStatus::Allowed);
        assert_eq!(observation.scripts(), ["Cyrl", "Latn"]);
    }

    #[test]
    fn rejects_invalid_identifier_using_authoritative_error() {
        assert!(matches!(
            observe_rename_identifier("9lives"),
            Err(ling_unicode::IdentifierError::InvalidStart {
                character: '9',
                byte_offset: 0,
            })
        ));
    }

    #[test]
    fn repeated_observation_is_deterministic() {
        let first = observe_rename_identifier("人物ID").unwrap();
        let second = observe_rename_identifier("人物ID").unwrap();

        assert_eq!(first, second);
        assert_eq!(first.scripts(), ["Hani", "Latn"]);
    }
}
