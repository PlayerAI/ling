use ling_bytecode::{
    BytecodeReason, CaptureOperand, Constant, EncodingErrorKind, FORMAT_VERSION_1_0,
    FORMAT_VERSION_1_1, FORMAT_VERSION_1_2, FORMAT_VERSION_1_3, FORMAT_VERSION_1_4, FunctionKind,
    Instruction, LoweringErrorKind, LoweringSource, decode_and_verify_v1_1, decode_and_verify_v1_2,
    decode_and_verify_v1_3, decode_and_verify_v1_4, disassemble_v1, disassemble_v1_1,
    disassemble_v1_2, disassemble_v1_3, encode_v1, encode_v1_1, encode_v1_2, encode_v1_3,
    encode_v1_4, encode_v1_with_limit, encode_verified_v1, lower_v1, lower_v1_1, lower_v1_2,
    lower_v1_3, lower_v1_4,
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

fn unique_bytes(bytes: &[u8], needle: &[u8]) -> usize {
    let positions = bytes
        .windows(needle.len())
        .enumerate()
        .filter_map(|(index, value)| (value == needle).then_some(index))
        .collect::<Vec<_>>();
    assert_eq!(positions.len(), 1, "byte pattern must be unique");
    positions[0]
}

fn overwrite_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
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
fn bytecode_1_3_alone_lowers_checked_handlers() {
    let text = concat!(
        "module Main\n\n",
        "let main () =\n",
        "    handle () with\n",
        "        operation Clock.now() -> ()\n",
    );
    let (source, snapshot) = checked_source("handler.ling", text);
    let sources = [LoweringSource::new(&source, "src/Main.ling")];
    let errors = [
        lower_v1(&snapshot, &sources).expect_err("bytecode 1.0 rejects Handler"),
        lower_v1_1(&snapshot, &sources).expect_err("bytecode 1.1 rejects Handler"),
        lower_v1_2(&snapshot, &sources).expect_err("bytecode 1.2 rejects Handler"),
    ];
    let start = u32::try_from(text.find("handle").expect("Handler token")).expect("span fits");
    for error in errors {
        assert_eq!(error.source_name(), Some("handler.ling"));
        assert_eq!(error.span().map(|span| span.start().get()), Some(start));
        assert_eq!(
            error.kind(),
            &LoweringErrorKind::UnsupportedFeature {
                feature: "handler".to_owned()
            }
        );
    }
    let lowered = lower_v1_3(&snapshot, &sources).expect("bytecode 1.3 lowers Handler");
    let handle = lowered
        .model()
        .functions()
        .iter()
        .flat_map(|function| &function.blocks)
        .flat_map(|block| &block.instructions)
        .find(|instruction| matches!(instruction, Instruction::Handle { .. }))
        .expect("one Handle instruction");
    assert_eq!(handle.opcode(), 0x1c);
    let bytes = encode_v1_3(&lowered).expect("bytecode 1.3 encodes");
    assert!(decode_and_verify_v1_2(&bytes).is_err(), "1.2 rejects 1.3");
    let verified = decode_and_verify_v1_3(&bytes).expect("bytecode 1.3 verifies");
    assert_eq!(verified.version().minor(), 3);
    assert!(disassemble_v1_3(&lowered).contains(" = handle "));
}

#[test]
fn bytecode_1_3_reader_accepts_every_earlier_minor_revision() {
    let (source, snapshot) = checked_source("compatibility.ling", HELLO);
    let sources = [LoweringSource::new(&source, "src/Main.ling")];
    let v1 = encode_v1(&lower_v1(&snapshot, &sources).expect("v1 lowers")).expect("v1 encodes");
    let v1_1 =
        encode_v1_1(&lower_v1_1(&snapshot, &sources).expect("v1.1 lowers")).expect("v1.1 encodes");
    let v1_2 =
        encode_v1_2(&lower_v1_2(&snapshot, &sources).expect("v1.2 lowers")).expect("v1.2 encodes");
    for (bytes, version) in [
        (v1, FORMAT_VERSION_1_0),
        (v1_1, FORMAT_VERSION_1_1),
        (v1_2, FORMAT_VERSION_1_2),
    ] {
        assert_eq!(
            decode_and_verify_v1_3(&bytes)
                .expect("1.3 reader accepts earlier revision")
                .version(),
            version
        );
    }
}

#[test]
fn bytecode_1_4_reader_accepts_every_earlier_minor_revision() {
    let (source, snapshot) = checked_source("compatibility-1.4.ling", HELLO);
    let sources = [LoweringSource::new(&source, "src/Main.ling")];
    let artifacts = [
        (
            encode_v1(&lower_v1(&snapshot, &sources).expect("v1 lowers")).expect("v1 encodes"),
            FORMAT_VERSION_1_0,
        ),
        (
            encode_v1_1(&lower_v1_1(&snapshot, &sources).expect("v1.1 lowers"))
                .expect("v1.1 encodes"),
            FORMAT_VERSION_1_1,
        ),
        (
            encode_v1_2(&lower_v1_2(&snapshot, &sources).expect("v1.2 lowers"))
                .expect("v1.2 encodes"),
            FORMAT_VERSION_1_2,
        ),
        (
            encode_v1_3(&lower_v1_3(&snapshot, &sources).expect("v1.3 lowers"))
                .expect("v1.3 encodes"),
            FORMAT_VERSION_1_3,
        ),
        (
            encode_v1_4(&lower_v1_4(&snapshot, &sources).expect("v1.4 lowers"))
                .expect("v1.4 encodes"),
            FORMAT_VERSION_1_4,
        ),
    ];
    for (bytes, version) in artifacts {
        let verified =
            decode_and_verify_v1_4(&bytes).expect("1.4 reader accepts every earlier revision");
        assert_eq!(verified.version(), version);
        assert_eq!(
            encode_verified_v1(&verified).expect("canonical re-encoding"),
            bytes
        );
    }
}

#[test]
fn bytecode_1_3_lowers_console_resume_and_lexical_captures() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () =\n",
        "    let suffix = \"captured\"\n",
        "    handle Console.write suffix with\n",
        "        operation Console.Write.write(message, resume) -> resume ()\n",
    );
    let (source, snapshot) = checked_source("handler-resume.ling", text);
    let lowered = lower_v1_3(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("bytecode 1.3 lowers captured resume Handler");
    let handle = lowered
        .model()
        .functions()
        .iter()
        .flat_map(|function| &function.blocks)
        .flat_map(|block| &block.instructions)
        .find_map(|instruction| match instruction {
            Instruction::Handle {
                body_captures,
                clauses,
                ..
            } => Some((body_captures, clauses)),
            _ => None,
        })
        .expect("Handle instruction exists");
    assert_eq!(handle.0.len(), 1);
    assert_eq!(handle.1.len(), 1);
    assert!(handle.1[0].resume_present);
    assert_eq!(handle.1[0].captures.len(), 0);
    let bytes = encode_v1_3(&lowered).expect("Handler artifact encodes");
    let verified = decode_and_verify_v1_3(&bytes).expect("Handler artifact verifies");
    assert_eq!(
        encode_verified_v1(&verified).expect("canonical re-encoding"),
        bytes
    );

    let (other_source, other_snapshot) = checked_source("D:/other/checkout/Main.ling", text);
    let other = lower_v1_3(
        &other_snapshot,
        &[LoweringSource::new(&other_source, "src/Main.ling")],
    )
    .expect("path-independent Handler lowers");
    assert_eq!(
        encode_v1_3(&other).expect("other Handler encodes"),
        bytes,
        "physical display paths cannot affect 1.3 bytes"
    );
}

