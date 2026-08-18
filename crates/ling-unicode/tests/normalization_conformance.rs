use unicode_normalization::UnicodeNormalization;

const NORMALIZATION_TEST: &str =
    include_str!("../../../tools/unicode-gen/data/17.0.0/ucd/NormalizationTest.txt");

#[test]
fn unicode_17_nfc_conformance() {
    let mut cases = 0_u32;
    for (line_index, line) in NORMALIZATION_TEST.lines().enumerate() {
        let data = line.split('#').next().unwrap_or_default().trim();
        if data.is_empty() || data.starts_with('@') {
            continue;
        }

        let fields = data.split(';').map(str::trim).collect::<Vec<_>>();
        assert!(
            fields.len() >= 5,
            "invalid NormalizationTest.txt line {}",
            line_index + 1
        );
        let c1 = decode(fields[0], line_index);
        let c2 = decode(fields[1], line_index);
        let c3 = decode(fields[2], line_index);
        let c4 = decode(fields[3], line_index);
        let c5 = decode(fields[4], line_index);

        assert_eq!(nfc(&c1), c2, "c1 NFC at line {}", line_index + 1);
        assert_eq!(nfc(&c2), c2, "c2 NFC at line {}", line_index + 1);
        assert_eq!(nfc(&c3), c2, "c3 NFC at line {}", line_index + 1);
        assert_eq!(nfc(&c4), c4, "c4 NFC at line {}", line_index + 1);
        assert_eq!(nfc(&c5), c4, "c5 NFC at line {}", line_index + 1);
        cases += 1;
    }

    assert_eq!(
        cases, 20_034,
        "unexpected Unicode 17 normalization case count"
    );
}

fn decode(field: &str, line_index: usize) -> String {
    field
        .split_whitespace()
        .map(|codepoint| {
            let codepoint = u32::from_str_radix(codepoint, 16).unwrap_or_else(|error| {
                panic!(
                    "invalid codepoint at NormalizationTest.txt line {}: {error}",
                    line_index + 1
                )
            });
            char::from_u32(codepoint).unwrap_or_else(|| {
                panic!(
                    "non-scalar codepoint at NormalizationTest.txt line {}",
                    line_index + 1
                )
            })
        })
        .collect()
}

fn nfc(input: &str) -> String {
    input.nfc().collect()
}
