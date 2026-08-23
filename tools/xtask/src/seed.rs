use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const HELLO_SOURCE: &str = "examples/hello.ling";
const SEMANTIC_GOLDEN: &str = "schemas/semantic/0.1/canonical/hello.bin";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproduceSummary {
    pub surface_count: usize,
    pub process_count: usize,
    pub compared_byte_count: usize,
}

struct Surface<'a> {
    name: &'a str,
    arguments: &'a [&'a str],
    expected_stdout: ExpectedStdout,
}

enum ExpectedStdout {
    Empty,
    Exact(&'static [u8]),
    SemanticGolden,
    Audit,
}

pub fn reproduce(root: &Path) -> Result<ReproduceSummary, Vec<String>> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    build_cli(root, &cargo)?;
    let binary = ling_binary(root);
    if !binary.is_file() {
        return Err(vec![format!(
            "GOV-REPRO-0002: built Ling binary is missing at {}",
            binary.display()
        )]);
    }
    let semantic_golden = fs::read(root.join(SEMANTIC_GOLDEN)).map_err(|error| {
        vec![format!(
            "GOV-REPRO-0002: cannot read {SEMANTIC_GOLDEN}: {error}"
        )]
    })?;
    let surfaces = [
        Surface {
            name: "check",
            arguments: &["check", "--format", "json", HELLO_SOURCE],
            expected_stdout: ExpectedStdout::Empty,
        },
        Surface {
            name: "run",
            arguments: &["run", "--format", "json", HELLO_SOURCE],
            expected_stdout: ExpectedStdout::Exact("你好，零\n".as_bytes()),
        },
        Surface {
            name: "semantic",
            arguments: &["semantic", "--format", "json", HELLO_SOURCE],
            expected_stdout: ExpectedStdout::SemanticGolden,
        },
        Surface {
            name: "audit",
            arguments: &["audit", "--format", "json", HELLO_SOURCE],
            expected_stdout: ExpectedStdout::Audit,
        },
    ];

    let mut errors = Vec::new();
    let mut compared_byte_count = 0;
    for surface in &surfaces {
        let first = run_surface(root, &binary, surface);
        let second = run_surface(root, &binary, surface);
        match (first, second) {
            (Ok(first), Ok(second)) => {
                compared_byte_count += first.stdout.len();
                validate_pair(surface, &first, &second, &semantic_golden, &mut errors);
            }
            (Err(first), Err(second)) => {
                errors.push(first);
                errors.push(second);
            }
            (Err(error), _) | (_, Err(error)) => errors.push(error),
        }
    }

    finish(errors).map(|()| ReproduceSummary {
        surface_count: surfaces.len(),
        process_count: surfaces.len() * 2,
        compared_byte_count,
    })
}

fn build_cli(root: &Path, cargo: &OsString) -> Result<(), Vec<String>> {
    let output = Command::new(cargo)
        .current_dir(root)
        .args(["build", "--package", "ling-cli", "--locked", "--offline"])
        .output()
        .map_err(|error| vec![format!("GOV-REPRO-0002: cannot start cargo build: {error}")])?;
    if output.status.success() {
        Ok(())
    } else {
        Err(vec![format!(
            "GOV-REPRO-0003: cargo build failed with {}: {}",
            display_status(&output),
            String::from_utf8_lossy(&output.stderr).trim()
        )])
    }
}

fn ling_binary(root: &Path) -> PathBuf {
    let target_root = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                root.join(path)
            }
        })
        .unwrap_or_else(|| root.join("target"));
    let profile = match env::var_os("CARGO_BUILD_TARGET") {
        Some(target) => target_root.join(target).join("debug"),
        None => target_root.join("debug"),
    };
    profile.join(format!("ling{}", env::consts::EXE_SUFFIX))
}

