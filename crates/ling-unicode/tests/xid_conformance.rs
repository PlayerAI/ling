const DERIVED_CORE_PROPERTIES: &str =
    include_str!("../../../tools/unicode-gen/data/17.0.0/ucd/DerivedCoreProperties.txt");
const CODEPOINT_COUNT: usize = 0x11_0000;

#[test]
fn unicode_17_xid_start_and_continue_conformance() {
    let mut xid_start = vec![false; CODEPOINT_COUNT];
    let mut xid_continue = vec![false; CODEPOINT_COUNT];

    for (line_index, line) in DERIVED_CORE_PROPERTIES.lines().enumerate() {
        let data = line.split('#').next().unwrap_or_default().trim();
        if data.is_empty() {
            continue;
        }
        let Some((range, property)) = data.split_once(';') else {
            panic!("invalid DerivedCoreProperties.txt line {}", line_index + 1);
        };
        let table = match property.trim() {
            "XID_Start" => &mut xid_start,
            "XID_Continue" => &mut xid_continue,
            _ => continue,
        };
        let (start, end) = parse_range(range.trim(), line_index);
        table[start as usize..=end as usize].fill(true);
    }

    for codepoint in 0..CODEPOINT_COUNT as u32 {
        let Some(character) = char::from_u32(codepoint) else {
            continue;
        };
        assert_eq!(
            unicode_ident::is_xid_start(character),
            xid_start[codepoint as usize],
            "XID_Start mismatch for U+{codepoint:04X}"
        );
        assert_eq!(
            unicode_ident::is_xid_continue(character),
            xid_continue[codepoint as usize],
            "XID_Continue mismatch for U+{codepoint:04X}"
        );
    }
}

fn parse_range(input: &str, line_index: usize) -> (u32, u32) {
    let mut fields = input.split("..");
    let start = parse_codepoint(
        fields.next().expect("split always returns one field"),
        line_index,
    );
    let end = fields
        .next()
        .map_or(start, |value| parse_codepoint(value, line_index));
    assert!(
        fields.next().is_none() && start <= end,
        "invalid range at DerivedCoreProperties.txt line {}",
        line_index + 1
    );
    (start, end)
}

fn parse_codepoint(input: &str, line_index: usize) -> u32 {
    u32::from_str_radix(input, 16).unwrap_or_else(|error| {
        panic!(
            "invalid codepoint at DerivedCoreProperties.txt line {}: {error}",
            line_index + 1
        )
    })
}
