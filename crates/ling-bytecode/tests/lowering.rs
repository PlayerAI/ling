use ling_bytecode::{
    CaptureOperand, Constant, EncodingErrorKind, FunctionKind, Instruction, LoweringErrorKind,
    LoweringSource, decode_and_verify_v1_1, disassemble_v1, disassemble_v1_1, encode_v1,
    encode_v1_1, encode_v1_with_limit, lower_v1, lower_v1_1,
};
use ling_semantic::ProgramSnapshot;
use ling_source::{SourceFile, SourceId};
use sha2::{Digest, Sha256};

const DIRECT_CALL: &str = include_str!("../../../tests/bytecode/v1/programs/direct-call.ling");
const HELLO: &str = include_str!("../../../tests/bytecode/v1/programs/hello.ling");
const HELLO_DISASSEMBLY: &str = include_str!("../../../tests/bytecode/v1/golden/hello.dis");
const HELLO_HEX: &str = include_str!("../../../tests/bytecode/v1/golden/hello.lbc.hex");

fn snapshot(source: &SourceFile) -> ProgramSnapshot {
    let parsed = ling_syntax::parse(source);
    assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    let ast = ling_ast::lower(source, &parsed).expect("fixture has valid AST");
    let hir = ling_hir::lower(source.name(), &ast).expect("fixture has valid HIR");
    let resolved = ling_resolve::resolve(vec![hir], "Main").expect("fixture resolves");
    let typed = ling_types::check(resolved).expect("fixture type-checks");
    let checked = ling_effects::check(typed).expect("fixture passes Effect/Capability checks");
    ling_semantic::build(checked).expect("fixture produces a checked snapshot")
}

fn checked_source(display_name: &str, text: &str) -> (SourceFile, ProgramSnapshot) {
    let source = SourceFile::from_bytes(SourceId::new(0), display_name, text.as_bytes().to_vec())
        .expect("fixture is valid source");
    let snapshot = snapshot(&source);
    (source, snapshot)
}

fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

#[test]
fn hello_lowering_matches_exact_bytes_and_debug_disassembly() {
    let (source, snapshot) = checked_source("C:/checkout/examples/hello.ling", HELLO);
    let lowered = lower_v1(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("the first checked slice lowers");
    let bytes = encode_v1(&lowered).expect("the canonical model encodes");

    assert_eq!(hex(&bytes), HELLO_HEX.trim());
    assert_eq!(disassemble_v1(&lowered), HELLO_DISASSEMBLY);
    assert_eq!(&bytes[..8], b"LINGBC\0\0");
    assert_eq!(
        u64::from_le_bytes(bytes[32..40].try_into().unwrap()),
        u64::try_from(bytes.len()).unwrap()
    );
}

#[test]
fn lowering_ignores_physical_display_paths_and_is_byte_deterministic() {
    let (left_source, left_snapshot) = checked_source("C:/first/root/Main.ling", HELLO);
    let (right_source, right_snapshot) = checked_source("D:/other/root/Main.ling", HELLO);
    let left = lower_v1(
        &left_snapshot,
        &[LoweringSource::new(&left_source, "src/Main.ling")],
    )
    .unwrap();
    let right = lower_v1(
        &right_snapshot,
        &[LoweringSource::new(&right_source, "src/Main.ling")],
    )
    .unwrap();

    assert_eq!(encode_v1(&left).unwrap(), encode_v1(&right).unwrap());
    assert_eq!(disassemble_v1(&left), disassemble_v1(&right));
    assert!(!disassemble_v1(&left).contains("C:/"));
    assert!(!disassemble_v1(&left).contains("D:/"));
}

#[test]
fn source_metadata_and_maps_preserve_exact_bom_crlf_bytes() {
    let text = concat!(
        "\u{feff}module Main\r\n",
        "    requires Console.Write\r\n",
        "\r\n",
        "let main () = Console.write \"你好，零🙂\"\r\n",
    );
    let (source, snapshot) = checked_source("C:/checkout/Main.ling", text);
    let lowered = lower_v1(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("BOM and CRLF source lowers from original byte spans");
    let source_record = lowered
        .model()
        .sources()
        .first()
        .expect("one source record");
    let expected_digest: [u8; 32] = Sha256::digest(text.as_bytes()).into();

    assert_eq!(source_record.original_byte_length, text.len() as u64);
    assert_eq!(source_record.content_sha256.as_bytes(), &expected_digest);

    let expression = "Console.write \"你好，零🙂\"";
    let start = text.find(expression).expect("fixture contains expression") as u64;
    let end = start + expression.len() as u64;
    assert!(lowered.model().source_map().iter().any(|entry| {
        entry.span.start_byte() == start
            && entry.span.end_byte() == end
            && text.is_char_boundary(entry.span.start_byte() as usize)
            && text.is_char_boundary(entry.span.end_byte() as usize)
    }));
}

#[test]
fn direct_call_and_local_alias_lower_without_function_values() {
    let (source, snapshot) = checked_source("direct-call.ling", DIRECT_CALL);
    let lowered = lower_v1(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("monomorphic direct call lowers");
    let model = lowered.model();

    assert_eq!(model.functions().len(), 2);
    assert!(model.functions().iter().any(|function| {
        function.blocks.iter().any(|block| {
            block
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, Instruction::Call { .. }))
        })
    }));
    assert!(model.functions().iter().any(|function| {
        function.blocks.iter().any(|block| {
            block
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, Instruction::ConsoleWrite { .. }))
        })
    }));
}

