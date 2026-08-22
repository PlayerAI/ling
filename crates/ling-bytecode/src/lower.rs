use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_effects::{Effect as CheckedEffect, EntryErrorKind};
use ling_hir as hir;
use ling_resolve::{
    BindingKey, Builtin, DefinitionId, DefinitionKind, DefinitionOrigin, ExpressionKey, ModuleId,
    ReferenceTarget,
};
use ling_semantic::ProgramSnapshot;
use ling_source::{SourceFile, SourceId, Span};
use ling_types::{Type, TypeId, TypedProgram};
use num_bigint::{BigInt, Sign};
use sha2::{Digest, Sha256};

use crate::{
    Block, BlockIndex, BlockParameter, Capability, CompareOperator, Constant, ConstantIndex,
    DecodeLimits, Effect, Function, FunctionIndex, FunctionKind, Instruction, IntBinaryOperator,
    IntUnaryOperator, IntegerSign, Module, ModuleIndex, PackageReference, ProgramParts,
    RegisterIndex, Source, SourceDigest, SourceIndex, SourceMapEntry, SourceOrigin, SourceSpan,
    StringIndex, Terminator, TypeIndex, UnverifiedProgram, ValueType,
};

mod v1_1;
mod v1_2;

pub use v1_1::{LoweredProgramV1_1, lower_v1_1};
pub use v1_2::{LoweredProgramV1_2, lower_v1_2};

/// Exact original source bytes and the logical name permitted in bytecode.
///
/// The source must be the same immutable [`SourceFile`] used to build the
/// checked snapshot. Lowering validates its source ID and display name before
/// using its original bytes for source metadata and source-map boundaries.
#[derive(Clone, Copy, Debug)]
pub struct LoweringSource<'source> {
    source: &'source SourceFile,
    logical_name: &'source str,
}

impl<'source> LoweringSource<'source> {
    #[must_use]
    pub const fn new(source: &'source SourceFile, logical_name: &'source str) -> Self {
        Self {
            source,
            logical_name,
        }
    }

    #[must_use]
    pub const fn source(&self) -> &'source SourceFile {
        self.source
    }

    #[must_use]
    pub const fn logical_name(&self) -> &'source str {
        self.logical_name
    }
}

/// Failure categories for checked-core to bytecode-1.0 lowering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoweringErrorKind {
    InvalidEntry {
        reason: String,
    },
    InvalidSource {
        source_id: u32,
        logical_name: String,
        reason: String,
    },
    UnsupportedFeature {
        feature: String,
    },
    ResourceLimit {
        resource: String,
        actual: u64,
        maximum: u64,
    },
    InvalidCheckedCore {
        invariant: String,
    },
}

/// A failure-atomic lowering error retaining original source provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweringError {
    kind: LoweringErrorKind,
    source_name: Option<String>,
    span: Option<Span>,
}

impl LoweringError {
    #[must_use]
    pub const fn kind(&self) -> &LoweringErrorKind {
        &self.kind
    }

    #[must_use]
    pub fn source_name(&self) -> Option<&str> {
        self.source_name.as_deref()
    }

    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        self.span
    }

    pub(crate) fn at(source_name: &str, span: Span, kind: LoweringErrorKind) -> Self {
        Self {
            kind,
            source_name: Some(source_name.to_owned()),
            span: Some(span),
        }
    }

    pub(crate) fn without_span(source_name: Option<&str>, kind: LoweringErrorKind) -> Self {
        Self {
            kind,
            source_name: source_name.map(str::to_owned),
            span: None,
        }
    }
}

impl fmt::Display for LoweringError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            LoweringErrorKind::InvalidEntry { reason } => {
                write!(formatter, "invalid bytecode entry: {reason}")
            }
            LoweringErrorKind::InvalidSource {
                source_id,
                logical_name,
                reason,
            } => write!(
                formatter,
                "invalid bytecode source {source_id} ({logical_name}): {reason}"
            ),
            LoweringErrorKind::UnsupportedFeature { feature } => {
                write!(formatter, "bytecode 1.0 does not support {feature}")
            }
            LoweringErrorKind::ResourceLimit {
                resource,
                actual,
                maximum,
            } => write!(
                formatter,
                "bytecode lowering resource {resource} is {actual}, maximum {maximum}"
            ),
            LoweringErrorKind::InvalidCheckedCore { invariant } => {
                write!(
                    formatter,
                    "invalid checked-core lowering input: {invariant}"
                )
            }
        }
    }
}

impl Error for LoweringError {}

/// Canonically ordered bytecode model produced only from checked input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredProgramV1 {
    model: UnverifiedProgram,
}

impl LoweredProgramV1 {
    /// Returns the data model without granting verifier or execution authority.
    #[must_use]
    pub const fn model(&self) -> &UnverifiedProgram {
        &self.model
    }

    pub(crate) const fn new(model: UnverifiedProgram) -> Self {
        Self { model }
    }
}

#[derive(Clone)]
struct FunctionPlan<'program> {
    id: DefinitionId,
    module: &'program ling_resolve::ResolvedModule,
    definition: &'program hir::Definition,
}

#[derive(Clone, Debug)]
struct FunctionSignature {
    parameters: Vec<TypeIndex>,
    result: TypeIndex,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ConstantKey {
    type_index: u32,
    tag: u8,
    payload: Vec<u8>,
}

struct SourcePlan<'source> {
    module: ModuleIndex,
    source: &'source SourceFile,
    logical_name: &'source str,
}

