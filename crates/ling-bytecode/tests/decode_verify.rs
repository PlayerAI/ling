mod support;

use ling_bytecode::{
    BytecodePhase, BytecodeReason, FORMAT_VERSION_1_0, FORMAT_VERSION_1_1, LoweringSource,
    decode_and_verify_v1, decode_and_verify_v1_1, decode_v1, encode_v1, encode_verified_v1,
    lower_v1, verify_v1,
};
use ling_diagnostics::{MessageLanguage, codes};
use ling_semantic::ProgramSnapshot;
use ling_source::{SourceFile, SourceId};
use support::{branch_artifact, closure_artifact};

#[test]
fn version_1_1_reader_accepts_both_revisions_and_round_trips_closures() {
    let closure = closure_artifact();
    let verified = decode_and_verify_v1_1(&closure.bytes).expect("independent closure artifact");
    assert_eq!(verified.version(), FORMAT_VERSION_1_1);
    assert_eq!(encode_verified_v1(&verified).unwrap(), closure.bytes);

    let legacy = decode_and_verify_v1_1(&hello_bytes()).expect("1.1 reader accepts 1.0");
    assert_eq!(legacy.version(), FORMAT_VERSION_1_0);

    let error = decode_and_verify_v1(&closure.bytes).expect_err("1.0 reader rejects 1.1");
    assert_eq!(error.phase(), BytecodePhase::Envelope);
    assert_eq!(error.reason(), BytecodeReason::UnsupportedVersion);
}

#[test]
fn version_1_1_closure_metadata_failures_are_bounded_and_structured() {
    let base = closure_artifact();
    let cases = [
        ("forward_function_type", BytecodeReason::InvalidTypeIndex, {
            let mut bytes = base.bytes.clone();
            overwrite_u32(
                &mut bytes,
                base.position("suffix_function_type_parameter"),
                5,
            );
            bytes
        }),
        ("invalid_function_kind", BytecodeReason::InvalidTag, {
            let mut bytes = base.bytes.clone();
            bytes[base.position("closure_body_kind")] = 2;
            bytes
        }),
        (
            "capture_count_mismatch",
            BytecodeReason::InvalidInstructionLength,
            {
                let mut bytes = base.bytes.clone();
                overwrite_u32(&mut bytes, base.position("make_closure_capture_count"), 0);
                bytes
            },
        ),
        ("invalid_capture_kind", BytecodeReason::InvalidTag, {
            let mut bytes = base.bytes.clone();
            bytes[base.position("make_closure_capture_kind")] = 2;
            bytes
        }),
        (
            "invalid_capture_register",
            BytecodeReason::InvalidRegisterIndex,
            {
                let mut bytes = base.bytes.clone();
                overwrite_u32(
                    &mut bytes,
                    base.position("make_closure_capture_register"),
                    u32::MAX,
                );
                bytes
            },
        ),
        (
            "wrong_self_capture_type",
            BytecodeReason::CallSignatureMismatch,
            {
                let mut bytes = base.bytes.clone();
                bytes[base.position("make_closure_capture_kind")] = 1;
                overwrite_u32(
                    &mut bytes,
                    base.position("make_closure_capture_register"),
                    u32::MAX,
                );
                bytes
            },
        ),
        (
            "invalid_direct_call_target",
            BytecodeReason::InvalidFunctionIndex,
            {
                let mut bytes = base.bytes.clone();
                bytes[base.position("partial_call_closure_opcode")] = 0x10;
                bytes
            },
        ),
        (
            "direct_opcode_on_closure_register",
            BytecodeReason::InvalidRegisterType,
            {
                let mut bytes = base.bytes.clone();
                bytes[base.position("partial_call_closure_opcode")] = 0x10;
                overwrite_u32(&mut bytes, base.position("partial_call_closure_callee"), 1);
                bytes
            },
        ),
        (
            "non_function_indirect_callee",
            BytecodeReason::InvalidRegisterType,
            {
                let mut bytes = base.bytes.clone();
                overwrite_u32(&mut bytes, base.position("complete_call_closure_callee"), 1);
                bytes
            },
        ),
        (
            "mistyped_indirect_argument",
            BytecodeReason::CallSignatureMismatch,
            {
                let mut bytes = base.bytes.clone();
                overwrite_u32(
                    &mut bytes,
                    base.position("partial_call_closure_argument"),
                    2,
                );
                bytes
            },
        ),
    ];

    for (name, expected, bytes) in cases {
        let result = std::panic::catch_unwind(|| decode_and_verify_v1_1(&bytes));
        let error = result
            .unwrap_or_else(|_| panic!("{name} panicked"))
            .expect_err(name);
        assert_eq!(error.reason(), expected, "{name}");
        assert_ne!(error.phase(), BytecodePhase::Envelope, "{name}");
    }
}

