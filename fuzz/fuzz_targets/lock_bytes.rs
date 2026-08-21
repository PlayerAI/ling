#![no_main]

use libfuzzer_sys::fuzz_target;
use ling_project::parse_lock_file;

fuzz_target!(|bytes: &[u8]| {
    let first = parse_lock_file("fuzz/ling.lock", bytes);
    let second = parse_lock_file("fuzz/ling.lock", bytes);

    match (first, second) {
        (Ok(first), Ok(second)) => {
            assert_eq!(first, second, "lock models must be deterministic");
            assert_eq!(first.to_canonical_bytes(), second.to_canonical_bytes());
            std::hint::black_box(first);
        }
        (Err(first), Err(second)) => {
            assert_eq!(first.diagnostics().len(), second.diagnostics().len());
            for (first, second) in first.diagnostics().iter().zip(second.diagnostics()) {
                assert_eq!(first.code(), second.code());
                assert_eq!(first.primary_span(), second.primary_span());
                let first_json = first
                    .render_json()
                    .expect("registered lock diagnostics render");
                let second_json = second
                    .render_json()
                    .expect("registered lock diagnostics render");
                assert_eq!(first_json, second_json);
                assert!(first_json.len() < 16_384, "lock diagnostics must remain bounded");
                std::hint::black_box(first_json);
            }
        }
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => {
            panic!("lock decoding must not depend on repeated invocation")
        }
    }
});
