use super::*;

use ling_effects::{CheckedFunctionType, EffectRow};

use crate::{CaptureOperand, Intrinsic, RecordField, RecordUpdate, VariantCase};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredProgramV1_1 {
    model: UnverifiedProgram,
}

impl LoweredProgramV1_1 {
    #[must_use]
    pub const fn model(&self) -> &UnverifiedProgram {
        &self.model
    }

    pub(crate) const fn new(model: UnverifiedProgram) -> Self {
        Self { model }
    }
}

pub fn lower_v1_1(
    snapshot: &ProgramSnapshot,
    sources: &[LoweringSource<'_>],
) -> Result<LoweredProgramV1_1, LoweringError> {
    ClosureLowerer::new(snapshot, sources)?.run()
}

pub(crate) fn lower_v1_2_model(
    snapshot: &ProgramSnapshot,
    sources: &[LoweringSource<'_>],
) -> Result<UnverifiedProgram, LoweringError> {
    ClosureLowerer::new_with_mode(snapshot, sources, true)?.run_model()
}

#[derive(Clone)]
struct NamedPlan<'a> {
    id: DefinitionId,
    module: &'a ling_resolve::ResolvedModule,
    definition: &'a hir::Definition,
}

#[derive(Clone)]
struct LocalPlan<'a> {
    key: BindingKey,
    module: &'a ling_resolve::ResolvedModule,
    binding: &'a hir::LocalBinding,
    label: String,
}

#[derive(Clone)]
struct BuiltinPlan<'a> {
    module: &'a ling_resolve::ResolvedModule,
    builtin: Builtin,
    label: String,
    span: Span,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum TypeKey {
    Unit,
    Bool,
    Int,
    Text,
    Function {
        parameters: Vec<TypeKey>,
        result: Box<TypeKey>,
        effects: Vec<Effect>,
    },
    Tuple(Vec<TypeKey>),
    Record {
        module: String,
        name: String,
        arguments: Vec<TypeKey>,
        fields: Vec<RecordFieldKey>,
    },
    Variant {
        module: String,
        name: String,
        arguments: Vec<TypeKey>,
        cases: Vec<VariantCaseKey>,
    },
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct RecordFieldKey {
    name: String,
    mutable: bool,
    value_type: TypeKey,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct VariantCaseKey {
    name: String,
    payload: Option<TypeKey>,
}

impl TypeKey {
    fn function(
        parameters: Vec<Self>,
        result: Self,
        effects: Vec<Effect>,
    ) -> Result<Self, LoweringError> {
        if parameters.is_empty() {
            return Err(invalid_without_span(
                "a bytecode function value type has no callable parameters",
            ));
        }
        Ok(Self::Function {
            parameters,
            result: Box::new(result),
            effects,
        })
    }
}

#[derive(Clone, Debug)]
struct SignatureKey {
    parameters: Vec<TypeKey>,
    result: TypeKey,
    effects: Vec<Effect>,
}

impl SignatureKey {
    fn complete_type(&self) -> Result<TypeKey, LoweringError> {
        TypeKey::function(
            self.parameters.clone(),
            self.result.clone(),
            self.effects.clone(),
        )
    }

    fn suffix_type(&self, applied: usize) -> Result<TypeKey, LoweringError> {
        TypeKey::function(
            self.parameters
                .get(applied..)
                .ok_or_else(|| invalid_without_span("partial application exceeds arity"))?
                .to_vec(),
            self.result.clone(),
            self.effects.clone(),
        )
    }
}

#[derive(Clone)]
struct CapturePlan {
    key: BindingKey,
    self_reference: bool,
    value_type: TypeKey,
}

#[derive(Clone)]
enum OrderedPlan {
    Named(DefinitionId),
    Local(BindingKey),
    Builtin(ModuleId, Builtin),
}

struct ClosureLowerer<'snapshot, 'source> {
    snapshot: &'snapshot ProgramSnapshot,
    limits: DecodeLimits,
    aggregate_mode: bool,
    modules: Vec<&'snapshot ling_resolve::ResolvedModule>,
    module_indices: BTreeMap<ModuleId, ModuleIndex>,
    source_plans: Vec<SourcePlan<'source>>,
    source_indices: BTreeMap<SourceId, SourceIndex>,
    source_inputs: BTreeMap<SourceId, &'source SourceFile>,
    named: BTreeMap<DefinitionId, NamedPlan<'snapshot>>,
    locals: BTreeMap<BindingKey, LocalPlan<'snapshot>>,
    builtins: BTreeMap<(ModuleId, Builtin), BuiltinPlan<'snapshot>>,
    captures: BTreeMap<BindingKey, Vec<CapturePlan>>,
    named_signatures: BTreeMap<DefinitionId, SignatureKey>,
    local_signatures: BTreeMap<BindingKey, SignatureKey>,
    builtin_signatures: BTreeMap<Builtin, SignatureKey>,
    ordered: Vec<OrderedPlan>,
    function_indices: BTreeMap<DefinitionId, FunctionIndex>,
    local_indices: BTreeMap<BindingKey, FunctionIndex>,
    builtin_indices: BTreeMap<(ModuleId, Builtin), FunctionIndex>,
    types: Vec<ValueType>,
    type_indices: BTreeMap<TypeKey, TypeIndex>,
    strings: Vec<String>,
    string_indices: BTreeMap<String, StringIndex>,
    constants: Vec<Constant>,
    constant_indices: BTreeMap<ConstantKey, ConstantIndex>,
}

impl<'snapshot, 'source> ClosureLowerer<'snapshot, 'source> {
    fn new(
        snapshot: &'snapshot ProgramSnapshot,
        sources: &'source [LoweringSource<'source>],
    ) -> Result<Self, LoweringError> {
        Self::new_with_mode(snapshot, sources, false)
    }

    fn new_with_mode(
        snapshot: &'snapshot ProgramSnapshot,
        sources: &'source [LoweringSource<'source>],
        aggregate_mode: bool,
    ) -> Result<Self, LoweringError> {
        let limits = if aggregate_mode {
            DecodeLimits::rfc_0016()
        } else {
            DecodeLimits::rfc_0015()
        };
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
        let module_count = modules.len() + usize::from(aggregate_mode);
        check_limit("modules", module_count, limits.modules())?;
        let module_indices = modules
            .iter()
            .enumerate()
            .map(|(index, module)| {
                Ok((
                    module.id,
                    ModuleIndex::new(to_u32(index + usize::from(aggregate_mode), "module index")?),
                ))
            })
            .collect::<Result<BTreeMap<_, _>, LoweringError>>()?;

        let (source_plans, source_indices, source_inputs) =
            prepare_sources(&modules, &module_indices, sources, limits)?;

        let mut named = BTreeMap::new();
        for module in &modules {
            for definition in &module.hir.definitions {
                if definition.mutable {
                    return Err(unsupported_module(
                        module,
                        definition.span,
                        "mutable top-level binding",
                    ));
                }
                let id = resolved
                    .definition_id(module.id, &definition.name.normalized)
                    .cloned()
                    .ok_or_else(|| {
                        invalid_module(
                            module,
                            definition.span,
                            "user definition has no resolved DefinitionId",
                        )
                    })?;
                named.insert(
                    id.clone(),
                    NamedPlan {
                        id,
                        module,
                        definition,
                    },
                );
            }
        }
        let mut locals = BTreeMap::new();
        let mut local_bindings = BTreeMap::new();
        let mut builtins = BTreeMap::new();
        let mut binding_order = BTreeMap::new();
        let mut order = 0_usize;
        let mut ordinals = BTreeMap::<ModuleId, u64>::new();
        for module in &modules {
            for definition in &module.hir.definitions {
                for pattern in &definition.parameters {
                    collect_pattern_order(module.id, pattern, &mut binding_order, &mut order);
                }
                collect_lifted(
                    snapshot,
                    module,
                    &definition.value,
                    &mut locals,
                    &mut local_bindings,
                    &mut builtins,
                    &mut binding_order,
                    &mut order,
                    ordinals.entry(module.id).or_default(),
                )?;
            }
        }

        let mut raw_captures = BTreeMap::new();
        let mut visiting = BTreeSet::new();
        for key in locals.keys().copied().collect::<Vec<_>>() {
            analyze_captures(
                key,
                resolved,
                &locals,
                &binding_order,
                &mut raw_captures,
                &mut visiting,
            )?;
        }

        let mut resolver =
            SignatureResolver::new(snapshot, &named, &locals, &local_bindings, aggregate_mode);
        let mut named_signatures = BTreeMap::new();
        for id in named.keys() {
            named_signatures.insert(id.clone(), resolver.named(id)?);
        }
        let mut local_signatures = BTreeMap::new();
        for key in locals.keys() {
            local_signatures.insert(*key, resolver.local(*key)?);
        }
        let mut builtin_signatures = BTreeMap::new();
        for builtin in builtins.values().map(|plan| plan.builtin) {
            if let std::collections::btree_map::Entry::Vacant(entry) =
                builtin_signatures.entry(builtin)
            {
                entry.insert(resolver.builtin(builtin)?);
            }
        }

        let mut captures = BTreeMap::new();
        for (owner, keys) in raw_captures {
            let owner_plan = &locals[&owner];
            let mut plans = Vec::with_capacity(keys.len());
            for key in keys {
                let info = resolved.bindings().get(&key).ok_or_else(|| {
                    invalid_module(
                        owner_plan.module,
                        owner_plan.binding.span,
                        "captured binding metadata is absent",
                    )
                })?;
                if info.mutable {
                    return Err(unsupported_module(
                        owner_plan.module,
                        info.span,
                        "mutable lexical capture",
                    ));
                }
                let self_reference = key == owner;
                if self_reference && !owner_plan.binding.recursive {
                    return Err(invalid_module(
                        owner_plan.module,
                        owner_plan.binding.span,
                        "non-recursive local function resolved a self reference",
                    ));
                }
                let value_type = if self_reference {
                    local_signatures[&owner].complete_type()?
                } else {
                    resolver.binding_value_type(key)?
                };
                plans.push(CapturePlan {
                    key,
                    self_reference,
                    value_type,
                });
            }
            captures.insert(owner, plans);
        }

        let mut ordered_keys = Vec::new();
        for plan in named.values() {
            ordered_keys.push((
                module_indices[&plan.module.id],
                FunctionKind::Named,
                plan.definition.name.normalized.as_bytes().to_vec(),
                OrderedPlan::Named(plan.id.clone()),
            ));
        }
        for plan in locals.values() {
            ordered_keys.push((
                module_indices[&plan.module.id],
                FunctionKind::ClosureBody,
                plan.label.as_bytes().to_vec(),
                OrderedPlan::Local(plan.key),
            ));
        }
        for plan in builtins.values() {
            ordered_keys.push((
                module_indices[&plan.module.id],
                FunctionKind::ClosureBody,
                plan.label.as_bytes().to_vec(),
                OrderedPlan::Builtin(plan.module.id, plan.builtin),
            ));
        }
        ordered_keys.sort_by(|left, right| {
            (&left.0, &left.1, &left.2).cmp(&(&right.0, &right.1, &right.2))
        });
        check_limit("functions", ordered_keys.len(), limits.functions())?;
        let ordered = ordered_keys
            .into_iter()
            .map(|(_, _, _, plan)| plan)
            .collect::<Vec<_>>();
        let mut function_indices = BTreeMap::new();
        let mut local_indices = BTreeMap::new();
        let mut builtin_indices = BTreeMap::new();
        for (index, plan) in ordered.iter().enumerate() {
            let index = FunctionIndex::new(to_u32(index, "function index")?);
            match plan {
                OrderedPlan::Named(id) => {
                    function_indices.insert(id.clone(), index);
                }
                OrderedPlan::Local(key) => {
                    local_indices.insert(*key, index);
                }
                OrderedPlan::Builtin(module, builtin) => {
                    builtin_indices.insert((*module, *builtin), index);
                }
            }
        }

        let mut string_set = BTreeSet::new();
        for module in &modules {
            string_set.insert(module.hir.module.name.normalized());
            for definition in &module.hir.definitions {
                string_set.insert(definition.name.normalized.clone());
                collect_text_strings(&definition.value, &mut string_set);
            }
            for declaration in &module.hir.types {
                string_set.insert(declaration.name.normalized.clone());
                match &declaration.definition {
                    hir::TypeDefinition::Record(fields) => {
                        for field in fields {
                            string_set.insert(field.name.normalized.clone());
                        }
                    }
                    hir::TypeDefinition::Variant(cases) => {
                        for case in cases {
                            string_set.insert(case.name.normalized.clone());
                        }
                    }
                    hir::TypeDefinition::Alias(_) => {}
                }
            }
        }
        for plan in locals.values() {
            string_set.insert(plan.label.clone());
        }
        for plan in builtins.values() {
            string_set.insert(plan.label.clone());
        }
        for source in &source_plans {
            string_set.insert(source.logical_name.to_owned());
        }
        if aggregate_mode {
            string_set.insert(ling_resolve::PRELUDE_MODULE.to_owned());
            for value in ["Option", "Some", "None", "Result", "Ok", "Error"] {
                string_set.insert(value.to_owned());
            }
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

        let mut module_name_indices = modules
            .iter()
            .map(|module| {
                Ok((
                    module.hir.module.name.normalized(),
                    module_indices[&module.id],
                ))
            })
            .collect::<Result<BTreeMap<_, _>, LoweringError>>()?;
        if aggregate_mode {
            module_name_indices
                .insert(ling_resolve::PRELUDE_MODULE.to_owned(), ModuleIndex::new(0));
        }
        let mut type_builder = TypeTableBuilder::default();
        for signature in named_signatures.values() {
            type_builder.add_signature(signature)?;
        }
        for signature in local_signatures.values() {
            type_builder.add_signature(signature)?;
        }
        for signature in builtin_signatures.values() {
            type_builder.add_signature(signature)?;
        }
        for plans in captures.values() {
            for capture in plans {
                type_builder.insert(capture.value_type.clone());
            }
        }
        if aggregate_mode {
            for plan in named.values() {
                collect_type_shapes(
                    snapshot,
                    plan.module,
                    &plan.definition.value,
                    &mut type_builder,
                )?;
            }
        }
        let (types, type_indices) = type_builder.finish(&string_indices, &module_name_indices)?;
        check_limit("types", types.len(), limits.types())?;

        let mut constant_set = BTreeMap::new();
        for plan in named.values() {
            collect_constants_v1_1(
                snapshot,
                plan.module,
                &plan.definition.value,
                &string_indices,
                &mut constant_set,
                aggregate_mode,
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
            aggregate_mode,
            modules,
            module_indices,
            source_plans,
            source_indices,
            source_inputs,
            named,
            locals,
            builtins,
            captures,
            named_signatures,
            local_signatures,
            builtin_signatures,
            ordered,
            function_indices,
            local_indices,
            builtin_indices,
            types,
            type_indices,
            strings,
            string_indices,
            constants,
            constant_indices,
        })
    }

    fn run(self) -> Result<LoweredProgramV1_1, LoweringError> {
        Ok(LoweredProgramV1_1::new(self.run_model()?))
    }

    fn run_model(self) -> Result<UnverifiedProgram, LoweringError> {
        let entry = ling_effects::locate_main(self.snapshot.checked()).map_err(|error| {
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
            invalid_without_span("validated main is absent from the function table")
        })?;

        let mut modules = Vec::with_capacity(self.modules.len() + usize::from(self.aggregate_mode));
        if self.aggregate_mode {
            modules.push(Module {
                package: PackageReference::Standalone,
                name: string_index(&self.string_indices, ling_resolve::PRELUDE_MODULE)?,
                capabilities: Vec::new(),
            });
        }
        modules.extend(
            self.modules
                .iter()
                .map(|module| {
                    Ok(Module {
                        package: PackageReference::Standalone,
                        name: string_index(
                            &self.string_indices,
                            &module.hir.module.name.normalized(),
                        )?,
                        capabilities: capabilities(self.snapshot, module)?,
                    })
                })
                .collect::<Result<Vec<_>, LoweringError>>()?,
        );
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

        let mut functions = Vec::with_capacity(self.ordered.len());
        let mut source_map = Vec::new();
        for plan in &self.ordered {
            let (function, mut entries) = match plan {
                OrderedPlan::Named(id) => self.lower_named(id)?,
                OrderedPlan::Local(key) => self.lower_local(*key)?,
                OrderedPlan::Builtin(module, builtin) => self.lower_builtin(*module, *builtin)?,
            };
            source_map.append(&mut entries);
            functions.push(function);
        }
        source_map.sort_by_key(|entry| (entry.function, entry.block, entry.ordinal));
        check_limit(
            "executable_locations",
            source_map.len(),
            self.limits.executable_locations(),
        )?;

        Ok(UnverifiedProgram::from_parts(ProgramParts {
            strings: self.strings,
            packages: Vec::new(),
            modules,
            types: self.types,
            constants: self.constants,
            sources,
            functions,
            entry,
            source_map,
        }))
    }

    fn lower_named(
        &self,
        id: &DefinitionId,
    ) -> Result<(Function, Vec<SourceMapEntry>), LoweringError> {
        let plan = &self.named[id];
        FunctionEmitter::new(self, plan.module, self.function_indices[id]).lower_source(
            FunctionKind::Named,
            &plan.definition.name.normalized,
            &[],
            &plan.definition.parameters,
            &plan.definition.value,
            &self.named_signatures[id],
        )
    }

    fn lower_local(
        &self,
        key: BindingKey,
    ) -> Result<(Function, Vec<SourceMapEntry>), LoweringError> {
        let plan = &self.locals[&key];
        FunctionEmitter::new(self, plan.module, self.local_indices[&key]).lower_source(
            FunctionKind::ClosureBody,
            &plan.label,
            &self.captures[&key],
            &plan.binding.parameters,
            &plan.binding.value,
            &self.local_signatures[&key],
        )
    }

    fn lower_builtin(
        &self,
        module: ModuleId,
        builtin: Builtin,
    ) -> Result<(Function, Vec<SourceMapEntry>), LoweringError> {
        let plan = &self.builtins[&(module, builtin)];
        FunctionEmitter::new(self, plan.module, self.builtin_indices[&(module, builtin)])
            .lower_builtin(plan, &self.builtin_signatures[&builtin])
    }
}

type PreparedSources<'source> = (
    Vec<SourcePlan<'source>>,
    BTreeMap<SourceId, SourceIndex>,
    BTreeMap<SourceId, &'source SourceFile>,
);

