use super::*;

use ling_effects::{CheckedFunctionType, EffectRow};

use crate::{CaptureOperand, Intrinsic};

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
        let limits = DecodeLimits::rfc_0015();
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

        let mut resolver = SignatureResolver::new(snapshot, &named, &locals, &local_bindings);
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
        let (types, type_indices) = type_builder.finish()?;
        check_limit("types", types.len(), limits.types())?;

        let mut string_set = BTreeSet::new();
        for module in &modules {
            string_set.insert(module.hir.module.name.normalized());
            for definition in &module.hir.definitions {
                string_set.insert(definition.name.normalized.clone());
                collect_text_strings(&definition.value, &mut string_set);
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
        for plan in named.values() {
            collect_constants_v1_1(
                snapshot,
                plan.module,
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
        check_limit(
            "executable_locations",
            source_map.len(),
            self.limits.executable_locations(),
        )?;

        Ok(LoweredProgramV1_1::new(UnverifiedProgram::from_parts(
            ProgramParts {
                strings: self.strings,
                packages: Vec::new(),
                modules,
                types: self.types,
                constants: self.constants,
                sources,
                functions,
                entry,
                source_map,
            },
        )))
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
    ) -> Self {
        Self {
            snapshot,
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
            Type::Tuple(_) => Err(source_or_global_unsupported(module, span, "tuple")),
            Type::List(_) => Err(source_or_global_unsupported(module, span, "list")),
            Type::NominalRecord { .. } => Err(source_or_global_unsupported(module, span, "record")),
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

#[derive(Default)]
struct TypeTableBuilder {
    values: BTreeSet<TypeKey>,
}

impl TypeTableBuilder {
    fn insert(&mut self, value: TypeKey) {
        if let TypeKey::Function {
            parameters, result, ..
        } = &value
        {
            for parameter in parameters {
                self.insert(parameter.clone());
            }
            self.insert((**result).clone());
            self.values.insert(value);
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

    fn finish(self) -> Result<(Vec<ValueType>, BTreeMap<TypeKey, TypeIndex>), LoweringError> {
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
                .filter_map(|value| wire_type(value, &indices).map(|wire| (value.clone(), wire)))
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

fn wire_type(value: &TypeKey, indices: &BTreeMap<TypeKey, TypeIndex>) -> Option<ValueType> {
    let TypeKey::Function {
        parameters,
        result,
        effects,
    } = value
    else {
        return None;
    };
    Some(ValueType::Function {
        parameters: parameters
            .iter()
            .map(|value| indices.get(value).copied())
            .collect::<Option<Vec<_>>>()?,
        result: indices.get(result.as_ref()).copied()?,
        effects: effects.clone(),
    })
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

struct FunctionEmitter<'a, 'snapshot, 'source> {
    owner: &'a ClosureLowerer<'snapshot, 'source>,
    module: &'snapshot ling_resolve::ResolvedModule,
    function_index: FunctionIndex,
    next_register: u32,
    parameters: Vec<BlockParameter>,
    instructions: Vec<Instruction>,
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
            parameters: Vec::new(),
            instructions: Vec::new(),
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
            self.parameters.push(BlockParameter {
                register,
                value_type,
            });
            parameter_types.push(value_type);
            environment.insert(capture.key, register);
        }
        for (pattern, value) in patterns.iter().zip(&signature.parameters) {
            let value_type = self.type_index(value)?;
            let register = self.new_register(pattern.span)?;
            self.parameters.push(BlockParameter {
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
        let ordinal = to_u32(self.instructions.len(), "terminator ordinal")?;
        self.push_source_map(body.span, ordinal, SourceOrigin::LoweringDerived)?;
        let function = Function {
            kind,
            module: self.owner.module_indices[&self.module.id],
            name: string_index(&self.owner.string_indices, name)?,
            capture_count: to_u32(captures.len(), "capture count")?,
            parameter_types,
            result_type: self.type_index(&signature.result)?,
            effects: signature.effects.clone(),
            register_count: self.next_register,
            blocks: vec![Block {
                parameters: std::mem::take(&mut self.parameters),
                instructions: std::mem::take(&mut self.instructions),
                terminator: Terminator::Return { value: result },
            }],
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
            self.parameters.push(BlockParameter {
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
        let ordinal = to_u32(self.instructions.len(), "terminator ordinal")?;
        self.push_source_map(plan.span, ordinal, SourceOrigin::LoweringDerived)?;
        let function = Function {
            kind: FunctionKind::ClosureBody,
            module: self.owner.module_indices[&self.module.id],
            name: string_index(&self.owner.string_indices, &plan.label)?,
            capture_count: 0,
            parameter_types,
            result_type: self.type_index(&signature.result)?,
            effects: signature.effects.clone(),
            register_count: self.next_register,
            blocks: vec![Block {
                parameters: std::mem::take(&mut self.parameters),
                instructions: std::mem::take(&mut self.instructions),
                terminator: Terminator::Return { value: destination },
            }],
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
            hir::ExpressionKind::Name { reference, .. }
            | hir::ExpressionKind::Projection { reference, .. } => {
                self.lower_reference(expression, *reference, environment)
            }
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
            hir::ExpressionKind::If { .. } => {
                Err(unsupported_module(self.module, expression.span, "if"))
            }
            hir::ExpressionKind::Match { .. } => {
                Err(unsupported_module(self.module, expression.span, "match"))
            }
            hir::ExpressionKind::Assignment { .. } => Err(unsupported_module(
                self.module,
                expression.span,
                "mutable assignment",
            )),
            hir::ExpressionKind::Binary { .. } => Err(unsupported_module(
                self.module,
                expression.span,
                "scalar operators",
            )),
            hir::ExpressionKind::Unary { .. } => Err(unsupported_module(
                self.module,
                expression.span,
                "integer unary operators",
            )),
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
        let key = constant_key(&value)?;
        let constant = self
            .owner
            .constant_indices
            .get(&key)
            .copied()
            .ok_or_else(|| {
                invalid_module(
                    self.module,
                    expression.span,
                    "constant is absent from canonical table",
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

    fn push_instruction(
        &mut self,
        instruction: Instruction,
        span: Span,
    ) -> Result<(), LoweringError> {
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
            block: BlockIndex::new(0),
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

fn collect_constants_v1_1(
    snapshot: &ProgramSnapshot,
    module: &ling_resolve::ResolvedModule,
    expression: &hir::Expression,
    strings: &BTreeMap<String, StringIndex>,
    constants: &mut BTreeMap<ConstantKey, Constant>,
) -> Result<(), LoweringError> {
    let value = snapshot
        .checked()
        .typed()
        .expression_type(ExpressionKey::new(module.id, expression.id))
        .ok_or_else(|| invalid_module(module, expression.span, "expression type is absent"))?;
    ensure_supported_shape(snapshot.checked().typed(), value, module, expression.span)?;
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
                        )?;
                        final_expression = false;
                    }
                    hir::SequenceElement::Expression(value) => {
                        collect_constants_v1_1(snapshot, module, value, strings, constants)?;
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
            collect_constants_v1_1(snapshot, module, function, strings, constants)?;
            for value in arguments {
                collect_constants_v1_1(snapshot, module, value, strings, constants)?;
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
        hir::ExpressionKind::If { .. } => {
            return Err(unsupported_module(module, expression.span, "if"));
        }
        hir::ExpressionKind::Match { .. } => {
            return Err(unsupported_module(module, expression.span, "match"));
        }
        hir::ExpressionKind::Assignment { .. } => {
            return Err(unsupported_module(
                module,
                expression.span,
                "mutable assignment",
            ));
        }
        hir::ExpressionKind::Binary { .. } => {
            return Err(unsupported_module(
                module,
                expression.span,
                "scalar operators",
            ));
        }
        hir::ExpressionKind::Unary { .. } => {
            return Err(unsupported_module(
                module,
                expression.span,
                "integer unary operators",
            ));
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

fn ensure_supported_shape(
    typed: &TypedProgram,
    value: TypeId,
    module: &ling_resolve::ResolvedModule,
    span: Span,
) -> Result<(), LoweringError> {
    match typed.arena().get(value) {
        Type::Unit | Type::Bool | Type::Int | Type::Text => Ok(()),
        Type::Function { parameters, result } => {
            for value in parameters {
                ensure_supported_shape(typed, *value, module, span)?;
            }
            ensure_supported_shape(typed, *result, module, span)
        }
        Type::Float64 => Err(unsupported_module(module, span, "Float64")),
        Type::Tuple(_) => Err(unsupported_module(module, span, "tuple")),
        Type::List(_) => Err(unsupported_module(module, span, "list")),
        Type::NominalRecord { .. } => Err(unsupported_module(module, span, "record")),
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
            let (sign, magnitude) = value.to_bytes_be();
            let sign = match sign {
                Sign::NoSign => IntegerSign::Zero,
                Sign::Plus => IntegerSign::Positive,
                Sign::Minus => IntegerSign::Negative,
            };
            Ok(Constant::Int { sign, magnitude })
        }
        hir::Literal::Float(_) => Err(unsupported_module(module, expression.span, "Float64")),
        hir::Literal::Text(value) => Ok(Constant::Text(string_index(strings, value)?)),
        hir::Literal::Boolean(value) => Ok(Constant::Bool(*value)),
    }
}
