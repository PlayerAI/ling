use std::cmp::Ordering;

use crate::generated;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CodepointRange {
    start: u32,
    end: u32,
}

impl CodepointRange {
    pub(super) const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    fn compare(self, codepoint: u32) -> Ordering {
        if self.end < codepoint {
            Ordering::Less
        } else if self.start > codepoint {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct NamedRange {
    range: CodepointRange,
    values: &'static [&'static str],
}

impl NamedRange {
    pub(super) const fn new(start: u32, end: u32, values: &'static [&'static str]) -> Self {
        Self {
            range: CodepointRange::new(start, end),
            values,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ConfusableMapping {
    source: u32,
    target: &'static [u32],
}

impl ConfusableMapping {
    pub(super) const fn new(source: u32, target: &'static [u32]) -> Self {
        Self { source, target }
    }
}

pub(super) fn contains(ranges: &[CodepointRange], character: char) -> bool {
    let codepoint = u32::from(character);
    ranges
        .binary_search_by(|range| range.compare(codepoint))
        .is_ok()
}

pub(super) fn scripts(character: char) -> &'static [&'static str] {
    let codepoint = u32::from(character);
    named_values(generated::SCRIPT_EXTENSIONS, codepoint)
        .or_else(|| named_values(generated::SCRIPTS, codepoint))
        .unwrap_or(&["Zzzz"])
}

pub(super) fn identifier_status_allowed(character: char) -> bool {
    contains(generated::IDENTIFIER_STATUS_ALLOWED, character)
}

pub(super) fn identifier_types(character: char) -> &'static [&'static str] {
    named_values(generated::IDENTIFIER_TYPES, u32::from(character)).unwrap_or(&[])
}

pub(super) fn confusable_target(character: char) -> Option<&'static [u32]> {
    generated::CONFUSABLES
        .binary_search_by_key(&u32::from(character), |mapping| mapping.source)
        .ok()
        .map(|index| generated::CONFUSABLES[index].target)
}

fn named_values(entries: &[NamedRange], codepoint: u32) -> Option<&'static [&'static str]> {
    entries
        .binary_search_by(|entry| entry.range.compare(codepoint))
        .ok()
        .map(|index| entries[index].values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_ranges_are_sorted_and_non_overlapping() {
        for ranges in [
            generated::BIDI_CONTROL,
            generated::DEFAULT_IGNORABLE_CODE_POINT,
            generated::DEPRECATED,
            generated::JOIN_CONTROL,
            generated::NONCHARACTER_CODE_POINT,
            generated::PATTERN_SYNTAX,
            generated::PATTERN_WHITE_SPACE,
            generated::PRIVATE_USE,
            generated::UNASSIGNED,
            generated::VARIATION_SELECTOR,
            generated::IDENTIFIER_STATUS_ALLOWED,
        ] {
            assert!(ranges.windows(2).all(|pair| pair[0].end < pair[1].start));
        }
    }

    #[test]
    fn generated_named_ranges_are_sorted_and_non_overlapping() {
        for entries in [
            generated::SCRIPTS,
            generated::SCRIPT_EXTENSIONS,
            generated::IDENTIFIER_TYPES,
        ] {
            assert!(
                entries
                    .windows(2)
                    .all(|pair| pair[0].range.end < pair[1].range.start)
            );
        }
    }
}
