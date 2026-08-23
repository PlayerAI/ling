mod ci;
mod compiler_compatibility;
mod dap_status;
mod documentation_matrix;
mod error_codes;
mod examples_matrix;
mod fault;
mod fuzz;
mod g0;
mod gaps;
mod governance;
mod historical_corpus;
mod lifecycle;
mod lsp_discovery;
mod migration_readiness;
mod performance;
mod performance_matrix;
mod protocols;
mod rc0_freeze;
mod rc1_validation;
mod rc2_change_control;
mod rc3_verification;
mod schema;
mod security;
mod seed;
mod status;
mod support;
mod traceability;
mod tutorial_matrix;
mod v1_artifact_inventory;
mod zed_extension;
mod zed_matrix;

use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const EXIT_INVALID_USAGE: u8 = 2;
const EXIT_VALIDATION_FAILED: u8 = 1;

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let root = match repository_root() {
        Ok(root) => root,
        Err(error) => {
            eprintln!("GOV-AUTH-0011: {error}");
            return ExitCode::from(EXIT_VALIDATION_FAILED);
        }
    };

    match args.as_slice() {
        [area, command] if area == "ci" && command == "verify" => match ci::verify(&root) {
            Ok(summary) => {
                println!(
                    "G0 CI contract OK: {} named gates, {} commands, {} workspace-test hosts",
                    summary.gate_count, summary.command_count, summary.host_count
                );
                ExitCode::SUCCESS
            }
            Err(errors) => {
                for error in errors {
                    eprintln!("{error}");
                }
                ExitCode::from(EXIT_VALIDATION_FAILED)
            }
        },
        [area, command] if area == "dap" && command == "verify" => {
            match dap_status::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "DAP status inventory OK: {} surfaces ({} unavailable, {} future, {} partial, {} unsupported), {} audit files",
                        summary.surface_count,
                        summary.unavailable_count,
                        summary.future_count,
                        summary.partial_count,
                        summary.unsupported_count,
                        summary.audit_file_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "governance" && command == "check-all" => {
            match g0::check_governance(&root) {
                Ok(summary) => {
                    println!(
                        "G0 governance OK: {} checks; {} documents, {} gaps, {} lifecycle records, {} protocols, {} diagnostic codes",
                        summary.check_count,
                        summary.document_count,
                        summary.gap_count,
                        summary.lifecycle_count,
                        summary.protocol_count,
                        summary.diagnostic_code_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "governance" && command == "check-authority" => {
            match governance::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "authority index OK: {} documents ({} Accepted)",
                        summary.document_count, summary.accepted_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "governance" && command == "render-authority" => {
            match governance::render_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "governance" && command == "check-gaps" => {
            match gaps::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "gap register OK: {} gaps ({} Open), {} gates",
                        summary.gap_count, summary.open_count, summary.gate_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "governance" && command == "render-gaps" => {
            match gaps::render_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "governance" && command == "check-lifecycle" => {
            match lifecycle::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "lifecycle registry OK: {} records ({} Accepted, {} legacy format)",
                        summary.record_count, summary.accepted_count, summary.legacy_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "governance" && command == "render-lifecycle" => {
            match lifecycle::render_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "governance" && command == "check-protocols" => {
            match protocols::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "protocol inventory OK: {} records ({} public: {} Experimental, {} Preview, {} Stable; {} Internal; {} Future)",
                        summary.protocol_count,
                        summary.public_count,
                        summary.experimental_count,
                        summary.preview_count,
                        summary.stable_count,
                        summary.internal_count,
                        summary.future_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "governance" && command == "render-protocols" => {
            match protocols::render_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "governance" && command == "check-error-codes" => {
            match error_codes::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "diagnostic registry OK: {} active, {} retired across {} domains ({} Rust constants)",
                        summary.active_count,
                        summary.retired_count,
                        summary.domain_count,
                        summary.rust_constant_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "governance" && command == "render-error-code-lock" => {
            match error_codes::render_lock_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command, release_flag, release]
            if area == "traceability" && command == "verify" && release_flag == "--release" =>
        {
            match traceability::check_repository(&root, release) {
                Ok(summary) => {
                    println!(
                        "traceability OK for {release}: {} features, {} conformance fixtures, {} total evidence records ({} differential paths deferred)",
                        summary.feature_count,
                        summary.fixture_count,
                        summary.evidence_count,
                        summary.deferred_differential_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command, release_flag, release]
            if area == "traceability" && command == "render" && release_flag == "--release" =>
        {
            match traceability::render_repository(&root, release) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "docs" && command == "verify" => {
            match documentation_matrix::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "documentation inventory OK: {} manuals ({} Future / Unsupported)",
                        summary.manual_count, summary.future_unsupported_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "corpus" && command == "verify" => {
            match historical_corpus::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "Seed corpus freeze OK: {} cases, {} files, {} surfaces, SHA-256 {}",
                        summary.case_count,
                        summary.file_count,
                        summary.surface_count,
                        summary.sha256
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "corpus" && command == "render" => {
            match historical_corpus::render_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "compatibility" && command == "verify" => {
            match compiler_compatibility::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "compiler compatibility boundary OK: {} releases ({} accept unchanged, {} unreleased), {} general N-1 edges",
                        summary.release_count,
                        summary.accepted_unchanged_count,
                        summary.unreleased_count,
                        summary.verified_n_minus_one_edges
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "compatibility" && command == "render" => {
            match compiler_compatibility::render_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "migration" && command == "verify" => {
            match migration_readiness::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "migration readiness OK: {} requirements ({} unavailable), {} released source version",
                        summary.requirement_count,
                        summary.unavailable_count,
                        summary.released_source_versions
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "migration" && command == "render" => {
            match migration_readiness::render_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "examples" && command == "verify" => {
            match examples_matrix::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "example matrix OK: {} two-layer requirements, {} feature traceability rows",
                        summary.requirement_count, summary.feature_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "tutorial" && command == "verify" => {
            match tutorial_matrix::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "tutorial matrix OK: {} bilingual sources, {} requirements",
                        summary.source_count, summary.requirement_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "lsp" && command == "verify" => {
            match lsp_discovery::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "LSP discovery inventory OK: {} priority sources ({} unavailable, {} not established)",
                        summary.priority_count,
                        summary.unavailable_count,
                        summary.not_established_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "rc0" && command == "verify" => {
            match rc0_freeze::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "RC0 inventory OK: {} criteria ({} BlockedSpec), {} audit files",
                        summary.criterion_count, summary.blocked_count, summary.audit_file_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "rc1" && command == "verify" => {
            match rc1_validation::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "RC1 inventory OK: {} criteria ({} BlockedSpec, {} Unsupported, {} partial), {} audit files",
                        summary.criterion_count,
                        summary.blocked_count,
                        summary.unsupported_count,
                        summary.partial_count,
                        summary.audit_file_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "rc2" && command == "verify" => {
            match rc2_change_control::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "RC2 inventory OK: {} evidence classes ({} BlockedSpec, {} partial), {} audit files",
                        summary.evidence_class_count,
                        summary.blocked_count,
                        summary.partial_count,
                        summary.audit_file_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "rc3" && command == "verify" => {
            match rc3_verification::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "RC3 inventory OK: {} checks ({} BlockedSpec, {} partial), {} audit files",
                        summary.check_count,
                        summary.blocked_count,
                        summary.partial_count,
                        summary.audit_file_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "v1" && command == "verify" => {
            match v1_artifact_inventory::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "v1.0 artifact inventory OK: {} items ({} partial, {} unavailable, {} unsupported, {} BlockedSpec), {} audit files",
                        summary.release_item_count,
                        summary.partial_count,
                        summary.unavailable_count,
                        summary.unsupported_count,
                        summary.blocked_count,
                        summary.audit_file_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "zed" && command == "verify" => {
            match zed_matrix::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "Zed compatibility matrix OK: {} surfaces, {} package evidence files",
                        summary.surface_count, summary.package_file_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "zed-extension" && command == "verify" => {
            match zed_extension::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "Zed extension acceptance inventory OK: {} areas ({} covered, {} partial, {} unsupported, {} future), {} evidence files",
                        summary.acceptance_count,
                        summary.covered_count,
                        summary.partial_count,
                        summary.unsupported_count,
                        summary.future_count,
                        summary.evidence_file_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "support" && command == "verify" => {
            match support::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "support matrix OK: {} features, {} profiles, {} hosts, {} native targets, {} backends, {} standard packages, {} protocols, {} explicit unsupported records",
                        summary.feature_count,
                        summary.profile_count,
                        summary.host_count,
                        summary.native_target_count,
                        summary.backend_count,
                        summary.standard_package_count,
                        summary.protocol_count,
                        summary.unsupported_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "support" && command == "render" => {
            match support::render_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "support" && command == "render-version-fixture" => {
            match support::render_version_fixture_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "support" && command == "render-support-fixture" => {
            match support::render_support_fixture_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "schema" && command == "validate-all" => {
            match schema::validate_all(&root) {
                Ok(summary) => {
                    println!(
                        "schema corpus OK: {} schemas, {} valid fixtures, {} invalid fixtures, {} canonical byte fixtures",
                        summary.schema_count,
                        summary.valid_fixture_count,
                        summary.invalid_fixture_count,
                        summary.canonical_fixture_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command, from_flag, from, to_flag, to]
            if area == "schema"
                && command == "compatibility"
                && from_flag == "--from"
                && to_flag == "--to" =>
        {
            match schema::compatibility(&root, from, to) {
                Ok(summary) => {
                    println!(
                        "schema compatibility OK: {} verified N-1 edges, {} NoPreviousVersion records",
                        summary.verified_edge_count, summary.no_previous_version_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "schema" && command == "corrupt-inputs" => {
            match schema::corrupt_inputs(&root) {
                Ok(summary) => {
                    println!(
                        "schema corrupt-input checks OK: {} deterministic mutations",
                        summary.mutation_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "seed" && command == "reproduce" => {
            match seed::reproduce(&root) {
                Ok(summary) => {
                    println!(
                        "Seed reproduction OK: {} surfaces, {} independent processes, {} compared output bytes",
                        summary.surface_count, summary.process_count, summary.compared_byte_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "performance" && command == "baseline" => {
            match performance::baseline() {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("PERF-BASELINE-0001: {error}");
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "performance" && command == "verify" => {
            match performance_matrix::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "performance matrix OK: {} measurements ({} Covered, {} Partial, {} Deferred)",
                        summary.measurement_count,
                        summary.covered_count,
                        summary.partial_count,
                        summary.deferred_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "fuzz" && command == "verify" => {
            match fuzz::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "fuzz inventory OK: {} targets, {} corpus files",
                        summary.target_count, summary.corpus_file_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "fault" && command == "verify" => {
            match fault::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "fault matrix OK: {} scenarios ({} Covered, {} Partial, {} Deferred)",
                        summary.scenario_count,
                        summary.covered_count,
                        summary.partial_count,
                        summary.deferred_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "security" && command == "verify" => {
            match security::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "security matrix OK: {} surfaces ({} Covered, {} Partial, {} Deferred)",
                        summary.surface_count,
                        summary.covered_count,
                        summary.partial_count,
                        summary.deferred_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "status" && command == "verify" => {
            match status::check_repository(&root) {
                Ok(summary) => {
                    println!(
                        "implementation status OK: {} tasks ({} Done), {} features ({} with stabilization blockers)",
                        summary.task_count,
                        summary.done_task_count,
                        summary.feature_count,
                        summary.blocked_feature_count
                    );
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "status" && command == "render" => {
            match status::render_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "status" && command == "render-release-notes" => {
            match status::render_release_notes_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        [area, command] if area == "status" && command == "render-cli-fixture" => {
            match status::render_cli_fixture_repository(&root) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                    ExitCode::from(EXIT_VALIDATION_FAILED)
                }
            }
        }
        _ => {
            eprintln!(
                "Usage:\n  cargo xtask ci verify\n  cargo xtask dap verify\n  cargo xtask governance check-all\n  cargo xtask governance check-authority\n  cargo xtask governance render-authority\n  cargo xtask governance check-gaps\n  cargo xtask governance render-gaps\n  cargo xtask governance check-lifecycle\n  cargo xtask governance render-lifecycle\n  cargo xtask governance check-protocols\n  cargo xtask governance render-protocols\n  cargo xtask governance check-error-codes\n  cargo xtask governance render-error-code-lock\n  cargo xtask traceability verify --release <release>\n  cargo xtask traceability render --release <release>\n  cargo xtask corpus verify\n  cargo xtask corpus render\n  cargo xtask compatibility verify\n  cargo xtask compatibility render\n  cargo xtask migration verify\n  cargo xtask migration render\n  cargo xtask docs verify\n  cargo xtask examples verify\n  cargo xtask tutorial verify\n  cargo xtask lsp verify\n  cargo xtask rc0 verify\n  cargo xtask rc1 verify\n  cargo xtask rc2 verify\n  cargo xtask rc3 verify\n  cargo xtask v1 verify\n  cargo xtask zed verify\n  cargo xtask zed-extension verify\n  cargo xtask support verify\n  cargo xtask support render\n  cargo xtask support render-version-fixture\n  cargo xtask support render-support-fixture\n  cargo xtask schema validate-all\n  cargo xtask schema compatibility --from N-1 --to N\n  cargo xtask schema corrupt-inputs\n  cargo xtask seed reproduce\n  cargo xtask performance baseline\n  cargo xtask performance verify\n  cargo xtask fuzz verify\n  cargo xtask fault verify\n  cargo xtask security verify\n  cargo xtask status verify\n  cargo xtask status render\n  cargo xtask status render-release-notes\n  cargo xtask status render-cli-fixture"
            );
            ExitCode::from(EXIT_INVALID_USAGE)
        }
    }
}

fn repository_root() -> Result<PathBuf, String> {
    let current =
        env::current_dir().map_err(|error| format!("cannot read current directory: {error}"))?;
    find_repository_root(&current)
        .ok_or_else(|| format!("cannot find a Cargo workspace above {}", current.display()))
}

fn find_repository_root(start: &Path) -> Option<PathBuf> {
    for candidate in start.ancestors() {
        let manifest = candidate.join("Cargo.toml");
        let Ok(text) = std::fs::read_to_string(manifest) else {
            continue;
        };
        if text.lines().any(|line| line.trim() == "[workspace]") {
            return Some(candidate.to_path_buf());
        }
    }
    None
}
