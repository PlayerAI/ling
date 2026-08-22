use std::cmp::Ordering;

/// Internal canonical ordering key for a future LSP diagnostic projection.
///
/// The key preserves logical names and original UTF-8 byte offsets. It carries
/// no protocol, snapshot, severity, message, or suppression state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DiagnosticOrderKey {
    file: String,
    start_byte: u64,
    code: String,
    end_byte: u64,
    tie_breaker: u64,
}

impl DiagnosticOrderKey {
    #[must_use]
    pub(crate) fn new(
        file: impl Into<String>,
        start_byte: u64,
        end_byte: u64,
        code: impl Into<String>,
        tie_breaker: u64,
    ) -> Self {
        Self {
            file: file.into(),
            start_byte,
            code: code.into(),
            end_byte,
            tie_breaker,
        }
    }

    #[must_use]
    pub(crate) fn file(&self) -> &str {
        &self.file
    }

    #[must_use]
    pub(crate) const fn start_byte(&self) -> u64 {
        self.start_byte
    }

    #[must_use]
    pub(crate) const fn end_byte(&self) -> u64 {
        self.end_byte
    }

    #[must_use]
    pub(crate) fn code(&self) -> &str {
        &self.code
    }

    #[must_use]
    pub(crate) const fn tie_breaker(&self) -> u64 {
        self.tie_breaker
    }
}

impl Ord for DiagnosticOrderKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file
            .cmp(&other.file)
            .then_with(|| self.start_byte.cmp(&other.start_byte))
            .then_with(|| self.code.cmp(&other.code))
            .then_with(|| self.end_byte.cmp(&other.end_byte))
            .then_with(|| self.tie_breaker.cmp(&other.tie_breaker))
    }
}

impl PartialOrd for DiagnosticOrderKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_order_preserves_file_bytes_span_and_code() {
        let mut keys = [
            DiagnosticOrderKey::new("z.ling", 0, 1, "L-TYPE-0001", 0),
            DiagnosticOrderKey::new("a.ling", 8, 9, "L-TYPE-0001", 0),
            DiagnosticOrderKey::new("a.ling", 2, 4, "L-TYPE-0001", 0),
            DiagnosticOrderKey::new("a.ling", 2, 4, "L-LEX-0001", 0),
        ];
        keys.sort();
        assert_eq!(
            keys.iter().map(|key| key.file()).collect::<Vec<_>>(),
            vec!["a.ling", "a.ling", "a.ling", "z.ling"]
        );
        assert_eq!(keys[0].start_byte(), 2);
        assert_eq!(keys[0].code(), "L-LEX-0001");
        assert_eq!(keys[1].code(), "L-TYPE-0001");
        assert_eq!(keys[2].start_byte(), 8);
    }

    #[test]
    fn tie_breaker_distinguishes_equal_diagnostic_facts() {
        let first = DiagnosticOrderKey::new("凌.ling", 3, 7, "L-LEX-0001", 1);
        let second = DiagnosticOrderKey::new("凌.ling", 3, 7, "L-LEX-0001", 2);
        assert!(first < second);
        assert_eq!(first.end_byte(), 7);
        assert_eq!(second.tie_breaker(), 2);
    }

    #[test]
    fn repeated_sorting_is_deterministic_for_crlf_byte_offsets() {
        let input = vec![
            DiagnosticOrderKey::new("main.ling", 5, 7, "L-SYNTAX-0002", 0),
            DiagnosticOrderKey::new("main.ling", 5, 6, "L-SYNTAX-0001", 0),
            DiagnosticOrderKey::new("main.ling", 12, 13, "L-SYNTAX-0001", 0),
        ];
        let mut left = input.clone();
        let mut right = input;
        left.sort();
        right.sort();
        assert_eq!(left, right);
    }
}
