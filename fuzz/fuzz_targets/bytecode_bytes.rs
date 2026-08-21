#![no_main]

use std::borrow::Cow;

use libfuzzer_sys::fuzz_target;
use ling_bytecode::decode_and_verify_v1;

fuzz_target!(|input: &[u8]| {
    let bytes = decode_hex_seed(input).unwrap_or(Cow::Borrowed(input));
    let first = decode_and_verify_v1(&bytes);
    let second = decode_and_verify_v1(&bytes);
    assert_eq!(first, second, "bytecode verification must be deterministic");

    match first {
        Ok(program) => {
            std::hint::black_box(program);
        }
        Err(error) => {
            let rendered = error
                .to_diagnostic("fuzz/input.lbc")
                .render_json()
                .expect("registered bytecode diagnostics render");
            assert!(rendered.len() < 4096, "bytecode diagnostics stay bounded");
            std::hint::black_box(rendered);
        }
    }
});

fn decode_hex_seed(input: &[u8]) -> Option<Cow<'_, [u8]>> {
    let hex = input.strip_prefix(b"hex:")?;
    if hex.len() % 2 != 0 {
        return None;
    }
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(hex.len() / 2).ok()?;
    for pair in hex.chunks_exact(2) {
        let high = digit(pair[0])?;
        let low = digit(pair[1])?;
        bytes.push((high << 4) | low);
    }
    Some(Cow::Owned(bytes))
}

fn digit(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}
