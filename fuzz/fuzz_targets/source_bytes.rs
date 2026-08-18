#![no_main]

use libfuzzer_sys::fuzz_target;
use ling_source::{SourceFile, SourceId};

fuzz_target!(|bytes: &[u8]| {
    let _ = SourceFile::from_bytes(SourceId::new(0), "fuzz.ling", bytes.to_vec());
});