const HELLO_HEX: &str = include_str!("../../../tests/bytecode/v1/golden/hello.lbc.hex");

fn hello_bytes() -> Vec<u8> {
    let value = HELLO_HEX.trim().as_bytes();
    assert_eq!(value.len() % 2, 0, "hex fixture has complete bytes");
    value
        .chunks_exact(2)
        .map(|pair| {
            let text = std::str::from_utf8(pair).expect("fixture is ASCII");
            u8::from_str_radix(text, 16).expect("fixture is hexadecimal")
        })
        .collect()
}

fn encode_source(text: &str) -> Vec<u8> {
    let source = SourceFile::from_bytes(SourceId::new(0), "fixture.ling", text.as_bytes().to_vec())
        .expect("fixture source is valid UTF-8");
    let parsed = ling_syntax::parse(&source);
    assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    let ast = ling_ast::lower(&source, &parsed).expect("fixture lowers to AST");
    let hir = ling_hir::lower(source.name(), &ast).expect("fixture lowers to HIR");
    let resolved = ling_resolve::resolve(vec![hir], "Main").expect("fixture resolves");
    let typed = ling_types::check(resolved).expect("fixture type-checks");
    let checked = ling_effects::check(typed).expect("fixture passes effect checks");
    let snapshot: ProgramSnapshot = ling_semantic::build(checked).expect("snapshot builds");
    let lowered = lower_v1(&snapshot, &[LoweringSource::new(&source, "src/Main.ling")])
        .expect("fixture lowers to bytecode");
    encode_v1(&lowered).expect("fixture encodes")
}

fn find_unique(bytes: &[u8], needle: &[u8]) -> usize {
    let matches = bytes
        .windows(needle.len())
        .enumerate()
        .filter_map(|(index, value)| (value == needle).then_some(index))
        .collect::<Vec<_>>();
    assert_eq!(
        matches.len(),
        1,
        "mutation pattern must be unique: {needle:02x?}"
    );
    matches[0]
}

fn update_total_length(bytes: &mut [u8]) {
    let length = u64::try_from(bytes.len()).unwrap();
    overwrite_u64(bytes, 32, length);
}