fn run_surface(root: &Path, binary: &Path, surface: &Surface<'_>) -> Result<Output, String> {
    let output = Command::new(binary)
        .current_dir(root)
        .args(surface.arguments)
        .output()
        .map_err(|error| {
            format!(
                "GOV-REPRO-0002: cannot start {} process: {error}",
                surface.name
            )
        })?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(format!(
            "GOV-REPRO-0003: {} process failed with {}: {}",
            surface.name,
            display_status(&output),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn validate_pair(
    surface: &Surface<'_>,
    first: &Output,
    second: &Output,
    semantic_golden: &[u8],
    errors: &mut Vec<String>,
) {
    if !first.stderr.is_empty() || !second.stderr.is_empty() {
        errors.push(format!(
            "GOV-REPRO-0004: {} emitted stderr in a successful process",
            surface.name
        ));
    }
    if first.stdout != second.stdout {
        errors.push(format!(
            "GOV-REPRO-0001: {} output differs across two independent processes",
            surface.name
        ));
        return;
    }
    match surface.expected_stdout {
        ExpectedStdout::Empty if !first.stdout.is_empty() => errors.push(format!(
            "GOV-REPRO-0004: {} must produce empty stdout",
            surface.name
        )),
        ExpectedStdout::Exact(expected) if first.stdout != expected => errors.push(format!(
            "GOV-REPRO-0004: {} output differs from its expected Seed bytes",
            surface.name
        )),
        ExpectedStdout::SemanticGolden => {
            if first.stdout != semantic_golden {
                errors.push(
                    "GOV-REPRO-0001: semantic output differs from its canonical byte golden"
                        .to_owned(),
                );
            }
            match std::str::from_utf8(&first.stdout) {
                Ok(text) => {
                    if let Err(error) = ling_semantic::read_json(text) {
                        errors.push(format!(
                            "GOV-REPRO-0004: semantic output fails the real reader: {error}"
                        ));
                    }
                }
                Err(error) => errors.push(format!(
                    "GOV-REPRO-0004: semantic output is not UTF-8: {error}"
                )),
            }
        }
        ExpectedStdout::Audit => validate_audit_bytes(&first.stdout, errors),
        ExpectedStdout::Empty | ExpectedStdout::Exact(_) => {}
    }
}

fn validate_audit_bytes(bytes: &[u8], errors: &mut Vec<String>) {
    let supported_header = bytes.starts_with(b"audit ling.audit/0.1 {\n")
        || bytes.starts_with(b"audit ling.audit/0.2 {\n");
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF])
        || !supported_header
        || bytes.last() != Some(&b'\n')
        || bytes.ends_with(b"\n\n")
        || bytes.contains(&b'\r')
    {
        errors.push(
            "GOV-REPRO-0004: Audit output violates its BOM/LF/header canonical boundary".to_owned(),
        );
    }
}

fn display_status(output: &Output) -> String {
    output.status.code().map_or_else(
        || "termination by signal".to_owned(),
        |code| code.to_string(),
    )
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
    fn audit_boundary_accepts_only_one_lf_and_no_bom_or_cr() {
        let mut errors = Vec::new();
        validate_audit_bytes(b"audit ling.audit/0.1 {\n}\n", &mut errors);
        assert!(errors.is_empty());
        validate_audit_bytes(b"audit ling.audit/0.2 {\n}\n", &mut errors);
        assert!(errors.is_empty());

        for invalid in [
            b"\xef\xbb\xbfaudit ling.audit/0.1 {\n}\n".as_slice(),
            b"audit ling.audit/0.1 {\r\n}\r\n".as_slice(),
            b"audit ling.audit/0.1 {\n}\n\n".as_slice(),
        ] {
            let mut errors = Vec::new();
            validate_audit_bytes(invalid, &mut errors);
            assert!(!errors.is_empty());
        }
    }

    #[test]
    fn repository_semantic_golden_passes_the_real_reader() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("xtask is under tools/xtask");
        let bytes = fs::read(root.join(SEMANTIC_GOLDEN)).expect("semantic golden is readable");
        let text = std::str::from_utf8(&bytes).expect("semantic golden is UTF-8");
        ling_semantic::read_json(text).expect("semantic golden passes the real reader");
    }
}
