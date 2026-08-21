#![no_main]

use libfuzzer_sys::fuzz_target;
use ling_format::{parse_audit, render_audit};

fuzz_target!(|bytes: &[u8]| {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return;
    };

    let first = parse_audit(text);
    let second = parse_audit(text);
    assert_eq!(first, second, "Audit schema decoding must be deterministic");

    match first {
        Ok(model) => {
            let rendered = render_audit(&model).expect("validated Audit models render");
            assert!(rendered.len() < 1_048_576, "Audit output must remain bounded");
            std::hint::black_box(rendered);
        }
        Err(error) => {
            let diagnostic = error.to_diagnostic("fuzz/model.audit");
            let rendered = diagnostic
                .render_json()
                .expect("registered Audit diagnostics render");
            assert!(rendered.len() < 16_384, "Audit diagnostics must remain bounded");
            std::hint::black_box(rendered);
        }
    }
});