/// Lowers one checked standalone snapshot into canonical bytecode-1.0 model data.
pub fn lower_v1(
    snapshot: &ProgramSnapshot,
    sources: &[LoweringSource<'_>],
) -> Result<LoweredProgramV1, LoweringError> {
    Lowerer::new(snapshot, sources)?.run()
}

struct Lowerer<'snapshot, 'source> {
    snapshot: &'snapshot ProgramSnapshot,
    limits: DecodeLimits,
    modules: Vec<&'snapshot ling_resolve::ResolvedModule>,
    module_indices: BTreeMap<ModuleId, ModuleIndex>,
    source_plans: Vec<SourcePlan<'source>>,
    source_indices: BTreeMap<SourceId, SourceIndex>,
    source_inputs: BTreeMap<SourceId, &'source SourceFile>,
    function_plans: Vec<FunctionPlan<'snapshot>>,
    function_indices: BTreeMap<DefinitionId, FunctionIndex>,
    signatures: BTreeMap<DefinitionId, FunctionSignature>,
    strings: Vec<String>,
    string_indices: BTreeMap<String, StringIndex>,
    constants: Vec<Constant>,
    constant_indices: BTreeMap<ConstantKey, ConstantIndex>,
}

impl<'snapshot, 'source> Lowerer<'snapshot, 'source> {
    fn new(
        snapshot: &'snapshot ProgramSnapshot,
        sources: &'source [LoweringSource<'source>],
    ) -> Result<Self, LoweringError> {
        let limits = DecodeLimits::rfc_0014();
        check_limit("sources", sources.len(), limits.sources())?;

        let checked = snapshot.checked();
        let resolved = checked.typed().resolved();
        let mut modules = resolved.modules().iter().collect::<Vec<_>>();
        modules.sort_by(|left, right| {
            left.hir
                .module
                .name
                .normalized()
                .as_bytes()
                .cmp(right.hir.module.name.normalized().as_bytes())
        });
        check_limit("modules", modules.len(), limits.modules())?;

        let module_indices = modules
            .iter()
            .enumerate()
            .map(|(index, module)| {
                Ok((module.id, ModuleIndex::new(to_u32(index, "module index")?)))
            })
            .collect::<Result<BTreeMap<_, _>, LoweringError>>()?;

        let mut supplied_sources = BTreeMap::new();
        for supplied in sources {
            let id = supplied.source.id();
            if supplied_sources.insert(id, supplied).is_some() {
                return Err(invalid_source(
                    supplied,
                    "duplicate_source_id",
                    supplied.logical_name,
                ));
            }
            validate_logical_name(supplied)?;
        }

        let mut source_plans = Vec::with_capacity(modules.len());
        let mut used_sources = BTreeSet::new();
        for module in &modules {
            let source_id = module.hir.span.source();
            let supplied = supplied_sources.get(&source_id).ok_or_else(|| {
                LoweringError::without_span(
                    Some(&module.hir.source_name),
                    LoweringErrorKind::InvalidSource {
                        source_id: source_id.get(),
                        logical_name: String::new(),
                        reason: "missing_source_snapshot".to_owned(),
                    },
                )
            })?;
            if supplied.source.name() != module.hir.source_name {
                return Err(invalid_source(
                    supplied,
                    "source_display_name_mismatch",
                    supplied.logical_name,
                ));
            }
            used_sources.insert(source_id);
            source_plans.push(SourcePlan {
                module: module_indices[&module.id],
                source: supplied.source,
                logical_name: supplied.logical_name,
            });
        }
        if let Some((_, unused)) = supplied_sources
            .iter()
            .find(|(id, _)| !used_sources.contains(id))
        {
            return Err(invalid_source(
                unused,
                "source_not_in_checked_snapshot",
                unused.logical_name,
            ));
        }
        source_plans.sort_by(|left, right| {
            (left.module, left.logical_name.as_bytes())
                .cmp(&(right.module, right.logical_name.as_bytes()))
        });
        let source_indices = source_plans
            .iter()
            .enumerate()
            .map(|(index, plan)| {
                Ok((
                    plan.source.id(),
                    SourceIndex::new(to_u32(index, "source index")?),
                ))
            })
            .collect::<Result<BTreeMap<_, _>, LoweringError>>()?;
        let source_inputs = source_plans
            .iter()
            .map(|plan| (plan.source.id(), plan.source))
            .collect();

        let mut function_plans = Vec::new();
        for module in &modules {
            for definition in &module.hir.definitions {
                let id = resolved
                    .definition_id(module.id, &definition.name.normalized)
                    .cloned()
                    .ok_or_else(|| {
                        invalid_core_at(
                            module,
                            definition.span,
                            "user definition has no resolved DefinitionId",
                        )
                    })?;
                function_plans.push(FunctionPlan {
                    id,
                    module,
                    definition,
                });
            }
        }
        function_plans.sort_by(|left, right| {
            (
                module_indices[&left.module.id],
                left.definition.name.normalized.as_bytes(),
            )
                .cmp(&(
                    module_indices[&right.module.id],
                    right.definition.name.normalized.as_bytes(),
                ))
        });
        check_limit("functions", function_plans.len(), limits.functions())?;
        let function_indices = function_plans
            .iter()
            .enumerate()
            .map(|(index, plan)| {
                Ok((
                    plan.id.clone(),
                    FunctionIndex::new(to_u32(index, "function index")?),
                ))
            })
            .collect::<Result<BTreeMap<_, _>, LoweringError>>()?;

        let mut signatures = BTreeMap::new();
        for plan in &function_plans {
            if plan.definition.recursive {
                return Err(unsupported_at(plan, plan.definition.span, "recursion"));
            }
            signatures.insert(plan.id.clone(), signature(checked.typed(), plan)?);
            effects(snapshot, plan)?;
        }

        let mut string_set = BTreeSet::new();
        for module in &modules {
            string_set.insert(module.hir.module.name.normalized());
        }
        for plan in &function_plans {
            string_set.insert(plan.definition.name.normalized.clone());
            collect_text_strings(&plan.definition.value, &mut string_set);
        }
        for source in &source_plans {
            string_set.insert(source.logical_name.to_owned());
        }
        check_limit("strings", string_set.len(), limits.string_entries())?;
        for value in &string_set {
            check_limit(
                "string_bytes",
                value.len(),
                limits.bytes_per_string_or_integer(),
            )?;
        }
        let strings = string_set.into_iter().collect::<Vec<_>>();
        let string_indices = strings
            .iter()
            .enumerate()
            .map(|(index, value)| {
                Ok((
                    value.clone(),
                    StringIndex::new(to_u32(index, "string index")?),
                ))
            })
            .collect::<Result<BTreeMap<_, _>, LoweringError>>()?;

        let mut constant_set = BTreeMap::new();
        for plan in &function_plans {
            collect_constants(
                checked.typed(),
                plan,
                &plan.definition.value,
                &string_indices,
                &mut constant_set,
            )?;
        }
        check_limit("constants", constant_set.len(), limits.constants())?;
        let mut constants = Vec::with_capacity(constant_set.len());
        let mut constant_indices = BTreeMap::new();
        for (index, (key, value)) in constant_set.into_iter().enumerate() {
            if let Constant::Int { magnitude, .. } = &value {
                check_limit(
                    "integer_magnitude_bytes",
                    magnitude.len(),
                    limits.bytes_per_string_or_integer(),
                )?;
            }
            let index = ConstantIndex::new(to_u32(index, "constant index")?);
            constant_indices.insert(key, index);
            constants.push(value);
        }

        Ok(Self {
            snapshot,
            limits,
            modules,
            module_indices,
            source_plans,
            source_indices,
            source_inputs,
            function_plans,
            function_indices,
            signatures,
            strings,
            string_indices,
            constants,
            constant_indices,
        })
    }

    fn run(self) -> Result<LoweredProgramV1, LoweringError> {
        let checked = self.snapshot.checked();
        let entry = ling_effects::locate_main(checked).map_err(|error| {
            let reason = match error.kind {
                EntryErrorKind::EntryModuleMustBeMain { .. } => "entry_module_not_main",
                EntryErrorKind::MissingMain => "missing_main",
                EntryErrorKind::InvalidMainSignature { .. } => "invalid_main_signature",
                EntryErrorKind::MainMustHaveUnitPattern => "main_must_have_unit_pattern",
            };
            LoweringError::at(
                &error.source_name,
                error.span,
                LoweringErrorKind::InvalidEntry {
                    reason: reason.to_owned(),
                },
            )
        })?;
        let entry = self.function_indices.get(&entry).copied().ok_or_else(|| {
            LoweringError::without_span(
                None,
                LoweringErrorKind::InvalidCheckedCore {
                    invariant: "validated main is absent from the function table".to_owned(),
                },
            )
        })?;

        let modules = self
            .modules
            .iter()
            .map(|module| {
                Ok(Module {
                    package: PackageReference::Standalone,
                    name: string_index(&self.string_indices, &module.hir.module.name.normalized())?,
                    capabilities: capabilities(self.snapshot, module)?,
                })
            })
            .collect::<Result<Vec<_>, LoweringError>>()?;

        let sources = self
            .source_plans
            .iter()
            .map(|plan| {
                let digest: [u8; 32] =
                    Sha256::digest(plan.source.original_text().as_bytes()).into();
                Ok(Source {
                    module: plan.module,
                    logical_name: string_index(&self.string_indices, plan.logical_name)?,
                    original_byte_length: u64::try_from(plan.source.original_text().len())
                        .map_err(|_| resource_error("source_bytes", u64::MAX, u64::MAX - 1))?,
                    content_sha256: SourceDigest::new(digest),
                })
            })
            .collect::<Result<Vec<_>, LoweringError>>()?;

        let mut functions = Vec::with_capacity(self.function_plans.len());
        let mut source_map = Vec::new();
        let mut executable_locations = 0usize;
        for plan in &self.function_plans {
            let function_index = self.function_indices[&plan.id];
            let signature = &self.signatures[&plan.id];
            let mut lowerer = FunctionLowerer::new(
                self.snapshot,
                plan,
                function_index,
                self.module_indices[&plan.module.id],
                signature,
                &self.function_indices,
                &self.signatures,
                &self.constant_indices,
                &self.string_indices,
                &self.source_indices,
                &self.source_inputs,
                self.limits,
            );
            let function = lowerer.run()?;
            executable_locations = executable_locations
                .checked_add(lowerer.source_map.len())
                .ok_or_else(|| resource_error("executable_locations", u64::MAX, u64::MAX - 1))?;
            check_limit(
                "executable_locations",
                executable_locations,
                self.limits.executable_locations(),
            )?;
            source_map.append(&mut lowerer.source_map);
            functions.push(function);
        }

        Ok(LoweredProgramV1::new(UnverifiedProgram::from_parts(
            ProgramParts {
                strings: self.strings,
                packages: Vec::new(),
                modules,
                types: vec![
                    ValueType::Unit,
                    ValueType::Bool,
                    ValueType::Int,
                    ValueType::Text,
                ],
                constants: self.constants,
                sources,
                functions,
                entry,
                source_map,
            },
        )))
    }
}