#[test]
fn bytecode_1_3_handler_source_maps_preserve_bom_crlf_and_resume_identifier_spans() {
    let text = concat!(
        "\u{feff}module Main\r\n",
        "    requires Console.Write\r\n\r\n",
        "let main () =\r\n",
        "    handle Console.write \"你好🙂\" with\r\n",
        "        operation Console.Write.write(message, resume) -> resume ()\r\n",
    );
    let (source, snapshot) = checked_source("C:/checkout/处理.ling", text);
    let lowered = lower_v1_3(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("Unicode Handler lowers");
    let mut handle_location = None;
    let mut resume_location = None;
    for (function, definition) in lowered.model().functions().iter().enumerate() {
        for (block, body) in definition.blocks.iter().enumerate() {
            for (ordinal, instruction) in body.instructions.iter().enumerate() {
                let location = (function as u32, block as u32, ordinal as u32);
                match instruction {
                    Instruction::Handle { .. } => handle_location = Some(location),
                    Instruction::CallClosure { .. } => resume_location = Some(location),
                    _ => {}
                }
            }
        }
    }
    let mapped_span = |location: (u32, u32, u32)| {
        lowered
            .model()
            .source_map()
            .iter()
            .find(|entry| (entry.function.get(), entry.block.get(), entry.ordinal) == location)
            .expect("executable location is mapped")
            .span
    };
    let handle_span = mapped_span(handle_location.expect("Handle instruction"));
    assert_eq!(
        handle_span.start_byte(),
        text.find("handle").unwrap() as u64
    );
    assert!(text.is_char_boundary(handle_span.start_byte() as usize));
    assert!(text.is_char_boundary(handle_span.end_byte() as usize));

    let resume_span = mapped_span(resume_location.expect("resume CallClosure"));
    let resume_start = text.rfind("resume ()").unwrap() as u64;
    assert_eq!(resume_span.start_byte(), resume_start);
    assert_eq!(resume_span.end_byte(), resume_start + "resume".len() as u64);
}

#[test]
fn bytecode_1_3_rejects_malformed_handler_records_without_partial_publication() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () =\n",
        "    let suffix = \"captured\"\n",
        "    handle Console.write suffix with\n",
        "        operation Console.Write.write(message, resume) -> resume ()\n",
    );
    let (source, snapshot) = checked_source("handler-malformed.ling", text);
    let lowered = lower_v1_3(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("Handler lowers");
    let bytes = encode_v1_3(&lowered).expect("Handler encodes");
    let opcode = unique_bytes(&bytes, &[0x1c, 0, 0, 0]);
    let cases = [
        ("operation_tag", BytecodeReason::InvalidTag, {
            let mut corrupt = bytes.clone();
            corrupt[opcode + 28] = 0xff;
            corrupt
        }),
        ("resume_boolean", BytecodeReason::InvalidTag, {
            let mut corrupt = bytes.clone();
            corrupt[opcode + 29] = 2;
            corrupt
        }),
        ("clause_reserved", BytecodeReason::ReservedNonzero, {
            let mut corrupt = bytes.clone();
            corrupt[opcode + 30] = 1;
            corrupt
        }),
        (
            "body_capture_register",
            BytecodeReason::InvalidRegisterIndex,
            {
                let mut corrupt = bytes.clone();
                overwrite_u32(&mut corrupt, opcode + 20, u32::MAX);
                corrupt
            },
        ),
    ];
    for (name, reason, corrupt) in cases {
        let error = decode_and_verify_v1_3(&corrupt).expect_err(name);
        assert_eq!(error.reason(), reason, "{name}");
    }
    assert!(decode_and_verify_v1_3(&bytes).is_ok());
}

