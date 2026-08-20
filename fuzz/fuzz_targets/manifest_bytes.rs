#![no_main]

use libfuzzer_sys::fuzz_target;
use ling_project::parse_manifest;

fuzz_target!(|bytes: &[u8]| {
    match parse_manifest("fuzz/ling.toml", bytes) {
        Ok(manifest) => {
            std::hint::black_box(manifest);
        }
        Err(error) => {
            if let Ok(rendered) = error.diagnostic().render_json() {
                std::hint::black_box(rendered);
            }
        }
    }
});
