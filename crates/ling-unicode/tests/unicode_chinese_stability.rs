use ling_unicode::{
    ForbiddenProperty, IdentifierError, UNICODE_VERSION, confusable_skeleton, equal_name,
    inspect_identifier, is_identifier_continue, is_identifier_start, unicode_data_checksums,
    validate_identifier,
};

const EXPECTED_UNICODE_DATA: [(&str, &str); 11] = [
    (
        "17.0.0/security/confusables.txt",
        "091c7f82fc39ef208faf8f94d29c244de99254675e09de163160c810d13ef22a",
    ),
    (
        "17.0.0/security/IdentifierStatus.txt",
        "617228a16da13850bf8af28b6cd08f5e9b6595d2eb60404fe6eee2c85b4e4a35",
    ),
    (
        "17.0.0/security/IdentifierType.txt",
        "924ac63faa97ed73420d6ac48d08279d90968c7da0502ab701e08bfbb9683c22",
    ),
    (
        "17.0.0/ucd/DerivedCoreProperties.txt",
        "24c7fed1195c482faaefd5c1e7eb821c5ee1fb6de07ecdbaa64b56a99da22c08",
    ),
    (
        "17.0.0/ucd/DerivedGeneralCategory.txt",
        "d62e5bab70ca74f099343f71224fa051cb1fdd61a1ab45c0488c44cfc0b6102e",
    ),
    (
        "17.0.0/ucd/NormalizationTest.txt",
        "5019ffd530751a741900c849c0e010332f142a3612234639bd200b82138a87db",
    ),
    (
        "17.0.0/ucd/PropertyValueAliases.txt",
        "64e9a5f76f7a1e8b5a47d6a1f9a26522a251208f5276bdfa1559dac7cf2e827a",
    ),
    (
        "17.0.0/ucd/PropList.txt",
        "130dcddcaadaf071008bdfce1e7743e04fdfbc910886f017d9f9ac931d8c64dd",
    ),
    (
        "17.0.0/ucd/ScriptExtensions.txt",
        "ec2107e58825a1586acee8e0911ce18260394ac8b87e535ca325f1ccbeb06bc6",
    ),
    (
        "17.0.0/ucd/Scripts.txt",
        "9f5e50d3abaee7d6ce09480f325c706f485ae3240912527e651954d2d6b035bf",
    ),
    (
        "LICENSE-UNICODE.txt",
        "e7a93b009565cfce55919a381437ac4db883e9da2126fa28b91d12732bc53d96",
    ),
];

#[test]
fn unicode_17_version_and_data_manifest_are_exact() {
    assert_eq!(UNICODE_VERSION.major(), 17);
    assert_eq!(UNICODE_VERSION.minor(), 0);
    assert_eq!(UNICODE_VERSION.patch(), 0);
    assert_eq!(UNICODE_VERSION.to_string(), "17.0.0");
    assert_eq!(unicode_data_checksums(), EXPECTED_UNICODE_DATA.as_slice());
}

#[test]
fn chinese_xid_nfc_and_confusable_behavior_remains_observable() {
    assert!(is_identifier_start('人'));
    assert!(is_identifier_continue('物'));

    let chinese = inspect_identifier("人物ID").expect("Chinese identifier is valid");
    assert_eq!(chinese.identifier().original(), "人物ID");
    assert_eq!(chinese.identifier().normalized(), "人物ID");
    assert_eq!(
        chinese
            .scripts()
            .iter()
            .map(|script| script.as_str())
            .collect::<Vec<_>>(),
        ["Hani", "Latn"]
    );
    assert!(!chinese.has_suspicious_mixed_script());

    let decomposed = validate_identifier("e\u{301}").expect("decomposed identifier is valid");
    assert_eq!(decomposed.original(), "e\u{301}");
    assert_eq!(decomposed.normalized(), "é");
    assert!(decomposed.was_normalized());
    assert!(equal_name("e\u{301}", "é").expect("both names are valid"));

    assert_eq!(confusable_skeleton("paypal"), confusable_skeleton("pаypal"));
    assert!(
        inspect_identifier("pаypal")
            .expect("spoofing example remains structurally valid")
            .has_suspicious_mixed_script()
    );
}

#[test]
fn all_seed_forbidden_property_classes_are_rejected_at_original_byte_offsets() {
    let cases = [
        ('\u{200d}', ForbiddenProperty::JoinControl),
        ('\u{202e}', ForbiddenProperty::BidiControl),
        ('\u{fe0f}', ForbiddenProperty::VariationSelector),
        ('\u{034f}', ForbiddenProperty::DefaultIgnorable),
        ('\u{e000}', ForbiddenProperty::PrivateUse),
        ('\u{fdd0}', ForbiddenProperty::Noncharacter),
        ('\u{0378}', ForbiddenProperty::Unassigned),
        ('\u{0149}', ForbiddenProperty::Deprecated),
        ('+', ForbiddenProperty::PatternSyntax),
        ('\t', ForbiddenProperty::PatternWhiteSpace),
    ];

    for (character, property) in cases {
        let raw = format!("a{character}");
        assert_eq!(
            validate_identifier(&raw),
            Err(IdentifierError::Forbidden {
                character,
                byte_offset: 1,
                property,
            }),
            "U+{:04X}",
            u32::from(character)
        );
    }
}