#[test]
fn bytecode_1_3_verifies_clause_order_uniqueness_signature_and_bounds() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () =\n",
        "    handle Console.write \"body\" with\n",
        "        operation Console.Write.write(message) -> ()\n",
        "        operation Clock.now() -> ()\n",
    );
    let (source, snapshot) = checked_source("handler-table.ling", text);
    let lowered = lower_v1_3(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("two-clause Handler lowers");
    let bytes = encode_v1_3(&lowered).expect("two-clause Handler encodes");
    let opcode = unique_bytes(&bytes, &[0x1c, 0, 0, 0]);
    assert_eq!(
        u32::from_le_bytes(bytes[opcode + 16..opcode + 20].try_into().unwrap()),
        2
    );

    let mut duplicate = bytes.clone();
    duplicate[opcode + 32] = 1;
    assert_eq!(
        decode_and_verify_v1_3(&duplicate)
            .expect_err("duplicate operation is rejected")
            .reason(),
        BytecodeReason::InvalidTableOrder
    );

    let mut wrong_signature = bytes.clone();
    let body_function = u32::from_le_bytes(bytes[opcode + 8..opcode + 12].try_into().unwrap());
    overwrite_u32(&mut wrong_signature, opcode + 24, body_function);
    assert_eq!(
        decode_and_verify_v1_3(&wrong_signature)
            .expect_err("clause signature is exact")
            .reason(),
        BytecodeReason::CallSignatureMismatch
    );

    let mut excessive = bytes.clone();
    overwrite_u32(&mut excessive, opcode + 16, u32::MAX);
    let excessive_error = decode_and_verify_v1_3(&excessive).expect_err("clause count is bounded");
    assert!(matches!(
        excessive_error.reason(),
        BytecodeReason::ResourceLimit | BytecodeReason::InvalidInstructionLength
    ));
}

