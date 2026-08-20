use ling_bytecode::{
    BYTECODE_MAGIC, BYTECODE_PROTOCOL, Capability, CompareOperator, Constant, ConstantIndex,
    DecodeLimits, Effect, FORMAT_VERSION, Function, FunctionIndex, Instruction, IntBinaryOperator,
    IntUnaryOperator, IntegerSign, Intrinsic, LANGUAGE_VERSION, Module, ModuleIndex,
    PackageReference, ProgramParts, RegisterIndex, Source, SourceDigest, SourceIndex,
    SourceMapEntry, SourceOrigin, SourceSpan, StringIndex, Terminator, TypeIndex, UNICODE_VERSION,
    UnverifiedProgram, ValueType,
};

#[test]
fn freezes_the_version_and_resource_contract() {
    assert_eq!(BYTECODE_PROTOCOL, "ling.bytecode/1.0");
    assert_eq!(BYTECODE_MAGIC, *b"LINGBC\0\0");
    assert_eq!(FORMAT_VERSION.major(), 1);
    assert_eq!(FORMAT_VERSION.minor(), 0);
    assert_eq!(LANGUAGE_VERSION.major(), 0);
    assert_eq!(LANGUAGE_VERSION.minor(), 1);
    assert_eq!(UNICODE_VERSION.major(), 17);
    assert_eq!(UNICODE_VERSION.minor(), 0);
    assert_eq!(UNICODE_VERSION.patch(), 0);

    let limits = DecodeLimits::rfc_0014();
    assert_eq!(limits.artifact_bytes(), 67_108_864);
    assert_eq!(limits.string_entries(), 262_144);
    assert_eq!(limits.bytes_per_string_or_integer(), 16_777_216);
    assert_eq!(limits.packages(), 65_536);
    assert_eq!(limits.modules(), 65_536);
    assert_eq!(limits.types(), 4);
    assert_eq!(limits.constants(), 1_048_576);
    assert_eq!(limits.sources(), 65_536);
    assert_eq!(limits.functions(), 262_144);
    assert_eq!(limits.registers_per_function(), 65_536);
    assert_eq!(limits.blocks_per_function(), 65_536);
    assert_eq!(limits.arguments_per_operation(), 65_536);
    assert_eq!(limits.executable_locations(), 4_194_304);
}

#[test]
fn freezes_explicit_tags_without_using_rust_layout() {
    assert_eq!(ValueType::Unit.tag(), 0x00);
    assert_eq!(ValueType::Bool.tag(), 0x01);
    assert_eq!(ValueType::Int.tag(), 0x02);
    assert_eq!(ValueType::Text.tag(), 0x03);
    assert_eq!(Capability::ConsoleWrite.tag(), 1);
    assert_eq!(Effect::ConsoleWrite.tag(), 1);

    assert_eq!(IntegerSign::Zero.tag(), 0);
    assert_eq!(IntegerSign::Positive.tag(), 1);
    assert_eq!(IntegerSign::Negative.tag(), 2);

    assert_eq!(CompareOperator::BoolEqual.tag(), 0x00);
    assert_eq!(CompareOperator::IntGreaterEqual.tag(), 0x07);
    assert_eq!(CompareOperator::TextNotEqual.tag(), 0x09);
    assert_eq!(Intrinsic::TextFormat.tag(), 0x00);
    assert_eq!(Intrinsic::MaxInt.tag(), 0x01);
    assert_eq!(Intrinsic::MinInt.tag(), 0x02);
    assert_eq!(IntUnaryOperator::Positive.tag(), 0);
    assert_eq!(IntUnaryOperator::Negative.tag(), 1);
    assert_eq!(IntBinaryOperator::Add.tag(), 0);
    assert_eq!(IntBinaryOperator::Subtract.tag(), 1);
    assert_eq!(IntBinaryOperator::Multiply.tag(), 2);
    assert_eq!(IntBinaryOperator::Divide.tag(), 3);
    assert_eq!(IntBinaryOperator::Remainder.tag(), 4);
    assert_eq!(SourceOrigin::Direct.tag(), 0);
    assert_eq!(SourceOrigin::LoweringDerived.tag(), 1);

    let instructions = [
        Instruction::Const {
            destination: RegisterIndex::new(0),
            constant: ConstantIndex::new(0),
        },
        Instruction::IntUnary {
            destination: RegisterIndex::new(0),
            operator: IntUnaryOperator::Negative,
            operand: RegisterIndex::new(1),
        },
        Instruction::IntBinary {
            destination: RegisterIndex::new(0),
            operator: IntBinaryOperator::Add,
            left: RegisterIndex::new(1),
            right: RegisterIndex::new(2),
        },
        Instruction::Compare {
            destination: RegisterIndex::new(0),
            operator: CompareOperator::IntEqual,
            left: RegisterIndex::new(1),
            right: RegisterIndex::new(2),
        },
        Instruction::Call {
            destination: RegisterIndex::new(0),
            function: FunctionIndex::new(0),
            arguments: vec![RegisterIndex::new(1)],
        },
        Instruction::Intrinsic {
            destination: RegisterIndex::new(0),
            intrinsic: Intrinsic::MaxInt,
            arguments: vec![RegisterIndex::new(1), RegisterIndex::new(2)],
        },
        Instruction::ConsoleWrite {
            destination: RegisterIndex::new(0),
            text: RegisterIndex::new(1),
        },
    ];
    assert_eq!(
        instructions.map(|instruction| instruction.opcode()),
        [0x01, 0x02, 0x03, 0x04, 0x10, 0x11, 0x20]
    );

    let terminators = [
        Terminator::Jump {
            target: ling_bytecode::BlockIndex::new(0),
            arguments: Vec::new(),
        },
        Terminator::Branch {
            condition: RegisterIndex::new(0),
            true_target: ling_bytecode::BlockIndex::new(1),
            true_arguments: Vec::new(),
            false_target: ling_bytecode::BlockIndex::new(2),
            false_arguments: Vec::new(),
        },
        Terminator::Return {
            value: RegisterIndex::new(0),
        },
    ];
    assert_eq!(
        terminators.map(|terminator| terminator.opcode()),
        [0x80, 0x81, 0x82]
    );
}