#[allow(clippy::too_many_arguments)]
struct FunctionLowerer<'a> {
    snapshot: &'a ProgramSnapshot,
    plan: &'a FunctionPlan<'a>,
    function_index: FunctionIndex,
    module_index: ModuleIndex,
    signature: &'a FunctionSignature,
    function_indices: &'a BTreeMap<DefinitionId, FunctionIndex>,
    signatures: &'a BTreeMap<DefinitionId, FunctionSignature>,
    constant_indices: &'a BTreeMap<ConstantKey, ConstantIndex>,
    string_indices: &'a BTreeMap<String, StringIndex>,
    source_indices: &'a BTreeMap<SourceId, SourceIndex>,
    source_inputs: &'a BTreeMap<SourceId, &'a SourceFile>,
    limits: DecodeLimits,
    next_register: u32,
    environment: BTreeMap<BindingKey, RegisterIndex>,
    parameters: Vec<BlockParameter>,
    instructions: Vec<Instruction>,
    source_map: Vec<SourceMapEntry>,
}

impl<'a> FunctionLowerer<'a> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        snapshot: &'a ProgramSnapshot,
        plan: &'a FunctionPlan<'a>,
        function_index: FunctionIndex,
        module_index: ModuleIndex,
        signature: &'a FunctionSignature,
        function_indices: &'a BTreeMap<DefinitionId, FunctionIndex>,
        signatures: &'a BTreeMap<DefinitionId, FunctionSignature>,
        constant_indices: &'a BTreeMap<ConstantKey, ConstantIndex>,
        string_indices: &'a BTreeMap<String, StringIndex>,
        source_indices: &'a BTreeMap<SourceId, SourceIndex>,
        source_inputs: &'a BTreeMap<SourceId, &'a SourceFile>,
        limits: DecodeLimits,
    ) -> Self {
        Self {
            snapshot,
            plan,
            function_index,
            module_index,
            signature,
            function_indices,
            signatures,
            constant_indices,
            string_indices,
            source_indices,
            source_inputs,
            limits,
            next_register: 0,
            environment: BTreeMap::new(),
            parameters: Vec::new(),
            instructions: Vec::new(),
            source_map: Vec::new(),
        }
    }

    fn run(&mut self) -> Result<Function, LoweringError> {
        if self.plan.definition.parameters.len() != self.signature.parameters.len() {
            return Err(self.invalid_core(
                self.plan.definition.span,
                "checked function parameter count disagrees with its type",
            ));
        }
        for (pattern, value_type) in self
            .plan
            .definition
            .parameters
            .iter()
            .zip(&self.signature.parameters)
        {
            let register = self.new_register(pattern.span)?;
            self.parameters.push(BlockParameter {
                register,
                value_type: *value_type,
            });
            match &pattern.kind {
                hir::PatternKind::Unit if *value_type == TypeIndex::new(0) => {}
                hir::PatternKind::Binding { id, .. } => {
                    self.environment
                        .insert(BindingKey::new(self.plan.module.id, *id), register);
                }
                hir::PatternKind::Unit => {
                    return Err(self.invalid_core(pattern.span, "Unit pattern has a non-Unit type"));
                }
                _ => return Err(self.unsupported(pattern.span, "parameter destructuring")),
            }
        }

        let mut environment = self.environment.clone();
        let result = self.lower_expression(&self.plan.definition.value, &mut environment)?;
        let ordinal = to_u32(self.instructions.len(), "terminator ordinal")?;
        self.push_source_map(
            self.plan.definition.value.span,
            ordinal,
            SourceOrigin::LoweringDerived,
        )?;

        Ok(Function {
            kind: FunctionKind::Named,
            module: self.module_index,
            name: string_index(self.string_indices, &self.plan.definition.name.normalized)?,
            capture_count: 0,
            parameter_types: self.signature.parameters.clone(),
            result_type: self.signature.result,
            effects: effects(self.snapshot, self.plan)?,
            register_count: self.next_register,
            blocks: vec![Block {
                parameters: std::mem::take(&mut self.parameters),
                instructions: std::mem::take(&mut self.instructions),
                terminator: Terminator::Return { value: result },
            }],
        })
    }

    fn lower_expression(
        &mut self,
        expression: &hir::Expression,
        environment: &mut BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        self.expression_type(expression)?;
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                let mut local = environment.clone();
                let mut result = None;
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            if binding.recursive {
                                return Err(self.unsupported(binding.span, "local recursion"));
                            }
                            if binding.mutable {
                                return Err(self.unsupported(binding.span, "mutable local binding"));
                            }
                            if !binding.parameters.is_empty() {
                                return Err(self.unsupported(binding.span, "local function"));
                            }
                            let value = self.lower_expression(&binding.value, &mut local)?;
                            local.insert(BindingKey::new(self.plan.module.id, binding.id), value);
                            result = None;
                        }
                        hir::SequenceElement::Expression(item) => {
                            result = Some(self.lower_expression(item, &mut local)?);
                        }
                    }
                }
                match result {
                    Some(result) => Ok(result),
                    None => self.emit_constant(expression, Constant::Unit),
                }
            }
            hir::ExpressionKind::Handle { .. } => Err(self.unsupported(expression.span, "handler")),
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => self.lower_application(expression, function, arguments, environment),
            hir::ExpressionKind::Name { reference, .. }
            | hir::ExpressionKind::Projection { reference, .. } => {
                self.lower_reference(expression, *reference, environment)
            }
            hir::ExpressionKind::Literal(literal) => {
                let constant = literal_constant(
                    self.snapshot.checked().typed(),
                    self.plan,
                    expression,
                    literal,
                    self.string_indices,
                )?;
                self.emit_constant(expression, constant)
            }
            hir::ExpressionKind::Unit => self.emit_constant(expression, Constant::Unit),
            hir::ExpressionKind::If { .. } => Err(self.unsupported(expression.span, "if")),
            hir::ExpressionKind::Match { .. } => Err(self.unsupported(expression.span, "match")),
            hir::ExpressionKind::Assignment { .. } => {
                Err(self.unsupported(expression.span, "mutable assignment"))
            }
            hir::ExpressionKind::Binary { .. } => {
                Err(self.unsupported(expression.span, "scalar operators"))
            }
            hir::ExpressionKind::Unary { .. } => {
                Err(self.unsupported(expression.span, "integer unary operators"))
            }
            hir::ExpressionKind::Tuple(_) => Err(self.unsupported(expression.span, "tuple")),
            hir::ExpressionKind::Record(_) | hir::ExpressionKind::RecordUpdate { .. } => {
                Err(self.unsupported(expression.span, "record"))
            }
            hir::ExpressionKind::List(_) => Err(self.unsupported(expression.span, "list")),
        }
    }

    fn lower_application(
        &mut self,
        expression: &hir::Expression,
        function: &hir::Expression,
        arguments: &[hir::Expression],
        environment: &mut BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        check_limit(
            "operation_arguments",
            arguments.len(),
            self.limits.arguments_per_operation(),
        )?;
        let reference = match &function.kind {
            hir::ExpressionKind::Name { reference, .. }
            | hir::ExpressionKind::Projection { reference, .. } => *reference,
            _ => return Err(self.unsupported(function.span, "indirect function value")),
        };
        let target = self
            .snapshot
            .checked()
            .typed()
            .resolved()
            .reference(self.plan.module.id, reference)
            .cloned()
            .ok_or_else(|| self.invalid_core(function.span, "call target is unresolved"))?;
        let ReferenceTarget::Definition(definition) = target else {
            return Err(self.unsupported(function.span, "local function value"));
        };
        let info = self
            .snapshot
            .checked()
            .typed()
            .resolved()
            .definition(&definition)
            .ok_or_else(|| self.invalid_core(function.span, "call definition is absent"))?;

        let mut lowered_arguments = Vec::with_capacity(arguments.len());
        for argument in arguments {
            lowered_arguments.push(self.lower_expression(argument, environment)?);
        }

        match info.origin {
            DefinitionOrigin::User { .. } => {
                let signature = self.signatures.get(&definition).ok_or_else(|| {
                    self.invalid_core(function.span, "called user function has no signature")
                })?;
                if signature.parameters.len() != lowered_arguments.len() {
                    return Err(self.invalid_core(
                        expression.span,
                        "checked direct call arity disagrees with the callee",
                    ));
                }
                let function =
                    self.function_indices
                        .get(&definition)
                        .copied()
                        .ok_or_else(|| {
                            self.invalid_core(function.span, "called user function is absent")
                        })?;
                let destination = self.new_register(expression.span)?;
                self.push_instruction(
                    Instruction::Call {
                        destination,
                        function,
                        arguments: lowered_arguments,
                    },
                    expression.span,
                )?;
                Ok(destination)
            }
            DefinitionOrigin::Builtin(Builtin::ConsoleWrite) => {
                let [text] = lowered_arguments.as_slice() else {
                    return Err(self.invalid_core(
                        expression.span,
                        "checked Console.write call does not have one argument",
                    ));
                };
                let destination = self.new_register(expression.span)?;
                self.push_instruction(
                    Instruction::ConsoleWrite {
                        destination,
                        text: *text,
                    },
                    expression.span,
                )?;
                Ok(destination)
            }
            DefinitionOrigin::Builtin(builtin) => {
                Err(self.unsupported(expression.span, builtin.qualified_name()))
            }
            DefinitionOrigin::Prelude(_) => {
                Err(self.unsupported(expression.span, "Prelude constructor call"))
            }
        }
    }

    fn lower_reference(
        &mut self,
        expression: &hir::Expression,
        reference: hir::ReferenceId,
        environment: &BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        let target = self
            .snapshot
            .checked()
            .typed()
            .resolved()
            .reference(self.plan.module.id, reference)
            .cloned()
            .ok_or_else(|| self.invalid_core(expression.span, "value reference is unresolved"))?;
        match target {
            ReferenceTarget::Binding(binding) => {
                environment.get(&binding).copied().ok_or_else(|| {
                    self.invalid_core(expression.span, "referenced local binding has no register")
                })
            }
            ReferenceTarget::Definition(definition) => {
                let info = self
                    .snapshot
                    .checked()
                    .typed()
                    .resolved()
                    .definition(&definition)
                    .ok_or_else(|| {
                        self.invalid_core(expression.span, "referenced definition is absent")
                    })?;
                if !matches!(info.origin, DefinitionOrigin::User { .. }) {
                    return Err(self.unsupported(expression.span, "first-class builtin value"));
                }
                let signature = self.signatures.get(&definition).ok_or_else(|| {
                    self.invalid_core(expression.span, "referenced definition has no signature")
                })?;
                if !signature.parameters.is_empty() {
                    return Err(self.unsupported(expression.span, "first-class function value"));
                }
                let function =
                    self.function_indices
                        .get(&definition)
                        .copied()
                        .ok_or_else(|| {
                            self.invalid_core(
                                expression.span,
                                "referenced definition is not encodable",
                            )
                        })?;
                let destination = self.new_register(expression.span)?;
                self.push_instruction(
                    Instruction::Call {
                        destination,
                        function,
                        arguments: Vec::new(),
                    },
                    expression.span,
                )?;
                Ok(destination)
            }
        }
    }

    fn emit_constant(
        &mut self,
        expression: &hir::Expression,
        constant: Constant,
    ) -> Result<RegisterIndex, LoweringError> {
        let key = constant_key(&constant)?;
        let constant = self.constant_indices.get(&key).copied().ok_or_else(|| {
            self.invalid_core(
                expression.span,
                "lowered constant is absent from its canonical table",
            )
        })?;
        let destination = self.new_register(expression.span)?;
        self.push_instruction(
            Instruction::Const {
                destination,
                constant,
            },
            expression.span,
        )?;
        Ok(destination)
    }

    fn expression_type(&self, expression: &hir::Expression) -> Result<TypeIndex, LoweringError> {
        let typed = self.snapshot.checked().typed();
        let value = typed
            .expression_type(ExpressionKey::new(self.plan.module.id, expression.id))
            .ok_or_else(|| self.invalid_core(expression.span, "expression type is absent"))?;
        type_index(typed, value, self.plan, expression.span)
    }

    fn new_register(&mut self, span: Span) -> Result<RegisterIndex, LoweringError> {
        if self.next_register >= self.limits.registers_per_function() {
            return Err(LoweringError::at(
                &self.plan.module.hir.source_name,
                span,
                LoweringErrorKind::ResourceLimit {
                    resource: "registers_per_function".to_owned(),
                    actual: u64::from(self.next_register) + 1,
                    maximum: u64::from(self.limits.registers_per_function()),
                },
            ));
        }
        let register = RegisterIndex::new(self.next_register);
        self.next_register += 1;
        Ok(register)
    }

    fn push_instruction(
        &mut self,
        instruction: Instruction,
        span: Span,
    ) -> Result<(), LoweringError> {
        let next = self
            .instructions
            .len()
            .checked_add(1)
            .ok_or_else(|| resource_error("executable_locations", u64::MAX, u64::MAX - 1))?;
        check_limit(
            "executable_locations",
            next,
            self.limits.executable_locations(),
        )?;
        let ordinal = to_u32(self.instructions.len(), "instruction ordinal")?;
        self.instructions.push(instruction);
        self.push_source_map(span, ordinal, SourceOrigin::Direct)
    }

    fn push_source_map(
        &mut self,
        span: Span,
        ordinal: u32,
        origin: SourceOrigin,
    ) -> Result<(), LoweringError> {
        let source = self.source_inputs.get(&span.source()).ok_or_else(|| {
            self.invalid_core(span, "source-map span has no exact source snapshot")
        })?;
        let start = span.start().get() as usize;
        let end = span.end().get() as usize;
        let original = source.original_text();
        if end > original.len()
            || start > end
            || !original.is_char_boundary(start)
            || !original.is_char_boundary(end)
        {
            return Err(
                self.invalid_core(span, "source-map span is not an original UTF-8 byte range")
            );
        }
        let source_index = self
            .source_indices
            .get(&span.source())
            .copied()
            .ok_or_else(|| self.invalid_core(span, "source-map source has no table index"))?;
        self.source_map.push(SourceMapEntry {
            function: self.function_index,
            block: BlockIndex::new(0),
            ordinal,
            source: source_index,
            span: SourceSpan::new(start as u64, end as u64),
            origin,
        });
        Ok(())
    }

    fn unsupported(&self, span: Span, feature: &str) -> LoweringError {
        LoweringError::at(
            &self.plan.module.hir.source_name,
            span,
            LoweringErrorKind::UnsupportedFeature {
                feature: feature.to_owned(),
            },
        )
    }

    fn invalid_core(&self, span: Span, invariant: &str) -> LoweringError {
        invalid_core_at(self.plan.module, span, invariant)
    }
}