#[test]
fn bytecode_1_3_rejects_unrepresentable_handler_state() {
    let mutable = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () =\n",
        "    let mutable cell = 0\n",
        "    handle Console.write \"body\" with\n",
        "        operation Console.Write.write(message, resume) ->\n",
        "            resume ()\n",
        "            if cell == 0 then () else ()\n",
    );
    let (source, snapshot) = checked_source("handler-state.ling", mutable);
    let error = lower_v1_3(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect_err("the accepted wire has no shared Cell capture representation");
    assert_eq!(
        error.kind(),
        &LoweringErrorKind::UnsupportedFeature {
            feature: "mutable Handler capture".to_owned()
        }
    );

    let lowered = lower_v1_4(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("bytecode 1.4 lowers the shared mutable Handler capture");
    assert!(
        lowered
            .model()
            .types()
            .iter()
            .any(|value| matches!(value, ling_bytecode::ValueType::Cell(_)))
    );
    assert!(lowered.model().functions().iter().any(|function| {
        function
            .effects
            .iter()
            .any(|effect| matches!(effect, ling_bytecode::Effect::State(_)))
    }));
    let bytes = encode_v1_4(&lowered).expect("shared mutable Handler encodes as 1.4");
    decode_and_verify_v1_4(&bytes).expect("shared mutable Handler verifies independently");
}

#[test]
fn bytecode_1_4_lowers_assignment_only_handler_capture_to_one_shared_cell() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let main () =\n",
        "    let mutable cell = 0\n",
        "    handle Console.write \"body\" with\n",
        "        operation Console.Write.write(message, resume) -> cell <- 1\n",
    );
    let (source, snapshot) = checked_source("handler-cell-set.ling", text);
    let lowered = lower_v1_4(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("assignment root alone is recognized as a mutable Handler capture");
    let instructions = lowered
        .model()
        .functions()
        .iter()
        .flat_map(|function| &function.blocks)
        .flat_map(|block| &block.instructions)
        .collect::<Vec<_>>();
    assert_eq!(
        instructions
            .iter()
            .filter(|instruction| matches!(instruction, Instruction::CellNew { .. }))
            .count(),
        1,
        "the lexical declaration owns exactly one Cell"
    );
    assert_eq!(
        instructions
            .iter()
            .filter(|instruction| matches!(instruction, Instruction::CellSet { .. }))
            .count(),
        1,
        "the clause assignment writes the captured Cell"
    );
    assert_eq!(
        instructions
            .iter()
            .filter(|instruction| matches!(instruction, Instruction::CellGet { .. }))
            .count(),
        0,
        "assignment does not introduce an unnecessary Cell read"
    );
    let bytes = encode_v1_4(&lowered).expect("CellSet Handler artifact encodes");
    let verified =
        decode_and_verify_v1_4(&bytes).expect("CellSet Handler artifact independently verifies");
    assert_eq!(
        encode_verified_v1(&verified).expect("verified CellSet artifact re-encodes"),
        bytes
    );

    let (other_source, other_snapshot) =
        checked_source("D:/other/root/handler-cell-set.ling", text);
    let other = lower_v1_4(
        &other_snapshot,
        &[LoweringSource::new(&other_source, "src/Main.ling")],
    )
    .expect("physical display path does not affect Cell lowering");
    assert_eq!(
        encode_v1_4(&other).expect("path-independent Cell artifact encodes"),
        bytes
    );
    assert_eq!(snapshot.program_id(), other_snapshot.program_id());
}