fn prepare_sources<'source>(
    modules: &[&ling_resolve::ResolvedModule],
    module_indices: &BTreeMap<ModuleId, ModuleIndex>,
    sources: &'source [LoweringSource<'source>],
    limits: DecodeLimits,
) -> Result<PreparedSources<'source>, LoweringError> {
    let mut supplied = BTreeMap::new();
    for source in sources {
        if supplied.insert(source.source().id(), source).is_some() {
            return Err(invalid_source_v1_1(source, "duplicate_source_id"));
        }
        if let Err(reason) = crate::path::validate_logical_name(source.logical_name()) {
            return Err(invalid_source_v1_1(source, reason.as_str()));
        }
    }
    let mut plans = Vec::with_capacity(modules.len());
    let mut used = BTreeSet::new();
    for module in modules {
        let source_id = module.hir.span.source();
        let source = supplied.get(&source_id).ok_or_else(|| {
            LoweringError::without_span(
                Some(&module.hir.source_name),
                LoweringErrorKind::InvalidSource {
                    source_id: source_id.get(),
                    logical_name: String::new(),
                    reason: "missing_source_snapshot".to_owned(),
                },
            )
        })?;
        if source.source().name() != module.hir.source_name {
            return Err(invalid_source_v1_1(source, "source_display_name_mismatch"));
        }
        used.insert(source_id);
        plans.push(SourcePlan {
            module: module_indices[&module.id],
            source: source.source(),
            logical_name: source.logical_name(),
        });
    }
    if let Some((_, source)) = supplied.iter().find(|(id, _)| !used.contains(id)) {
        return Err(invalid_source_v1_1(
            source,
            "source_not_in_checked_snapshot",
        ));
    }
    plans.sort_by(|left, right| {
        (left.module, left.logical_name.as_bytes())
            .cmp(&(right.module, right.logical_name.as_bytes()))
    });
    check_limit("sources", plans.len(), limits.sources())?;
    let indices = plans
        .iter()
        .enumerate()
        .map(|(index, plan)| {
            Ok((
                plan.source.id(),
                SourceIndex::new(to_u32(index, "source index")?),
            ))
        })
        .collect::<Result<BTreeMap<_, _>, LoweringError>>()?;
    let inputs = plans
        .iter()
        .map(|plan| (plan.source.id(), plan.source))
        .collect();
    Ok((plans, indices, inputs))
}

fn invalid_source_v1_1(source: &LoweringSource<'_>, reason: &str) -> LoweringError {
    LoweringError::without_span(
        Some(source.source().name()),
        LoweringErrorKind::InvalidSource {
            source_id: source.source().id().get(),
            logical_name: source.logical_name().to_owned(),
            reason: reason.to_owned(),
        },
    )
}

fn collect_pattern_order(
    module: ModuleId,
    pattern: &hir::Pattern,
    output: &mut BTreeMap<BindingKey, usize>,
    order: &mut usize,
) {
    match &pattern.kind {
        hir::PatternKind::Binding { id, .. } => {
            output.insert(BindingKey::new(module, *id), *order);
            *order = order.saturating_add(1);
        }
        hir::PatternKind::Tuple(values) => {
            for value in values {
                collect_pattern_order(module, value, output, order);
            }
        }
        hir::PatternKind::Record(fields) => {
            for field in fields {
                collect_pattern_order(module, &field.pattern, output, order);
            }
        }
        hir::PatternKind::Constructor { arguments, .. } => {
            for value in arguments {
                collect_pattern_order(module, value, output, order);
            }
        }
        hir::PatternKind::Wildcard | hir::PatternKind::Unit | hir::PatternKind::Literal(_) => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_lifted<'a>(
    snapshot: &'a ProgramSnapshot,
    module: &'a ling_resolve::ResolvedModule,
    expression: &'a hir::Expression,
    locals: &mut BTreeMap<BindingKey, LocalPlan<'a>>,
    local_bindings: &mut BTreeMap<BindingKey, &'a hir::LocalBinding>,
    builtins: &mut BTreeMap<(ModuleId, Builtin), BuiltinPlan<'a>>,
    binding_order: &mut BTreeMap<BindingKey, usize>,
    order: &mut usize,
    ordinal: &mut u64,
) -> Result<(), LoweringError> {
    match &expression.kind {
        hir::ExpressionKind::Sequence(elements) => {
            for element in elements {
                match element {
                    hir::SequenceElement::Let(binding) => {
                        let key = BindingKey::new(module.id, binding.id);
                        local_bindings.insert(key, binding);
                        binding_order.insert(key, *order);
                        *order = order.saturating_add(1);
                        for pattern in &binding.parameters {
                            collect_pattern_order(module.id, pattern, binding_order, order);
                        }
                        if !binding.parameters.is_empty() {
                            let label = closure_label(binding.span, *ordinal);
                            *ordinal = ordinal.checked_add(1).ok_or_else(|| {
                                resource_error("closure_ordinal", u64::MAX, u64::MAX - 1)
                            })?;
                            locals.insert(
                                key,
                                LocalPlan {
                                    key,
                                    module,
                                    binding,
                                    label,
                                },
                            );
                        }
                        collect_lifted(
                            snapshot,
                            module,
                            &binding.value,
                            locals,
                            local_bindings,
                            builtins,
                            binding_order,
                            order,
                            ordinal,
                        )?;
                    }
                    hir::SequenceElement::Expression(value) => collect_lifted(
                        snapshot,
                        module,
                        value,
                        locals,
                        local_bindings,
                        builtins,
                        binding_order,
                        order,
                        ordinal,
                    )?,
                }
            }
        }
        hir::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            for value in [
                condition.as_ref(),
                then_branch.as_ref(),
                else_branch.as_ref(),
            ] {
                collect_lifted(
                    snapshot,
                    module,
                    value,
                    locals,
                    local_bindings,
                    builtins,
                    binding_order,
                    order,
                    ordinal,
                )?;
            }
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            collect_lifted(
                snapshot,
                module,
                scrutinee,
                locals,
                local_bindings,
                builtins,
                binding_order,
                order,
                ordinal,
            )?;
            for case in cases {
                if let Some(guard) = &case.guard {
                    collect_lifted(
                        snapshot,
                        module,
                        guard,
                        locals,
                        local_bindings,
                        builtins,
                        binding_order,
                        order,
                        ordinal,
                    )?;
                }
                collect_lifted(
                    snapshot,
                    module,
                    &case.body,
                    locals,
                    local_bindings,
                    builtins,
                    binding_order,
                    order,
                    ordinal,
                )?;
            }
        }
        hir::ExpressionKind::Assignment { value, .. } => collect_lifted(
            snapshot,
            module,
            value,
            locals,
            local_bindings,
            builtins,
            binding_order,
            order,
            ordinal,
        )?,
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            collect_lifted(
                snapshot,
                module,
                function,
                locals,
                local_bindings,
                builtins,
                binding_order,
                order,
                ordinal,
            )?;
            for value in arguments {
                collect_lifted(
                    snapshot,
                    module,
                    value,
                    locals,
                    local_bindings,
                    builtins,
                    binding_order,
                    order,
                    ordinal,
                )?;
            }
        }
        hir::ExpressionKind::Projection {
            target, reference, ..
        } => {
            collect_builtin_plan(
                snapshot,
                module,
                *reference,
                expression.span,
                builtins,
                ordinal,
            )?;
            collect_lifted(
                snapshot,
                module,
                target,
                locals,
                local_bindings,
                builtins,
                binding_order,
                order,
                ordinal,
            )?;
        }
        hir::ExpressionKind::Name { reference, .. } => {
            collect_builtin_plan(
                snapshot,
                module,
                *reference,
                expression.span,
                builtins,
                ordinal,
            )?;
        }
        hir::ExpressionKind::Binary { left, right, .. } => {
            for value in [left.as_ref(), right.as_ref()] {
                collect_lifted(
                    snapshot,
                    module,
                    value,
                    locals,
                    local_bindings,
                    builtins,
                    binding_order,
                    order,
                    ordinal,
                )?;
            }
        }
        hir::ExpressionKind::Unary { operand, .. } => collect_lifted(
            snapshot,
            module,
            operand,
            locals,
            local_bindings,
            builtins,
            binding_order,
            order,
            ordinal,
        )?,
        hir::ExpressionKind::Tuple(values) | hir::ExpressionKind::List(values) => {
            for value in values {
                collect_lifted(
                    snapshot,
                    module,
                    value,
                    locals,
                    local_bindings,
                    builtins,
                    binding_order,
                    order,
                    ordinal,
                )?;
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                collect_lifted(
                    snapshot,
                    module,
                    &field.value,
                    locals,
                    local_bindings,
                    builtins,
                    binding_order,
                    order,
                    ordinal,
                )?;
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            collect_lifted(
                snapshot,
                module,
                base,
                locals,
                local_bindings,
                builtins,
                binding_order,
                order,
                ordinal,
            )?;
            for field in fields {
                collect_lifted(
                    snapshot,
                    module,
                    &field.value,
                    locals,
                    local_bindings,
                    builtins,
                    binding_order,
                    order,
                    ordinal,
                )?;
            }
        }
        hir::ExpressionKind::Literal(_) | hir::ExpressionKind::Unit => {}
    }
    Ok(())
}

fn collect_builtin_plan<'a>(
    snapshot: &ProgramSnapshot,
    module: &'a ling_resolve::ResolvedModule,
    reference: hir::ReferenceId,
    span: Span,
    builtins: &mut BTreeMap<(ModuleId, Builtin), BuiltinPlan<'a>>,
    ordinal: &mut u64,
) -> Result<(), LoweringError> {
    let Some(ReferenceTarget::Definition(definition)) = snapshot
        .checked()
        .typed()
        .resolved()
        .reference(module.id, reference)
    else {
        return Ok(());
    };
    let Some(info) = snapshot.checked().typed().resolved().definition(definition) else {
        return Ok(());
    };
    let DefinitionOrigin::Builtin(
        builtin @ (Builtin::ConsoleWrite | Builtin::TextFormat | Builtin::Max | Builtin::Min),
    ) = info.origin
    else {
        return Ok(());
    };
    let key = (module.id, builtin);
    if builtins.contains_key(&key) {
        return Ok(());
    }
    let label = closure_label(span, *ordinal);
    *ordinal = ordinal
        .checked_add(1)
        .ok_or_else(|| resource_error("closure_ordinal", u64::MAX, u64::MAX - 1))?;
    builtins.insert(
        key,
        BuiltinPlan {
            module,
            builtin,
            label,
            span,
        },
    );
    Ok(())
}

fn closure_label(span: Span, ordinal: u64) -> String {
    format!(
        "closure_{}_{}_{}",
        span.start().get(),
        span.end().get(),
        ordinal
    )
}

fn analyze_captures(
    key: BindingKey,
    resolved: &ling_resolve::ResolvedProgram,
    locals: &BTreeMap<BindingKey, LocalPlan<'_>>,
    binding_order: &BTreeMap<BindingKey, usize>,
    memo: &mut BTreeMap<BindingKey, Vec<BindingKey>>,
    visiting: &mut BTreeSet<BindingKey>,
) -> Result<Vec<BindingKey>, LoweringError> {
    if let Some(value) = memo.get(&key) {
        return Ok(value.clone());
    }
    let plan = &locals[&key];
    if !visiting.insert(key) {
        return Err(unsupported_module(
            plan.module,
            plan.binding.span,
            "mutually recursive local function group",
        ));
    }
    let mut declared = BTreeSet::new();
    for pattern in &plan.binding.parameters {
        collect_pattern_bindings(plan.module.id, pattern, &mut declared);
    }
    collect_declared_bindings(plan.module.id, &plan.binding.value, &mut declared);
    let mut free = BTreeSet::new();
    collect_free_bindings(
        plan.module.id,
        &plan.binding.value,
        resolved,
        locals,
        binding_order,
        memo,
        visiting,
        &declared,
        &mut free,
    )?;
    visiting.remove(&key);
    let mut free = free.into_iter().collect::<Vec<_>>();
    free.sort_by_key(|binding| binding_order.get(binding).copied().unwrap_or(usize::MAX));
    memo.insert(key, free.clone());
    Ok(free)
}

fn collect_pattern_bindings(
    module: ModuleId,
    pattern: &hir::Pattern,
    output: &mut BTreeSet<BindingKey>,
) {
    match &pattern.kind {
        hir::PatternKind::Binding { id, .. } => {
            output.insert(BindingKey::new(module, *id));
        }
        hir::PatternKind::Tuple(values) => {
            for value in values {
                collect_pattern_bindings(module, value, output);
            }
        }
        hir::PatternKind::Record(fields) => {
            for field in fields {
                collect_pattern_bindings(module, &field.pattern, output);
            }
        }
        hir::PatternKind::Constructor { arguments, .. } => {
            for value in arguments {
                collect_pattern_bindings(module, value, output);
            }
        }
        hir::PatternKind::Wildcard | hir::PatternKind::Unit | hir::PatternKind::Literal(_) => {}
    }
}

