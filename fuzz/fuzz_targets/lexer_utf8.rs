#![no_main]

use libfuzzer_sys::fuzz_target;
use ling_source::{SourceFile, SourceId};
use ling_syntax::lex;

fuzz_target!(|bytes: &[u8]| {
    if std::str::from_utf8(bytes).is_err() {
        return;
    }
    if let Ok(source) = SourceFile::from_bytes(SourceId::new(0), "fuzz.ling", bytes.to_vec()) {
        let _ = lex(&source);
    }
});
