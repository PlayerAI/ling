use crate::BYTECODE_PROTOCOL;

macro_rules! index_type {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u32);

        impl $name {
            /// Creates an artifact-local index without claiming verification.
            #[must_use]
            pub const fn new(value: u32) -> Self {
                Self(value)
            }

            /// Returns the encoded numeric index.
            #[must_use]
            pub const fn get(self) -> u32 {
                self.0
            }
        }
    };
}

index_type!(StringIndex, "An artifact-local string-table index.");
index_type!(PackageIndex, "An artifact-local package-table index.");
index_type!(ModuleIndex, "An artifact-local module-table index.");
index_type!(TypeIndex, "An artifact-local type-table index.");
index_type!(ConstantIndex, "An artifact-local constant-table index.");
index_type!(SourceIndex, "An artifact-local source-table index.");
index_type!(FunctionIndex, "An artifact-local function-table index.");
index_type!(BlockIndex, "An artifact-local function block index.");
index_type!(RegisterIndex, "An artifact-local function register index.");

/// A package-content digest, distinct from source and Semantic IDs.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PackageContentDigest([u8; 32]);

impl PackageContentDigest {
    /// Wraps the 32 RFC-0002 package-content digest bytes.
    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the raw digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// An exact-source digest, distinct from package and Semantic IDs.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceDigest([u8; 32]);

impl SourceDigest {
    /// Wraps the SHA-256 digest of exact original source bytes.
    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the raw digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// One RFC-0002 package identity record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Package {
    pub name: StringIndex,
    pub version: StringIndex,
    pub content_sha256: PackageContentDigest,
}

/// Encoded module ownership without using an untyped sentinel in memory.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PackageReference {
    Standalone,
    Package(PackageIndex),
}

/// A compile-time/runtime capability tag defined by bytecode version 1.0.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Capability {
    ConsoleWrite,
}

impl Capability {
    /// Returns the explicit wire tag.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::ConsoleWrite => 1,
        }
    }
}

/// A canonical function Effect record admitted by the selected bytecode revision.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Effect {
    ConsoleWrite,
    State(TypeIndex),
}

impl Effect {
    /// Returns the explicit wire tag.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::ConsoleWrite => 1,
            Self::State(_) => 2,
        }
    }
}

/// One unverified module-table record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Module {
    pub package: PackageReference,
    pub name: StringIndex,
    pub capabilities: Vec<Capability>,
}

/// A structural value type admitted by the selected bytecode 1.x revision.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ValueType {
    Unit,
    Bool,
    Int,
    Text,
    Function {
        parameters: Vec<TypeIndex>,
        result: TypeIndex,
        effects: Vec<Effect>,
    },
    Tuple {
        elements: Vec<TypeIndex>,
    },
    Record {
        module: ModuleIndex,
        name: StringIndex,
        arguments: Vec<TypeIndex>,
        fields: Vec<RecordField>,
    },
    Variant {
        module: ModuleIndex,
        name: StringIndex,
        arguments: Vec<TypeIndex>,
        cases: Vec<VariantCase>,
    },
    Cell(TypeIndex),
}

impl ValueType {
    /// Returns the explicit wire tag.
    #[must_use]
    pub const fn tag(&self) -> u8 {
        match self {
            Self::Unit => 0x00,
            Self::Bool => 0x01,
            Self::Int => 0x02,
            Self::Text => 0x03,
            Self::Function { .. } => 0x10,
            Self::Tuple { .. } => 0x11,
            Self::Record { .. } => 0x12,
            Self::Variant { .. } => 0x13,
            Self::Cell(_) => 0x14,
        }
    }
}

/// One nominal record field in a version-1.2 aggregate type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RecordField {
    pub name: StringIndex,
    pub value_type: TypeIndex,
    pub mutable: bool,
}

/// One nominal variant case in a version-1.2 aggregate type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VariantCase {
    pub name: StringIndex,
    pub payload: Option<TypeIndex>,
}

/// One immutable record field replacement in `UpdateRecord`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RecordUpdate {
    pub field: u32,
    pub value: RegisterIndex,
}

/// Function-table role encoded explicitly by bytecode version 1.1.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FunctionKind {
    Named,
    ClosureBody,
}

impl FunctionKind {
    /// Returns the explicit version-1.1 wire tag.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::Named => 0,
            Self::ClosureBody => 1,
        }
    }
}

/// One source for a lexically ordered closure capture.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CaptureOperand {
    Register(RegisterIndex),
    SelfReference,
}