#[test]
fn represents_a_hello_program_only_as_unverified_data() {
    let strings = vec![
        "Main".to_owned(),
        "main".to_owned(),
        "src/Main.ling".to_owned(),
        "你好，零".to_owned(),
    ];
    let module = Module {
        package: PackageReference::Standalone,
        name: StringIndex::new(0),
        capabilities: vec![Capability::ConsoleWrite],
    };
    let constants = vec![
        Constant::Unit,
        Constant::Text(StringIndex::new(3)),
        Constant::Int {
            sign: IntegerSign::Positive,
            magnitude: vec![0x01, 0x00],
        },
    ];
    let source = Source {
        module: ModuleIndex::new(0),
        logical_name: StringIndex::new(2),
        original_byte_length: 96,
        content_sha256: SourceDigest::new([0x5a; 32]),
    };
    let function = Function {
        module: ModuleIndex::new(0),
        name: StringIndex::new(1),
        parameter_types: vec![TypeIndex::new(0)],
        result_type: TypeIndex::new(0),
        effects: vec![Effect::ConsoleWrite],
        register_count: 3,
        blocks: vec![ling_bytecode::Block {
            parameters: vec![ling_bytecode::BlockParameter {
                register: RegisterIndex::new(0),
                value_type: TypeIndex::new(0),
            }],
            instructions: vec![
                Instruction::Const {
                    destination: RegisterIndex::new(1),
                    constant: ConstantIndex::new(1),
                },
                Instruction::ConsoleWrite {
                    destination: RegisterIndex::new(2),
                    text: RegisterIndex::new(1),
                },
            ],
            terminator: Terminator::Return {
                value: RegisterIndex::new(2),
            },
        }],
    };
    let source_span = SourceSpan::new(48, 85);
    let source_map = (0..=2)
        .map(|ordinal| SourceMapEntry {
            function: FunctionIndex::new(0),
            block: ling_bytecode::BlockIndex::new(0),
            ordinal,
            source: SourceIndex::new(0),
            span: source_span,
            origin: SourceOrigin::Direct,
        })
        .collect();

    let program = UnverifiedProgram::from_parts(ProgramParts {
        strings,
        packages: Vec::new(),
        modules: vec![module],
        types: vec![
            ValueType::Unit,
            ValueType::Bool,
            ValueType::Int,
            ValueType::Text,
        ],
        constants,
        sources: vec![source],
        functions: vec![function],
        entry: FunctionIndex::new(0),
        source_map,
    });

    assert_eq!(program.protocol(), BYTECODE_PROTOCOL);
    assert_eq!(program.strings()[3], "你好，零");
    assert_eq!(program.modules()[0].package, PackageReference::Standalone);
    assert_eq!(program.functions()[0].blocks[0].instructions.len(), 2);
    assert_eq!(program.entry(), FunctionIndex::new(0));
    assert_eq!(program.sources()[0].content_sha256.as_bytes(), &[0x5a; 32]);
    assert_eq!(program.source_map()[2].span.start_byte(), 48);
    assert_eq!(program.source_map()[2].span.end_byte(), 85);
}
