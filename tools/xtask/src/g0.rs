use std::path::Path;

use crate::{error_codes, gaps, governance, lifecycle, protocols};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceSummary {
    pub check_count: usize,
    pub document_count: usize,
    pub gap_count: usize,
    pub lifecycle_count: usize,
    pub protocol_count: usize,
    pub diagnostic_code_count: usize,
}

pub fn check_governance(root: &Path) -> Result<GovernanceSummary, Vec<String>> {
    let mut errors = Vec::new();
    let authority = collect(governance::check_repository(root), &mut errors);
    let gaps = collect(gaps::check_repository(root), &mut errors);
    let lifecycle = collect(lifecycle::check_repository(root), &mut errors);
    let protocols = collect(protocols::check_repository(root), &mut errors);
    let diagnostics = collect(error_codes::check_repository(root), &mut errors);

    finish(errors)?;
    let authority = authority.expect("successful aggregate has authority summary");
    let gaps = gaps.expect("successful aggregate has gap summary");
    let lifecycle = lifecycle.expect("successful aggregate has lifecycle summary");
    let protocols = protocols.expect("successful aggregate has protocol summary");
    let diagnostics = diagnostics.expect("successful aggregate has diagnostic summary");
    Ok(GovernanceSummary {
        check_count: 5,
        document_count: authority.document_count,
        gap_count: gaps.gap_count,
        lifecycle_count: lifecycle.record_count,
        protocol_count: protocols.protocol_count,
        diagnostic_code_count: diagnostics.active_count + diagnostics.retired_count,
    })
}

fn collect<T>(result: Result<T, Vec<String>>, errors: &mut Vec<String>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(mut failures) => {
            errors.append(&mut failures);
            None
        }
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
    use super::*;

    #[test]
    fn repository_passes_the_aggregate_governance_gate() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let summary = check_governance(root).expect("all governance registries are valid");
        assert_eq!(summary.check_count, 5);
        assert_eq!(summary.document_count, 71);
        assert_eq!(summary.gap_count, 28);
        assert_eq!(summary.lifecycle_count, 46);
        assert_eq!(summary.protocol_count, 25);
        assert_eq!(summary.diagnostic_code_count, 82);
    }
}
