mod gaps;
mod governance;
mod lifecycle;

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
        _ => {
            eprintln!(
                "Usage:\n  cargo xtask governance check-authority\n  cargo xtask governance render-authority\n  cargo xtask governance check-gaps\n  cargo xtask governance render-gaps\n  cargo xtask governance check-lifecycle\n  cargo xtask governance render-lifecycle"
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