fn signature(
    typed: &TypedProgram,
    plan: &FunctionPlan<'_>,
) -> Result<FunctionSignature, LoweringError> {
    let value = typed.definition_type(&plan.id).ok_or_else(|| {
        invalid_core_at(
            plan.module,
            plan.definition.span,
            "definition type is absent",
        )
    })?;
    if plan.definition.parameters.is_empty() {
        return Ok(FunctionSignature {
            parameters: Vec::new(),
            result: type_index(typed, value, plan, plan.definition.span)?,
        });
    }
    let Type::Function { parameters, result } = typed.arena().get(value) else {
        return Err(invalid_core_at(
            plan.module,
            plan.definition.span,
            "parameterized definition does not have a function type",
        ));
    };
    let parameters = parameters
        .iter()
        .map(|value| type_index(typed, *value, plan, plan.definition.span))
        .collect::<Result<Vec<_>, _>>()?;
    let result = type_index(typed, *result, plan, plan.definition.span)?;
    Ok(FunctionSignature { parameters, result })
}

fn type_index(
    typed: &TypedProgram,
    value: TypeId,
    plan: &FunctionPlan<'_>,
    span: Span,
) -> Result<TypeIndex, LoweringError> {
    match typed.arena().get(value) {
        Type::Unit => Ok(TypeIndex::new(0)),
        Type::Bool => Ok(TypeIndex::new(1)),
        Type::Int => Ok(TypeIndex::new(2)),
        Type::Text => Ok(TypeIndex::new(3)),
        Type::Float64 => Err(unsupported_at(plan, span, "Float64")),
        Type::Tuple(_) => Err(unsupported_at(plan, span, "tuple")),
        Type::List(_) => Err(unsupported_at(plan, span, "list")),
        Type::Function { .. } => Err(unsupported_at(plan, span, "first-class function")),
        Type::NominalRecord { .. } => Err(unsupported_at(plan, span, "record")),
        Type::NominalVariant { .. } => Err(unsupported_at(plan, span, "variant")),
        Type::Variable(_) => Err(unsupported_at(plan, span, "polymorphic function")),
        Type::Error => Err(invalid_core_at(
            plan.module,
            span,
            "checked expression retains an error type",
        )),
    }
}