#[test]
fn all_four_scalar_types_and_arbitrary_integers_lower() {
    let text = concat!(
        "module Main\n\n",
        "let integer () = 340282366920938463463374607431768211456\n",
        "let boolean () = true\n",
        "let text () = \"你好，零\"\n",
        "let main () = ()\n",
    );
    let (source, snapshot) = checked_source("scalars.ling", text);
    let lowered = lower_v1(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("the four scalar types lower");

    assert_eq!(lowered.model().types().len(), 4);
    assert!(
        lowered
            .model()
            .constants()
            .iter()
            .any(|constant| matches!(constant, Constant::Unit))
    );
    assert!(lowered.model().constants().iter().any(|constant| {
        matches!(
            constant,
            Constant::Int { magnitude, .. } if magnitude.len() > 16
        )
    }));
    assert!(
        lowered
            .model()
            .constants()
            .iter()
            .any(|constant| matches!(constant, Constant::Bool(true)))
    );
    assert!(
        lowered
            .model()
            .constants()
            .iter()
            .any(|constant| matches!(constant, Constant::Text(_)))
    );
}

#[test]
fn unsupported_checked_features_fail_atomically_with_original_span() {
    let text = "module Main\n\nlet floating = 1.5\nlet main () = ()\n";
    let (source, snapshot) = checked_source("unsupported.ling", text);
    let error = lower_v1(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect_err("Float64 is outside bytecode 1.0");

    assert_eq!(error.source_name(), Some("unsupported.ling"));
    let definition = "let floating = 1.5";
    let definition_start = text.find(definition).expect("fixture contains definition") as u32;
    assert_eq!(
        error
            .span()
            .map(|span| (span.start().get(), span.end().get())),
        Some((definition_start, definition_start + definition.len() as u32))
    );
    assert_eq!(
        error.kind(),
        &LoweringErrorKind::UnsupportedFeature {
            feature: "Float64".to_owned()
        }
    );
}

#[test]
fn physical_or_ambiguous_logical_source_names_are_rejected() {
    let (source, snapshot) = checked_source("display-only.ling", HELLO);
    let error = lower_v1(
        &snapshot,
        &[LoweringSource::new(&source, "C:/secret/Main.ling")],
    )
    .expect_err("physical paths cannot enter bytecode");

    assert!(matches!(
        error.kind(),
        LoweringErrorKind::InvalidSource { reason, .. } if reason == "logical_name_not_relative"
    ));

    for (logical_name, expected_reason) in [
        ("http:src/Main.ling", "logical_name_has_uri_scheme"),
        ("src/Cafe\u{301}.ling", "logical_name_not_nfc"),
        ("src/hidden\0name.ling", "logical_name_contains_nul"),
    ] {
        let error = lower_v1(&snapshot, &[LoweringSource::new(&source, logical_name)])
            .expect_err("noncanonical logical paths cannot enter bytecode");
        assert!(matches!(
            error.kind(),
            LoweringErrorKind::InvalidSource { reason, .. } if reason == expected_reason
        ));
    }
}

#[test]
fn lowering_rejects_a_source_snapshot_with_the_wrong_display_identity() {
    let (checked_source, snapshot) = checked_source("checked/Main.ling", HELLO);
    let replacement = SourceFile::from_bytes(
        checked_source.id(),
        "replacement/Main.ling",
        checked_source.original_text().as_bytes().to_vec(),
    )
    .expect("replacement is valid source");
    let error = lower_v1(
        &snapshot,
        &[LoweringSource::new(&replacement, "src/Main.ling")],
    )
    .expect_err("source identity must match the checked snapshot");

    assert!(matches!(
        error.kind(),
        LoweringErrorKind::InvalidSource { reason, .. }
            if reason == "source_display_name_mismatch"
    ));
}

#[test]
fn encoder_reports_a_structured_error_instead_of_panicking() {
    let (source, snapshot) = checked_source("hello.ling", HELLO);
    let lowered = lower_v1(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")]).unwrap();
    let error = encode_v1_with_limit(&lowered, 40)
        .expect_err("the configured artifact limit is smaller than Hello bytecode");
    assert!(matches!(
        error.kind(),
        EncodingErrorKind::ResourceLimit {
            resource,
            actual,
            maximum: 40,
        } if resource == "artifact_bytes" && *actual > 40
    ));
}

#[test]
fn v1_1_lowering_emits_lexical_capture_partial_application_and_indirect_call() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let select prefix value = value\n\n",
        "let main () =\n",
        "    let prefix = \"hello \"\n",
        "    let local value = Console.write prefix\n",
        "    let partial = select prefix\n",
        "    local (partial \"world\")\n",
    );
    let (source, snapshot) = checked_source("closures.ling", text);
    let lowered = lower_v1_1(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("the accepted first-class closure slice lowers");
    let model = lowered.model();

    assert!(model.functions().iter().any(|function| {
        function.kind == FunctionKind::ClosureBody && function.capture_count > 0
    }));
    assert!(model.functions().iter().any(|function| {
        function.blocks.iter().any(|block| {
            block.instructions.iter().any(|instruction| {
                matches!(instruction, Instruction::MakeClosure { captures, .. } if captures.iter().any(|capture| matches!(capture, CaptureOperand::Register(_))))
            })
        })
    }));
    assert!(model.functions().iter().any(|function| {
        function.blocks.iter().any(|block| {
            block
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, Instruction::CallClosure { .. }))
        })
    }));

    let bytes = encode_v1_1(&lowered).expect("the lowered 1.1 model encodes");
    let verified = decode_and_verify_v1_1(&bytes).expect("the encoded 1.1 model verifies");
    assert_eq!(
        encode_v1_1(&lowered).unwrap(),
        encode_v1_1(&lowered).unwrap()
    );
    assert!(disassemble_v1_1(&lowered).contains("kind=closure-body"));
    assert_eq!(verified.version().minor(), 1);
}