fn collect_declared_bindings(
    module: ModuleId,
    expression: &hir::Expression,
    output: &mut BTreeSet<BindingKey>,
) {
    match &expression.kind {
        hir::ExpressionKind::Sequence(elements) => {
            for element in elements {
                match element {
                    hir::SequenceElement::Let(binding) => {
                        output.insert(BindingKey::new(module, binding.id));
                        if binding.parameters.is_empty() {
                            collect_declared_bindings(module, &binding.value, output);
                        }
                    }
                    hir::SequenceElement::Expression(value) => {
                        collect_declared_bindings(module, value, output);
                    }
                }
            }
        }
        hir::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            for value in [
                condition.as_ref(),
                then_branch.as_ref(),
                else_branch.as_ref(),
            ] {
                collect_declared_bindings(module, value, output);
            }
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            collect_declared_bindings(module, scrutinee, output);
            for case in cases {
                if let Some(guard) = &case.guard {
                    collect_declared_bindings(module, guard, output);
                }
                collect_declared_bindings(module, &case.body, output);
            }
        }
        hir::ExpressionKind::Assignment { value, .. } => {
            collect_declared_bindings(module, value, output)
        }
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            collect_declared_bindings(module, function, output);
            for value in arguments {
                collect_declared_bindings(module, value, output);
            }
        }
        hir::ExpressionKind::Projection { target, .. } => {
            collect_declared_bindings(module, target, output)
        }
        hir::ExpressionKind::Binary { left, right, .. } => {
            collect_declared_bindings(module, left, output);
            collect_declared_bindings(module, right, output);
        }
        hir::ExpressionKind::Unary { operand, .. } => {
            collect_declared_bindings(module, operand, output)
        }
        hir::ExpressionKind::Tuple(values) | hir::ExpressionKind::List(values) => {
            for value in values {
                collect_declared_bindings(module, value, output);
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                collect_declared_bindings(module, &field.value, output);
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            collect_declared_bindings(module, base, output);
            for field in fields {
                collect_declared_bindings(module, &field.value, output);
            }
        }
        hir::ExpressionKind::Name { .. }
        | hir::ExpressionKind::Literal(_)
        | hir::ExpressionKind::Unit => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_free_bindings(
    module: ModuleId,
    expression: &hir::Expression,
    resolved: &ling_resolve::ResolvedProgram,
    locals: &BTreeMap<BindingKey, LocalPlan<'_>>,
    binding_order: &BTreeMap<BindingKey, usize>,
    memo: &mut BTreeMap<BindingKey, Vec<BindingKey>>,
    visiting: &mut BTreeSet<BindingKey>,
    declared: &BTreeSet<BindingKey>,
    output: &mut BTreeSet<BindingKey>,
) -> Result<(), LoweringError> {
    let mut add_reference = |reference: hir::ReferenceId| {
        if let Some(ReferenceTarget::Binding(binding)) = resolved.reference(module, reference)
            && !declared.contains(binding)
        {
            output.insert(*binding);
        }
    };
    match &expression.kind {
        hir::ExpressionKind::Name { reference, .. } => add_reference(*reference),
        hir::ExpressionKind::Projection { reference, .. } => add_reference(*reference),
        hir::ExpressionKind::Sequence(elements) => {
            for element in elements {
                match element {
                    hir::SequenceElement::Let(binding) if !binding.parameters.is_empty() => {
                        let nested = BindingKey::new(module, binding.id);
                        for capture in analyze_captures(
                            nested,
                            resolved,
                            locals,
                            binding_order,
                            memo,
                            visiting,
                        )? {
                            if !declared.contains(&capture) {
                                output.insert(capture);
                            }
                        }
                    }
                    hir::SequenceElement::Let(binding) => collect_free_bindings(
                        module,
                        &binding.value,
                        resolved,
                        locals,
                        binding_order,
                        memo,
                        visiting,
                        declared,
                        output,
                    )?,
                    hir::SequenceElement::Expression(value) => collect_free_bindings(
                        module,
                        value,
                        resolved,
                        locals,
                        binding_order,
                        memo,
                        visiting,
                        declared,
                        output,
                    )?,
                }
            }
        }
        hir::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            for value in [
                condition.as_ref(),
                then_branch.as_ref(),
                else_branch.as_ref(),
            ] {
                collect_free_bindings(
                    module,
                    value,
                    resolved,
                    locals,
                    binding_order,
                    memo,
                    visiting,
                    declared,
                    output,
                )?;
            }
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            collect_free_bindings(
                module,
                scrutinee,
                resolved,
                locals,
                binding_order,
                memo,
                visiting,
                declared,
                output,
            )?;
            for case in cases {
                if let Some(guard) = &case.guard {
                    collect_free_bindings(
                        module,
                        guard,
                        resolved,
                        locals,
                        binding_order,
                        memo,
                        visiting,
                        declared,
                        output,
                    )?;
                }
                collect_free_bindings(
                    module,
                    &case.body,
                    resolved,
                    locals,
                    binding_order,
                    memo,
                    visiting,
                    declared,
                    output,
                )?;
            }
        }
        hir::ExpressionKind::Assignment { value, .. } => collect_free_bindings(
            module,
            value,
            resolved,
            locals,
            binding_order,
            memo,
            visiting,
            declared,
            output,
        )?,
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            collect_free_bindings(
                module,
                function,
                resolved,
                locals,
                binding_order,
                memo,
                visiting,
                declared,
                output,
            )?;
            for value in arguments {
                collect_free_bindings(
                    module,
                    value,
                    resolved,
                    locals,
                    binding_order,
                    memo,
                    visiting,
                    declared,
                    output,
                )?;
            }
        }
        hir::ExpressionKind::Binary { left, right, .. } => {
            collect_free_bindings(
                module,
                left,
                resolved,
                locals,
                binding_order,
                memo,
                visiting,
                declared,
                output,
            )?;
            collect_free_bindings(
                module,
                right,
                resolved,
                locals,
                binding_order,
                memo,
                visiting,
                declared,
                output,
            )?;
        }
        hir::ExpressionKind::Unary { operand, .. } => collect_free_bindings(
            module,
            operand,
            resolved,
            locals,
            binding_order,
            memo,
            visiting,
            declared,
            output,
        )?,
        hir::ExpressionKind::Tuple(values) | hir::ExpressionKind::List(values) => {
            for value in values {
                collect_free_bindings(
                    module,
                    value,
                    resolved,
                    locals,
                    binding_order,
                    memo,
                    visiting,
                    declared,
                    output,
                )?;
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                collect_free_bindings(
                    module,
                    &field.value,
                    resolved,
                    locals,
                    binding_order,
                    memo,
                    visiting,
                    declared,
                    output,
                )?;
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            collect_free_bindings(
                module,
                base,
                resolved,
                locals,
                binding_order,
                memo,
                visiting,
                declared,
                output,
            )?;
            for field in fields {
                collect_free_bindings(
                    module,
                    &field.value,
                    resolved,
                    locals,
                    binding_order,
                    memo,
                    visiting,
                    declared,
                    output,
                )?;
            }
        }
        hir::ExpressionKind::Literal(_) | hir::ExpressionKind::Unit => {}
    }
    Ok(())
}

struct SignatureResolver<'a> {
    snapshot: &'a ProgramSnapshot,
    aggregate_mode: bool,
    named: &'a BTreeMap<DefinitionId, NamedPlan<'a>>,
    locals: &'a BTreeMap<BindingKey, LocalPlan<'a>>,
    local_bindings: &'a BTreeMap<BindingKey, &'a hir::LocalBinding>,
    named_cache: BTreeMap<DefinitionId, SignatureKey>,
    local_cache: BTreeMap<BindingKey, SignatureKey>,
    builtin_cache: BTreeMap<Builtin, SignatureKey>,
    named_visiting: BTreeSet<DefinitionId>,
    local_visiting: BTreeSet<BindingKey>,
}

impl<'a> SignatureResolver<'a> {
    fn new(
        snapshot: &'a ProgramSnapshot,
        named: &'a BTreeMap<DefinitionId, NamedPlan<'a>>,
        locals: &'a BTreeMap<BindingKey, LocalPlan<'a>>,
        local_bindings: &'a BTreeMap<BindingKey, &'a hir::LocalBinding>,
        aggregate_mode: bool,
    ) -> Self {
        Self {
            snapshot,
            aggregate_mode,
            named,
            locals,
            local_bindings,
            named_cache: BTreeMap::new(),
            local_cache: BTreeMap::new(),
            builtin_cache: BTreeMap::new(),
            named_visiting: BTreeSet::new(),
            local_visiting: BTreeSet::new(),
        }
    }

    fn named(&mut self, id: &DefinitionId) -> Result<SignatureKey, LoweringError> {
        if let Some(value) = self.named_cache.get(id) {
            return Ok(value.clone());
        }
        let plan = self
            .named
            .get(id)
            .cloned()
            .ok_or_else(|| invalid_without_span("named callable plan is absent"))?;
        if !self.named_visiting.insert(id.clone()) {
            return Err(unsupported_module(
                plan.module,
                plan.definition.span,
                "recursive function-valued result type",
            ));
        }
        let signature = if plan.definition.parameters.is_empty() {
            let type_id = self
                .snapshot
                .checked()
                .typed()
                .definition_type(id)
                .ok_or_else(|| {
                    invalid_module(
                        plan.module,
                        plan.definition.span,
                        "definition type is absent",
                    )
                })?;
            SignatureKey {
                parameters: Vec::new(),
                result: self.result_type(plan.module, type_id, &plan.definition.value)?,
                effects: bytecode_effects(
                    self.snapshot
                        .checked()
                        .definition_effect(id)
                        .ok_or_else(|| {
                            invalid_module(
                                plan.module,
                                plan.definition.span,
                                "definition Effect row is absent",
                            )
                        })?,
                    plan.module,
                    plan.definition.span,
                )?,
            }
        } else {
            let checked = self
                .snapshot
                .checked()
                .definition_function_type(id)
                .ok_or_else(|| {
                    invalid_module(
                        plan.module,
                        plan.definition.span,
                        "definition function type is absent",
                    )
                })?;
            self.signature_from_checked(
                plan.module,
                &plan.definition.parameters,
                &plan.definition.value,
                &checked,
            )?
        };
        self.named_visiting.remove(id);
        self.named_cache.insert(id.clone(), signature.clone());
        Ok(signature)
    }

    fn local(&mut self, key: BindingKey) -> Result<SignatureKey, LoweringError> {
        if let Some(value) = self.local_cache.get(&key) {
            return Ok(value.clone());
        }
        let plan = self
            .locals
            .get(&key)
            .cloned()
            .ok_or_else(|| invalid_without_span("local callable plan is absent"))?;
        if !self.local_visiting.insert(key) {
            return Err(unsupported_module(
                plan.module,
                plan.binding.span,
                "mutually recursive local function result type",
            ));
        }
        let checked = self
            .snapshot
            .checked()
            .binding_function_type(key)
            .ok_or_else(|| {
                invalid_module(
                    plan.module,
                    plan.binding.span,
                    "local function type or Effect row is absent",
                )
            })?;
        let signature = self.signature_from_checked(
            plan.module,
            &plan.binding.parameters,
            &plan.binding.value,
            &checked,
        )?;
        self.local_visiting.remove(&key);
        self.local_cache.insert(key, signature.clone());
        Ok(signature)
    }

    fn builtin(&mut self, builtin: Builtin) -> Result<SignatureKey, LoweringError> {
        if let Some(value) = self.builtin_cache.get(&builtin) {
            return Ok(value.clone());
        }
        if !matches!(
            builtin,
            Builtin::ConsoleWrite | Builtin::TextFormat | Builtin::Max | Builtin::Min
        ) {
            return Err(invalid_without_span(
                "unsupported builtin wrapper requested",
            ));
        }
        let checked = self
            .snapshot
            .checked()
            .definition_function_type(
                self.snapshot
                    .checked()
                    .typed()
                    .resolved()
                    .builtin_id(builtin),
            )
            .ok_or_else(|| invalid_without_span("builtin function type is absent"))?;
        let parameters = checked
            .parameters()
            .iter()
            .map(|value| self.plain_type(*value, None, None))
            .collect::<Result<Vec<_>, _>>()?;
        let result = self.plain_type(checked.result(), None, None)?;
        let signature = SignatureKey {
            parameters,
            result,
            effects: bytecode_effects_without_source(checked.effects())?,
        };
        self.builtin_cache.insert(builtin, signature.clone());
        Ok(signature)
    }

    fn signature_from_checked(
        &mut self,
        module: &ling_resolve::ResolvedModule,
        patterns: &[hir::Pattern],
        body: &hir::Expression,
        checked: &CheckedFunctionType,
    ) -> Result<SignatureKey, LoweringError> {
        if patterns.len() != checked.parameters().len() {
            return Err(invalid_module(
                module,
                body.span,
                "checked callable parameter count disagrees with HIR",
            ));
        }
        let parameters = patterns
            .iter()
            .zip(checked.parameters())
            .map(|(pattern, value)| self.parameter_type(module, pattern, *value))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(SignatureKey {
            parameters,
            result: self.result_type(module, checked.result(), body)?,
            effects: bytecode_effects(checked.effects(), module, body.span)?,
        })
    }

    fn parameter_type(
        &mut self,
        module: &ling_resolve::ResolvedModule,
        pattern: &hir::Pattern,
        value: TypeId,
    ) -> Result<TypeKey, LoweringError> {
        if !matches!(
            self.snapshot.checked().typed().arena().get(value),
            Type::Function { .. }
        ) {
            return self.plain_type(value, Some(module), Some(pattern.span));
        }
        let hir::PatternKind::Binding { id, .. } = pattern.kind else {
            return Err(unsupported_module(
                module,
                pattern.span,
                "function-typed parameter destructuring",
            ));
        };
        let key = BindingKey::new(module.id, id);
        let checked = self
            .snapshot
            .checked()
            .binding_function_type(key)
            .ok_or_else(|| {
                invalid_module(
                    module,
                    pattern.span,
                    "function parameter Effect row is absent",
                )
            })?;
        self.shallow_function_type(module, pattern.span, &checked)
    }

    fn result_type(
        &mut self,
        module: &ling_resolve::ResolvedModule,
        value: TypeId,
        expression: &hir::Expression,
    ) -> Result<TypeKey, LoweringError> {
        if matches!(
            self.snapshot.checked().typed().arena().get(value),
            Type::Function { .. }
        ) {
            self.function_value_type(module, expression)
        } else {
            self.plain_type(value, Some(module), Some(expression.span))
        }
    }

    fn plain_type(
        &self,
        value: TypeId,
        module: Option<&ling_resolve::ResolvedModule>,
        span: Option<Span>,
    ) -> Result<TypeKey, LoweringError> {
        match self.snapshot.checked().typed().arena().get(value) {
            Type::Unit => Ok(TypeKey::Unit),
            Type::Bool => Ok(TypeKey::Bool),
            Type::Int => Ok(TypeKey::Int),
            Type::Text => Ok(TypeKey::Text),
            Type::Function { .. } => Err(source_or_global_unsupported(
                module,
                span,
                "nested function type without Effect provenance",
            )),
            Type::Float64 => Err(source_or_global_unsupported(module, span, "Float64")),
            Type::Tuple(elements) if self.aggregate_mode => Ok(TypeKey::Tuple(
                elements
                    .iter()
                    .map(|element| self.plain_type(*element, module, span))
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            Type::Tuple(_) => Err(source_or_global_unsupported(module, span, "tuple")),
            Type::List(_) => Err(source_or_global_unsupported(module, span, "list")),
            Type::NominalRecord {
                definition,
                arguments,
            } if self.aggregate_mode => {
                let info = self
                    .snapshot
                    .checked()
                    .typed()
                    .records()
                    .get(definition)
                    .ok_or_else(|| invalid_without_span("record metadata is absent"))?;
                let identity = self
                    .snapshot
                    .checked()
                    .typed()
                    .resolved()
                    .definition(definition)
                    .ok_or_else(|| invalid_without_span("record definition is absent"))?;
                Ok(TypeKey::Record {
                    module: identity.module_name.clone(),
                    name: info.name.clone(),
                    arguments: arguments
                        .iter()
                        .map(|argument| self.plain_type(*argument, module, span))
                        .collect::<Result<Vec<_>, _>>()?,
                    fields: info
                        .fields
                        .iter()
                        .map(|field| {
                            Ok(RecordFieldKey {
                                name: field.name.clone(),
                                mutable: field.mutable,
                                value_type: self.plain_type(field.field_type, module, span)?,
                            })
                        })
                        .collect::<Result<Vec<_>, LoweringError>>()?,
                })
            }
            Type::NominalRecord { .. } => Err(source_or_global_unsupported(module, span, "record")),
            Type::NominalVariant {
                definition,
                arguments,
            } if self.aggregate_mode => {
                let info = self
                    .snapshot
                    .checked()
                    .typed()
                    .variants()
                    .get(definition)
                    .ok_or_else(|| invalid_without_span("variant metadata is absent"))?;
                let identity = self
                    .snapshot
                    .checked()
                    .typed()
                    .resolved()
                    .definition(definition)
                    .ok_or_else(|| invalid_without_span("variant definition is absent"))?;
                Ok(TypeKey::Variant {
                    module: identity.module_name.clone(),
                    name: info.name.clone(),
                    arguments: arguments
                        .iter()
                        .map(|argument| self.plain_type(*argument, module, span))
                        .collect::<Result<Vec<_>, _>>()?,
                    cases: info
                        .cases
                        .iter()
                        .map(|case| {
                            Ok(VariantCaseKey {
                                name: case.name.clone(),
                                payload: case
                                    .payload
                                    .map(|payload| self.plain_type(payload, module, span))
                                    .transpose()?,
                            })
                        })
                        .collect::<Result<Vec<_>, LoweringError>>()?,
                })
            }
            Type::NominalVariant { .. } => {
                Err(source_or_global_unsupported(module, span, "variant"))
            }
            Type::Variable(_) => Err(source_or_global_unsupported(
                module,
                span,
                "polymorphic function",
            )),
            Type::Error => Err(invalid_without_span("checked type retains an error node")),
        }
    }

    fn shallow_function_type(
        &self,
        module: &ling_resolve::ResolvedModule,
        span: Span,
        checked: &CheckedFunctionType,
    ) -> Result<TypeKey, LoweringError> {
        let parameters = checked
            .parameters()
            .iter()
            .map(|value| self.plain_type(*value, Some(module), Some(span)))
            .collect::<Result<Vec<_>, _>>()?;
        TypeKey::function(
            parameters,
            self.plain_type(checked.result(), Some(module), Some(span))?,
            bytecode_effects(checked.effects(), module, span)?,
        )
    }

    fn function_value_type(
        &mut self,
        module: &ling_resolve::ResolvedModule,
        expression: &hir::Expression,
    ) -> Result<TypeKey, LoweringError> {
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                let Some(hir::SequenceElement::Expression(value)) = elements.last() else {
                    return Err(invalid_module(
                        module,
                        expression.span,
                        "function-valued sequence has no final expression",
                    ));
                };
                self.function_value_type(module, value)
            }
            hir::ExpressionKind::Name { reference, .. }
            | hir::ExpressionKind::Projection { reference, .. } => {
                let target = self
                    .snapshot
                    .checked()
                    .typed()
                    .resolved()
                    .reference(module.id, *reference)
                    .cloned()
                    .ok_or_else(|| {
                        invalid_module(
                            module,
                            expression.span,
                            "function value reference is unresolved",
                        )
                    })?;
                match target {
                    ReferenceTarget::Definition(definition) => {
                        let info = self
                            .snapshot
                            .checked()
                            .typed()
                            .resolved()
                            .definition(&definition)
                            .ok_or_else(|| {
                                invalid_module(
                                    module,
                                    expression.span,
                                    "function value definition is absent",
                                )
                            })?;
                        match info.origin {
                            DefinitionOrigin::User { .. } => {
                                let plan =
                                    self.named.get(&definition).cloned().ok_or_else(|| {
                                        invalid_module(
                                            module,
                                            expression.span,
                                            "function value plan is absent",
                                        )
                                    })?;
                                if plan.definition.parameters.is_empty() {
                                    self.function_value_type(plan.module, &plan.definition.value)
                                } else {
                                    self.named(&definition)?.complete_type()
                                }
                            }
                            DefinitionOrigin::Builtin(
                                builtin @ (Builtin::ConsoleWrite
                                | Builtin::TextFormat
                                | Builtin::Max
                                | Builtin::Min),
                            ) => self.builtin(builtin)?.complete_type(),
                            DefinitionOrigin::Builtin(builtin) => Err(unsupported_module(
                                module,
                                expression.span,
                                builtin.qualified_name(),
                            )),
                            DefinitionOrigin::Prelude(_) => Err(unsupported_module(
                                module,
                                expression.span,
                                "Prelude function value",
                            )),
                        }
                    }
                    ReferenceTarget::Binding(binding) => self.binding_value_type(binding),
                }
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                let TypeKey::Function {
                    parameters,
                    result,
                    effects,
                } = self.function_value_type(module, function)?
                else {
                    return Err(invalid_module(
                        module,
                        function.span,
                        "checked application callee has no function type",
                    ));
                };
                if arguments.is_empty() || arguments.len() > parameters.len() {
                    return Err(invalid_module(
                        module,
                        expression.span,
                        "checked application arity is invalid",
                    ));
                }
                if arguments.len() == parameters.len() {
                    match *result {
                        value @ TypeKey::Function { .. } => Ok(value),
                        _ => Err(invalid_module(
                            module,
                            expression.span,
                            "function-valued application has scalar result",
                        )),
                    }
                } else {
                    TypeKey::function(parameters[arguments.len()..].to_vec(), *result, effects)
                }
            }
            _ => Err(unsupported_module(
                module,
                expression.span,
                "function value expression without stable Effect provenance",
            )),
        }
    }