fn effects(
    snapshot: &ProgramSnapshot,
    plan: &FunctionPlan<'_>,
) -> Result<Vec<Effect>, LoweringError> {
    let row = snapshot
        .checked()
        .definition_effect(&plan.id)
        .ok_or_else(|| {
            invalid_core_at(
                plan.module,
                plan.definition.span,
                "definition Effect row is absent",
            )
        })?;
    row.effects()
        .map(|effect| match effect {
            CheckedEffect::ConsoleWrite => Ok(Effect::ConsoleWrite),
            CheckedEffect::State { .. } => {
                Err(unsupported_at(plan, plan.definition.span, "State Effect"))
            }
        })
        .collect()
}

fn capabilities(
    snapshot: &ProgramSnapshot,
    module: &ling_resolve::ResolvedModule,
) -> Result<Vec<Capability>, LoweringError> {
    snapshot
        .checked()
        .module_capabilities(module.id)
        .ok_or_else(|| {
            invalid_core_at(
                module,
                module.hir.module.span,
                "module Capability closure is absent",
            )
        })?
        .iter()
        .map(|capability| match capability {
            ling_effects::Capability::ConsoleWrite => Ok(Capability::ConsoleWrite),
        })
        .collect()
}

fn collect_text_strings(expression: &hir::Expression, strings: &mut BTreeSet<String>) {
    match &expression.kind {
        hir::ExpressionKind::Sequence(elements) => {
            for element in elements {
                match element {
                    hir::SequenceElement::Let(binding) => {
                        collect_text_strings(&binding.value, strings);
                    }
                    hir::SequenceElement::Expression(expression) => {
                        collect_text_strings(expression, strings);
                    }
                }
            }
        }
        hir::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            collect_text_strings(condition, strings);
            collect_text_strings(then_branch, strings);
            collect_text_strings(else_branch, strings);
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            collect_text_strings(scrutinee, strings);
            for case in cases {
                if let Some(guard) = &case.guard {
                    collect_text_strings(guard, strings);
                }
                collect_text_strings(&case.body, strings);
            }
        }
        hir::ExpressionKind::Assignment { value, .. } => collect_text_strings(value, strings),
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            collect_text_strings(function, strings);
            for argument in arguments {
                collect_text_strings(argument, strings);
            }
        }
        hir::ExpressionKind::Projection { target, .. } => collect_text_strings(target, strings),
        hir::ExpressionKind::Binary { left, right, .. } => {
            collect_text_strings(left, strings);
            collect_text_strings(right, strings);
        }
        hir::ExpressionKind::Unary { operand, .. } => collect_text_strings(operand, strings),
        hir::ExpressionKind::Literal(hir::Literal::Text(value)) => {
            strings.insert(value.clone());
        }
        hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
            for element in elements {
                collect_text_strings(element, strings);
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                collect_text_strings(&field.value, strings);
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            collect_text_strings(base, strings);
            for field in fields {
                collect_text_strings(&field.value, strings);
            }
        }
        hir::ExpressionKind::Handle { .. } => {}
        hir::ExpressionKind::Name { .. }
        | hir::ExpressionKind::Literal(_)
        | hir::ExpressionKind::Unit => {}
    }
}