impl CaptureOperand {
    /// Returns the explicit version-1.1 wire tag.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::Register(_) => 0,
            Self::SelfReference => 1,
        }
    }
}

/// Canonical sign representation for an arbitrary-precision integer constant.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IntegerSign {
    Zero,
    Positive,
    Negative,
}

impl IntegerSign {
    /// Returns the explicit wire tag.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::Positive => 1,
            Self::Negative => 2,
        }
    }
}

/// One unverified constant-table record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Constant {
    Unit,
    Bool(bool),
    Int {
        sign: IntegerSign,
        magnitude: Vec<u8>,
    },
    Text(StringIndex),
}

impl Constant {
    /// Returns the explicit wire tag.
    #[must_use]
    pub const fn tag(&self) -> u8 {
        match self {
            Self::Unit => 0x00,
            Self::Bool(_) => 0x01,
            Self::Int { .. } => 0x02,
            Self::Text(_) => 0x03,
        }
    }
}

/// One source-table record with no physical path or embedded source text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Source {
    pub module: ModuleIndex,
    pub logical_name: StringIndex,
    pub original_byte_length: u64,
    pub content_sha256: SourceDigest,
}

/// Integer unary operations defined by bytecode version 1.0.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IntUnaryOperator {
    Positive,
    Negative,
}

impl IntUnaryOperator {
    /// Returns the explicit instruction operand tag.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::Positive => 0,
            Self::Negative => 1,
        }
    }
}

/// Integer binary operations defined by bytecode version 1.0.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IntBinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

impl IntBinaryOperator {
    /// Returns the explicit instruction operand tag.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::Add => 0,
            Self::Subtract => 1,
            Self::Multiply => 2,
            Self::Divide => 3,
            Self::Remainder => 4,
        }
    }
}

/// Typed comparison operations defined by bytecode version 1.0.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CompareOperator {
    BoolEqual,
    BoolNotEqual,
    IntEqual,
    IntNotEqual,
    IntLess,
    IntLessEqual,
    IntGreater,
    IntGreaterEqual,
    TextEqual,
    TextNotEqual,
}

impl CompareOperator {
    /// Returns the explicit instruction operand tag.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::BoolEqual => 0x00,
            Self::BoolNotEqual => 0x01,
            Self::IntEqual => 0x02,
            Self::IntNotEqual => 0x03,
            Self::IntLess => 0x04,
            Self::IntLessEqual => 0x05,
            Self::IntGreater => 0x06,
            Self::IntGreaterEqual => 0x07,
            Self::TextEqual => 0x08,
            Self::TextNotEqual => 0x09,
        }
    }

    /// Returns the exact operand type required by this comparison.
    #[must_use]
    pub const fn operand_type(self) -> ValueType {
        match self {
            Self::BoolEqual | Self::BoolNotEqual => ValueType::Bool,
            Self::IntEqual
            | Self::IntNotEqual
            | Self::IntLess
            | Self::IntLessEqual
            | Self::IntGreater
            | Self::IntGreaterEqual => ValueType::Int,
            Self::TextEqual | Self::TextNotEqual => ValueType::Text,
        }
    }
}

/// Accepted scalar built-ins represented without resolver-specific enum layout.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Intrinsic {
    TextFormat,
    MaxInt,
    MinInt,
}

/// Canonical operation tags accepted by the bytecode-1.3 Handler instruction.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HandlerOperation {
    ConsoleWrite,
    ClockNow,
    RandomNext,
}

impl HandlerOperation {
    /// Returns the DEC-0261 wire tag.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::ConsoleWrite => 1,
            Self::ClockNow => 2,
            Self::RandomNext => 3,
        }
    }
}

/// One ordered bytecode-1.3 Handler clause closure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandlerClause {
    pub operation: HandlerOperation,
    pub resume_present: bool,
    pub function: FunctionIndex,
    pub captures: Vec<CaptureOperand>,
}

impl Intrinsic {
    /// Returns the explicit instruction operand tag.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::TextFormat => 0x00,
            Self::MaxInt => 0x01,
            Self::MinInt => 0x02,
        }
    }
}