    fn binding_value_type(&mut self, key: BindingKey) -> Result<TypeKey, LoweringError> {
        if self.locals.contains_key(&key) {
            return self.local(key)?.complete_type();
        }
        let info = self
            .snapshot
            .checked()
            .typed()
            .resolved()
            .bindings()
            .get(&key)
            .cloned()
            .ok_or_else(|| invalid_without_span("binding metadata is absent"))?;
        let value = self
            .snapshot
            .checked()
            .typed()
            .binding_type(key)
            .ok_or_else(|| invalid_without_span("binding type is absent"))?;
        if !matches!(
            self.snapshot.checked().typed().arena().get(value),
            Type::Function { .. }
        ) {
            return self.plain_type(value, None, None);
        }
        if info.parameter {
            let checked = self
                .snapshot
                .checked()
                .binding_function_type(key)
                .ok_or_else(|| invalid_without_span("function parameter Effect row is absent"))?;
            let module = self
                .snapshot
                .checked()
                .typed()
                .resolved()
                .module(key.module())
                .ok_or_else(|| invalid_without_span("function parameter module is absent"))?;
            return self.shallow_function_type(module, info.span, &checked);
        }
        let binding = self
            .local_bindings
            .get(&key)
            .copied()
            .ok_or_else(|| invalid_without_span("function-valued local alias is absent"))?;
        let module = self
            .snapshot
            .checked()
            .typed()
            .resolved()
            .module(key.module())
            .ok_or_else(|| invalid_without_span("function-valued local module is absent"))?;
        self.function_value_type(module, &binding.value)
    }
}

fn checked_type_key(
    typed: &TypedProgram,
    value: TypeId,
    module: &ling_resolve::ResolvedModule,
    span: Span,
) -> Result<TypeKey, LoweringError> {
    checked_type_key_with_substitution(typed, value, module, span, &BTreeMap::new())
}