#[test]
fn v1_1_lowering_rejects_mutable_lexical_capture() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () =\n",
        "    let mutable prefix = \"hello \"\n",
        "    let local value = prefix\n",
        "    Console.write (local \"world\")\n",
    );
    let (source, snapshot) = checked_source("mutable-capture.ling", text);
    let error = lower_v1_1(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect_err("mutable captures are explicitly outside RFC-0015");
    assert!(matches!(
        error.kind(),
        LoweringErrorKind::UnsupportedFeature { feature }
            if feature == "mutable lexical capture" || feature == "mutable local binding"
    ));
}

#[test]
fn v1_1_lowering_emits_direct_and_local_self_recursion() {
    let text = concat!(
        "module Main\n\n",
        "let rec top () : Unit = top ()\n\n",
        "let main () =\n",
        "    let rec local () : Unit = local ()\n",
        "    local ()\n",
    );
    let (source, snapshot) = checked_source("recursion.ling", text);
    let lowered = lower_v1_1(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("top-level and local recursion are accepted by RFC-0015");
    let model = lowered.model();

    assert!(model.functions().iter().any(|function| {
        function.kind == FunctionKind::Named
            && function.capture_count == 0
            && function.blocks.iter().any(|block| {
                block
                    .instructions
                    .iter()
                    .any(|instruction| matches!(instruction, Instruction::Call { .. }))
            })
    }));
    assert!(model.functions().iter().any(|function| {
        function.kind == FunctionKind::ClosureBody
            && function.capture_count == 1
            && function.blocks.iter().any(|block| {
                block
                    .instructions
                    .iter()
                    .any(|instruction| matches!(instruction, Instruction::CallClosure { .. }))
            })
    }));
    assert!(model.functions().iter().any(|function| {
        function.blocks.iter().any(|block| {
            block.instructions.iter().any(|instruction| {
                matches!(
                    instruction,
                    Instruction::MakeClosure { captures, .. }
                        if captures == &[CaptureOperand::SelfReference]
                )
            })
        })
    }));

    let bytes = encode_v1_1(&lowered).expect("recursive 1.1 model encodes");
    decode_and_verify_v1_1(&bytes).expect("recursive 1.1 model verifies");
}

#[test]
fn v1_1_lowering_emits_a_returned_closure_value() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let factory prefix =\n",
        "    let local value = Console.write prefix\n",
        "    local\n\n",
        "let main () =\n",
        "    let callback = factory \"hello\"\n",
        "    callback \"world\"\n",
    );
    let (source, snapshot) = checked_source("returned-closure.ling", text);
    let lowered = lower_v1_1(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("a closure returned from a checked function lowers");
    assert!(lowered.model().functions().iter().any(|function| {
        function.kind == FunctionKind::ClosureBody && function.capture_count > 0
    }));
    assert!(lowered.model().functions().iter().any(|function| {
        function.blocks.iter().any(|block| {
            block
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, Instruction::CallClosure { .. }))
        })
    }));
    decode_and_verify_v1_1(&encode_v1_1(&lowered).unwrap())
        .expect("returned closure artifact verifies");
}

#[test]
fn v1_1_lowering_emits_higher_order_function_parameters() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let apply callback value: Unit =\n",
        "    let ignored: Unit = callback value\n",
        "    Console.write value\n\n",
        "let main () =\n",
        "    let local value = Console.write value\n",
        "    apply local \"hello\"\n",
    );
    let (source, snapshot) = checked_source("higher-order.ling", text);
    let lowered = lower_v1_1(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("a checked function-typed parameter lowers");
    assert!(lowered.model().functions().iter().any(|function| {
        function.blocks.iter().any(|block| {
            block
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, Instruction::CallClosure { .. }))
        })
    }));
    decode_and_verify_v1_1(&encode_v1_1(&lowered).unwrap())
        .expect("higher-order artifact verifies");
}