fn overwrite_u16(bytes: &mut [u8], offset: usize, value: u16) {
    bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn overwrite_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn overwrite_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

#[test]
fn independently_decodes_and_verifies_the_exact_hello_golden() {
    let bytes = hello_bytes();
    let fuzz_seed = include_str!("../../../fuzz/corpus/bytecode_bytes/valid-hello.hexseed");
    assert_eq!(
        fuzz_seed.trim().strip_prefix("hex:").unwrap(),
        HELLO_HEX.trim(),
        "the valid fuzz seed preserves the exact blessed bytes"
    );
    let decoded = decode_v1(&bytes).expect("the exact golden decodes");
    assert_eq!(decoded.model().strings().len(), 4);
    assert_eq!(decoded.model().functions().len(), 1);
    assert_eq!(decoded.model().source_map().len(), 3);

    let verified = verify_v1(decoded).expect("the exact golden verifies independently");
    assert_eq!(verified.model().entry().get(), 0);
    assert_eq!(verified.model().functions()[0].register_count, 3);
    assert_eq!(
        encode_verified_v1(&verified).expect("verified model re-encodes"),
        bytes
    );
}

#[test]
fn verifies_direct_calls_branches_and_block_parameters() {
    let direct_call = include_str!("../../../tests/bytecode/v1/programs/direct-call.ling");
    let direct_bytes = encode_source(direct_call);
    let direct_verified = decode_and_verify_v1(&direct_bytes).expect("valid direct calls verify");
    assert_eq!(encode_verified_v1(&direct_verified).unwrap(), direct_bytes);

    let branch = branch_artifact();
    let branch_verified =
        decode_and_verify_v1(&branch.bytes).expect("independent branch fixture verifies");
    assert_eq!(encode_verified_v1(&branch_verified).unwrap(), branch.bytes);

    let mut loop_bytes = branch.bytes.clone();
    overwrite_u32(&mut loop_bytes, branch.position("block1_jump_target"), 0);
    decode_and_verify_v1(&loop_bytes).expect("cyclic control flow reaches a dominator fixpoint");

    let transitive_effect = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let write value = Console.write value\n",
        "let main () = write \"transitive\"\n",
    );
    let transitive_bytes = encode_source(transitive_effect);
    decode_and_verify_v1(&transitive_bytes).expect("transitive effect closure verifies");
}

#[test]
fn validates_caller_limits_names_types_constants_and_source_map_indexes() {
    let hello = hello_bytes();
    let error =
        ling_bytecode::decode_v1_with_limit(&hello, u64::try_from(hello.len() - 1).unwrap())
            .unwrap_err();
    assert_eq!(error.reason(), BytecodeReason::ResourceLimit);

    let mut invalid_module_name = hello.clone();
    let module_name = find_unique(&invalid_module_name, b"\x04\0\0\0Main") + 4;
    invalid_module_name[module_name..module_name + 4].copy_from_slice(b"M-in");
    assert_eq!(
        decode_and_verify_v1(&invalid_module_name)
            .unwrap_err()
            .reason(),
        BytecodeReason::InvalidName
    );

    let mut invalid_logical_path = hello.clone();
    let path = find_unique(&invalid_logical_path, b"\x0d\0\0\0src/Main.ling") + 4;
    invalid_logical_path[path..path + 13].copy_from_slice(b"src\\Main.ling");
    assert_eq!(
        decode_and_verify_v1(&invalid_logical_path)
            .unwrap_err()
            .reason(),
        BytecodeReason::InvalidLogicalPath
    );

    let type_table = find_unique(
        &hello,
        &[
            4, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 2, 1, 0, 0, 0, 3,
        ],
    );
    let mut incomplete_types = hello.clone();
    overwrite_u32(&mut incomplete_types, type_table, 3);
    incomplete_types.drain(type_table + 19..type_table + 24);
    update_total_length(&mut incomplete_types);
    assert_eq!(
        decode_and_verify_v1(&incomplete_types)
            .unwrap_err()
            .reason(),
        BytecodeReason::InvalidTableOrder
    );

    let constant_table = find_unique(
        &hello,
        &[1, 0, 0, 0, 12, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0],
    );
    let mut wrong_constant_type = hello.clone();
    overwrite_u32(&mut wrong_constant_type, constant_table + 12, 2);
    assert_eq!(
        decode_and_verify_v1(&wrong_constant_type)
            .unwrap_err()
            .reason(),
        BytecodeReason::InvalidTypeIndex
    );

    let source_map_count = hello.len() - (4 + 3 * 44);
    for (field_offset, reason) in [
        (source_map_count + 8, BytecodeReason::InvalidFunctionIndex),
        (source_map_count + 12, BytecodeReason::InvalidBlockIndex),
        (source_map_count + 20, BytecodeReason::InvalidSourceIndex),
    ] {
        let mut bytes = hello.clone();
        overwrite_u32(&mut bytes, field_offset, u32::MAX);
        assert_eq!(decode_and_verify_v1(&bytes).unwrap_err().reason(), reason);
    }

    let mut duplicate_source_map = hello;
    overwrite_u32(&mut duplicate_source_map, source_map_count + 60, 0);
    assert_eq!(
        decode_and_verify_v1(&duplicate_source_map)
            .unwrap_err()
            .reason(),
        BytecodeReason::DuplicateSourceMap
    );
}

