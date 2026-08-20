#![no_main]

use libfuzzer_sys::fuzz_target;
use ling_project::parse_manifest;

fuzz_target!(|bytes: &[u8]| {
    let first = parse_manifest("fuzz/first/ling.toml", bytes);
    let second = parse_manifest("fuzz/second/ling.toml", bytes);

    match (first, second) {
        (Ok(first), Ok(second)) => {
            assert_eq!(first, second);
            std::hint::black_box((first, second));
        }
        (Err(first), Err(second)) => {
            assert_eq!(first.code(), second.code());
            assert_eq!(first.span(), second.span());
            for error in [first, second] {
                if let Ok(rendered) = error.diagnostic().render_json() {
                    std::hint::black_box(rendered);
                }
            }
        }
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => {
            panic!("manifest decoding depends on the diagnostic source label")
        }
    }
});