#[test]
fn bytecode_1_4_retains_aggregate_state_without_adding_a_capability() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "type Counter = { mutable value: Int }\n\n",
        "let main () =\n",
        "    let mutable counter = { value = 0 }\n",
        "    handle Console.write \"body\" with\n",
        "        operation Console.Write.write(message, resume) -> counter.value <- 1\n",
    );
    let (source, snapshot) = checked_source("handler-record-cell.ling", text);
    let lowered = lower_v1_4(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("aggregate mutable Handler capture lowers");
    let record = lowered
        .model()
        .types()
        .iter()
        .position(|value| matches!(value, ling_bytecode::ValueType::Record { .. }))
        .map(|index| ling_bytecode::TypeIndex::new(u32::try_from(index).expect("type index fits")))
        .expect("Counter record type exists");
    assert!(
        lowered
            .model()
            .types()
            .contains(&ling_bytecode::ValueType::Cell(record))
    );
    assert!(lowered.model().functions().iter().any(|function| {
        function
            .effects
            .contains(&ling_bytecode::Effect::State(record))
    }));
    assert_eq!(
        lowered
            .model()
            .modules()
            .iter()
            .flat_map(|module| module.capabilities.iter().copied())
            .collect::<Vec<_>>(),
        vec![ling_bytecode::Capability::ConsoleWrite],
        "State does not introduce a host Capability"
    );
    let bytes = encode_v1_4(&lowered).expect("aggregate State artifact encodes");
    decode_and_verify_v1_4(&bytes).expect("aggregate State artifact independently verifies");
}

#[test]
fn bytecode_1_4_retains_state_for_non_captured_mutable_bindings() {
    let text = concat!(
        "module Main\n\n",
        "let main () =\n",
        "    let mutable cell = 0\n",
        "    cell <- 7\n",
        "    ()\n",
    );
    let (source, snapshot) = checked_source("lexical-cell.ling", text);
    let sources = [LoweringSource::new(&source, "src/Main.ling")];
    let lowered = lower_v1_4(&snapshot, &sources)
        .expect("ordinary mutable lexical binding lowers through a version-1.4 Cell");
    let int = lowered
        .model()
        .types()
        .iter()
        .position(|value| value == &ling_bytecode::ValueType::Int)
        .map(|index| ling_bytecode::TypeIndex::new(u32::try_from(index).expect("type index fits")))
        .expect("Int type exists");
    assert!(
        lowered
            .model()
            .types()
            .contains(&ling_bytecode::ValueType::Cell(int))
    );
    let main = lowered
        .model()
        .functions()
        .iter()
        .find(|function| lowered.model().strings()[function.name.get() as usize] == "main")
        .expect("main function exists");
    assert_eq!(main.effects, [ling_bytecode::Effect::State(int)]);
    assert!(
        main.blocks
            .iter()
            .flat_map(|block| &block.instructions)
            .any(|instruction| matches!(instruction, ling_bytecode::Instruction::CellNew { .. }))
    );
    assert!(
        main.blocks
            .iter()
            .flat_map(|block| &block.instructions)
            .any(|instruction| matches!(instruction, ling_bytecode::Instruction::CellSet { .. }))
    );
    let bytes = encode_v1_4(&lowered).expect("ordinary lexical State artifact encodes");
    decode_and_verify_v1_4(&bytes).expect("ordinary lexical State artifact independently verifies");
}