/// One unverified single-assignment register instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Const {
        destination: RegisterIndex,
        constant: ConstantIndex,
    },
    IntUnary {
        destination: RegisterIndex,
        operator: IntUnaryOperator,
        operand: RegisterIndex,
    },
    IntBinary {
        destination: RegisterIndex,
        operator: IntBinaryOperator,
        left: RegisterIndex,
        right: RegisterIndex,
    },
    Compare {
        destination: RegisterIndex,
        operator: CompareOperator,
        left: RegisterIndex,
        right: RegisterIndex,
    },
    Call {
        destination: RegisterIndex,
        function: FunctionIndex,
        arguments: Vec<RegisterIndex>,
    },
    MakeClosure {
        destination: RegisterIndex,
        function: FunctionIndex,
        captures: Vec<CaptureOperand>,
    },
    CallClosure {
        destination: RegisterIndex,
        callee: RegisterIndex,
        arguments: Vec<RegisterIndex>,
    },
    Handle {
        destination: RegisterIndex,
        body_function: FunctionIndex,
        body_captures: Vec<CaptureOperand>,
        clauses: Vec<HandlerClause>,
    },
    MakeTuple {
        destination: RegisterIndex,
        tuple: TypeIndex,
        elements: Vec<RegisterIndex>,
    },
    GetTuple {
        destination: RegisterIndex,
        tuple: RegisterIndex,
        element: u32,
    },
    MakeRecord {
        destination: RegisterIndex,
        record: TypeIndex,
        fields: Vec<RegisterIndex>,
    },
    GetField {
        destination: RegisterIndex,
        record: RegisterIndex,
        field: u32,
    },
    UpdateRecord {
        destination: RegisterIndex,
        base: RegisterIndex,
        updates: Vec<RecordUpdate>,
    },
    MakeVariant {
        destination: RegisterIndex,
        variant: TypeIndex,
        case: u32,
        payload: Option<RegisterIndex>,
    },
    VariantIs {
        destination: RegisterIndex,
        variant: RegisterIndex,
        case: u32,
    },
    GetVariantPayload {
        destination: RegisterIndex,
        variant: RegisterIndex,
        case: u32,
    },
    CellNew {
        destination: RegisterIndex,
        initial: RegisterIndex,
    },
    CellGet {
        destination: RegisterIndex,
        cell: RegisterIndex,
    },
    CellSet {
        destination: RegisterIndex,
        cell: RegisterIndex,
        value: RegisterIndex,
    },
    Intrinsic {
        destination: RegisterIndex,
        intrinsic: Intrinsic,
        arguments: Vec<RegisterIndex>,
    },
    ConsoleWrite {
        destination: RegisterIndex,
        text: RegisterIndex,
    },
}

impl Instruction {
    /// Returns the explicit opcode fixed by the selected bytecode revision.
    #[must_use]
    pub const fn opcode(&self) -> u8 {
        match self {
            Self::Const { .. } => 0x01,
            Self::IntUnary { .. } => 0x02,
            Self::IntBinary { .. } => 0x03,
            Self::Compare { .. } => 0x04,
            Self::Call { .. } => 0x10,
            Self::Intrinsic { .. } => 0x11,
            Self::MakeClosure { .. } => 0x12,
            Self::CallClosure { .. } => 0x13,
            Self::Handle { .. } => 0x1c,
            Self::MakeTuple { .. } => 0x14,
            Self::GetTuple { .. } => 0x15,
            Self::MakeRecord { .. } => 0x16,
            Self::GetField { .. } => 0x17,
            Self::UpdateRecord { .. } => 0x18,
            Self::MakeVariant { .. } => 0x19,
            Self::VariantIs { .. } => 0x1a,
            Self::GetVariantPayload { .. } => 0x1b,
            Self::CellNew { .. } => 0x1d,
            Self::CellGet { .. } => 0x1e,
            Self::CellSet { .. } => 0x1f,
            Self::ConsoleWrite { .. } => 0x20,
        }
    }
}

/// One unverified block terminator; blocks never fall through implicitly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Terminator {
    Jump {
        target: BlockIndex,
        arguments: Vec<RegisterIndex>,
    },
    Branch {
        condition: RegisterIndex,
        true_target: BlockIndex,
        true_arguments: Vec<RegisterIndex>,
        false_target: BlockIndex,
        false_arguments: Vec<RegisterIndex>,
    },
    Return {
        value: RegisterIndex,
    },
}

impl Terminator {
    /// Returns the explicit opcode fixed by RFC-0014.
    #[must_use]
    pub const fn opcode(&self) -> u8 {
        match self {
            Self::Jump { .. } => 0x80,
            Self::Branch { .. } => 0x81,
            Self::Return { .. } => 0x82,
        }
    }
}

/// A typed register assigned when control enters one block.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlockParameter {
    pub register: RegisterIndex,
    pub value_type: TypeIndex,
}

/// One unverified control-flow block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub parameters: Vec<BlockParameter>,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