#[test]
fn envelope_failures_have_stable_reasons_offsets_and_codes() {
    let mut cases = Vec::new();

    let mut wrong_magic = hello_bytes();
    wrong_magic[0] ^= 0xff;
    cases.push((wrong_magic, BytecodeReason::InvalidMagic, 0_u64));

    let mut unsupported_version = hello_bytes();
    overwrite_u16(&mut unsupported_version, 12, 2);
    cases.push((unsupported_version, BytecodeReason::UnsupportedVersion, 12));

    let mut reserved = hello_bytes();
    overwrite_u16(&mut reserved, 26, 1);
    cases.push((reserved, BytecodeReason::ReservedNonzero, 26));

    let mut truncated = hello_bytes();
    let declared = u64::try_from(truncated.len()).unwrap() + 1;
    overwrite_u64(&mut truncated, 32, declared);
    cases.push((truncated, BytecodeReason::TruncatedArtifact, 32));

    let mut trailing = hello_bytes();
    let trailing_offset = u64::try_from(trailing.len()).unwrap();
    trailing.push(0);
    cases.push((trailing, BytecodeReason::TrailingBytes, trailing_offset));

    for (bytes, reason, offset) in cases {
        let error = decode_v1(&bytes).expect_err("corrupt envelope must be rejected");
        assert_eq!(error.phase(), BytecodePhase::Envelope);
        assert_eq!(error.reason(), reason);
        assert_eq!(error.offset(), offset);
        assert_eq!(error.code(), codes::INVALID_BYTECODE_ENVELOPE);
    }
}

#[test]
fn table_and_instruction_corruptions_are_structured() {
    let mut excessive_strings = hello_bytes();
    overwrite_u32(&mut excessive_strings, 40, 262_145);
    let error = decode_v1(&excessive_strings).expect_err("hard count limit is checked first");
    assert_eq!(error.reason(), BytecodeReason::ResourceLimit);
    assert_eq!(error.code(), codes::BYTECODE_RESOURCE_LIMIT_EXCEEDED);

    let mut invalid_utf8 = hello_bytes();
    let first_string = find_unique(&invalid_utf8, b"\x04\0\0\0Main") + 4;
    invalid_utf8[first_string] = 0xff;
    let error = decode_v1(&invalid_utf8).expect_err("string payload must be UTF-8");
    assert_eq!(error.phase(), BytecodePhase::Table);
    assert_eq!(error.reason(), BytecodeReason::InvalidUtf8);
    assert_eq!(error.code(), codes::INVALID_BYTECODE_TABLE);

    let mut unsorted = hello_bytes();
    let first_string = find_unique(&unsorted, b"\x04\0\0\0Main") + 4;
    unsorted[first_string..first_string + 4].copy_from_slice(b"zain");
    let error = verify_v1(decode_v1(&unsorted).expect("spelling is valid UTF-8"))
        .expect_err("string ordering is verified independently");
    assert_eq!(error.reason(), BytecodeReason::InvalidTableOrder);
    assert_eq!(error.code(), codes::INVALID_BYTECODE_TABLE);

    let console = [0x20, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0];
    let mut unknown_opcode = hello_bytes();
    let instruction = find_unique(&unknown_opcode, &console);
    unknown_opcode[instruction] = 0x7f;
    let error = decode_v1(&unknown_opcode).expect_err("unknown opcodes are never skipped");
    assert_eq!(error.phase(), BytecodePhase::Instruction);
    assert_eq!(error.reason(), BytecodeReason::UnknownOpcode);
    assert_eq!(error.code(), codes::INVALID_BYTECODE_PROGRAM);

    let mut wrong_length = hello_bytes();
    let instruction = find_unique(&wrong_length, &console);
    overwrite_u32(&mut wrong_length, instruction - 4, 11);
    let error = decode_v1(&wrong_length).expect_err("instruction framing is exact");
    assert_eq!(error.reason(), BytecodeReason::InvalidInstructionLength);
    assert_eq!(error.code(), codes::INVALID_BYTECODE_PROGRAM);
}