fn collect_constants(
    typed: &TypedProgram,
    plan: &FunctionPlan<'_>,
    expression: &hir::Expression,
    strings: &BTreeMap<String, StringIndex>,
    constants: &mut BTreeMap<ConstantKey, Constant>,
) -> Result<(), LoweringError> {
    type_index(
        typed,
        typed
            .expression_type(ExpressionKey::new(plan.module.id, expression.id))
            .ok_or_else(|| {
                invalid_core_at(plan.module, expression.span, "expression type is absent")
            })?,
        plan,
        expression.span,
    )?;
    match &expression.kind {
        hir::ExpressionKind::Sequence(elements) => {
            let mut has_final_expression = false;
            for element in elements {
                match element {
                    hir::SequenceElement::Let(binding) => {
                        if binding.recursive {
                            return Err(unsupported_at(plan, binding.span, "local recursion"));
                        }
                        if binding.mutable {
                            return Err(unsupported_at(
                                plan,
                                binding.span,
                                "mutable local binding",
                            ));
                        }
                        if !binding.parameters.is_empty() {
                            return Err(unsupported_at(plan, binding.span, "local function"));
                        }
                        collect_constants(typed, plan, &binding.value, strings, constants)?;
                        has_final_expression = false;
                    }
                    hir::SequenceElement::Expression(item) => {
                        collect_constants(typed, plan, item, strings, constants)?;
                        has_final_expression = true;
                    }
                }
            }
            if !has_final_expression {
                insert_constant(constants, Constant::Unit)?;
            }
        }
        hir::ExpressionKind::Application { arguments, .. } => {
            for argument in arguments {
                collect_constants(typed, plan, argument, strings, constants)?;
            }
        }
        hir::ExpressionKind::Name { .. } | hir::ExpressionKind::Projection { .. } => {}
        hir::ExpressionKind::Literal(literal) => {
            insert_constant(
                constants,
                literal_constant(typed, plan, expression, literal, strings)?,
            )?;
        }
        hir::ExpressionKind::Unit => insert_constant(constants, Constant::Unit)?,
        hir::ExpressionKind::If { .. } => {
            return Err(unsupported_at(plan, expression.span, "if"));
        }
        hir::ExpressionKind::Match { .. } => {
            return Err(unsupported_at(plan, expression.span, "match"));
        }
        hir::ExpressionKind::Assignment { .. } => {
            return Err(unsupported_at(plan, expression.span, "mutable assignment"));
        }
        hir::ExpressionKind::Binary { .. } => {
            return Err(unsupported_at(plan, expression.span, "scalar operators"));
        }
        hir::ExpressionKind::Unary { .. } => {
            return Err(unsupported_at(
                plan,
                expression.span,
                "integer unary operators",
            ));
        }
        hir::ExpressionKind::Tuple(_) => {
            return Err(unsupported_at(plan, expression.span, "tuple"));
        }
        hir::ExpressionKind::Record(_) | hir::ExpressionKind::RecordUpdate { .. } => {
            return Err(unsupported_at(plan, expression.span, "record"));
        }
        hir::ExpressionKind::List(_) => {
            return Err(unsupported_at(plan, expression.span, "list"));
        }
        hir::ExpressionKind::Handle { .. } => {
            return Err(unsupported_at(plan, expression.span, "handler"));
        }
    }
    Ok(())
}

