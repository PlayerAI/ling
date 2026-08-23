#![no_main]

use libfuzzer_sys::fuzz_target;
use ling_semantic::{read_json, read_project_json};

const MAX_INPUT_BYTES: usize = 1_048_576;

fuzz_target!(|bytes: &[u8]| {
    if bytes.len() > MAX_INPUT_BYTES {
        return;
    }
    let Ok(text) = std::str::from_utf8(bytes) else {
        return;
    };

    let seed_first = read_json(text);
    let seed_second = read_json(text);
    assert_eq!(
        seed_first, seed_second,
        "Seed Semantic Graph decoding must be deterministic"
    );

    let project_first = read_project_json(text);
    let project_second = read_project_json(text);
    assert_eq!(
        project_first, project_second,
        "project Semantic Graph decoding must be deterministic"
    );

    let _ = std::hint::black_box(seed_first);
    let _ = std::hint::black_box(project_first);
});