#[test]
fn bytecode_1_4_preserves_effect_provenance_for_function_valued_cells() {
    let text = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let increment value = value + 1\n\n",
        "let main () =\n",
        "    let mutable callback = increment\n",
        "    handle Console.write \"body\" with\n",
        "        operation Console.Write.write(message, resume) ->\n",
        "            let ignored = callback 1\n",
        "            resume ()\n",
    );
    let (source, snapshot) = checked_source("handler-function-cell.ling", text);
    let lowered = lower_v1_4(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("function-valued mutable Handler capture lowers with callable Effect provenance");
    let function_type = lowered
        .model()
        .types()
        .iter()
        .find_map(|value| match value {
            ling_bytecode::ValueType::Cell(payload) => Some(*payload),
            _ => None,
        })
        .expect("Cell payload type exists");
    assert!(matches!(
        lowered.model().types()[usize::try_from(function_type.get()).expect("type index fits")],
        ling_bytecode::ValueType::Function { .. }
    ));
    assert!(lowered.model().functions().iter().any(|function| {
        function
            .effects
            .contains(&ling_bytecode::Effect::State(function_type))
    }));
    let bytes = encode_v1_4(&lowered).expect("function Cell artifact encodes");
    decode_and_verify_v1_4(&bytes).expect("function Cell artifact independently verifies");
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

#[test]
fn v1_2_lowering_emits_nominal_aggregates_and_round_trips() {
    let text = concat!(
        "module Main\n\n",
        "type Point = { x: Int; y: Int }\n",
        "type State =\n",
        "    | Idle\n",
        "    | Ready of Int\n\n",
        "let point = { y = 2; x = 1 }\n",
        "let projected = point.x\n",
        "let pair = (1, 2)\n",
        "let changed = { point with x = 3 }\n",
        "let state = Ready 5\n",
        "let main () = ()\n",
    );
    let (source, snapshot) = checked_source("aggregates.ling", text);
    let lowered = lower_v1_2(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("checked nominal aggregates lower to bytecode 1.2");
    let model = lowered.model();
    assert!(
        model
            .types()
            .iter()
            .any(|value| { matches!(value, ling_bytecode::ValueType::Record { .. }) })
    );
    assert!(
        model
            .types()
            .iter()
            .any(|value| { matches!(value, ling_bytecode::ValueType::Variant { .. }) })
    );
    assert!(
        model
            .types()
            .iter()
            .any(|value| { matches!(value, ling_bytecode::ValueType::Tuple { .. }) })
    );
    assert!(model.functions().iter().any(|function| {
        function.blocks.iter().any(|block| {
            block.instructions.iter().any(|instruction| {
                matches!(
                    instruction,
                    Instruction::MakeRecord { .. }
                        | Instruction::UpdateRecord { .. }
                        | Instruction::MakeVariant { .. }
                )
            })
        })
    }));

    let bytes = encode_v1_2(&lowered).expect("the aggregate artifact encodes");
    let verified = decode_and_verify_v1_2(&bytes).expect("the aggregate artifact verifies");
    assert_eq!(verified.version().minor(), 2);
    assert_eq!(encode_v1_2(&lowered).unwrap(), bytes);
    let disassembly = disassemble_v1_2(&lowered);
    assert!(disassembly.contains("make-record"));
    assert!(disassembly.contains("make-variant"));
}

#[test]
fn v1_2_lowering_emits_checked_variant_match_control_flow() {
    let text = concat!(
        "module Main\n\n",
        "type State =\n",
        "    | Idle\n",
        "    | Ready of Int\n\n",
        "let classify state =\n",
        "    match state with\n",
        "    | Ready value -> value\n",
        "    | Idle -> 0\n\n",
        "let main () = ()\n",
    );
    let (source, snapshot) = checked_source("variant-match.ling", text);
    let lowered = lower_v1_2(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("checked variant matches lower to control flow");
    assert!(lowered.model().functions().iter().any(|function| {
        function.blocks.len() > 1
            && function
                .blocks
                .iter()
                .any(|block| matches!(block.terminator, ling_bytecode::Terminator::Branch { .. }))
    }));
    let bytes = encode_v1_2(&lowered).expect("the match artifact encodes");
    decode_and_verify_v1_2(&bytes).expect("the match artifact verifies");
}

#[test]
fn v1_2_lowering_emits_guards_and_scalar_control_flow() {
    let text = concat!(
        "module Main\n\n",
        "let classify value =\n",
        "    match value with\n",
        "    | true when true -> if 1 < 2 then 1 else 0\n",
        "    | _ -> 0\n\n",
        "let main () = ()\n",
    );
    let (source, snapshot) = checked_source("guards.ling", text);
    let lowered = lower_v1_2(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("checked guards and scalar control flow lower");
    assert!(lowered.model().functions().iter().any(|function| {
        function.blocks.iter().any(|block| {
            block
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, Instruction::Compare { .. }))
        })
    }));
    decode_and_verify_v1_2(&encode_v1_2(&lowered).unwrap()).expect("the guarded artifact verifies");
}

#[test]
fn v1_2_lowering_emits_nested_tuple_payload_patterns() {
    let text = concat!(
        "module Main\n\n",
        "type PairBox =\n",
        "    | Pair of Int * Int\n",
        "    | Empty\n\n",
        "let sumPair value =\n",
        "    match value with\n",
        "    | Pair (left, right) -> left + right\n",
        "    | Empty -> 0\n\n",
        "let main () = ()\n",
    );
    let (source, snapshot) = checked_source("nested-match.ling", text);
    let lowered = lower_v1_2(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("nested tuple payload patterns lower");
    assert!(lowered.model().functions().iter().any(|function| {
        function.blocks.iter().any(|block| {
            block.instructions.iter().any(|instruction| {
                matches!(
                    instruction,
                    Instruction::GetVariantPayload { .. } | Instruction::GetTuple { .. }
                )
            })
        })
    }));
    decode_and_verify_v1_2(&encode_v1_2(&lowered).unwrap())
        .expect("the nested pattern artifact verifies");
}

#[test]
fn v1_2_lowering_emits_prelude_option_constructors_and_patterns() {
    let text = concat!(
        "module Main\n\n",
        "let value =\n",
        "    match (Some 1) with\n",
        "    | Some value -> value\n",
        "    | None -> 0\n",
        "\nlet main () = ()\n",
    );
    let (source, snapshot) = checked_source("prelude-option.ling", text);
    let lowered = lower_v1_2(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("prelude option constructors lower");
    assert!(lowered.model().modules().iter().any(|module| {
        lowered
            .model()
            .strings()
            .get(module.name.get() as usize)
            .is_some_and(|name| name == ling_resolve::PRELUDE_MODULE)
    }));
    decode_and_verify_v1_2(&encode_v1_2(&lowered).unwrap())
        .expect("the prelude option artifact verifies");
}

#[test]
fn v1_2_lowering_emits_mutable_place_updates_and_branch_joins() {
    let text = concat!(
        "module Main\n\n",
        "type Inner = { mutable value: Int }\n",
        "type Counter = { mutable inner: Inner }\n\n",
        "let mutate flag =\n",
        "    let mutable counter = { inner = { value = 0 } }\n",
        "    counter <- { inner = { value = 9 } }\n",
        "    if flag then\n",
        "        counter.inner.value <- 1\n",
        "    else\n",
        "        counter.inner.value <- 2\n",
        "    counter.inner.value\n\n",
        "let main () = ()\n",
    );
    let (source, snapshot) = checked_source("mutable-place.ling", text);
    let lowered = lower_v1_2(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("checked mutable places lower to SSA updates");
    let model = lowered.model();
    assert!(model.functions().iter().any(|function| {
        function.blocks.iter().any(|block| {
            block
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, Instruction::UpdateRecord { .. }))
        })
    }));
    assert!(model.functions().iter().any(|function| {
        function
            .blocks
            .iter()
            .any(|block| block.parameters.len() > 1)
    }));
    decode_and_verify_v1_2(&encode_v1_2(&lowered).unwrap())
        .expect("the mutable-place artifact verifies");
}

#[test]
fn v1_2_lowering_carries_mutable_places_through_match_joins() {
    let text = concat!(
        "module Main\n\n",
        "type Counter = { mutable value: Int }\n\n",
        "let mutate flag =\n",
        "    let mutable counter = { value = 0 }\n",
        "    match flag with\n",
        "    | true ->\n",
        "        counter.value <- 1\n",
        "    | false ->\n",
        "        counter.value <- 2\n",
        "    counter.value\n\n",
        "let main () = ()\n",
    );
    let (source, snapshot) = checked_source("mutable-match.ling", text);
    let lowered = lower_v1_2(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("checked mutable match lowers to SSA updates");
    assert!(lowered.model().functions().iter().any(|function| {
        function.blocks.iter().any(|block| {
            block
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, Instruction::UpdateRecord { .. }))
        })
    }));
    assert!(lowered.model().functions().iter().any(|function| {
        function
            .blocks
            .iter()
            .any(|block| block.parameters.len() > 1)
    }));
    decode_and_verify_v1_2(&encode_v1_2(&lowered).unwrap())
        .expect("the mutable match artifact verifies");
}