#[test]
fn bytecode_errors_render_bilingual_registered_diagnostics() {
    let mut bytes = hello_bytes();
    bytes[0] = 0;
    let error = decode_v1(&bytes).unwrap_err();
    let diagnostic = error.to_diagnostic("corrupt/hello.lbc");

    assert_eq!(diagnostic.code(), codes::INVALID_BYTECODE_ENVELOPE);
    let span = diagnostic
        .primary_span()
        .expect("artifact byte span exists");
    assert_eq!(span.file(), "corrupt/hello.lbc");
    assert_eq!(span.start_byte(), 0);
    assert!(span.end_byte() > span.start_byte());
    assert!(
        diagnostic
            .render_human(MessageLanguage::Chinese)
            .contains("字节码")
    );
    assert!(
        diagnostic
            .render_human(MessageLanguage::English)
            .contains("bytecode")
    );
    let json = diagnostic.render_json().expect("diagnostic JSON renders");
    assert!(json.contains("\"reason\":\"invalid_magic\""));
    assert!(json.contains("\"phase\":\"envelope\""));
}

#[test]
fn deterministic_arbitrary_bytes_never_panic_or_publish_partial_state() {
    let mut state = 0x0012_0300_1417_u64;
    for case in 0..512_usize {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let length = (state as usize) % 1024;
        let mut bytes = Vec::with_capacity(length);
        for index in 0..length {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            bytes.push((state >> (index % 8)) as u8);
        }
        let result = decode_and_verify_v1(&bytes);
        if let Err(error) = result {
            let rendered = error.to_diagnostic("fuzz/input.lbc").render_json().unwrap();
            assert!(
                rendered.len() < 4096,
                "case {case} rendered a bounded error"
            );
        }
    }
}

struct MalformedCase {
    id: &'static str,
    reason: BytecodeReason,
    bytes: Vec<u8>,
}

#[test]
fn malformed_corpus_covers_every_registered_reason_without_panics() {
    let cases = malformed_cases();
    let registry = include_str!("../../../tests/bytecode/v1/malformed-cases.tsv");
    let rows = registry
        .lines()
        .skip(1)
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    assert_eq!(cases.len(), rows.len(), "every corpus row has one vector");
    for (case, row) in cases.iter().zip(rows) {
        assert_eq!(case.id, row[0]);
        assert_eq!(case.reason.as_str(), row[3]);
        let result = std::panic::catch_unwind(|| decode_and_verify_v1(&case.bytes));
        let error = result
            .unwrap_or_else(|_| panic!("{} panicked", case.id))
            .unwrap_err();
        assert_eq!(
            error.reason(),
            case.reason,
            "{} selected the wrong stable reason at byte {}",
            case.id,
            error.offset()
        );
        let diagnostic = error.to_diagnostic(case.id).render_json().unwrap();
        assert!(
            diagnostic.len() < 4096,
            "{} renders bounded output",
            case.id
        );
    }
}