fn literal_constant(
    typed: &TypedProgram,
    plan: &FunctionPlan<'_>,
    expression: &hir::Expression,
    literal: &hir::Literal,
    strings: &BTreeMap<String, StringIndex>,
) -> Result<Constant, LoweringError> {
    match literal {
        hir::Literal::Integer { .. } => {
            let value = typed
                .integer(ExpressionKey::new(plan.module.id, expression.id))
                .ok_or_else(|| {
                    invalid_core_at(plan.module, expression.span, "typed integer is absent")
                })?;
            let (sign, magnitude) = value.to_bytes_be();
            let sign = match sign {
                Sign::NoSign => IntegerSign::Zero,
                Sign::Plus => IntegerSign::Positive,
                Sign::Minus => IntegerSign::Negative,
            };
            Ok(Constant::Int { sign, magnitude })
        }
        hir::Literal::Float(_) => Err(unsupported_at(plan, expression.span, "Float64")),
        hir::Literal::Text(value) => Ok(Constant::Text(string_index(strings, value)?)),
        hir::Literal::Boolean(value) => Ok(Constant::Bool(*value)),
    }
}

fn insert_constant(
    constants: &mut BTreeMap<ConstantKey, Constant>,
    constant: Constant,
) -> Result<(), LoweringError> {
    constants.insert(constant_key(&constant)?, constant);
    Ok(())
}