/// One unverified function-table record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    pub kind: FunctionKind,
    pub module: ModuleIndex,
    pub name: StringIndex,
    pub capture_count: u32,
    pub parameter_types: Vec<TypeIndex>,
    pub result_type: TypeIndex,
    pub effects: Vec<Effect>,
    pub register_count: u32,
    pub blocks: Vec<Block>,
}

/// An original-source half-open UTF-8 byte range.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceSpan {
    start_byte: u64,
    end_byte: u64,
}

impl SourceSpan {
    /// Creates an unverified byte range without rounding or clamping.
    #[must_use]
    pub const fn new(start_byte: u64, end_byte: u64) -> Self {
        Self {
            start_byte,
            end_byte,
        }
    }

    /// Returns the inclusive start byte offset.
    #[must_use]
    pub const fn start_byte(self) -> u64 {
        self.start_byte
    }

    /// Returns the exclusive end byte offset.
    #[must_use]
    pub const fn end_byte(self) -> u64 {
        self.end_byte
    }
}

/// Provenance of one source-map location.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SourceOrigin {
    Direct,
    LoweringDerived,
}

impl SourceOrigin {
    /// Returns the explicit wire tag.
    #[must_use]
    pub const fn tag(self) -> u8 {
        match self {
            Self::Direct => 0,
            Self::LoweringDerived => 1,
        }
    }
}

/// One unverified source-map-table record.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceMapEntry {
    pub function: FunctionIndex,
    pub block: BlockIndex,
    pub ordinal: u32,
    pub source: SourceIndex,
    pub span: SourceSpan,
    pub origin: SourceOrigin,
}

/// Complete table parts used to construct an explicitly unverified program.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramParts {
    pub strings: Vec<String>,
    pub packages: Vec<Package>,
    pub modules: Vec<Module>,
    pub types: Vec<ValueType>,
    pub constants: Vec<Constant>,
    pub sources: Vec<Source>,
    pub functions: Vec<Function>,
    pub entry: FunctionIndex,
    pub source_map: Vec<SourceMapEntry>,
}

/// A bytecode model that carries no verification or execution authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnverifiedProgram {
    strings: Box<[String]>,
    packages: Box<[Package]>,
    modules: Box<[Module]>,
    types: Box<[ValueType]>,
    constants: Box<[Constant]>,
    sources: Box<[Source]>,
    functions: Box<[Function]>,
    entry: FunctionIndex,
    source_map: Box<[SourceMapEntry]>,
}

impl UnverifiedProgram {
    /// Preserves decoded or lowering-produced parts without validating them.
    #[must_use]
    pub fn from_parts(parts: ProgramParts) -> Self {
        Self {
            strings: parts.strings.into_boxed_slice(),
            packages: parts.packages.into_boxed_slice(),
            modules: parts.modules.into_boxed_slice(),
            types: parts.types.into_boxed_slice(),
            constants: parts.constants.into_boxed_slice(),
            sources: parts.sources.into_boxed_slice(),
            functions: parts.functions.into_boxed_slice(),
            entry: parts.entry,
            source_map: parts.source_map.into_boxed_slice(),
        }
    }

    /// Returns the protocol whose model is represented.
    #[must_use]
    pub const fn protocol(&self) -> &'static str {
        BYTECODE_PROTOCOL
    }

    /// Returns unverified string-table entries.
    #[must_use]
    pub const fn strings(&self) -> &[String] {
        &self.strings
    }

    /// Returns unverified package-table entries.
    #[must_use]
    pub const fn packages(&self) -> &[Package] {
        &self.packages
    }

    /// Returns unverified module-table entries.
    #[must_use]
    pub const fn modules(&self) -> &[Module] {
        &self.modules
    }

    /// Returns unverified type-table entries.
    #[must_use]
    pub const fn types(&self) -> &[ValueType] {
        &self.types
    }

    /// Returns unverified constant-table entries.
    #[must_use]
    pub const fn constants(&self) -> &[Constant] {
        &self.constants
    }

    /// Returns unverified source-table entries.
    #[must_use]
    pub const fn sources(&self) -> &[Source] {
        &self.sources
    }

    /// Returns unverified function-table entries.
    #[must_use]
    pub const fn functions(&self) -> &[Function] {
        &self.functions
    }

    /// Returns the unverified entry function index.
    #[must_use]
    pub const fn entry(&self) -> FunctionIndex {
        self.entry
    }

    /// Returns unverified source-map entries.
    #[must_use]
    pub const fn source_map(&self) -> &[SourceMapEntry] {
        &self.source_map
    }
}