fn malformed_cases() -> Vec<MalformedCase> {
    let hello = hello_bytes();
    let console_record = find_unique(
        &hello,
        &[0x0c, 0, 0, 0, 0x20, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0],
    );
    let main_string = find_unique(&hello, b"\x04\0\0\0main") + 4;
    let module_record = find_unique(
        &hello,
        &[
            0x0d, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 1, 0, 0, 0, 1,
        ],
    );
    let source_map_count = hello.len() - (4 + 3 * 44);
    assert_eq!(
        &hello[source_map_count..source_map_count + 4],
        &[3, 0, 0, 0]
    );

    let mut wrong_magic = hello.clone();
    wrong_magic[0] ^= 0xff;
    let mut unsupported_version = hello.clone();
    overwrite_u16(&mut unsupported_version, 12, 2);
    let mut reserved_nonzero = hello.clone();
    overwrite_u16(&mut reserved_nonzero, 26, 1);
    let mut truncated = hello.clone();
    overwrite_u64(&mut truncated, 32, u64::try_from(hello.len()).unwrap() + 1);
    let mut resource_limit = hello.clone();
    overwrite_u32(&mut resource_limit, 40, 262_145);
    let mut invalid_utf8 = hello.clone();
    let main_module = find_unique(&invalid_utf8, b"\x04\0\0\0Main") + 4;
    invalid_utf8[main_module] = 0xff;
    let mut invalid_order = hello.clone();
    invalid_order[main_module..main_module + 4].copy_from_slice(b"zain");

    let integer_source = concat!(
        "module Main\n\n",
        "let number () = 256\n",
        "let main () = ()\n",
    );
    let integer = encode_source(integer_source);
    decode_and_verify_v1(&integer).expect("canonical integer fixture verifies");
    let integer_record = find_unique(
        &integer,
        &[
            0x12, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 1, 0,
        ],
    );
    let mut noncanonical_integer = integer;
    noncanonical_integer[integer_record + 20] = 0;

    let mut unknown_opcode = hello.clone();
    unknown_opcode[console_record + 4] = 0x7f;
    let mut invalid_instruction_length = hello.clone();
    overwrite_u32(&mut invalid_instruction_length, console_record, 11);

    let branch = branch_artifact();
    let mut invalid_block_index = branch.bytes.clone();
    overwrite_u32(
        &mut invalid_block_index,
        branch.position("block1_jump_target"),
        u32::MAX,
    );
    let mut invalid_register_index = hello.clone();
    overwrite_u32(&mut invalid_register_index, console_record + 12, u32::MAX);
    let mut duplicate_register = branch.bytes.clone();
    overwrite_u32(
        &mut duplicate_register,
        branch.position("block1_parameter_register"),
        0,
    );
    let mut not_dominated = branch.bytes.clone();
    overwrite_u32(
        &mut not_dominated,
        branch.position("block1_jump_argument"),
        3,
    );
    let mut block_type_mismatch = branch.bytes.clone();
    overwrite_u32(
        &mut block_type_mismatch,
        branch.position("branch_true_argument"),
        1,
    );

    let call_source = concat!(
        "module Main\n",
        "    requires Console.Write\n\n",
        "let consume value = Console.write value\n",
        "let message () = \"direct call\"\n",
        "let main () =\n",
        "    let value = message ()\n",
        "    Console.write value\n",
    );
    let direct = encode_source(call_source);
    let call_record = find_unique(&direct, &[0x14, 0, 0, 0, 0x10, 0, 0, 0]);
    let decoded_direct = decode_v1(&direct).unwrap();
    let consume = decoded_direct
        .model()
        .functions()
        .iter()
        .position(|function| {
            decoded_direct.model().strings()[usize::try_from(function.name.get()).unwrap()]
                == "consume"
        })
        .and_then(|index| u32::try_from(index).ok())
        .expect("consume function has an artifact index");
    let mut call_signature_mismatch = direct;
    overwrite_u32(&mut call_signature_mismatch, call_record + 12, consume);

    let function_payload = find_unique(
        &hello,
        &[
            0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 3, 0, 0, 0,
        ],
    );
    let mut effect_mismatch = hello.clone();
    let function_length = read_u32(&effect_mismatch, function_payload - 4);
    overwrite_u32(
        &mut effect_mismatch,
        function_payload - 4,
        function_length - 1,
    );
    overwrite_u32(&mut effect_mismatch, function_payload + 20, 0);
    effect_mismatch.remove(function_payload + 24);
    update_total_length(&mut effect_mismatch);

    let mut capability_mismatch = hello.clone();
    overwrite_u32(&mut capability_mismatch, module_record, 12);
    overwrite_u32(&mut capability_mismatch, module_record + 12, 0);
    capability_mismatch.remove(module_record + 16);
    update_total_length(&mut capability_mismatch);

    let mut invalid_entry = hello.clone();
    invalid_entry[main_string..main_string + 4].copy_from_slice(b"nain");

    let mut incomplete_source_map = hello.clone();
    overwrite_u32(&mut incomplete_source_map, source_map_count, 2);
    incomplete_source_map.truncate(incomplete_source_map.len() - 44);
    update_total_length(&mut incomplete_source_map);

    let mut invalid_source_span = hello.clone();
    overwrite_u64(&mut invalid_source_span, source_map_count + 32, u64::MAX);

    let mut trailing = hello;
    trailing.push(0);

    vec![
        malformed("BC-MAL-0001", BytecodeReason::InvalidMagic, wrong_magic),
        malformed(
            "BC-MAL-0002",
            BytecodeReason::UnsupportedVersion,
            unsupported_version,
        ),
        malformed(
            "BC-MAL-0003",
            BytecodeReason::ReservedNonzero,
            reserved_nonzero,
        ),
        malformed("BC-MAL-0004", BytecodeReason::TruncatedArtifact, truncated),
        malformed("BC-MAL-0005", BytecodeReason::ResourceLimit, resource_limit),
        malformed("BC-MAL-0006", BytecodeReason::InvalidUtf8, invalid_utf8),
        malformed(
            "BC-MAL-0007",
            BytecodeReason::InvalidTableOrder,
            invalid_order,
        ),
        malformed(
            "BC-MAL-0008",
            BytecodeReason::NoncanonicalInteger,
            noncanonical_integer,
        ),
        malformed("BC-MAL-0009", BytecodeReason::UnknownOpcode, unknown_opcode),
        malformed(
            "BC-MAL-0010",
            BytecodeReason::InvalidInstructionLength,
            invalid_instruction_length,
        ),
        malformed(
            "BC-MAL-0011",
            BytecodeReason::InvalidBlockIndex,
            invalid_block_index,
        ),
        malformed(
            "BC-MAL-0012",
            BytecodeReason::InvalidRegisterIndex,
            invalid_register_index,
        ),
        malformed(
            "BC-MAL-0013",
            BytecodeReason::DuplicateRegisterDefinition,
            duplicate_register,
        ),
        malformed(
            "BC-MAL-0014",
            BytecodeReason::RegisterNotDominated,
            not_dominated,
        ),
        malformed(
            "BC-MAL-0015",
            BytecodeReason::BlockArgumentTypeMismatch,
            block_type_mismatch,
        ),
        malformed(
            "BC-MAL-0016",
            BytecodeReason::CallSignatureMismatch,
            call_signature_mismatch,
        ),
        malformed(
            "BC-MAL-0017",
            BytecodeReason::EffectMismatch,
            effect_mismatch,
        ),
        malformed(
            "BC-MAL-0018",
            BytecodeReason::CapabilityMismatch,
            capability_mismatch,
        ),
        malformed("BC-MAL-0019", BytecodeReason::InvalidEntry, invalid_entry),
        malformed(
            "BC-MAL-0020",
            BytecodeReason::IncompleteSourceMap,
            incomplete_source_map,
        ),
        malformed(
            "BC-MAL-0021",
            BytecodeReason::InvalidSourceSpan,
            invalid_source_span,
        ),
        malformed("BC-MAL-0022", BytecodeReason::TrailingBytes, trailing),
    ]
}

fn malformed(id: &'static str, reason: BytecodeReason, bytes: Vec<u8>) -> MalformedCase {
    MalformedCase { id, reason, bytes }
}
