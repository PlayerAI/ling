use unicode_normalization::UnicodeNormalization;

pub(crate) const MAX_LOGICAL_NAME_BYTES: usize = 4_096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LogicalNameError {
    Empty,
    TooLong,
    ContainsNul,
    NotRelative,
    UriScheme,
    Backslash,
    NoncanonicalSegment,
    NotNfc,
}

impl LogicalNameError {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "logical_name_empty",
            Self::TooLong => "logical_name_too_long",
            Self::ContainsNul => "logical_name_contains_nul",
            Self::NotRelative => "logical_name_not_relative",
            Self::UriScheme => "logical_name_has_uri_scheme",
            Self::Backslash => "logical_name_uses_backslash",
            Self::NoncanonicalSegment => "logical_name_has_noncanonical_segment",
            Self::NotNfc => "logical_name_not_nfc",
        }
    }
}

pub(crate) fn validate_logical_name(value: &str) -> Result<(), LogicalNameError> {
    if value.is_empty() {
        return Err(LogicalNameError::Empty);
    }
    if value.len() > MAX_LOGICAL_NAME_BYTES {
        return Err(LogicalNameError::TooLong);
    }
    if value.contains('\0') {
        return Err(LogicalNameError::ContainsNul);
    }
    if value.starts_with('/') || value.starts_with('\\') || has_windows_drive_prefix(value) {
        return Err(LogicalNameError::NotRelative);
    }
    if has_uri_scheme(value) {
        return Err(LogicalNameError::UriScheme);
    }
    if value.contains('\\') {
        return Err(LogicalNameError::Backslash);
    }
    for segment in value.split('/') {
        if segment.is_empty() || matches!(segment, "." | "..") {
            return Err(LogicalNameError::NoncanonicalSegment);
        }
        if segment.nfc().ne(segment.chars()) {
            return Err(LogicalNameError::NotNfc);
        }
    }
    Ok(())
}

fn has_windows_drive_prefix(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

fn has_uri_scheme(value: &str) -> bool {
    let mut bytes = value.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() {
        return false;
    }
    for byte in bytes {
        if byte == b':' {
            return true;
        }
        if !(byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.')) {
            return false;
        }
    }
    false
}