fn constant_key(constant: &Constant) -> Result<ConstantKey, LoweringError> {
    let (type_index, payload) = match constant {
        Constant::Unit => (0, Vec::new()),
        Constant::Bool(value) => (1, vec![u8::from(*value)]),
        Constant::Int { sign, magnitude } => {
            let mut payload = vec![sign.tag(), 0, 0, 0];
            payload.extend_from_slice(
                &to_u32(magnitude.len(), "integer magnitude length")?.to_le_bytes(),
            );
            payload.extend_from_slice(magnitude);
            (2, payload)
        }
        Constant::Text(value) => (3, value.get().to_le_bytes().to_vec()),
    };
    Ok(ConstantKey {
        type_index,
        tag: constant.tag(),
        payload,
    })
}

fn validate_logical_name(source: &LoweringSource<'_>) -> Result<(), LoweringError> {
    let value = source.logical_name;
    if let Err(reason) = crate::path::validate_logical_name(value) {
        return Err(invalid_source(source, reason.as_str(), value));
    }
    Ok(())
}

fn invalid_source(source: &LoweringSource<'_>, reason: &str, logical_name: &str) -> LoweringError {
    LoweringError::without_span(
        Some(source.source.name()),
        LoweringErrorKind::InvalidSource {
            source_id: source.source.id().get(),
            logical_name: logical_name.to_owned(),
            reason: reason.to_owned(),
        },
    )
}

fn unsupported_at(plan: &FunctionPlan<'_>, span: Span, feature: &str) -> LoweringError {
    LoweringError::at(
        &plan.module.hir.source_name,
        span,
        LoweringErrorKind::UnsupportedFeature {
            feature: feature.to_owned(),
        },
    )
}

fn invalid_core_at(
    module: &ling_resolve::ResolvedModule,
    span: Span,
    invariant: &str,
) -> LoweringError {
    LoweringError::at(
        &module.hir.source_name,
        span,
        LoweringErrorKind::InvalidCheckedCore {
            invariant: invariant.to_owned(),
        },
    )
}

fn string_index(
    strings: &BTreeMap<String, StringIndex>,
    value: &str,
) -> Result<StringIndex, LoweringError> {
    strings.get(value).copied().ok_or_else(|| {
        LoweringError::without_span(
            None,
            LoweringErrorKind::InvalidCheckedCore {
                invariant: "canonical string table omitted a referenced string".to_owned(),
            },
        )
    })
}

fn check_limit(resource: &str, actual: usize, maximum: u32) -> Result<(), LoweringError> {
    if actual <= maximum as usize {
        Ok(())
    } else {
        Err(resource_error(
            resource,
            u64::try_from(actual).unwrap_or(u64::MAX),
            u64::from(maximum),
        ))
    }
}

fn resource_error(resource: &str, actual: u64, maximum: u64) -> LoweringError {
    LoweringError::without_span(
        None,
        LoweringErrorKind::ResourceLimit {
            resource: resource.to_owned(),
            actual,
            maximum,
        },
    )
}

fn to_u32(value: usize, label: &str) -> Result<u32, LoweringError> {
    u32::try_from(value).map_err(|_| {
        LoweringError::without_span(
            None,
            LoweringErrorKind::InvalidCheckedCore {
                invariant: format!("{label} does not fit u32"),
            },
        )
    })
}
