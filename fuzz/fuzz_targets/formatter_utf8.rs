#![no_main]

use libfuzzer_sys::fuzz_target;
use ling_format::{build_format_ir, format_core_with_disposition};
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;

fuzz_target!(|bytes: &[u8]| {
    if let Ok(source) = SourceFile::from_bytes(SourceId::new(0), "fuzz.ling", bytes.to_vec()) {
        let parsed = parse(&source);
        let first = build_format_ir(&source, &parsed);
        let second = build_format_ir(&source, &parsed);
        assert_eq!(first, second, "format IR projection must be deterministic");

        if let Ok(document) = first {
            let first_result = format_core_with_disposition(&document);
            let second_result = format_core_with_disposition(&document);
            assert_eq!(first_result.text(), second_result.text());
            assert_eq!(
                first_result.disposition(),
                second_result.disposition(),
                "formatter disposition must be deterministic"
            );
            std::hint::black_box(first_result);
        }
    }
});