fn checked_type_key_with_substitution(
    typed: &TypedProgram,
    value: TypeId,
    module: &ling_resolve::ResolvedModule,
    span: Span,
    substitutions: &BTreeMap<u32, TypeId>,
) -> Result<TypeKey, LoweringError> {
    match typed.arena().get(value) {
        Type::Unit => Ok(TypeKey::Unit),
        Type::Bool => Ok(TypeKey::Bool),
        Type::Int => Ok(TypeKey::Int),
        Type::Text => Ok(TypeKey::Text),
        Type::Tuple(elements) => Ok(TypeKey::Tuple(
            elements
                .iter()
                .map(|element| {
                    checked_type_key_with_substitution(typed, *element, module, span, substitutions)
                })
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Type::NominalRecord {
            definition,
            arguments,
        } => {
            let info = typed
                .records()
                .get(definition)
                .ok_or_else(|| invalid_module(module, span, "record metadata is absent"))?;
            let identity = typed
                .resolved()
                .definition(definition)
                .ok_or_else(|| invalid_module(module, span, "record definition is absent"))?;
            let substitutions =
                extend_nominal_substitutions(typed, definition, arguments, substitutions);
            Ok(TypeKey::Record {
                module: identity.module_name.clone(),
                name: info.name.clone(),
                arguments: arguments
                    .iter()
                    .map(|argument| {
                        checked_type_key_with_substitution(
                            typed,
                            *argument,
                            module,
                            span,
                            &substitutions,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                fields: info
                    .fields
                    .iter()
                    .map(|field| {
                        Ok(RecordFieldKey {
                            name: field.name.clone(),
                            mutable: field.mutable,
                            value_type: checked_type_key_with_substitution(
                                typed,
                                field.field_type,
                                module,
                                span,
                                &substitutions,
                            )?,
                        })
                    })
                    .collect::<Result<Vec<_>, LoweringError>>()?,
            })
        }
        Type::NominalVariant {
            definition,
            arguments,
        } => {
            let info = typed
                .variants()
                .get(definition)
                .ok_or_else(|| invalid_module(module, span, "variant metadata is absent"))?;
            let identity = typed
                .resolved()
                .definition(definition)
                .ok_or_else(|| invalid_module(module, span, "variant definition is absent"))?;
            let substitutions =
                extend_nominal_substitutions(typed, definition, arguments, substitutions);
            Ok(TypeKey::Variant {
                module: identity.module_name.clone(),
                name: info.name.clone(),
                arguments: arguments
                    .iter()
                    .map(|argument| {
                        checked_type_key_with_substitution(
                            typed,
                            *argument,
                            module,
                            span,
                            &substitutions,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                cases: info
                    .cases
                    .iter()
                    .map(|case| {
                        Ok(VariantCaseKey {
                            name: case.name.clone(),
                            payload: case
                                .payload
                                .map(|payload| {
                                    checked_type_key_with_substitution(
                                        typed,
                                        payload,
                                        module,
                                        span,
                                        &substitutions,
                                    )
                                })
                                .transpose()?,
                        })
                    })
                    .collect::<Result<Vec<_>, LoweringError>>()?,
            })
        }
        Type::Function { .. } => Err(invalid_module(
            module,
            span,
            "nested function type lacks Effect provenance",
        )),
        Type::Float64 => Err(unsupported_module(module, span, "Float64")),
        Type::List(_) => Err(unsupported_module(module, span, "list")),
        Type::Variable(variable) => substitutions
            .get(variable)
            .copied()
            .ok_or_else(|| unsupported_module(module, span, "polymorphic function"))
            .and_then(|resolved| {
                checked_type_key_with_substitution(typed, resolved, module, span, substitutions)
            }),
        Type::Error => Err(invalid_module(
            module,
            span,
            "checked type retains an error node",
        )),
    }
}

fn extend_nominal_substitutions(
    typed: &TypedProgram,
    definition: &DefinitionId,
    arguments: &[TypeId],
    substitutions: &BTreeMap<u32, TypeId>,
) -> BTreeMap<u32, TypeId> {
    let mut output = substitutions.clone();
    if let Some(info) = typed.variants().get(definition) {
        if let Some(case) = info.cases.first() {
            if let Some(constructor_type) = typed.definition_type(&case.definition) {
                let result = match typed.arena().get(constructor_type) {
                    Type::Function { result, .. } => *result,
                    _ => constructor_type,
                };
                if let Type::NominalVariant {
                    arguments: generic_arguments,
                    ..
                } = typed.arena().get(result)
                {
                    for (generic, actual) in generic_arguments.iter().zip(arguments) {
                        collect_type_substitutions(typed, *generic, *actual, &mut output);
                    }
                }
            }
        }
    } else if let Some(info) = typed.records().get(definition) {
        let mut variables = Vec::new();
        for field in &info.fields {
            collect_variable_ids(typed, field.field_type, &mut variables);
        }
        for (variable, actual) in variables.into_iter().zip(arguments) {
            output.entry(variable).or_insert(*actual);
        }
    }
    output
}

fn collect_variable_ids(typed: &TypedProgram, value: TypeId, output: &mut Vec<u32>) {
    match typed.arena().get(value) {
        Type::Variable(variable) if !output.contains(variable) => output.push(*variable),
        Type::Tuple(elements) => {
            for element in elements {
                collect_variable_ids(typed, *element, output);
            }
        }
        Type::NominalRecord { arguments, .. } | Type::NominalVariant { arguments, .. } => {
            for argument in arguments {
                collect_variable_ids(typed, *argument, output);
            }
        }
        _ => {}
    }
}

fn collect_type_substitutions(
    typed: &TypedProgram,
    generic: TypeId,
    actual: TypeId,
    output: &mut BTreeMap<u32, TypeId>,
) {
    match (typed.arena().get(generic), typed.arena().get(actual)) {
        (Type::Variable(variable), _) => {
            output.insert(*variable, actual);
        }
        (Type::Tuple(generic), Type::Tuple(actual)) => {
            for (generic, actual) in generic.iter().zip(actual) {
                collect_type_substitutions(typed, *generic, *actual, output);
            }
        }
        (
            Type::NominalRecord {
                arguments: generic, ..
            },
            Type::NominalRecord {
                arguments: actual, ..
            },
        )
        | (
            Type::NominalVariant {
                arguments: generic, ..
            },
            Type::NominalVariant {
                arguments: actual, ..
            },
        ) => {
            for (generic, actual) in generic.iter().zip(actual) {
                collect_type_substitutions(typed, *generic, *actual, output);
            }
        }
        _ => {}
    }
}

#[derive(Default)]
struct TypeTableBuilder {
    values: BTreeSet<TypeKey>,
}

impl TypeTableBuilder {
    fn insert(&mut self, value: TypeKey) {
        match &value {
            TypeKey::Unit | TypeKey::Bool | TypeKey::Int | TypeKey::Text => {}
            TypeKey::Tuple(elements) => {
                for element in elements {
                    self.insert(element.clone());
                }
                self.values.insert(value);
            }
            TypeKey::Record {
                arguments, fields, ..
            } => {
                for argument in arguments {
                    self.insert(argument.clone());
                }
                for field in fields {
                    self.insert(field.value_type.clone());
                }
                self.values.insert(value);
            }
            TypeKey::Variant {
                arguments, cases, ..
            } => {
                for argument in arguments {
                    self.insert(argument.clone());
                }
                for case in cases {
                    if let Some(payload) = &case.payload {
                        self.insert(payload.clone());
                    }
                }
                self.values.insert(value);
            }
            TypeKey::Function {
                parameters, result, ..
            } => {
                for parameter in parameters {
                    self.insert(parameter.clone());
                }
                self.insert((**result).clone());
                self.values.insert(value);
            }
        }
    }

    fn add_signature(&mut self, signature: &SignatureKey) -> Result<(), LoweringError> {
        for parameter in &signature.parameters {
            self.insert(parameter.clone());
        }
        self.insert(signature.result.clone());
        for applied in 0..signature.parameters.len() {
            self.insert(signature.suffix_type(applied)?);
        }
        Ok(())
    }

    fn finish(
        self,
        strings: &BTreeMap<String, StringIndex>,
        modules: &BTreeMap<String, ModuleIndex>,
    ) -> Result<(Vec<ValueType>, BTreeMap<TypeKey, TypeIndex>), LoweringError> {
        let mut types = vec![
            ValueType::Unit,
            ValueType::Bool,
            ValueType::Int,
            ValueType::Text,
        ];
        let mut indices = BTreeMap::from([
            (TypeKey::Unit, TypeIndex::new(0)),
            (TypeKey::Bool, TypeIndex::new(1)),
            (TypeKey::Int, TypeIndex::new(2)),
            (TypeKey::Text, TypeIndex::new(3)),
        ]);
        let mut remaining = self.values;
        while !remaining.is_empty() {
            let mut ready = remaining
                .iter()
                .filter_map(|value| {
                    wire_type(value, &indices, strings, modules).map(|wire| (value.clone(), wire))
                })
                .collect::<Vec<_>>();
            ready.sort_by(|left, right| left.1.cmp(&right.1));
            let Some((key, wire)) = ready.into_iter().next() else {
                return Err(invalid_without_span(
                    "function type graph is cyclic or incomplete",
                ));
            };
            let index = TypeIndex::new(to_u32(types.len(), "type index")?);
            remaining.remove(&key);
            indices.insert(key, index);
            types.push(wire);
        }
        Ok((types, indices))
    }
}

fn wire_type(
    value: &TypeKey,
    indices: &BTreeMap<TypeKey, TypeIndex>,
    strings: &BTreeMap<String, StringIndex>,
    modules: &BTreeMap<String, ModuleIndex>,
) -> Option<ValueType> {
    match value {
        TypeKey::Unit | TypeKey::Bool | TypeKey::Int | TypeKey::Text => None,
        TypeKey::Function {
            parameters,
            result,
            effects,
        } => Some(ValueType::Function {
            parameters: parameters
                .iter()
                .map(|value| indices.get(value).copied())
                .collect::<Option<Vec<_>>>()?,
            result: indices.get(result.as_ref()).copied()?,
            effects: effects.clone(),
        }),
        TypeKey::Tuple(elements) => Some(ValueType::Tuple {
            elements: elements
                .iter()
                .map(|element| indices.get(element).copied())
                .collect::<Option<Vec<_>>>()?,
        }),
        TypeKey::Record {
            module,
            name,
            arguments,
            fields,
        } => Some(ValueType::Record {
            module: *modules.get(module)?,
            name: *strings.get(name)?,
            arguments: arguments
                .iter()
                .map(|argument| indices.get(argument).copied())
                .collect::<Option<Vec<_>>>()?,
            fields: fields
                .iter()
                .map(|field| {
                    Some(RecordField {
                        name: *strings.get(&field.name)?,
                        value_type: indices.get(&field.value_type).copied()?,
                        mutable: field.mutable,
                    })
                })
                .collect::<Option<Vec<_>>>()?,
        }),
        TypeKey::Variant {
            module,
            name,
            arguments,
            cases,
        } => Some(ValueType::Variant {
            module: *modules.get(module)?,
            name: *strings.get(name)?,
            arguments: arguments
                .iter()
                .map(|argument| indices.get(argument).copied())
                .collect::<Option<Vec<_>>>()?,
            cases: cases
                .iter()
                .map(|case| {
                    let payload = match &case.payload {
                        Some(payload) => Some(indices.get(payload).copied()?),
                        None => None,
                    };
                    Some(VariantCase {
                        name: *strings.get(&case.name)?,
                        payload,
                    })
                })
                .collect::<Option<Vec<_>>>()?,
        }),
    }
}

fn bytecode_effects(
    row: &EffectRow,
    module: &ling_resolve::ResolvedModule,
    span: Span,
) -> Result<Vec<Effect>, LoweringError> {
    row.effects()
        .map(|effect| match effect {
            CheckedEffect::ConsoleWrite => Ok(Effect::ConsoleWrite),
            CheckedEffect::State { .. } => Err(unsupported_module(module, span, "State Effect")),
        })
        .collect()
}

fn bytecode_effects_without_source(row: &EffectRow) -> Result<Vec<Effect>, LoweringError> {
    row.effects()
        .map(|effect| match effect {
            CheckedEffect::ConsoleWrite => Ok(Effect::ConsoleWrite),
            CheckedEffect::State { .. } => Err(invalid_without_span(
                "builtin unexpectedly has a State Effect",
            )),
        })
        .collect()
}

fn source_or_global_unsupported(
    module: Option<&ling_resolve::ResolvedModule>,
    span: Option<Span>,
    feature: &str,
) -> LoweringError {
    match (module, span) {
        (Some(module), Some(span)) => unsupported_module(module, span, feature),
        _ => LoweringError::without_span(
            None,
            LoweringErrorKind::UnsupportedFeature {
                feature: feature.to_owned(),
            },
        ),
    }
}

fn unsupported_module(
    module: &ling_resolve::ResolvedModule,
    span: Span,
    feature: &str,
) -> LoweringError {
    LoweringError::at(
        &module.hir.source_name,
        span,
        LoweringErrorKind::UnsupportedFeature {
            feature: feature.to_owned(),
        },
    )
}

fn invalid_module(
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

fn invalid_without_span(invariant: &str) -> LoweringError {
    LoweringError::without_span(
        None,
        LoweringErrorKind::InvalidCheckedCore {
            invariant: invariant.to_owned(),
        },
    )
}

// Function emission and constant collection are kept below the planning/type phase
// so no bytecode is published until every index and capture has been validated.

struct BlockBuilder {
    parameters: Vec<BlockParameter>,
    instructions: Vec<Instruction>,
    terminator: Option<Terminator>,
}

impl BlockBuilder {
    fn new() -> Self {
        Self {
            parameters: Vec::new(),
            instructions: Vec::new(),
            terminator: None,
        }
    }
}

struct FunctionEmitter<'a, 'snapshot, 'source> {
    owner: &'a ClosureLowerer<'snapshot, 'source>,
    module: &'snapshot ling_resolve::ResolvedModule,
    function_index: FunctionIndex,
    next_register: u32,
    blocks: Vec<BlockBuilder>,
    current_block: usize,
    source_map: Vec<SourceMapEntry>,
}

impl<'a, 'snapshot, 'source> FunctionEmitter<'a, 'snapshot, 'source> {
    fn new(
        owner: &'a ClosureLowerer<'snapshot, 'source>,
        module: &'snapshot ling_resolve::ResolvedModule,
        function_index: FunctionIndex,
    ) -> Self {
        Self {
            owner,
            module,
            function_index,
            next_register: 0,
            blocks: vec![BlockBuilder::new()],
            current_block: 0,
            source_map: Vec::new(),
        }
    }

    fn lower_source(
        mut self,
        kind: FunctionKind,
        name: &str,
        captures: &[CapturePlan],
        patterns: &[hir::Pattern],
        body: &hir::Expression,
        signature: &SignatureKey,
    ) -> Result<(Function, Vec<SourceMapEntry>), LoweringError> {
        if patterns.len() != signature.parameters.len() {
            return Err(invalid_module(
                self.module,
                body.span,
                "function parameter count disagrees with signature",
            ));
        }
        let mut environment = BTreeMap::new();
        let mut parameter_types = Vec::with_capacity(captures.len() + signature.parameters.len());
        for capture in captures {
            let value_type = self.type_index(&capture.value_type)?;
            let register = self.new_register(body.span)?;
            self.blocks[0].parameters.push(BlockParameter {
                register,
                value_type,
            });
            parameter_types.push(value_type);
            environment.insert(capture.key, register);
        }
        for (pattern, value) in patterns.iter().zip(&signature.parameters) {
            let value_type = self.type_index(value)?;
            let register = self.new_register(pattern.span)?;
            self.blocks[0].parameters.push(BlockParameter {
                register,
                value_type,
            });
            parameter_types.push(value_type);
            match &pattern.kind {
                hir::PatternKind::Unit if value_type == TypeIndex::new(0) => {}
                hir::PatternKind::Binding { id, .. } => {
                    environment.insert(BindingKey::new(self.module.id, *id), register);
                }
                hir::PatternKind::Unit => {
                    return Err(invalid_module(
                        self.module,
                        pattern.span,
                        "Unit pattern has a non-Unit type",
                    ));
                }
                _ => {
                    return Err(unsupported_module(
                        self.module,
                        pattern.span,
                        "parameter destructuring",
                    ));
                }
            }
        }
        let result = self.lower_expression(body, &mut environment)?;
        if self.blocks[self.current_block].terminator.is_some() {
            return Err(invalid_module(
                self.module,
                body.span,
                "function body continues after a terminated block",
            ));
        }
        let ordinal = to_u32(
            self.blocks[self.current_block].instructions.len(),
            "terminator ordinal",
        )?;
        self.push_source_map(body.span, ordinal, SourceOrigin::LoweringDerived)?;
        self.blocks[self.current_block].terminator = Some(Terminator::Return { value: result });
        let blocks = self.finish_blocks()?;
        let function = Function {
            kind,
            module: self.owner.module_indices[&self.module.id],
            name: string_index(&self.owner.string_indices, name)?,
            capture_count: to_u32(captures.len(), "capture count")?,
            parameter_types,
            result_type: self.type_index(&signature.result)?,
            effects: signature.effects.clone(),
            register_count: self.next_register,
            blocks,
        };
        Ok((function, self.source_map))
    }

    fn lower_builtin(
        mut self,
        plan: &BuiltinPlan<'_>,
        signature: &SignatureKey,
    ) -> Result<(Function, Vec<SourceMapEntry>), LoweringError> {
        let mut arguments = Vec::with_capacity(signature.parameters.len());
        let mut parameter_types = Vec::with_capacity(signature.parameters.len());
        for value in &signature.parameters {
            let value_type = self.type_index(value)?;
            let register = self.new_register(plan.span)?;
            self.blocks[0].parameters.push(BlockParameter {
                register,
                value_type,
            });
            arguments.push(register);
            parameter_types.push(value_type);
        }
        let destination = self.new_register(plan.span)?;
        self.push_instruction(
            builtin_instruction(plan.builtin, destination, &arguments)?,
            plan.span,
        )?;
        let ordinal = to_u32(
            self.blocks[self.current_block].instructions.len(),
            "terminator ordinal",
        )?;
        self.push_source_map(plan.span, ordinal, SourceOrigin::LoweringDerived)?;
        self.blocks[self.current_block].terminator =
            Some(Terminator::Return { value: destination });
        let blocks = self.finish_blocks()?;
        let function = Function {
            kind: FunctionKind::ClosureBody,
            module: self.owner.module_indices[&self.module.id],
            name: string_index(&self.owner.string_indices, &plan.label)?,
            capture_count: 0,
            parameter_types,
            result_type: self.type_index(&signature.result)?,
            effects: signature.effects.clone(),
            register_count: self.next_register,
            blocks,
        };
        Ok((function, self.source_map))
    }

    fn lower_expression(
        &mut self,
        expression: &hir::Expression,
        environment: &mut BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        self.ensure_expression_type(expression)?;
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                let mut local = environment.clone();
                let mut result = None;
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            if binding.mutable {
                                return Err(unsupported_module(
                                    self.module,
                                    binding.span,
                                    "mutable local binding",
                                ));
                            }
                            let key = BindingKey::new(self.module.id, binding.id);
                            let value = if binding.parameters.is_empty() {
                                if binding.recursive {
                                    return Err(unsupported_module(
                                        self.module,
                                        binding.span,
                                        "recursive non-function local binding",
                                    ));
                                }
                                self.lower_expression(&binding.value, &mut local)?
                            } else {
                                let destination = self.new_register(binding.span)?;
                                let captures = self.owner.captures.get(&key).ok_or_else(|| {
                                    invalid_module(
                                        self.module,
                                        binding.span,
                                        "local closure capture plan is absent",
                                    )
                                })?;
                                let mut operands = Vec::with_capacity(captures.len());
                                for capture in captures {
                                    operands.push(if capture.self_reference {
                                        CaptureOperand::SelfReference
                                    } else {
                                        CaptureOperand::Register(
                                            *local.get(&capture.key).ok_or_else(|| {
                                                invalid_module(
                                                    self.module,
                                                    binding.span,
                                                    "captured binding has no lexical register",
                                                )
                                            })?,
                                        )
                                    });
                                }
                                self.push_instruction(
                                    Instruction::MakeClosure {
                                        destination,
                                        function: self.owner.local_indices[&key],
                                        captures: operands,
                                    },
                                    binding.span,
                                )?;
                                destination
                            };
                            local.insert(key, value);
                            result = None;
                        }
                        hir::SequenceElement::Expression(value) => {
                            result = Some(self.lower_expression(value, &mut local)?);
                        }
                    }
                }
                result.map_or_else(|| self.emit_constant(expression, Constant::Unit), Ok)
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => self.lower_application(expression, function, arguments, environment),
            hir::ExpressionKind::Name { reference, .. } => {
                self.lower_reference(expression, *reference, environment)
            }
            hir::ExpressionKind::Projection {
                reference,
                target,
                field,
            } => self.lower_projection(expression, *reference, target, field, environment),
            hir::ExpressionKind::Literal(literal) => {
                let constant = literal_constant_v1_1(
                    self.owner.snapshot,
                    self.module,
                    expression,
                    literal,
                    &self.owner.string_indices,
                )?;
                self.emit_constant(expression, constant)
            }
            hir::ExpressionKind::Unit => self.emit_constant(expression, Constant::Unit),
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } if self.owner.aggregate_mode => {
                self.lower_if(expression, condition, then_branch, else_branch, environment)
            }
            hir::ExpressionKind::If { .. } => {
                Err(unsupported_module(self.module, expression.span, "if"))
            }
            hir::ExpressionKind::Match { scrutinee, cases } if self.owner.aggregate_mode => {
                self.lower_match(expression, scrutinee, cases, environment)
            }
            hir::ExpressionKind::Match { .. } => {
                Err(unsupported_module(self.module, expression.span, "match"))
            }
            hir::ExpressionKind::Assignment { .. } => Err(unsupported_module(
                self.module,
                expression.span,
                "mutable assignment",
            )),
            hir::ExpressionKind::Binary {
                operator,
                left,
                right,
            } if self.owner.aggregate_mode => {
                self.lower_binary(expression, *operator, left, right, environment)
            }
            hir::ExpressionKind::Binary { .. } => Err(unsupported_module(
                self.module,
                expression.span,
                "scalar operators",
            )),
            hir::ExpressionKind::Unary { operator, operand } if self.owner.aggregate_mode => {
                self.lower_unary(expression, *operator, operand, environment)
            }
            hir::ExpressionKind::Unary { .. } => Err(unsupported_module(
                self.module,
                expression.span,
                "integer unary operators",
            )),
            hir::ExpressionKind::Tuple(elements) if self.owner.aggregate_mode => {
                self.lower_tuple(expression, elements, environment)
            }
            hir::ExpressionKind::Record(fields) if self.owner.aggregate_mode => {
                self.lower_record(expression, fields, environment)
            }
            hir::ExpressionKind::RecordUpdate { base, fields } if self.owner.aggregate_mode => {
                self.lower_record_update(expression, base, fields, environment)
            }
            hir::ExpressionKind::Tuple(_) => {
                Err(unsupported_module(self.module, expression.span, "tuple"))
            }
            hir::ExpressionKind::Record(_) | hir::ExpressionKind::RecordUpdate { .. } => {
                Err(unsupported_module(self.module, expression.span, "record"))
            }
            hir::ExpressionKind::List(_) => {
                Err(unsupported_module(self.module, expression.span, "list"))
            }
        }
    }

    fn lower_match(
        &mut self,
        expression: &hir::Expression,
        scrutinee: &hir::Expression,
        cases: &[hir::MatchCase],
        environment: &BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        if cases.is_empty() {
            return Err(invalid_module(
                self.module,
                expression.span,
                "checked match has no cases",
            ));
        }
        let typed = self.owner.snapshot.checked().typed();
        let scrutinee_type = typed
            .expression_type(ExpressionKey::new(self.module.id, scrutinee.id))
            .ok_or_else(|| {
                invalid_module(
                    self.module,
                    scrutinee.span,
                    "match scrutinee type is absent",
                )
            })?;
        let scrutinee_register = self.lower_expression(scrutinee, &mut environment.clone())?;
        let result_type = self.type_index(&self.expression_type_key(expression)?)?;
        let merge = self.new_block()?;
        let result_register = self.new_register(expression.span)?;
        self.blocks[usize::try_from(merge.get()).map_err(|_| {
            invalid_without_span("match merge block index does not fit host usize")
        })?]
        .parameters
        .push(BlockParameter {
            register: result_register,
            value_type: result_type,
        });

        let original_environment = environment.clone();
        for (index, case) in cases.iter().enumerate() {
            let is_last = index + 1 == cases.len();
            let pattern_success = self.new_block()?;
            let failure = if is_last {
                if case.guard.is_some() {
                    return Err(unsupported_module(
                        self.module,
                        case.span,
                        "guarded final match case without fallback",
                    ));
                }
                None
            } else {
                Some(self.new_block()?)
            };
            let mut case_environment = original_environment.clone();
            self.emit_pattern(
                scrutinee_register,
                scrutinee_type,
                &case.pattern,
                &mut case_environment,
                pattern_success,
                failure,
            )?;

            let body_block = if let Some(guard) = &case.guard {
                let failure = failure.ok_or_else(|| {
                    invalid_module(self.module, case.span, "guard has no failure successor")
                })?;
                self.set_current_block(pattern_success)?;
                let condition = self.lower_expression(guard, &mut case_environment)?;
                let body = self.new_block()?;
                self.set_terminator(
                    Terminator::Branch {
                        condition,
                        true_target: body,
                        true_arguments: Vec::new(),
                        false_target: failure,
                        false_arguments: Vec::new(),
                    },
                    guard.span,
                )?;
                body
            } else {
                pattern_success
            };

            self.set_current_block(body_block)?;
            let value = self.lower_expression(&case.body, &mut case_environment)?;
            self.set_terminator(
                Terminator::Jump {
                    target: merge,
                    arguments: vec![value],
                },
                case.body.span,
            )?;

            if let Some(failure) = failure {
                self.set_current_block(failure)?;
            }
        }
        self.set_current_block(merge)?;
        Ok(result_register)
    }

    fn lower_if(
        &mut self,
        expression: &hir::Expression,
        condition: &hir::Expression,
        then_branch: &hir::Expression,
        else_branch: &hir::Expression,
        environment: &BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        let condition_register = self.lower_expression(condition, &mut environment.clone())?;
        let result_type = self.type_index(&self.expression_type_key(expression)?)?;
        let merge = self.new_block()?;
        let result_register = self.new_register(expression.span)?;
        self.blocks[usize::try_from(merge.get())
            .map_err(|_| invalid_without_span("if merge block index does not fit host usize"))?]
        .parameters
        .push(BlockParameter {
            register: result_register,
            value_type: result_type,
        });
        let then_block = self.new_block()?;
        let else_block = self.new_block()?;
        self.set_terminator(
            Terminator::Branch {
                condition: condition_register,
                true_target: then_block,
                true_arguments: Vec::new(),
                false_target: else_block,
                false_arguments: Vec::new(),
            },
            condition.span,
        )?;

        self.set_current_block(then_block)?;
        let then_value = self.lower_expression(then_branch, &mut environment.clone())?;
        self.set_terminator(
            Terminator::Jump {
                target: merge,
                arguments: vec![then_value],
            },
            then_branch.span,
        )?;

        self.set_current_block(else_block)?;
        let else_value = self.lower_expression(else_branch, &mut environment.clone())?;
        self.set_terminator(
            Terminator::Jump {
                target: merge,
                arguments: vec![else_value],
            },
            else_branch.span,
        )?;
        self.set_current_block(merge)?;
        Ok(result_register)
    }

    fn lower_binary(
        &mut self,
        expression: &hir::Expression,
        operator: hir::BinaryOperator,
        left: &hir::Expression,
        right: &hir::Expression,
        environment: &BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        if matches!(
            operator,
            hir::BinaryOperator::BooleanAnd | hir::BinaryOperator::BooleanOr
        ) {
            let left_register = self.lower_expression(left, &mut environment.clone())?;
            let merge = self.new_block()?;
            let result_register = self.new_register(expression.span)?;
            self.blocks[usize::try_from(merge.get()).map_err(|_| {
                invalid_without_span("boolean merge block index does not fit host usize")
            })?]
            .parameters
            .push(BlockParameter {
                register: result_register,
                value_type: TypeIndex::new(1),
            });
            let right_block = self.new_block()?;
            let short_block = self.new_block()?;
            let (true_target, false_target) = match operator {
                hir::BinaryOperator::BooleanAnd => (right_block, short_block),
                hir::BinaryOperator::BooleanOr => (short_block, right_block),
                _ => unreachable!("boolean operator checked above"),
            };
            self.set_terminator(
                Terminator::Branch {
                    condition: left_register,
                    true_target,
                    true_arguments: Vec::new(),
                    false_target,
                    false_arguments: Vec::new(),
                },
                expression.span,
            )?;
            self.set_current_block(right_block)?;
            let right_register = self.lower_expression(right, &mut environment.clone())?;
            self.set_terminator(
                Terminator::Jump {
                    target: merge,
                    arguments: vec![right_register],
                },
                right.span,
            )?;
            self.set_current_block(short_block)?;
            let short_value = self.emit_constant_span(
                expression.span,
                Constant::Bool(matches!(operator, hir::BinaryOperator::BooleanOr)),
            )?;
            self.set_terminator(
                Terminator::Jump {
                    target: merge,
                    arguments: vec![short_value],
                },
                expression.span,
            )?;
            self.set_current_block(merge)?;
            return Ok(result_register);
        }

        let left_register = self.lower_expression(left, &mut environment.clone())?;
        let right_register = self.lower_expression(right, &mut environment.clone())?;
        let typed = self.owner.snapshot.checked().typed();
        let left_type = typed
            .expression_type(ExpressionKey::new(self.module.id, left.id))
            .ok_or_else(|| invalid_module(self.module, left.span, "binary left type is absent"))?;
        let destination = self.new_register(expression.span)?;
        let instruction = match operator {
            hir::BinaryOperator::Add => Instruction::IntBinary {
                destination,
                operator: IntBinaryOperator::Add,
                left: left_register,
                right: right_register,
            },
            hir::BinaryOperator::Subtract => Instruction::IntBinary {
                destination,
                operator: IntBinaryOperator::Subtract,
                left: left_register,
                right: right_register,
            },
            hir::BinaryOperator::Multiply => Instruction::IntBinary {
                destination,
                operator: IntBinaryOperator::Multiply,
                left: left_register,
                right: right_register,
            },
            hir::BinaryOperator::Divide => Instruction::IntBinary {
                destination,
                operator: IntBinaryOperator::Divide,
                left: left_register,
                right: right_register,
            },
            hir::BinaryOperator::Remainder => Instruction::IntBinary {
                destination,
                operator: IntBinaryOperator::Remainder,
                left: left_register,
                right: right_register,
            },
            hir::BinaryOperator::Equal | hir::BinaryOperator::NotEqual => {
                let operator = match (typed.arena().get(left_type), operator) {
                    (Type::Bool, hir::BinaryOperator::Equal) => CompareOperator::BoolEqual,
                    (Type::Bool, hir::BinaryOperator::NotEqual) => CompareOperator::BoolNotEqual,
                    (Type::Int, hir::BinaryOperator::Equal) => CompareOperator::IntEqual,
                    (Type::Int, hir::BinaryOperator::NotEqual) => CompareOperator::IntNotEqual,
                    (Type::Text, hir::BinaryOperator::Equal) => CompareOperator::TextEqual,
                    (Type::Text, hir::BinaryOperator::NotEqual) => CompareOperator::TextNotEqual,
                    _ => {
                        return Err(invalid_module(
                            self.module,
                            expression.span,
                            "equality operands are not supported by bytecode",
                        ));
                    }
                };
                Instruction::Compare {
                    destination,
                    operator,
                    left: left_register,
                    right: right_register,
                }
            }
            hir::BinaryOperator::Less
            | hir::BinaryOperator::LessEqual
            | hir::BinaryOperator::Greater
            | hir::BinaryOperator::GreaterEqual => {
                let operator = match operator {
                    hir::BinaryOperator::Less => CompareOperator::IntLess,
                    hir::BinaryOperator::LessEqual => CompareOperator::IntLessEqual,
                    hir::BinaryOperator::Greater => CompareOperator::IntGreater,
                    hir::BinaryOperator::GreaterEqual => CompareOperator::IntGreaterEqual,
                    _ => unreachable!("comparison operator checked above"),
                };
                if !matches!(typed.arena().get(left_type), Type::Int) {
                    return Err(invalid_module(
                        self.module,
                        expression.span,
                        "ordered comparison operands are not Int",
                    ));
                }
                Instruction::Compare {
                    destination,
                    operator,
                    left: left_register,
                    right: right_register,
                }
            }
            hir::BinaryOperator::BooleanAnd | hir::BinaryOperator::BooleanOr => {
                unreachable!("boolean operators handled above")
            }
        };
        self.push_instruction(instruction, expression.span)?;
        Ok(destination)
    }

    fn lower_unary(
        &mut self,
        expression: &hir::Expression,
        operator: hir::UnaryOperator,
        operand: &hir::Expression,
        environment: &BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        let operand_register = self.lower_expression(operand, &mut environment.clone())?;
        let destination = self.new_register(expression.span)?;
        let operator = match operator {
            hir::UnaryOperator::Positive => IntUnaryOperator::Positive,
            hir::UnaryOperator::Negative => IntUnaryOperator::Negative,
        };
        self.push_instruction(
            Instruction::IntUnary {
                destination,
                operator,
                operand: operand_register,
            },
            expression.span,
        )?;
        Ok(destination)
    }

    fn emit_pattern(
        &mut self,
        value: RegisterIndex,
        value_type: TypeId,
        pattern: &hir::Pattern,
        environment: &mut BTreeMap<BindingKey, RegisterIndex>,
        success: BlockIndex,
        failure: Option<BlockIndex>,
    ) -> Result<(), LoweringError> {
        let typed = self.owner.snapshot.checked().typed();
        match &pattern.kind {
            hir::PatternKind::Binding { id, .. } => {
                environment.insert(BindingKey::new(self.module.id, *id), value);
                self.set_terminator(
                    Terminator::Jump {
                        target: success,
                        arguments: Vec::new(),
                    },
                    pattern.span,
                )
            }
            hir::PatternKind::Wildcard | hir::PatternKind::Unit => self.set_terminator(
                Terminator::Jump {
                    target: success,
                    arguments: Vec::new(),
                },
                pattern.span,
            ),
            hir::PatternKind::Literal(literal) => {
                let Some(failure) = failure else {
                    return self.set_terminator(
                        Terminator::Jump {
                            target: success,
                            arguments: Vec::new(),
                        },
                        pattern.span,
                    );
                };
                let constant = pattern_constant(
                    self.owner.snapshot,
                    self.module,
                    pattern,
                    literal,
                    &self.owner.string_indices,
                )?;
                let constant_register = self.emit_constant_span(pattern.span, constant)?;
                let operator = match (typed.arena().get(value_type), literal) {
                    (Type::Bool, hir::Literal::Boolean(_)) => CompareOperator::BoolEqual,
                    (Type::Int, hir::Literal::Integer { .. }) => CompareOperator::IntEqual,
                    (Type::Text, hir::Literal::Text(_)) => CompareOperator::TextEqual,
                    _ => {
                        return Err(invalid_module(
                            self.module,
                            pattern.span,
                            "literal pattern type is not comparable by bytecode",
                        ));
                    }
                };
                let condition = self.new_register(pattern.span)?;
                self.push_instruction(
                    Instruction::Compare {
                        destination: condition,
                        operator,
                        left: value,
                        right: constant_register,
                    },
                    pattern.span,
                )?;
                self.set_terminator(
                    Terminator::Branch {
                        condition,
                        true_target: success,
                        true_arguments: Vec::new(),
                        false_target: failure,
                        false_arguments: Vec::new(),
                    },
                    pattern.span,
                )
            }
            hir::PatternKind::Tuple(patterns) => {
                let Type::Tuple(types) = typed.arena().get(value_type) else {
                    return Err(invalid_module(
                        self.module,
                        pattern.span,
                        "tuple pattern value is not a tuple",
                    ));
                };
                if patterns.len() != types.len() {
                    return Err(invalid_module(
                        self.module,
                        pattern.span,
                        "tuple pattern arity disagrees with checked type",
                    ));
                }
                for (index, (child, child_type)) in patterns.iter().zip(types).enumerate() {
                    let next = if index + 1 == patterns.len() {
                        success
                    } else {
                        self.new_block()?
                    };
                    let child_register = self.new_register(child.span)?;
                    self.push_instruction(
                        Instruction::GetTuple {
                            destination: child_register,
                            tuple: value,
                            element: u32::try_from(index).map_err(|_| {
                                invalid_without_span("tuple element index overflow")
                            })?,
                        },
                        child.span,
                    )?;
                    self.emit_pattern(
                        child_register,
                        *child_type,
                        child,
                        environment,
                        next,
                        failure,
                    )?;
                    if index + 1 != patterns.len() {
                        self.set_current_block(next)?;
                    }
                }
                Ok(())
            }
            hir::PatternKind::Record(fields) => {
                let Type::NominalRecord { definition, .. } = typed.arena().get(value_type) else {
                    return Err(invalid_module(
                        self.module,
                        pattern.span,
                        "record pattern value is not a nominal record",
                    ));
                };
                let info = typed.records().get(definition).ok_or_else(|| {
                    invalid_module(self.module, pattern.span, "record metadata is absent")
                })?;
                for (index, field) in fields.iter().enumerate() {
                    let (field_index, declaration) = info
                        .fields
                        .iter()
                        .enumerate()
                        .find(|(_, candidate)| candidate.name == field.name.normalized)
                        .ok_or_else(|| {
                            invalid_module(self.module, field.span, "record field is absent")
                        })?;
                    let next = if index + 1 == fields.len() {
                        success
                    } else {
                        self.new_block()?
                    };
                    let child_register = self.new_register(field.pattern.span)?;
                    self.push_instruction(
                        Instruction::GetField {
                            destination: child_register,
                            record: value,
                            field: u32::try_from(field_index)
                                .map_err(|_| invalid_without_span("record field index overflow"))?,
                        },
                        field.span,
                    )?;
                    self.emit_pattern(
                        child_register,
                        declaration.field_type,
                        &field.pattern,
                        environment,
                        next,
                        failure,
                    )?;
                    if index + 1 != fields.len() {
                        self.set_current_block(next)?;
                    }
                }
                if fields.is_empty() {
                    self.set_terminator(
                        Terminator::Jump {
                            target: success,
                            arguments: Vec::new(),
                        },
                        pattern.span,
                    )?;
                }
                Ok(())
            }
            hir::PatternKind::Constructor { arguments, .. } => {
                let constructor = typed
                    .resolved()
                    .pattern_constructor(self.module.id, pattern.id)
                    .cloned()
                    .ok_or_else(|| {
                        invalid_module(
                            self.module,
                            pattern.span,
                            "constructor pattern is unresolved",
                        )
                    })?;
                let Type::NominalVariant { definition, .. } = typed.arena().get(value_type) else {
                    return Err(invalid_module(
                        self.module,
                        pattern.span,
                        "constructor pattern value is not a nominal variant",
                    ));
                };
                let info = typed.variants().get(definition).ok_or_else(|| {
                    invalid_module(self.module, pattern.span, "variant metadata is absent")
                })?;
                let (case_index, case) = info
                    .cases
                    .iter()
                    .enumerate()
                    .find(|(_, candidate)| candidate.definition == constructor)
                    .ok_or_else(|| {
                        invalid_module(
                            self.module,
                            pattern.span,
                            "variant constructor case is absent",
                        )
                    })?;
                match (case.payload, arguments.as_slice()) {
                    (None, []) => {
                        if let Some(failure) = failure {
                            let condition = self.new_register(pattern.span)?;
                            self.push_instruction(
                                Instruction::VariantIs {
                                    destination: condition,
                                    variant: value,
                                    case: u32::try_from(case_index).map_err(|_| {
                                        invalid_without_span("variant case index overflow")
                                    })?,
                                },
                                pattern.span,
                            )?;
                            self.set_terminator(
                                Terminator::Branch {
                                    condition,
                                    true_target: success,
                                    true_arguments: Vec::new(),
                                    false_target: failure,
                                    false_arguments: Vec::new(),
                                },
                                pattern.span,
                            )
                        } else {
                            self.set_terminator(
                                Terminator::Jump {
                                    target: success,
                                    arguments: Vec::new(),
                                },
                                pattern.span,
                            )
                        }
                    }
                    (Some(payload_type), [payload_pattern]) => {
                        let payload_block = if let Some(failure) = failure {
                            let condition = self.new_register(pattern.span)?;
                            self.push_instruction(
                                Instruction::VariantIs {
                                    destination: condition,
                                    variant: value,
                                    case: u32::try_from(case_index).map_err(|_| {
                                        invalid_without_span("variant case index overflow")
                                    })?,
                                },
                                pattern.span,
                            )?;
                            let payload_block = self.new_block()?;
                            self.set_terminator(
                                Terminator::Branch {
                                    condition,
                                    true_target: payload_block,
                                    true_arguments: Vec::new(),
                                    false_target: failure,
                                    false_arguments: Vec::new(),
                                },
                                pattern.span,
                            )?;
                            payload_block
                        } else {
                            let payload_block = self.new_block()?;
                            self.set_terminator(
                                Terminator::Jump {
                                    target: payload_block,
                                    arguments: Vec::new(),
                                },
                                pattern.span,
                            )?;
                            payload_block
                        };
                        self.set_current_block(payload_block)?;
                        let payload_register = self.new_register(payload_pattern.span)?;
                        self.push_instruction(
                            Instruction::GetVariantPayload {
                                destination: payload_register,
                                variant: value,
                                case: u32::try_from(case_index).map_err(|_| {
                                    invalid_without_span("variant case index overflow")
                                })?,
                            },
                            payload_pattern.span,
                        )?;
                        self.emit_pattern(
                            payload_register,
                            payload_type,
                            payload_pattern,
                            environment,
                            success,
                            failure,
                        )
                    }
                    _ => Err(invalid_module(
                        self.module,
                        pattern.span,
                        "constructor pattern arity disagrees with checked type",
                    )),
                }
            }
        }
    }

    fn lower_projection(
        &mut self,
        expression: &hir::Expression,
        reference: hir::ReferenceId,
        target: &hir::Expression,
        field: &hir::Name,
        environment: &mut BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        let typed = self.owner.snapshot.checked().typed();
        let target_type = typed
            .expression_type(ExpressionKey::new(self.module.id, target.id))
            .ok_or_else(|| {
                invalid_module(self.module, target.span, "projection target type is absent")
            })?;
        if let Type::NominalRecord { definition, .. } = typed.arena().get(target_type) {
            let info = typed.records().get(definition).ok_or_else(|| {
                invalid_module(self.module, expression.span, "record metadata is absent")
            })?;
            let (field_index, _) = info
                .fields
                .iter()
                .enumerate()
                .find(|(_, candidate)| candidate.name == field.normalized)
                .ok_or_else(|| {
                    invalid_module(self.module, expression.span, "record field is absent")
                })?;
            let base = self.lower_expression(target, environment)?;
            let destination = self.new_register(expression.span)?;
            self.push_instruction(
                Instruction::GetField {
                    destination,
                    record: base,
                    field: u32::try_from(field_index)
                        .map_err(|_| invalid_without_span("record field index overflow"))?,
                },
                expression.span,
            )?;
            Ok(destination)
        } else {
            self.lower_reference(expression, reference, environment)
        }
    }

    fn lower_tuple(
        &mut self,
        expression: &hir::Expression,
        elements: &[hir::Expression],
        environment: &mut BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        let mut registers = Vec::with_capacity(elements.len());
        for element in elements {
            registers.push(self.lower_expression(element, environment)?);
        }
        let value = self.expression_type_key(expression)?;
        let destination = self.new_register(expression.span)?;
        self.push_instruction(
            Instruction::MakeTuple {
                destination,
                tuple: self.type_index(&value)?,
                elements: registers,
            },
            expression.span,
        )?;
        Ok(destination)
    }

    fn lower_record(
        &mut self,
        expression: &hir::Expression,
        fields: &[hir::RecordField],
        environment: &mut BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        let typed = self.owner.snapshot.checked().typed();
        let value_type = typed
            .expression_type(ExpressionKey::new(self.module.id, expression.id))
            .ok_or_else(|| invalid_module(self.module, expression.span, "record type is absent"))?;
        let Type::NominalRecord { definition, .. } = typed.arena().get(value_type) else {
            return Err(invalid_module(
                self.module,
                expression.span,
                "record literal is not nominal",
            ));
        };
        let info = typed.records().get(definition).ok_or_else(|| {
            invalid_module(self.module, expression.span, "record metadata is absent")
        })?;
        let mut values = Vec::with_capacity(info.fields.len());
        for declared in &info.fields {
            let field = fields
                .iter()
                .find(|field| field.name.normalized == declared.name)
                .ok_or_else(|| {
                    invalid_module(self.module, expression.span, "record field is absent")
                })?;
            values.push(self.lower_expression(&field.value, environment)?);
        }
        let destination = self.new_register(expression.span)?;
        self.push_instruction(
            Instruction::MakeRecord {
                destination,
                record: self.type_index(&self.expression_type_key(expression)?)?,
                fields: values,
            },
            expression.span,
        )?;
        Ok(destination)
    }

    fn lower_record_update(
        &mut self,
        expression: &hir::Expression,
        base: &hir::Expression,
        fields: &[hir::RecordField],
        environment: &mut BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        let typed = self.owner.snapshot.checked().typed();
        let base_type = typed
            .expression_type(ExpressionKey::new(self.module.id, base.id))
            .ok_or_else(|| invalid_module(self.module, base.span, "record base type is absent"))?;
        let Type::NominalRecord { definition, .. } = typed.arena().get(base_type) else {
            return Err(invalid_module(
                self.module,
                expression.span,
                "record update base is not nominal",
            ));
        };
        let info = typed.records().get(definition).ok_or_else(|| {
            invalid_module(self.module, expression.span, "record metadata is absent")
        })?;
        let base_register = self.lower_expression(base, environment)?;
        let mut updates = Vec::with_capacity(fields.len());
        for field in fields {
            let index = info
                .fields
                .iter()
                .position(|declared| declared.name == field.name.normalized)
                .ok_or_else(|| invalid_module(self.module, field.span, "record field is absent"))?;
            updates.push(RecordUpdate {
                field: u32::try_from(index)
                    .map_err(|_| invalid_without_span("record field index overflow"))?,
                value: self.lower_expression(&field.value, environment)?,
            });
        }
        let destination = self.new_register(expression.span)?;
        self.push_instruction(
            Instruction::UpdateRecord {
                destination,
                base: base_register,
                updates,
            },
            expression.span,
        )?;
        Ok(destination)
    }

    fn lower_application(
        &mut self,
        expression: &hir::Expression,
        function: &hir::Expression,
        arguments: &[hir::Expression],
        environment: &mut BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        if arguments.is_empty() {
            return Err(invalid_module(
                self.module,
                expression.span,
                "checked application has zero arguments",
            ));
        }
        check_limit(
            "operation_arguments",
            arguments.len(),
            self.owner.limits.arguments_per_operation(),
        )?;
        let target = simple_reference_target(self.owner.snapshot, self.module.id, function)?;
        if let Some(ReferenceTarget::Definition(definition)) = &target {
            let info = self
                .owner
                .snapshot
                .checked()
                .typed()
                .resolved()
                .definition(definition)
                .ok_or_else(|| {
                    invalid_module(self.module, function.span, "call definition is absent")
                })?;
            match info.origin {
                DefinitionOrigin::User { .. } => {
                    if self.owner.aggregate_mode && info.kind == DefinitionKind::Constructor {
                        return self.lower_constructor_application(
                            expression,
                            definition,
                            arguments,
                            environment,
                        );
                    }
                    let plan = &self.owner.named[definition];
                    let signature = &self.owner.named_signatures[definition];
                    if !plan.definition.parameters.is_empty()
                        && arguments.len() == signature.parameters.len()
                    {
                        let values = self.lower_arguments(arguments, environment)?;
                        let destination = self.new_register(expression.span)?;
                        self.push_instruction(
                            Instruction::Call {
                                destination,
                                function: self.owner.function_indices[definition],
                                arguments: values,
                            },
                            expression.span,
                        )?;
                        return Ok(destination);
                    }
                }
                DefinitionOrigin::Builtin(
                    builtin @ (Builtin::ConsoleWrite
                    | Builtin::TextFormat
                    | Builtin::Max
                    | Builtin::Min),
                ) => {
                    let signature = &self.owner.builtin_signatures[&builtin];
                    if arguments.len() == signature.parameters.len() {
                        let values = self.lower_arguments(arguments, environment)?;
                        let destination = self.new_register(expression.span)?;
                        self.push_instruction(
                            builtin_instruction(builtin, destination, &values)?,
                            expression.span,
                        )?;
                        return Ok(destination);
                    }
                }
                DefinitionOrigin::Prelude(_) if self.owner.aggregate_mode => {
                    return self.lower_constructor_application(
                        expression,
                        definition,
                        arguments,
                        environment,
                    );
                }
                DefinitionOrigin::Builtin(_) | DefinitionOrigin::Prelude(_) => {}
            }
        }
        let callee = self.lower_expression(function, environment)?;
        let values = self.lower_arguments(arguments, environment)?;
        let destination = self.new_register(expression.span)?;
        self.push_instruction(
            Instruction::CallClosure {
                destination,
                callee,
                arguments: values,
            },
            expression.span,
        )?;
        Ok(destination)
    }

    fn lower_arguments(
        &mut self,
        arguments: &[hir::Expression],
        environment: &mut BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<Vec<RegisterIndex>, LoweringError> {
        let mut output = Vec::with_capacity(arguments.len());
        for argument in arguments {
            output.push(self.lower_expression(argument, environment)?);
        }
        Ok(output)
    }

    fn lower_constructor_application(
        &mut self,
        expression: &hir::Expression,
        definition: &DefinitionId,
        arguments: &[hir::Expression],
        environment: &mut BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        let typed = self.owner.snapshot.checked().typed();
        let (variant_definition, case_index, payload_type) = typed
            .variants()
            .iter()
            .find_map(|(variant_definition, info)| {
                info.cases.iter().enumerate().find_map(|(index, case)| {
                    (case.definition == *definition).then_some((
                        variant_definition.clone(),
                        index,
                        case.payload,
                    ))
                })
            })
            .ok_or_else(|| {
                invalid_module(
                    self.module,
                    expression.span,
                    "constructor variant is absent",
                )
            })?;
        match (payload_type, arguments) {
            (None, []) => {}
            (Some(_), [payload]) => {
                let register = self.lower_expression(payload, environment)?;
                let destination = self.new_register(expression.span)?;
                let variant_type = self.expression_type_key(expression)?;
                self.push_instruction(
                    Instruction::MakeVariant {
                        destination,
                        variant: self.type_index(&variant_type)?,
                        case: u32::try_from(case_index)
                            .map_err(|_| invalid_without_span("variant case index overflow"))?,
                        payload: Some(register),
                    },
                    expression.span,
                )?;
                return Ok(destination);
            }
            (None, _) => {
                return Err(invalid_module(
                    self.module,
                    expression.span,
                    "nullary constructor received a payload",
                ));
            }
            (Some(_), _) => {
                return Err(invalid_module(
                    self.module,
                    expression.span,
                    "constructor payload arity disagrees with checked type",
                ));
            }
        }
        let _variant_definition = variant_definition;
        let destination = self.new_register(expression.span)?;
        let variant_type = self.expression_type_key(expression)?;
        self.push_instruction(
            Instruction::MakeVariant {
                destination,
                variant: self.type_index(&variant_type)?,
                case: u32::try_from(case_index)
                    .map_err(|_| invalid_without_span("variant case index overflow"))?,
                payload: None,
            },
            expression.span,
        )?;
        Ok(destination)
    }

    fn lower_reference(
        &mut self,
        expression: &hir::Expression,
        reference: hir::ReferenceId,
        environment: &BTreeMap<BindingKey, RegisterIndex>,
    ) -> Result<RegisterIndex, LoweringError> {
        let target = self
            .owner
            .snapshot
            .checked()
            .typed()
            .resolved()
            .reference(self.module.id, reference)
            .cloned()
            .ok_or_else(|| {
                invalid_module(
                    self.module,
                    expression.span,
                    "value reference is unresolved",
                )
            })?;
        match target {
            ReferenceTarget::Binding(binding) => {
                environment.get(&binding).copied().ok_or_else(|| {
                    invalid_module(
                        self.module,
                        expression.span,
                        "referenced binding has no register",
                    )
                })
            }
            ReferenceTarget::Definition(definition) => {
                let info = self
                    .owner
                    .snapshot
                    .checked()
                    .typed()
                    .resolved()
                    .definition(&definition)
                    .ok_or_else(|| {
                        invalid_module(
                            self.module,
                            expression.span,
                            "referenced definition is absent",
                        )
                    })?;
                if self.owner.aggregate_mode && info.kind == DefinitionKind::Constructor {
                    return self.lower_constructor_application(
                        expression,
                        &definition,
                        &[],
                        &mut environment.clone(),
                    );
                }
                let destination = self.new_register(expression.span)?;
                match info.origin {
                    DefinitionOrigin::User { .. } => {
                        let plan = &self.owner.named[&definition];
                        let instruction = if plan.definition.parameters.is_empty() {
                            Instruction::Call {
                                destination,
                                function: self.owner.function_indices[&definition],
                                arguments: Vec::new(),
                            }
                        } else {
                            Instruction::MakeClosure {
                                destination,
                                function: self.owner.function_indices[&definition],
                                captures: Vec::new(),
                            }
                        };
                        self.push_instruction(instruction, expression.span)?;
                    }
                    DefinitionOrigin::Builtin(
                        builtin @ (Builtin::ConsoleWrite
                        | Builtin::TextFormat
                        | Builtin::Max
                        | Builtin::Min),
                    ) => {
                        self.push_instruction(
                            Instruction::MakeClosure {
                                destination,
                                function: self.owner.builtin_indices[&(self.module.id, builtin)],
                                captures: Vec::new(),
                            },
                            expression.span,
                        )?;
                    }
                    DefinitionOrigin::Builtin(builtin) => {
                        return Err(unsupported_module(
                            self.module,
                            expression.span,
                            builtin.qualified_name(),
                        ));
                    }
                    DefinitionOrigin::Prelude(_) if self.owner.aggregate_mode => {
                        return self.lower_constructor_application(
                            expression,
                            &definition,
                            &[],
                            &mut environment.clone(),
                        );
                    }
                    DefinitionOrigin::Prelude(_) => {
                        return Err(unsupported_module(
                            self.module,
                            expression.span,
                            "Prelude value",
                        ));
                    }
                }
                Ok(destination)
            }
        }
    }

    fn emit_constant(
        &mut self,
        expression: &hir::Expression,
        value: Constant,
    ) -> Result<RegisterIndex, LoweringError> {
        self.emit_constant_span(expression.span, value)
    }

    fn emit_constant_span(
        &mut self,
        span: Span,
        value: Constant,
    ) -> Result<RegisterIndex, LoweringError> {
        let key = constant_key(&value)?;
        let constant = self
            .owner
            .constant_indices
            .get(&key)
            .copied()
            .ok_or_else(|| {
                invalid_module(self.module, span, "constant is absent from canonical table")
            })?;
        let destination = self.new_register(span)?;
        self.push_instruction(
            Instruction::Const {
                destination,
                constant,
            },
            span,
        )?;
        Ok(destination)
    }

    fn ensure_expression_type(&self, expression: &hir::Expression) -> Result<(), LoweringError> {
        let value = self
            .owner
            .snapshot
            .checked()
            .typed()
            .expression_type(ExpressionKey::new(self.module.id, expression.id))
            .ok_or_else(|| {
                invalid_module(self.module, expression.span, "expression type is absent")
            })?;
        ensure_supported_shape(
            self.owner.snapshot.checked().typed(),
            value,
            self.module,
            expression.span,
            self.owner.aggregate_mode,
        )
    }

    fn expression_type_key(&self, expression: &hir::Expression) -> Result<TypeKey, LoweringError> {
        let value = self
            .owner
            .snapshot
            .checked()
            .typed()
            .expression_type(ExpressionKey::new(self.module.id, expression.id))
            .ok_or_else(|| {
                invalid_module(self.module, expression.span, "expression type is absent")
            })?;
        checked_type_key(
            self.owner.snapshot.checked().typed(),
            value,
            self.module,
            expression.span,
        )
    }

    fn type_index(&self, value: &TypeKey) -> Result<TypeIndex, LoweringError> {
        self.owner
            .type_indices
            .get(value)
            .copied()
            .ok_or_else(|| invalid_without_span("canonical type table omitted a referenced type"))
    }

    fn new_register(&mut self, span: Span) -> Result<RegisterIndex, LoweringError> {
        if self.next_register >= self.owner.limits.registers_per_function() {
            return Err(LoweringError::at(
                &self.module.hir.source_name,
                span,
                LoweringErrorKind::ResourceLimit {
                    resource: "registers_per_function".to_owned(),
                    actual: u64::from(self.next_register) + 1,
                    maximum: u64::from(self.owner.limits.registers_per_function()),
                },
            ));
        }
        let register = RegisterIndex::new(self.next_register);
        self.next_register += 1;
        Ok(register)
    }

    fn new_block(&mut self) -> Result<BlockIndex, LoweringError> {
        let index = to_u32(self.blocks.len(), "block index")?;
        self.blocks.push(BlockBuilder::new());
        Ok(BlockIndex::new(index))
    }

    fn set_current_block(&mut self, block: BlockIndex) -> Result<(), LoweringError> {
        let index = usize::try_from(block.get())
            .map_err(|_| invalid_without_span("block index does not fit host usize"))?;
        if index >= self.blocks.len() {
            return Err(invalid_without_span("current block index is absent"));
        }
        self.current_block = index;
        Ok(())
    }

    fn set_terminator(&mut self, terminator: Terminator, span: Span) -> Result<(), LoweringError> {
        if self.blocks[self.current_block].terminator.is_some() {
            return Err(invalid_module(
                self.module,
                span,
                "block already has a terminator",
            ));
        }
        let ordinal = to_u32(
            self.blocks[self.current_block].instructions.len(),
            "terminator ordinal",
        )?;
        self.push_source_map(span, ordinal, SourceOrigin::LoweringDerived)?;
        self.blocks[self.current_block].terminator = Some(terminator);
        Ok(())
    }

    fn finish_blocks(&mut self) -> Result<Vec<Block>, LoweringError> {
        self.blocks
            .drain(..)
            .map(|block| {
                let terminator = block.terminator.ok_or_else(|| {
                    invalid_without_span("reachable lowering block has no terminator")
                })?;
                Ok(Block {
                    parameters: block.parameters,
                    instructions: block.instructions,
                    terminator,
                })
            })
            .collect()
    }

    fn push_instruction(
        &mut self,
        instruction: Instruction,
        span: Span,
    ) -> Result<(), LoweringError> {
        if self.blocks[self.current_block].terminator.is_some() {
            return Err(invalid_module(
                self.module,
                span,
                "instruction emitted after block terminator",
            ));
        }
        let ordinal = to_u32(
            self.blocks[self.current_block].instructions.len(),
            "instruction ordinal",
        )?;
        self.blocks[self.current_block]
            .instructions
            .push(instruction);
        self.push_source_map(span, ordinal, SourceOrigin::Direct)
    }

    fn push_source_map(
        &mut self,
        span: Span,
        ordinal: u32,
        origin: SourceOrigin,
    ) -> Result<(), LoweringError> {
        let source = self
            .owner
            .source_inputs
            .get(&span.source())
            .ok_or_else(|| {
                invalid_module(
                    self.module,
                    span,
                    "source-map span has no exact source snapshot",
                )
            })?;
        let start = span.start().get() as usize;
        let end = span.end().get() as usize;
        let original = source.original_text();
        if end > original.len()
            || start > end
            || !original.is_char_boundary(start)
            || !original.is_char_boundary(end)
        {
            return Err(invalid_module(
                self.module,
                span,
                "source-map span is not an original UTF-8 byte range",
            ));
        }
        self.source_map.push(SourceMapEntry {
            function: self.function_index,
            block: BlockIndex::new(
                u32::try_from(self.current_block)
                    .map_err(|_| invalid_without_span("source-map block index overflow"))?,
            ),
            ordinal,
            source: self.owner.source_indices[&span.source()],
            span: SourceSpan::new(start as u64, end as u64),
            origin,
        });
        Ok(())
    }
}

fn simple_reference_target(
    snapshot: &ProgramSnapshot,
    module: ModuleId,
    expression: &hir::Expression,
) -> Result<Option<ReferenceTarget>, LoweringError> {
    let reference = match &expression.kind {
        hir::ExpressionKind::Name { reference, .. }
        | hir::ExpressionKind::Projection { reference, .. } => *reference,
        _ => return Ok(None),
    };
    Ok(snapshot
        .checked()
        .typed()
        .resolved()
        .reference(module, reference)
        .cloned())
}

fn builtin_instruction(
    builtin: Builtin,
    destination: RegisterIndex,
    arguments: &[RegisterIndex],
) -> Result<Instruction, LoweringError> {
    match (builtin, arguments) {
        (Builtin::ConsoleWrite, [text]) => Ok(Instruction::ConsoleWrite {
            destination,
            text: *text,
        }),
        (Builtin::TextFormat, [_, _]) => Ok(Instruction::Intrinsic {
            destination,
            intrinsic: Intrinsic::TextFormat,
            arguments: arguments.to_vec(),
        }),
        (Builtin::Max, [_, _]) => Ok(Instruction::Intrinsic {
            destination,
            intrinsic: Intrinsic::MaxInt,
            arguments: arguments.to_vec(),
        }),
        (Builtin::Min, [_, _]) => Ok(Instruction::Intrinsic {
            destination,
            intrinsic: Intrinsic::MinInt,
            arguments: arguments.to_vec(),
        }),
        _ => Err(invalid_without_span(
            "builtin wrapper arity or kind is invalid",
        )),
    }
}

fn collect_type_shapes(
    snapshot: &ProgramSnapshot,
    module: &ling_resolve::ResolvedModule,
    expression: &hir::Expression,
    builder: &mut TypeTableBuilder,
) -> Result<(), LoweringError> {
    let typed = snapshot.checked().typed();
    let value = typed
        .expression_type(ExpressionKey::new(module.id, expression.id))
        .ok_or_else(|| invalid_module(module, expression.span, "expression type is absent"))?;
    if !matches!(typed.arena().get(value), Type::Function { .. }) {
        builder.insert(checked_type_key(typed, value, module, expression.span)?);
    }
    match &expression.kind {
        hir::ExpressionKind::Sequence(elements) => {
            for element in elements {
                match element {
                    hir::SequenceElement::Let(binding) => {
                        collect_type_shapes(snapshot, module, &binding.value, builder)?
                    }
                    hir::SequenceElement::Expression(value) => {
                        collect_type_shapes(snapshot, module, value, builder)?;
                    }
                }
            }
        }
        hir::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            collect_type_shapes(snapshot, module, condition, builder)?;
            collect_type_shapes(snapshot, module, then_branch, builder)?;
            collect_type_shapes(snapshot, module, else_branch, builder)?;
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            collect_type_shapes(snapshot, module, scrutinee, builder)?;
            for case in cases {
                if let Some(guard) = &case.guard {
                    collect_type_shapes(snapshot, module, guard, builder)?;
                }
                collect_type_shapes(snapshot, module, &case.body, builder)?;
            }
        }
        hir::ExpressionKind::Assignment { value, .. } => {
            collect_type_shapes(snapshot, module, value, builder)?;
        }
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            let is_constructor = matches!(
                simple_reference_target(snapshot, module.id, function)?,
                Some(ReferenceTarget::Definition(ref definition))
                    if typed
                        .resolved()
                        .definition(definition)
                        .is_some_and(|info| info.kind == DefinitionKind::Constructor)
            );
            if !is_constructor {
                collect_type_shapes(snapshot, module, function, builder)?;
            }
            for argument in arguments {
                collect_type_shapes(snapshot, module, argument, builder)?;
            }
        }
        hir::ExpressionKind::Projection { .. } => {}
        hir::ExpressionKind::Binary { left, right, .. } => {
            collect_type_shapes(snapshot, module, left, builder)?;
            collect_type_shapes(snapshot, module, right, builder)?;
        }
        hir::ExpressionKind::Unary { operand, .. } => {
            collect_type_shapes(snapshot, module, operand, builder)?;
        }
        hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
            for element in elements {
                collect_type_shapes(snapshot, module, element, builder)?;
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                collect_type_shapes(snapshot, module, &field.value, builder)?;
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            collect_type_shapes(snapshot, module, base, builder)?;
            for field in fields {
                collect_type_shapes(snapshot, module, &field.value, builder)?;
            }
        }
        hir::ExpressionKind::Name { .. }
        | hir::ExpressionKind::Literal(_)
        | hir::ExpressionKind::Unit => {}
    }
    Ok(())
}

fn collect_constants_v1_1(
    snapshot: &ProgramSnapshot,
    module: &ling_resolve::ResolvedModule,
    expression: &hir::Expression,
    strings: &BTreeMap<String, StringIndex>,
    constants: &mut BTreeMap<ConstantKey, Constant>,
    aggregate_mode: bool,
) -> Result<(), LoweringError> {
    let value = snapshot
        .checked()
        .typed()
        .expression_type(ExpressionKey::new(module.id, expression.id))
        .ok_or_else(|| invalid_module(module, expression.span, "expression type is absent"))?;
    ensure_supported_shape(
        snapshot.checked().typed(),
        value,
        module,
        expression.span,
        aggregate_mode,
    )?;
    match &expression.kind {
        hir::ExpressionKind::Sequence(elements) => {
            let mut final_expression = false;
            for element in elements {
                match element {
                    hir::SequenceElement::Let(binding) => {
                        if binding.mutable {
                            return Err(unsupported_module(
                                module,
                                binding.span,
                                "mutable local binding",
                            ));
                        }
                        if binding.recursive && binding.parameters.is_empty() {
                            return Err(unsupported_module(
                                module,
                                binding.span,
                                "recursive non-function local binding",
                            ));
                        }
                        collect_constants_v1_1(
                            snapshot,
                            module,
                            &binding.value,
                            strings,
                            constants,
                            aggregate_mode,
                        )?;
                        final_expression = false;
                    }
                    hir::SequenceElement::Expression(value) => {
                        collect_constants_v1_1(
                            snapshot,
                            module,
                            value,
                            strings,
                            constants,
                            aggregate_mode,
                        )?;
                        final_expression = true;
                    }
                }
            }
            if !final_expression {
                insert_constant(constants, Constant::Unit)?;
            }
        }
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            let is_constructor = aggregate_mode
                && matches!(
                    simple_reference_target(snapshot, module.id, function)?,
                    Some(ReferenceTarget::Definition(ref definition))
                        if snapshot
                            .checked()
                            .typed()
                            .resolved()
                            .definition(definition)
                            .is_some_and(|info| info.kind == DefinitionKind::Constructor)
                );
            if !is_constructor {
                collect_constants_v1_1(
                    snapshot,
                    module,
                    function,
                    strings,
                    constants,
                    aggregate_mode,
                )?;
            }
            for value in arguments {
                collect_constants_v1_1(
                    snapshot,
                    module,
                    value,
                    strings,
                    constants,
                    aggregate_mode,
                )?;
            }
        }
        hir::ExpressionKind::Name { .. } | hir::ExpressionKind::Projection { .. } => {}
        hir::ExpressionKind::Literal(literal) => {
            insert_constant(
                constants,
                literal_constant_v1_1(snapshot, module, expression, literal, strings)?,
            )?;
        }
        hir::ExpressionKind::Unit => insert_constant(constants, Constant::Unit)?,
        hir::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } if aggregate_mode => {
            collect_constants_v1_1(
                snapshot,
                module,
                condition,
                strings,
                constants,
                aggregate_mode,
            )?;
            collect_constants_v1_1(
                snapshot,
                module,
                then_branch,
                strings,
                constants,
                aggregate_mode,
            )?;
            collect_constants_v1_1(
                snapshot,
                module,
                else_branch,
                strings,
                constants,
                aggregate_mode,
            )?;
        }
        hir::ExpressionKind::Match { scrutinee, cases } if aggregate_mode => {
            collect_constants_v1_1(
                snapshot,
                module,
                scrutinee,
                strings,
                constants,
                aggregate_mode,
            )?;
            for case in cases {
                collect_pattern_constants(snapshot, module, &case.pattern, strings, constants)?;
                if let Some(guard) = &case.guard {
                    collect_constants_v1_1(
                        snapshot,
                        module,
                        guard,
                        strings,
                        constants,
                        aggregate_mode,
                    )?;
                }
                collect_constants_v1_1(
                    snapshot,
                    module,
                    &case.body,
                    strings,
                    constants,
                    aggregate_mode,
                )?;
            }
        }
        hir::ExpressionKind::If { .. } | hir::ExpressionKind::Match { .. } => {
            return Err(unsupported_module(module, expression.span, "control flow"));
        }
        hir::ExpressionKind::Assignment { .. } => {
            return Err(unsupported_module(
                module,
                expression.span,
                "mutable assignment",
            ));
        }
        hir::ExpressionKind::Binary { left, right, .. } if aggregate_mode => {
            collect_constants_v1_1(snapshot, module, left, strings, constants, aggregate_mode)?;
            collect_constants_v1_1(snapshot, module, right, strings, constants, aggregate_mode)?;
        }
        hir::ExpressionKind::Unary { operand, .. } if aggregate_mode => {
            collect_constants_v1_1(
                snapshot,
                module,
                operand,
                strings,
                constants,
                aggregate_mode,
            )?;
        }
        hir::ExpressionKind::Binary { .. } | hir::ExpressionKind::Unary { .. } => {
            return Err(unsupported_module(
                module,
                expression.span,
                "scalar operators",
            ));
        }
        hir::ExpressionKind::Tuple(elements) if aggregate_mode => {
            for element in elements {
                collect_constants_v1_1(
                    snapshot,
                    module,
                    element,
                    strings,
                    constants,
                    aggregate_mode,
                )?;
            }
        }
        hir::ExpressionKind::Record(fields) if aggregate_mode => {
            for field in fields {
                collect_constants_v1_1(
                    snapshot,
                    module,
                    &field.value,
                    strings,
                    constants,
                    aggregate_mode,
                )?;
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } if aggregate_mode => {
            collect_constants_v1_1(snapshot, module, base, strings, constants, aggregate_mode)?;
            for field in fields {
                collect_constants_v1_1(
                    snapshot,
                    module,
                    &field.value,
                    strings,
                    constants,
                    aggregate_mode,
                )?;
            }
        }
        hir::ExpressionKind::Tuple(_) => {
            return Err(unsupported_module(module, expression.span, "tuple"));
        }
        hir::ExpressionKind::Record(_) | hir::ExpressionKind::RecordUpdate { .. } => {
            return Err(unsupported_module(module, expression.span, "record"));
        }
        hir::ExpressionKind::List(_) => {
            return Err(unsupported_module(module, expression.span, "list"));
        }
    }
    Ok(())
}

fn collect_pattern_constants(
    snapshot: &ProgramSnapshot,
    module: &ling_resolve::ResolvedModule,
    pattern: &hir::Pattern,
    strings: &BTreeMap<String, StringIndex>,
    constants: &mut BTreeMap<ConstantKey, Constant>,
) -> Result<(), LoweringError> {
    match &pattern.kind {
        hir::PatternKind::Literal(literal) => {
            insert_constant(
                constants,
                pattern_constant(snapshot, module, pattern, literal, strings)?,
            )?;
        }
        hir::PatternKind::Tuple(patterns) => {
            for pattern in patterns {
                collect_pattern_constants(snapshot, module, pattern, strings, constants)?;
            }
        }
        hir::PatternKind::Record(fields) => {
            for field in fields {
                collect_pattern_constants(snapshot, module, &field.pattern, strings, constants)?;
            }
        }
        hir::PatternKind::Constructor { arguments, .. } => {
            for argument in arguments {
                collect_pattern_constants(snapshot, module, argument, strings, constants)?;
            }
        }
        hir::PatternKind::Binding { .. } | hir::PatternKind::Wildcard | hir::PatternKind::Unit => {}
    }
    Ok(())
}

fn ensure_supported_shape(
    typed: &TypedProgram,
    value: TypeId,
    module: &ling_resolve::ResolvedModule,
    span: Span,
    aggregate_mode: bool,
) -> Result<(), LoweringError> {
    match typed.arena().get(value) {
        Type::Unit | Type::Bool | Type::Int | Type::Text => Ok(()),
        Type::Function { parameters, result } => {
            for value in parameters {
                ensure_supported_shape(typed, *value, module, span, aggregate_mode)?;
            }
            ensure_supported_shape(typed, *result, module, span, aggregate_mode)
        }
        Type::Float64 => Err(unsupported_module(module, span, "Float64")),
        Type::Tuple(elements) if aggregate_mode => {
            for element in elements {
                ensure_supported_shape(typed, *element, module, span, aggregate_mode)?;
            }
            Ok(())
        }
        Type::Tuple(_) => Err(unsupported_module(module, span, "tuple")),
        Type::List(_) => Err(unsupported_module(module, span, "list")),
        Type::NominalRecord {
            definition,
            arguments,
        } if aggregate_mode => {
            for argument in arguments {
                ensure_supported_shape(typed, *argument, module, span, aggregate_mode)?;
            }
            let info = typed
                .records()
                .get(definition)
                .ok_or_else(|| invalid_without_span("record metadata is absent"))?;
            for field in &info.fields {
                if !matches!(typed.arena().get(field.field_type), Type::Variable(_)) {
                    ensure_supported_shape(typed, field.field_type, module, span, aggregate_mode)?;
                }
            }
            Ok(())
        }
        Type::NominalRecord { .. } => Err(unsupported_module(module, span, "record")),
        Type::NominalVariant {
            definition,
            arguments,
        } if aggregate_mode => {
            for argument in arguments {
                ensure_supported_shape(typed, *argument, module, span, aggregate_mode)?;
            }
            let info = typed
                .variants()
                .get(definition)
                .ok_or_else(|| invalid_without_span("variant metadata is absent"))?;
            for case in &info.cases {
                if let Some(payload) = case.payload {
                    if !matches!(typed.arena().get(payload), Type::Variable(_)) {
                        ensure_supported_shape(typed, payload, module, span, aggregate_mode)?;
                    }
                }
            }
            Ok(())
        }
        Type::NominalVariant { .. } => Err(unsupported_module(module, span, "variant")),
        Type::Variable(_) => Err(unsupported_module(module, span, "polymorphic function")),
        Type::Error => Err(invalid_module(
            module,
            span,
            "checked expression retains an error type",
        )),
    }
}

fn literal_constant_v1_1(
    snapshot: &ProgramSnapshot,
    module: &ling_resolve::ResolvedModule,
    expression: &hir::Expression,
    literal: &hir::Literal,
    strings: &BTreeMap<String, StringIndex>,
) -> Result<Constant, LoweringError> {
    match literal {
        hir::Literal::Integer { .. } => {
            let value = snapshot
                .checked()
                .typed()
                .integer(ExpressionKey::new(module.id, expression.id))
                .ok_or_else(|| {
                    invalid_module(module, expression.span, "typed integer is absent")
                })?;
            Ok(integer_constant(value.clone()))
        }
        hir::Literal::Float(_) => Err(unsupported_module(module, expression.span, "Float64")),
        hir::Literal::Text(value) => Ok(Constant::Text(string_index(strings, value)?)),
        hir::Literal::Boolean(value) => Ok(Constant::Bool(*value)),
    }
}

fn pattern_constant(
    _snapshot: &ProgramSnapshot,
    module: &ling_resolve::ResolvedModule,
    pattern: &hir::Pattern,
    literal: &hir::Literal,
    strings: &BTreeMap<String, StringIndex>,
) -> Result<Constant, LoweringError> {
    match literal {
        hir::Literal::Integer { radix, digits } => {
            let value = BigInt::parse_bytes(digits.as_bytes(), *radix).ok_or_else(|| {
                invalid_module(module, pattern.span, "integer pattern literal is invalid")
            })?;
            Ok(integer_constant(value))
        }
        hir::Literal::Float(_) => Err(unsupported_module(module, pattern.span, "Float64")),
        hir::Literal::Text(value) => Ok(Constant::Text(string_index(strings, value)?)),
        hir::Literal::Boolean(value) => Ok(Constant::Bool(*value)),
    }
}

fn integer_constant(value: BigInt) -> Constant {
    let (sign, magnitude) = value.to_bytes_be();
    let magnitude = if sign == Sign::NoSign {
        Vec::new()
    } else {
        let first = magnitude
            .iter()
            .position(|byte| *byte != 0)
            .unwrap_or(magnitude.len());
        magnitude[first..].to_vec()
    };
    let sign = match sign {
        Sign::NoSign => IntegerSign::Zero,
        Sign::Plus => IntegerSign::Positive,
        Sign::Minus => IntegerSign::Negative,
    };
    Constant::Int { sign, magnitude }
}
