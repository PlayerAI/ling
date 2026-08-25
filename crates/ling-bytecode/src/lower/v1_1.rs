use super::*;

use ling_effects::{CheckedFunctionType, EffectRow};

use crate::{
    CaptureOperand, HandlerClause as BytecodeHandlerClause, HandlerOperation, Intrinsic,
    RecordField, RecordUpdate, VariantCase,
};

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
    super::reject_checked_tasks(snapshot)?;
    ClosureLowerer::new(snapshot, sources)?.run()
}

pub(crate) fn lower_v1_2_model(
    snapshot: &ProgramSnapshot,
    sources: &[LoweringSource<'_>],
) -> Result<UnverifiedProgram, LoweringError> {
    super::reject_checked_tasks(snapshot)?;
    ClosureLowerer::new_with_mode(snapshot, sources, true)?.run_model()
}

pub(crate) fn lower_v1_3_model(
    snapshot: &ProgramSnapshot,
    sources: &[LoweringSource<'_>],
) -> Result<UnverifiedProgram, LoweringError> {
    super::reject_checked_tasks(snapshot)?;
    ClosureLowerer::new_with_modes(snapshot, sources, true, true, false)?.run_model()
}

pub(crate) fn lower_v1_4_model(
    snapshot: &ProgramSnapshot,
    sources: &[LoweringSource<'_>],
) -> Result<UnverifiedProgram, LoweringError> {
    super::reject_checked_tasks(snapshot)?;
    ClosureLowerer::new_with_modes(snapshot, sources, true, true, true)?.run_model()
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

#[derive(Clone)]
struct HandlerPlan<'a> {
    key: ExpressionKey,
    module: &'a ling_resolve::ResolvedModule,
    expression: &'a hir::Expression,
    body: &'a hir::Expression,
    clauses: &'a [hir::HandlerClause],
    body_label: String,
    clause_labels: Vec<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandlerFunctionKey {
    Body(ExpressionKey),
    Clause(ExpressionKey, usize),
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
        effects: Vec<EffectKey>,
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
    Cell(Box<TypeKey>),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum EffectKey {
    ConsoleWrite,
    State(Box<TypeKey>),
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
        effects: Vec<EffectKey>,
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
    effects: Vec<EffectKey>,
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
    cell: bool,
    value_type: TypeKey,
}

#[derive(Clone)]
enum OrderedPlan {
    Named(DefinitionId),
    Local(BindingKey),
    Builtin(ModuleId, Builtin),
    Handler(HandlerFunctionKey),
}

struct ClosureLowerer<'snapshot, 'source> {
    snapshot: &'snapshot ProgramSnapshot,
    limits: DecodeLimits,
    aggregate_mode: bool,
    handler_mode: bool,
    modules: Vec<&'snapshot ling_resolve::ResolvedModule>,
    module_indices: BTreeMap<ModuleId, ModuleIndex>,
    source_plans: Vec<SourcePlan<'source>>,
    source_indices: BTreeMap<SourceId, SourceIndex>,
    source_inputs: BTreeMap<SourceId, &'source SourceFile>,
    named: BTreeMap<DefinitionId, NamedPlan<'snapshot>>,
    locals: BTreeMap<BindingKey, LocalPlan<'snapshot>>,
    builtins: BTreeMap<(ModuleId, Builtin), BuiltinPlan<'snapshot>>,
    handlers: BTreeMap<ExpressionKey, HandlerPlan<'snapshot>>,
    captures: BTreeMap<BindingKey, Vec<CapturePlan>>,
    named_signatures: BTreeMap<DefinitionId, SignatureKey>,
    local_signatures: BTreeMap<BindingKey, SignatureKey>,
    builtin_signatures: BTreeMap<Builtin, SignatureKey>,
    handler_signatures: BTreeMap<HandlerFunctionKey, SignatureKey>,
    handler_captures: BTreeMap<HandlerFunctionKey, Vec<CapturePlan>>,
    cell_bindings: BTreeSet<BindingKey>,
    cell_binding_types: BTreeMap<BindingKey, TypeKey>,
    ordered: Vec<OrderedPlan>,
    function_indices: BTreeMap<DefinitionId, FunctionIndex>,
    local_indices: BTreeMap<BindingKey, FunctionIndex>,
    builtin_indices: BTreeMap<(ModuleId, Builtin), FunctionIndex>,
    handler_indices: BTreeMap<HandlerFunctionKey, FunctionIndex>,
    types: Vec<ValueType>,
    type_indices: BTreeMap<TypeKey, TypeIndex>,
    strings: Vec<String>,
    string_indices: BTreeMap<String, StringIndex>,
    constants: Vec<Constant>,
    constant_indices: BTreeMap<ConstantKey, ConstantIndex>,
}

#[derive(Clone, Copy)]
struct PatternFailure<'a> {
    block: BlockIndex,
    parameters: Option<&'a [(BindingKey, RegisterIndex)]>,
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
        Self::new_with_modes(snapshot, sources, aggregate_mode, false, false)
    }

    fn new_with_modes(
        snapshot: &'snapshot ProgramSnapshot,
        sources: &'source [LoweringSource<'source>],
        aggregate_mode: bool,
        handler_mode: bool,
        cell_mode: bool,
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
            for (impl_ordinal, implementation) in module.hir.impls.iter().enumerate() {
                for (member_ordinal, definition) in implementation.members.iter().enumerate() {
                    let id = resolved
                        .impl_members()
                        .values()
                        .find(|member| {
                            member.module == module.id
                                && member.impl_ordinal == impl_ordinal
                                && member.member_ordinal == member_ordinal
                        })
                        .map(|member| member.definition.clone())
                        .ok_or_else(|| {
                            invalid_module(
                                module,
                                definition.span,
                                "implementation member has no resolved DefinitionId",
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
        }
        let mut locals = BTreeMap::new();
        let mut local_bindings = BTreeMap::new();
        let mut builtins = BTreeMap::new();
        let mut handlers = BTreeMap::new();
        let mut binding_order = BTreeMap::new();
        let mut order = 0_usize;
        let mut ordinals = BTreeMap::<ModuleId, u64>::new();
        for module in &modules {
            for definition in &module.hir.definitions {
                for pattern in &definition.parameters {
                    collect_pattern_order(module.id, pattern, &mut binding_order, &mut order);
                }
                if handler_mode {
                    collect_handler_plans(
                        module,
                        &definition.value,
                        &mut handlers,
                        ordinals.entry(module.id).or_default(),
                    )?;
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
            for implementation in &module.hir.impls {
                for definition in &implementation.members {
                    for pattern in &definition.parameters {
                        collect_pattern_order(module.id, pattern, &mut binding_order, &mut order);
                    }
                    if handler_mode {
                        collect_handler_plans(
                            module,
                            &definition.value,
                            &mut handlers,
                            ordinals.entry(module.id).or_default(),
                        )?;
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
        let mut raw_handler_captures = BTreeMap::new();
        for plan in handlers.values() {
            let body_key = HandlerFunctionKey::Body(plan.key);
            let mut declared = BTreeSet::new();
            collect_declared_bindings(plan.module.id, plan.body, &mut declared);
            let mut free = BTreeSet::new();
            collect_free_bindings(
                plan.module.id,
                plan.body,
                resolved,
                &locals,
                &binding_order,
                &mut raw_captures,
                &mut visiting,
                &declared,
                &mut free,
            )?;
            let mut free = free.into_iter().collect::<Vec<_>>();
            free.sort_by_key(|binding| binding_order.get(binding).copied().unwrap_or(usize::MAX));
            raw_handler_captures.insert(body_key, free);

            for (index, clause) in plan.clauses.iter().enumerate() {
                let mut declared = BTreeSet::new();
                for parameter in &clause.parameters {
                    collect_pattern_bindings(plan.module.id, parameter, &mut declared);
                }
                if let Some(resume) = &clause.resume {
                    declared.insert(BindingKey::new(plan.module.id, resume.id));
                }
                collect_declared_bindings(plan.module.id, &clause.body, &mut declared);
                let mut free = BTreeSet::new();
                collect_free_bindings(
                    plan.module.id,
                    &clause.body,
                    resolved,
                    &locals,
                    &binding_order,
                    &mut raw_captures,
                    &mut visiting,
                    &declared,
                    &mut free,
                )?;
                let mut free = free.into_iter().collect::<Vec<_>>();
                free.sort_by_key(|binding| {
                    binding_order.get(binding).copied().unwrap_or(usize::MAX)
                });
                raw_handler_captures.insert(HandlerFunctionKey::Clause(plan.key, index), free);
            }
        }

        let cell_bindings = if cell_mode {
            resolved
                .bindings()
                .iter()
                .filter_map(|(key, binding)| binding.mutable.then_some(*key))
                .collect()
        } else {
            BTreeSet::new()
        };
        let mut cell_binding_types = BTreeMap::new();
        for key in &cell_bindings {
            let info = resolved
                .bindings()
                .get(key)
                .ok_or_else(|| invalid_without_span("Cell-selected binding metadata is absent"))?;
            let module = resolved
                .module(key.module())
                .ok_or_else(|| invalid_without_span("Cell-selected binding module is absent"))?;
            let value = checked
                .typed()
                .binding_type(*key)
                .ok_or_else(|| invalid_module(module, info.span, "Cell value type is absent"))?;
            if !matches!(checked.typed().arena().get(value), Type::Function { .. }) {
                cell_binding_types.insert(
                    *key,
                    checked_type_key(checked.typed(), value, module, info.span)?,
                );
            }
        }
        let mut state_types = BTreeMap::new();
        for key in &cell_bindings {
            let info = resolved
                .bindings()
                .get(key)
                .ok_or_else(|| invalid_without_span("Cell-selected binding metadata is absent"))?;
            let module = resolved
                .module(key.module())
                .ok_or_else(|| invalid_without_span("mutable binding module is absent"))?;
            let value = checked.typed().binding_type(*key).ok_or_else(|| {
                invalid_module(module, info.span, "mutable binding type is absent")
            })?;
            if matches!(checked.typed().arena().get(value), Type::Function { .. }) {
                continue;
            }
            let identity = checked.typed().arena().display(value);
            let value_type = checked_type_key(checked.typed(), value, module, info.span)?;
            if let Some(previous) = state_types.insert(identity, value_type.clone()) {
                if previous != value_type {
                    return Err(invalid_module(
                        module,
                        info.span,
                        "State identity maps to inconsistent value types",
                    ));
                }
            }
        }
        let provisional_state_types = state_types.clone();
        if cell_binding_types.len() != cell_bindings.len() {
            let mut provisional = SignatureResolver::new(
                snapshot,
                &named,
                &locals,
                &local_bindings,
                aggregate_mode,
                provisional_state_types,
                cell_binding_types.clone(),
            );
            for key in &cell_bindings {
                if !cell_binding_types.contains_key(key) {
                    cell_binding_types.insert(*key, provisional.binding_value_type(*key)?);
                }
            }
        }
        for key in &cell_bindings {
            let info = resolved
                .bindings()
                .get(key)
                .ok_or_else(|| invalid_without_span("Cell-selected binding metadata is absent"))?;
            let module = resolved
                .module(key.module())
                .ok_or_else(|| invalid_without_span("Cell-selected binding module is absent"))?;
            let value = checked
                .typed()
                .binding_type(*key)
                .ok_or_else(|| invalid_module(module, info.span, "Cell value type is absent"))?;
            let identity = checked.typed().arena().display(value);
            let value_type = cell_binding_types
                .get(key)
                .cloned()
                .ok_or_else(|| invalid_module(module, info.span, "Cell value type is absent"))?;
            if let Some(previous) = state_types.insert(identity, value_type.clone()) {
                if previous != value_type {
                    return Err(invalid_module(
                        module,
                        info.span,
                        "Cell State identity maps to inconsistent value types",
                    ));
                }
            }
        }

        let mut resolver = SignatureResolver::new(
            snapshot,
            &named,
            &locals,
            &local_bindings,
            aggregate_mode,
            state_types,
            cell_binding_types.clone(),
        );
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
        let mut handler_signatures = BTreeMap::new();
        for plan in handlers.values() {
            handler_signatures.insert(
                HandlerFunctionKey::Body(plan.key),
                resolver.handler_body(plan)?,
            );
            for (index, clause) in plan.clauses.iter().enumerate() {
                handler_signatures.insert(
                    HandlerFunctionKey::Clause(plan.key, index),
                    resolver.handler_clause(plan, clause)?,
                );
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
                // Experimental 1.3 Handler lowering retains its historical
                // immutable-capture boundary; DEC-0262 assigns shared Cells to 1.4.
                let cell = info.mutable && cell_bindings.contains(&key);
                if info.mutable && !cell {
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
                    cell,
                    value_type,
                });
            }
            captures.insert(owner, plans);
        }
        let mut handler_captures = BTreeMap::new();
        for (owner, keys) in raw_handler_captures {
            let plan = match &owner {
                HandlerFunctionKey::Body(key) | HandlerFunctionKey::Clause(key, _) => {
                    &handlers[key]
                }
            };
            let mut plans = Vec::with_capacity(keys.len());
            for key in keys {
                let info = resolved.bindings().get(&key).ok_or_else(|| {
                    invalid_module(
                        plan.module,
                        plan.expression.span,
                        "Handler capture binding metadata is absent",
                    )
                })?;
                let cell = info.mutable && cell_bindings.contains(&key);
                if info.mutable && !cell {
                    return Err(unsupported_module(
                        plan.module,
                        info.span,
                        "mutable Handler capture",
                    ));
                }
                plans.push(CapturePlan {
                    key,
                    self_reference: false,
                    cell,
                    value_type: resolver.binding_value_type(key)?,
                });
            }
            handler_captures.insert(owner, plans);
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
        for plan in handlers.values() {
            ordered_keys.push((
                module_indices[&plan.module.id],
                FunctionKind::ClosureBody,
                plan.body_label.as_bytes().to_vec(),
                OrderedPlan::Handler(HandlerFunctionKey::Body(plan.key)),
            ));
            for (index, label) in plan.clause_labels.iter().enumerate() {
                ordered_keys.push((
                    module_indices[&plan.module.id],
                    FunctionKind::ClosureBody,
                    label.as_bytes().to_vec(),
                    OrderedPlan::Handler(HandlerFunctionKey::Clause(plan.key, index)),
                ));
            }
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
        let mut handler_indices = BTreeMap::new();
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
                OrderedPlan::Handler(key) => {
                    handler_indices.insert(key.clone(), index);
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
            for implementation in &module.hir.impls {
                for definition in &implementation.members {
                    string_set.insert(definition.name.normalized.clone());
                    collect_text_strings(&definition.value, &mut string_set);
                }
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
        for plan in handlers.values() {
            string_set.insert(plan.body_label.clone());
            string_set.extend(plan.clause_labels.iter().cloned());
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
        for signature in handler_signatures.values() {
            type_builder.add_signature(signature)?;
        }
        for plans in captures.values() {
            for capture in plans {
                type_builder.insert(capture.value_type.clone());
                if capture.cell {
                    type_builder.insert(TypeKey::Cell(Box::new(capture.value_type.clone())));
                }
            }
        }
        for plans in handler_captures.values() {
            for capture in plans {
                type_builder.insert(capture.value_type.clone());
                if capture.cell {
                    type_builder.insert(TypeKey::Cell(Box::new(capture.value_type.clone())));
                }
            }
        }
        for value_type in cell_binding_types.values() {
            type_builder.insert(value_type.clone());
            type_builder.insert(TypeKey::Cell(Box::new(value_type.clone())));
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
            handler_mode,
            modules,
            module_indices,
            source_plans,
            source_indices,
            source_inputs,
            named,
            locals,
            builtins,
            handlers,
            captures,
            named_signatures,
            local_signatures,
            builtin_signatures,
            handler_signatures,
            handler_captures,
            cell_bindings,
            cell_binding_types,
            ordered,
            function_indices,
            local_indices,
            builtin_indices,
            handler_indices,
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
                OrderedPlan::Handler(key) => self.lower_handler_function(key)?,
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
            (&plan.definition.parameters, None),
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
            (&plan.binding.parameters, None),
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

    fn lower_handler_function(
        &self,
        key: &HandlerFunctionKey,
    ) -> Result<(Function, Vec<SourceMapEntry>), LoweringError> {
        let plan = match key {
            HandlerFunctionKey::Body(expression) | HandlerFunctionKey::Clause(expression, _) => {
                &self.handlers[expression]
            }
        };
        let emitter = FunctionEmitter::new(self, plan.module, self.handler_indices[key]);
        match key {
            HandlerFunctionKey::Body(_) => emitter.lower_source(
                FunctionKind::ClosureBody,
                &plan.body_label,
                &self.handler_captures[key],
                (&[], None),
                plan.body,
                &self.handler_signatures[key],
            ),
            HandlerFunctionKey::Clause(_, index) => {
                let clause = &plan.clauses[*index];
                emitter.lower_source(
                    FunctionKind::ClosureBody,
                    &plan.clause_labels[*index],
                    &self.handler_captures[key],
                    (&clause.parameters, clause.resume.as_ref()),
                    &clause.body,
                    &self.handler_signatures[key],
                )
            }
        }
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

fn collect_handler_plans<'a>(
    module: &'a ling_resolve::ResolvedModule,
    expression: &'a hir::Expression,
    output: &mut BTreeMap<ExpressionKey, HandlerPlan<'a>>,
    ordinal: &mut u64,
) -> Result<(), LoweringError> {
    let visit =
        |value: &'a hir::Expression,
         output: &mut BTreeMap<ExpressionKey, HandlerPlan<'a>>,
         ordinal: &mut u64| { collect_handler_plans(module, value, output, ordinal) };
    match &expression.kind {
        hir::ExpressionKind::Sequence(elements) => {
            for element in elements {
                visit(
                    match element {
                        hir::SequenceElement::Let(binding) => &binding.value,
                        hir::SequenceElement::LetAwait(binding) => &binding.call,
                        hir::SequenceElement::Expression(value) => value,
                    },
                    output,
                    ordinal,
                )?;
            }
        }
        hir::ExpressionKind::TaskScope { body, .. } => visit(body, output, ordinal)?,
        hir::ExpressionKind::TaskSpawn { call, .. } => visit(call, output, ordinal)?,
        hir::ExpressionKind::TaskAwait { handle, .. } => visit(handle, output, ordinal)?,
        hir::ExpressionKind::TaskReturn { value, .. } => visit(value, output, ordinal)?,
        hir::ExpressionKind::Handle { body, clauses } => {
            let key = ExpressionKey::new(module.id, expression.id);
            let body_label = closure_label(expression.span, *ordinal);
            *ordinal = ordinal
                .checked_add(1)
                .ok_or_else(|| resource_error("closure_ordinal", u64::MAX, u64::MAX - 1))?;
            let mut clause_labels = Vec::with_capacity(clauses.len());
            for clause in clauses {
                clause_labels.push(closure_label(clause.span, *ordinal));
                *ordinal = ordinal
                    .checked_add(1)
                    .ok_or_else(|| resource_error("closure_ordinal", u64::MAX, u64::MAX - 1))?;
            }
            if output
                .insert(
                    key,
                    HandlerPlan {
                        key,
                        module,
                        expression,
                        body,
                        clauses,
                        body_label,
                        clause_labels,
                    },
                )
                .is_some()
            {
                return Err(invalid_module(
                    module,
                    expression.span,
                    "duplicate checked Handler expression identity",
                ));
            }
            visit(body, output, ordinal)?;
            for clause in clauses {
                visit(&clause.body, output, ordinal)?;
            }
        }
        hir::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            visit(condition, output, ordinal)?;
            visit(then_branch, output, ordinal)?;
            visit(else_branch, output, ordinal)?;
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            visit(scrutinee, output, ordinal)?;
            for case in cases {
                if let Some(guard) = &case.guard {
                    visit(guard, output, ordinal)?;
                }
                visit(&case.body, output, ordinal)?;
            }
        }
        hir::ExpressionKind::Assignment { value, .. }
        | hir::ExpressionKind::Unary { operand: value, .. }
        | hir::ExpressionKind::Projection { target: value, .. } => {
            visit(value, output, ordinal)?;
        }
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            visit(function, output, ordinal)?;
            for argument in arguments {
                visit(argument, output, ordinal)?;
            }
        }
        hir::ExpressionKind::Binary { left, right, .. } => {
            visit(left, output, ordinal)?;
            visit(right, output, ordinal)?;
        }
        hir::ExpressionKind::Tuple(values) | hir::ExpressionKind::List(values) => {
            for value in values {
                visit(value, output, ordinal)?;
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                visit(&field.value, output, ordinal)?;
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            visit(base, output, ordinal)?;
            for field in fields {
                visit(&field.value, output, ordinal)?;
            }
        }
        hir::ExpressionKind::Name { .. }
        | hir::ExpressionKind::Literal(_)
        | hir::ExpressionKind::Unit => {}
    }
    Ok(())
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
                    hir::SequenceElement::LetAwait(binding) => {
                        return Err(unsupported_module(
                            module,
                            binding.span,
                            "structured task execution (L-TASK-0004)",
                        ));
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
        hir::ExpressionKind::TaskScope { .. }
        | hir::ExpressionKind::TaskSpawn { .. }
        | hir::ExpressionKind::TaskAwait { .. }
        | hir::ExpressionKind::TaskReturn { .. } => {
            return Err(unsupported_module(
                module,
                expression.span,
                "structured task execution (L-TASK-0004)",
            ));
        }
        hir::ExpressionKind::Handle { body, clauses } => {
            collect_lifted(
                snapshot,
                module,
                body,
                locals,
                local_bindings,
                builtins,
                binding_order,
                order,
                ordinal,
            )?;
            for clause in clauses {
                for parameter in &clause.parameters {
                    collect_pattern_order(module.id, parameter, binding_order, order);
                }
                if let Some(resume) = &clause.resume {
                    binding_order.insert(BindingKey::new(module.id, resume.id), *order);
                    *order = order.saturating_add(1);
                }
                collect_lifted(
                    snapshot,
                    module,
                    &clause.body,
                    locals,
                    local_bindings,
                    builtins,
                    binding_order,
                    order,
                    ordinal,
                )?;
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
                    hir::SequenceElement::LetAwait(binding) => {
                        collect_pattern_bindings(module, &binding.pattern, output);
                        collect_declared_bindings(module, &binding.call, output);
                    }
                    hir::SequenceElement::Expression(value) => {
                        collect_declared_bindings(module, value, output);
                    }
                }
            }
        }
        hir::ExpressionKind::TaskScope { body, .. } => {
            collect_declared_bindings(module, body, output)
        }
        hir::ExpressionKind::TaskSpawn { call, .. } => {
            collect_declared_bindings(module, call, output)
        }
        hir::ExpressionKind::TaskAwait { handle, .. } => {
            collect_declared_bindings(module, handle, output)
        }
        hir::ExpressionKind::TaskReturn { value, .. } => {
            collect_declared_bindings(module, value, output)
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
        hir::ExpressionKind::Handle { body, clauses } => {
            collect_declared_bindings(module, body, output);
            for clause in clauses {
                for parameter in &clause.parameters {
                    collect_pattern_bindings(module, parameter, output);
                }
                if let Some(resume) = &clause.resume {
                    output.insert(BindingKey::new(module, resume.id));
                }
                collect_declared_bindings(module, &clause.body, output);
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
                    hir::SequenceElement::LetAwait(binding) => {
                        return Err(LoweringError::without_span(
                            None,
                            LoweringErrorKind::UnsupportedFeature {
                                feature: format!(
                                    "structured task execution at byte {} (L-TASK-0004)",
                                    binding.span.start().get()
                                ),
                            },
                        ));
                    }
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
        hir::ExpressionKind::TaskScope { .. }
        | hir::ExpressionKind::TaskSpawn { .. }
        | hir::ExpressionKind::TaskAwait { .. }
        | hir::ExpressionKind::TaskReturn { .. } => {
            return Err(LoweringError::without_span(
                None,
                LoweringErrorKind::UnsupportedFeature {
                    feature: "structured task execution (L-TASK-0004)".to_owned(),
                },
            ));
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
        hir::ExpressionKind::Assignment { place, value } => {
            add_reference(place.root_reference);
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
        hir::ExpressionKind::Handle { body, clauses } => {
            collect_free_bindings(
                module,
                body,
                resolved,
                locals,
                binding_order,
                memo,
                visiting,
                declared,
                output,
            )?;
            for clause in clauses {
                let mut clause_declared = declared.clone();
                for parameter in &clause.parameters {
                    collect_pattern_bindings(module, parameter, &mut clause_declared);
                }
                if let Some(resume) = &clause.resume {
                    clause_declared.insert(BindingKey::new(module, resume.id));
                }
                collect_free_bindings(
                    module,
                    &clause.body,
                    resolved,
                    locals,
                    binding_order,
                    memo,
                    visiting,
                    &clause_declared,
                    output,
                )?;
            }
        }
        hir::ExpressionKind::Literal(_) | hir::ExpressionKind::Unit => {}
    }
    Ok(())
}

fn collect_cell_effect_bindings(
    module: ModuleId,
    expression: &hir::Expression,
    resolved: &ling_resolve::ResolvedProgram,
    cell_bindings: &BTreeMap<BindingKey, TypeKey>,
    output: &mut BTreeSet<BindingKey>,
) {
    let mut add_reference = |reference: hir::ReferenceId| {
        if let Some(ReferenceTarget::Binding(binding)) = resolved.reference(module, reference)
            && cell_bindings.contains_key(binding)
        {
            output.insert(*binding);
        }
    };
    match &expression.kind {
        hir::ExpressionKind::Sequence(elements) => {
            for element in elements {
                match element {
                    hir::SequenceElement::Let(binding) => {
                        let key = BindingKey::new(module, binding.id);
                        if cell_bindings.contains_key(&key) {
                            // The lexical owner emits the unique CellNew, regardless of
                            // whether the initializer is a value or a lifted closure.
                            output.insert(key);
                        }
                        if binding.parameters.is_empty() {
                            collect_cell_effect_bindings(
                                module,
                                &binding.value,
                                resolved,
                                cell_bindings,
                                output,
                            );
                        }
                    }
                    hir::SequenceElement::LetAwait(binding) => {
                        collect_cell_effect_bindings(
                            module,
                            &binding.call,
                            resolved,
                            cell_bindings,
                            output,
                        );
                    }
                    hir::SequenceElement::Expression(value) => {
                        collect_cell_effect_bindings(module, value, resolved, cell_bindings, output)
                    }
                }
            }
        }
        hir::ExpressionKind::TaskScope { body, .. } => {
            collect_cell_effect_bindings(module, body, resolved, cell_bindings, output)
        }
        hir::ExpressionKind::TaskSpawn { call, .. } => {
            collect_cell_effect_bindings(module, call, resolved, cell_bindings, output)
        }
        hir::ExpressionKind::TaskAwait { handle, .. } => {
            collect_cell_effect_bindings(module, handle, resolved, cell_bindings, output)
        }
        hir::ExpressionKind::TaskReturn { value, .. } => {
            collect_cell_effect_bindings(module, value, resolved, cell_bindings, output)
        }
        hir::ExpressionKind::Handle { body, clauses } => {
            // Handle invokes its lifted body and selected clause in this function's
            // dynamic extent. State is never masked, so retain their Cell effects in
            // the enclosing declaration as well as their own lifted signatures.
            collect_cell_effect_bindings(module, body, resolved, cell_bindings, output);
            for clause in clauses {
                collect_cell_effect_bindings(module, &clause.body, resolved, cell_bindings, output);
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
                collect_cell_effect_bindings(module, value, resolved, cell_bindings, output);
            }
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            collect_cell_effect_bindings(module, scrutinee, resolved, cell_bindings, output);
            for case in cases {
                if let Some(guard) = &case.guard {
                    collect_cell_effect_bindings(module, guard, resolved, cell_bindings, output);
                }
                collect_cell_effect_bindings(module, &case.body, resolved, cell_bindings, output);
            }
        }
        hir::ExpressionKind::Assignment { place, value } => {
            add_reference(place.root_reference);
            collect_cell_effect_bindings(module, value, resolved, cell_bindings, output);
        }
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            collect_cell_effect_bindings(module, function, resolved, cell_bindings, output);
            for argument in arguments {
                collect_cell_effect_bindings(module, argument, resolved, cell_bindings, output);
            }
        }
        hir::ExpressionKind::Projection { reference, .. }
        | hir::ExpressionKind::Name { reference, .. } => add_reference(*reference),
        hir::ExpressionKind::Binary { left, right, .. } => {
            collect_cell_effect_bindings(module, left, resolved, cell_bindings, output);
            collect_cell_effect_bindings(module, right, resolved, cell_bindings, output);
        }
        hir::ExpressionKind::Unary { operand, .. } => {
            collect_cell_effect_bindings(module, operand, resolved, cell_bindings, output);
        }
        hir::ExpressionKind::Tuple(values) | hir::ExpressionKind::List(values) => {
            for value in values {
                collect_cell_effect_bindings(module, value, resolved, cell_bindings, output);
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                collect_cell_effect_bindings(module, &field.value, resolved, cell_bindings, output);
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            collect_cell_effect_bindings(module, base, resolved, cell_bindings, output);
            for field in fields {
                collect_cell_effect_bindings(module, &field.value, resolved, cell_bindings, output);
            }
        }
        hir::ExpressionKind::Literal(_) | hir::ExpressionKind::Unit => {}
    }
}

struct SignatureResolver<'a> {
    snapshot: &'a ProgramSnapshot,
    aggregate_mode: bool,
    named: &'a BTreeMap<DefinitionId, NamedPlan<'a>>,
    locals: &'a BTreeMap<BindingKey, LocalPlan<'a>>,
    local_bindings: &'a BTreeMap<BindingKey, &'a hir::LocalBinding>,
    state_types: BTreeMap<String, TypeKey>,
    cell_binding_types: BTreeMap<BindingKey, TypeKey>,
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
        state_types: BTreeMap<String, TypeKey>,
        cell_binding_types: BTreeMap<BindingKey, TypeKey>,
    ) -> Self {
        Self {
            snapshot,
            aggregate_mode,
            named,
            locals,
            local_bindings,
            state_types,
            cell_binding_types,
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
        let mut signature = if plan.definition.parameters.is_empty() {
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
                effects: self.bytecode_effects(
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
        self.extend_cell_effects(plan.module, &plan.definition.value, &mut signature.effects);
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
        let mut signature = self.signature_from_checked(
            plan.module,
            &plan.binding.parameters,
            &plan.binding.value,
            &checked,
        )?;
        self.extend_cell_effects(plan.module, &plan.binding.value, &mut signature.effects);
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

    fn handler_body(&mut self, plan: &HandlerPlan<'_>) -> Result<SignatureKey, LoweringError> {
        let checked = self.snapshot.checked();
        let result = checked.typed().expression_type(plan.key).ok_or_else(|| {
            invalid_module(
                plan.module,
                plan.expression.span,
                "checked Handler result type is absent",
            )
        })?;
        let effects = checked
            .expression_effect(ExpressionKey::new(plan.module.id, plan.body.id))
            .ok_or_else(|| {
                invalid_module(
                    plan.module,
                    plan.body.span,
                    "checked Handler body Effect row is absent",
                )
            })?;
        let mut signature = SignatureKey {
            parameters: Vec::new(),
            result: self.plain_type(result, Some(plan.module), Some(plan.expression.span))?,
            effects: self.bytecode_effects(effects, plan.module, plan.body.span)?,
        };
        self.extend_cell_effects(plan.module, plan.body, &mut signature.effects);
        Ok(signature)
    }

    fn handler_clause(
        &mut self,
        plan: &HandlerPlan<'_>,
        clause: &hir::HandlerClause,
    ) -> Result<SignatureKey, LoweringError> {
        let checked = self.snapshot.checked();
        let operation =
            resolve_handler_operation(&clause.operation.normalized()).ok_or_else(|| {
                invalid_module(
                    plan.module,
                    clause.operation.span,
                    "checked Handler operation is unregistered",
                )
            })?;
        let result_id = checked.typed().expression_type(plan.key).ok_or_else(|| {
            invalid_module(
                plan.module,
                plan.expression.span,
                "checked Handler result type is absent",
            )
        })?;
        let result = self.plain_type(result_id, Some(plan.module), Some(plan.expression.span))?;
        let mut body_effects = checked
            .expression_effect(ExpressionKey::new(plan.module.id, plan.body.id))
            .map(|row| self.bytecode_effects(row, plan.module, plan.body.span))
            .transpose()?
            .ok_or_else(|| {
                invalid_module(
                    plan.module,
                    plan.body.span,
                    "checked Handler body Effect row is absent",
                )
            })?;
        self.extend_cell_effects(plan.module, plan.body, &mut body_effects);
        if plan
            .clauses
            .iter()
            .any(|value| value.operation.normalized() == "Console.Write.write")
        {
            body_effects.retain(|effect| *effect != EffectKey::ConsoleWrite);
        }
        let mut parameters = operation
            .inputs()
            .iter()
            .copied()
            .map(handler_value_type_key)
            .collect::<Vec<_>>();
        if let Some(resume) = &clause.resume {
            parameters.push(TypeKey::function(
                vec![handler_value_type_key(operation.output())],
                result.clone(),
                body_effects.clone(),
            )?);
            if checked
                .typed()
                .resolved()
                .handler_resume_uses(BindingKey::new(plan.module.id, resume.id))
                .is_some_and(|uses| uses > 0)
            {
                let clause_effects = checked
                    .expression_effect(ExpressionKey::new(plan.module.id, clause.body.id))
                    .ok_or_else(|| {
                        invalid_module(
                            plan.module,
                            clause.body.span,
                            "checked Handler clause Effect row is absent",
                        )
                    })?;
                let mut effects =
                    self.bytecode_effects(clause_effects, plan.module, clause.body.span)?;
                self.extend_cell_effects(plan.module, &clause.body, &mut effects);
                effects.extend(body_effects);
                effects.sort();
                effects.dedup();
                return Ok(SignatureKey {
                    parameters,
                    result,
                    effects,
                });
            }
        }
        let mut effects = checked
            .expression_effect(ExpressionKey::new(plan.module.id, clause.body.id))
            .map(|row| self.bytecode_effects(row, plan.module, clause.body.span))
            .transpose()?
            .ok_or_else(|| {
                invalid_module(
                    plan.module,
                    clause.body.span,
                    "checked Handler clause Effect row is absent",
                )
            })?;
        self.extend_cell_effects(plan.module, &clause.body, &mut effects);
        Ok(SignatureKey {
            parameters,
            result,
            effects,
        })
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
            effects: self.bytecode_effects(checked.effects(), module, body.span)?,
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
            Type::Task { .. } | Type::TaskHandle { .. } => Err(source_or_global_unsupported(
                module,
                span,
                "structured task execution (L-TASK-0004)",
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
            self.bytecode_effects(checked.effects(), module, span)?,
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

    fn bytecode_effects(
        &self,
        row: &EffectRow,
        module: &ling_resolve::ResolvedModule,
        span: Span,
    ) -> Result<Vec<EffectKey>, LoweringError> {
        if row
            .effects()
            .any(|effect| matches!(effect, CheckedEffect::TaskSpawn | CheckedEffect::TaskAwait))
        {
            return Err(unsupported_module(
                module,
                span,
                "structured task execution (L-TASK-0004)",
            ));
        }
        let mut effects = row
            .effects()
            .filter_map(|effect| match effect {
                CheckedEffect::ConsoleWrite => Some(EffectKey::ConsoleWrite),
                CheckedEffect::State { identity, .. } => self
                    .state_types
                    .get(identity)
                    .cloned()
                    .map(|value_type| EffectKey::State(Box::new(value_type))),
                CheckedEffect::TaskSpawn | CheckedEffect::TaskAwait => None,
            })
            .collect::<Vec<_>>();
        effects.sort();
        effects.dedup();
        Ok(effects)
    }

    fn extend_cell_effects(
        &self,
        module: &ling_resolve::ResolvedModule,
        expression: &hir::Expression,
        effects: &mut Vec<EffectKey>,
    ) {
        let mut bindings = BTreeSet::new();
        collect_cell_effect_bindings(
            module.id,
            expression,
            self.snapshot.checked().typed().resolved(),
            &self.cell_binding_types,
            &mut bindings,
        );
        effects.extend(bindings.into_iter().filter_map(|binding| {
            self.cell_binding_types
                .get(&binding)
                .cloned()
                .map(|value_type| EffectKey::State(Box::new(value_type)))
        }));
        effects.sort();
        effects.dedup();
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

const fn handler_value_type_key(value: HandlerValueType) -> TypeKey {
    match value {
        HandlerValueType::Unit => TypeKey::Unit,
        HandlerValueType::Int => TypeKey::Int,
        HandlerValueType::Text => TypeKey::Text,
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
        Type::Task { .. } | Type::TaskHandle { .. } => Err(unsupported_module(
            module,
            span,
            "structured task execution (L-TASK-0004)",
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

fn substitute_nominal_type(
    typed: &TypedProgram,
    definition: &DefinitionId,
    arguments: &[TypeId],
    value: TypeId,
) -> Option<TypeId> {
    let substitutions =
        extend_nominal_substitutions(typed, definition, arguments, &BTreeMap::new());
    match typed.arena().get(value) {
        Type::Variable(variable) => substitutions.get(variable).copied(),
        _ => Some(value),
    }
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
            TypeKey::Cell(value_type) => {
                self.insert((**value_type).clone());
                self.values.insert(value);
            }
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
                parameters,
                result,
                effects,
            } => {
                for parameter in parameters {
                    self.insert(parameter.clone());
                }
                self.insert((**result).clone());
                for effect in effects {
                    if let EffectKey::State(value_type) = effect {
                        self.insert((**value_type).clone());
                    }
                }
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
            effects: wire_effects(effects, indices)?,
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
        TypeKey::Cell(value_type) => {
            Some(ValueType::Cell(indices.get(value_type.as_ref()).copied()?))
        }
    }
}

fn wire_effects(
    effects: &[EffectKey],
    indices: &BTreeMap<TypeKey, TypeIndex>,
) -> Option<Vec<Effect>> {
    let mut effects = effects
        .iter()
        .map(|effect| match effect {
            EffectKey::ConsoleWrite => Some(Effect::ConsoleWrite),
            EffectKey::State(value_type) => {
                indices.get(value_type.as_ref()).copied().map(Effect::State)
            }
        })
        .collect::<Option<Vec<_>>>()?;
    effects.sort();
    effects.dedup();
    Some(effects)
}

fn bytecode_effects_without_source(row: &EffectRow) -> Result<Vec<EffectKey>, LoweringError> {
    row.effects()
        .map(|effect| match effect {
            CheckedEffect::ConsoleWrite => Ok(EffectKey::ConsoleWrite),
            CheckedEffect::State { .. } => Err(invalid_without_span(
                "builtin unexpectedly has a State Effect",
            )),
            CheckedEffect::TaskSpawn | CheckedEffect::TaskAwait => Err(invalid_without_span(
                "builtin unexpectedly has a Task Effect",
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BindingStorage {
    Direct(RegisterIndex),
    Cell {
        handle: RegisterIndex,
        value_type: TypeIndex,
    },
}

type BindingEnvironment = BTreeMap<BindingKey, BindingStorage>;

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
        parameters: (&[hir::Pattern], Option<&hir::HandlerResume>),
        body: &hir::Expression,
        signature: &SignatureKey,
    ) -> Result<(Function, Vec<SourceMapEntry>), LoweringError> {
        let (patterns, resume) = parameters;
        if patterns.len() + usize::from(resume.is_some()) != signature.parameters.len() {
            return Err(invalid_module(
                self.module,
                body.span,
                "function parameter count disagrees with signature",
            ));
        }
        let mut environment = BTreeMap::new();
        let mut parameter_types = Vec::with_capacity(captures.len() + signature.parameters.len());
        for capture in captures {
            let source_value_type = self.type_index(&capture.value_type)?;
            let parameter_type = if capture.cell {
                self.type_index(&TypeKey::Cell(Box::new(capture.value_type.clone())))?
            } else {
                source_value_type
            };
            let register = self.new_register(body.span)?;
            self.blocks[0].parameters.push(BlockParameter {
                register,
                value_type: parameter_type,
            });
            parameter_types.push(parameter_type);
            environment.insert(
                capture.key,
                if capture.cell {
                    BindingStorage::Cell {
                        handle: register,
                        value_type: source_value_type,
                    }
                } else {
                    BindingStorage::Direct(register)
                },
            );
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
                    let key = BindingKey::new(self.module.id, *id);
                    environment.insert(key, self.bind_storage(key, register, pattern.span)?);
                }
                hir::PatternKind::Wildcard => {}
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
        if let Some(resume) = resume {
            let value_type = self.type_index(&signature.parameters[patterns.len()])?;
            let register = self.new_register(resume.name.span)?;
            self.blocks[0].parameters.push(BlockParameter {
                register,
                value_type,
            });
            parameter_types.push(value_type);
            environment.insert(
                BindingKey::new(self.module.id, resume.id),
                BindingStorage::Direct(register),
            );
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
            effects: self.effects(&signature.effects)?,
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
            effects: self.effects(&signature.effects)?,
            register_count: self.next_register,
            blocks,
        };
        Ok((function, self.source_map))
    }

    fn lower_expression(
        &mut self,
        expression: &hir::Expression,
        environment: &mut BindingEnvironment,
    ) -> Result<RegisterIndex, LoweringError> {
        self.ensure_expression_type(expression)?;
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                let mut local = environment.clone();
                let mut result = None;
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            if binding.mutable && !self.owner.aggregate_mode {
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
                                        CaptureOperand::Register(Self::storage_register(
                                            *local.get(&capture.key).ok_or_else(|| {
                                                invalid_module(
                                                    self.module,
                                                    binding.span,
                                                    "captured binding has no lexical register",
                                                )
                                            })?,
                                        ))
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
                            local.insert(key, self.bind_storage(key, value, binding.span)?);
                            result = None;
                        }
                        hir::SequenceElement::LetAwait(binding) => {
                            return Err(unsupported_module(
                                self.module,
                                binding.span,
                                "structured task execution (L-TASK-0004)",
                            ));
                        }
                        hir::SequenceElement::Expression(value) => {
                            result = Some(self.lower_expression(value, &mut local)?);
                        }
                    }
                }
                if self.owner.aggregate_mode {
                    self.propagate_mutable_bindings(environment, &local);
                }
                result.map_or_else(|| self.emit_constant(expression, Constant::Unit), Ok)
            }
            hir::ExpressionKind::TaskScope { .. }
            | hir::ExpressionKind::TaskSpawn { .. }
            | hir::ExpressionKind::TaskAwait { .. }
            | hir::ExpressionKind::TaskReturn { .. } => Err(unsupported_module(
                self.module,
                expression.span,
                "structured task execution (L-TASK-0004)",
            )),
            hir::ExpressionKind::Handle { clauses, .. } => {
                if !self.owner.handler_mode {
                    return Err(unsupported_module(self.module, expression.span, "handler"));
                }
                let key = ExpressionKey::new(self.module.id, expression.id);
                let body_key = HandlerFunctionKey::Body(key);
                let destination = self.new_register(expression.span)?;
                let body_captures = self.capture_operands(
                    &self.owner.handler_captures[&body_key],
                    environment,
                    expression.span,
                )?;
                let mut lowered_clauses = clauses
                    .iter()
                    .enumerate()
                    .map(|(index, clause)| {
                        let operation = match clause.operation.normalized().as_str() {
                            "Console.Write.write" => HandlerOperation::ConsoleWrite,
                            "Clock.now" => HandlerOperation::ClockNow,
                            "Random.next" => HandlerOperation::RandomNext,
                            _ => {
                                return Err(invalid_module(
                                    self.module,
                                    clause.operation.span,
                                    "checked Handler operation is unregistered",
                                ));
                            }
                        };
                        let clause_key = HandlerFunctionKey::Clause(key, index);
                        Ok(BytecodeHandlerClause {
                            operation,
                            resume_present: clause.resume.is_some(),
                            function: self.owner.handler_indices[&clause_key],
                            captures: self.capture_operands(
                                &self.owner.handler_captures[&clause_key],
                                environment,
                                clause.span,
                            )?,
                        })
                    })
                    .collect::<Result<Vec<_>, LoweringError>>()?;
                lowered_clauses.sort_by_key(|clause| clause.operation);
                self.push_instruction(
                    Instruction::Handle {
                        destination,
                        body_function: self.owner.handler_indices[&body_key],
                        body_captures,
                        clauses: lowered_clauses,
                    },
                    expression.span,
                )?;
                Ok(destination)
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
            hir::ExpressionKind::Assignment { place, value } if self.owner.aggregate_mode => {
                self.lower_assignment(expression, place, value, environment)
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

    fn capture_operands(
        &self,
        captures: &[CapturePlan],
        environment: &BindingEnvironment,
        span: Span,
    ) -> Result<Vec<CaptureOperand>, LoweringError> {
        captures
            .iter()
            .map(|capture| {
                if capture.self_reference {
                    Ok(CaptureOperand::SelfReference)
                } else {
                    environment
                        .get(&capture.key)
                        .copied()
                        .map(Self::storage_register)
                        .map(CaptureOperand::Register)
                        .ok_or_else(|| {
                            invalid_module(
                                self.module,
                                span,
                                "captured binding has no lexical register",
                            )
                        })
                }
            })
            .collect()
    }

    const fn storage_register(storage: BindingStorage) -> RegisterIndex {
        match storage {
            BindingStorage::Direct(register) => register,
            BindingStorage::Cell { handle, .. } => handle,
        }
    }

    fn bind_storage(
        &mut self,
        key: BindingKey,
        value: RegisterIndex,
        span: Span,
    ) -> Result<BindingStorage, LoweringError> {
        if !self.owner.cell_bindings.contains(&key) {
            return Ok(BindingStorage::Direct(value));
        }
        let value_key = self
            .owner
            .cell_binding_types
            .get(&key)
            .cloned()
            .ok_or_else(|| {
                invalid_module(self.module, span, "Cell-selected binding type is absent")
            })?;
        let value_type = self.type_index(&value_key)?;
        self.type_index(&TypeKey::Cell(Box::new(value_key)))?;
        let handle = self.new_register(span)?;
        self.push_instruction(
            Instruction::CellNew {
                destination: handle,
                initial: value,
            },
            span,
        )?;
        Ok(BindingStorage::Cell { handle, value_type })
    }

    fn ensure_cell_value_type(
        &self,
        value_type: TypeIndex,
        span: Span,
    ) -> Result<(), LoweringError> {
        self.owner
            .types
            .get(usize::try_from(value_type.get()).unwrap_or(usize::MAX))
            .ok_or_else(|| invalid_module(self.module, span, "Cell value type is absent"))
            .map(|_| ())
    }

    fn read_binding(
        &mut self,
        key: BindingKey,
        environment: &BindingEnvironment,
        span: Span,
    ) -> Result<RegisterIndex, LoweringError> {
        match environment.get(&key).copied() {
            Some(BindingStorage::Direct(register)) => Ok(register),
            Some(BindingStorage::Cell { handle, value_type }) => {
                self.ensure_cell_value_type(value_type, span)?;
                let destination = self.new_register(span)?;
                self.push_instruction(
                    Instruction::CellGet {
                        destination,
                        cell: handle,
                    },
                    span,
                )?;
                Ok(destination)
            }
            None => Err(invalid_module(
                self.module,
                span,
                "referenced binding has no storage",
            )),
        }
    }

    fn write_binding(
        &mut self,
        key: BindingKey,
        value: RegisterIndex,
        environment: &mut BindingEnvironment,
        span: Span,
    ) -> Result<Option<RegisterIndex>, LoweringError> {
        match environment.get(&key).copied() {
            Some(BindingStorage::Direct(_)) => {
                environment.insert(key, BindingStorage::Direct(value));
                Ok(None)
            }
            Some(BindingStorage::Cell { handle, value_type }) => {
                self.ensure_cell_value_type(value_type, span)?;
                let destination = self.new_register(span)?;
                self.push_instruction(
                    Instruction::CellSet {
                        destination,
                        cell: handle,
                        value,
                    },
                    span,
                )?;
                Ok(Some(destination))
            }
            None => Err(invalid_module(
                self.module,
                span,
                "assigned binding has no storage",
            )),
        }
    }

    fn mutable_binding_keys(&self, environment: &BindingEnvironment) -> Vec<BindingKey> {
        let resolved = self.owner.snapshot.checked().typed().resolved();
        environment
            .keys()
            .filter(|key| {
                matches!(environment.get(key), Some(BindingStorage::Direct(_)))
                    && resolved
                        .bindings()
                        .get(key)
                        .is_some_and(|binding| binding.mutable)
            })
            .copied()
            .collect()
    }

    fn propagate_mutable_bindings(
        &self,
        destination: &mut BindingEnvironment,
        source: &BindingEnvironment,
    ) {
        for key in self.mutable_binding_keys(destination) {
            if let Some(BindingStorage::Direct(register)) = source.get(&key) {
                destination.insert(key, BindingStorage::Direct(*register));
            }
        }
    }

    fn add_mutable_merge_parameters(
        &mut self,
        merge: BlockIndex,
        environment: &BindingEnvironment,
        span: Span,
    ) -> Result<Vec<(BindingKey, RegisterIndex)>, LoweringError> {
        let keys = self.mutable_binding_keys(environment);
        self.add_mutable_parameters_for_keys(merge, &keys, span)
    }

    fn add_mutable_parameters_for_keys(
        &mut self,
        block: BlockIndex,
        keys: &[BindingKey],
        span: Span,
    ) -> Result<Vec<(BindingKey, RegisterIndex)>, LoweringError> {
        let typed = self.owner.snapshot.checked().typed();
        let block = usize::try_from(block.get())
            .map_err(|_| invalid_without_span("merge block index does not fit host usize"))?;
        let mut parameters = Vec::with_capacity(keys.len());
        for key in keys.iter().copied() {
            let value = typed.binding_type(key).ok_or_else(|| {
                invalid_module(self.module, span, "mutable binding type is absent")
            })?;
            let value_type =
                self.type_index(&checked_type_key(typed, value, self.module, span)?)?;
            let register = self.new_register(span)?;
            self.blocks[block].parameters.push(BlockParameter {
                register,
                value_type,
            });
            parameters.push((key, register));
        }
        Ok(parameters)
    }

    fn mutable_merge_arguments(
        &self,
        parameters: &[(BindingKey, RegisterIndex)],
        environment: &BindingEnvironment,
        span: Span,
    ) -> Result<Vec<RegisterIndex>, LoweringError> {
        parameters
            .iter()
            .map(|(key, _)| match environment.get(key).copied() {
                Some(BindingStorage::Direct(register)) => Ok(register),
                Some(BindingStorage::Cell { .. }) | None => Err(invalid_module(
                    self.module,
                    span,
                    "mutable direct binding has no branch value",
                )),
            })
            .collect()
    }

    fn environment_from_parameters(
        &self,
        base: &BindingEnvironment,
        parameters: &[(BindingKey, RegisterIndex)],
    ) -> BindingEnvironment {
        let mut environment = base.clone();
        for (key, register) in parameters {
            environment.insert(*key, BindingStorage::Direct(*register));
        }
        environment
    }

    fn lower_assignment(
        &mut self,
        expression: &hir::Expression,
        place: &hir::Place,
        value: &hir::Expression,
        environment: &mut BindingEnvironment,
    ) -> Result<RegisterIndex, LoweringError> {
        let target = self
            .owner
            .snapshot
            .checked()
            .typed()
            .resolved()
            .reference(self.module.id, place.root_reference)
            .cloned()
            .ok_or_else(|| invalid_module(self.module, place.span, "assignment root is absent"))?;
        let ReferenceTarget::Binding(binding) = target else {
            return Err(invalid_module(
                self.module,
                place.span,
                "assignment root is not a local binding",
            ));
        };
        let binding_info = self
            .owner
            .snapshot
            .checked()
            .typed()
            .resolved()
            .bindings()
            .get(&binding)
            .ok_or_else(|| {
                invalid_module(self.module, place.span, "assignment binding is absent")
            })?;
        if !binding_info.mutable {
            return Err(invalid_module(
                self.module,
                place.span,
                "checked assignment root is not mutable",
            ));
        }
        let assigned = self.lower_expression(value, environment)?;
        if place.fields.is_empty() {
            return self
                .write_binding(binding, assigned, environment, place.span)?
                .map_or_else(|| self.emit_constant(expression, Constant::Unit), Ok);
        }
        let root = self.read_binding(binding, environment, place.span)?;
        let typed = self.owner.snapshot.checked().typed();
        let root_type = typed
            .place_root_type(ExpressionKey::new(self.module.id, expression.id))
            .ok_or_else(|| {
                invalid_module(self.module, place.span, "assignment root type is absent")
            })?;
        let mut record_registers = vec![root];
        let mut record_types = Vec::with_capacity(place.fields.len());
        let mut current_type = root_type;
        for field in &place.fields {
            let Type::NominalRecord {
                definition,
                arguments,
            } = typed.arena().get(current_type)
            else {
                return Err(invalid_module(
                    self.module,
                    field.span,
                    "assignment path traverses a non-record value",
                ));
            };
            let info = typed.records().get(definition).ok_or_else(|| {
                invalid_module(
                    self.module,
                    field.span,
                    "assignment record metadata is absent",
                )
            })?;
            let (field_index, declaration) = info
                .fields
                .iter()
                .enumerate()
                .find(|(_, candidate)| candidate.name == field.normalized)
                .ok_or_else(|| {
                    invalid_module(self.module, field.span, "assignment field is absent")
                })?;
            record_types.push(current_type);
            let field_type =
                substitute_nominal_type(typed, definition, arguments, declaration.field_type)
                    .ok_or_else(|| {
                        invalid_module(self.module, field.span, "assignment field type is absent")
                    })?;
            let next = self.new_register(field.span)?;
            self.push_instruction(
                Instruction::GetField {
                    destination: next,
                    record: *record_registers.last().ok_or_else(|| {
                        invalid_module(self.module, field.span, "assignment record path is empty")
                    })?,
                    field: u32::try_from(field_index)
                        .map_err(|_| invalid_without_span("assignment field index overflow"))?,
                },
                field.span,
            )?;
            record_registers.push(next);
            current_type = field_type;
        }
        let mut updated = assigned;
        for (index, record_type) in record_types.iter().enumerate().rev() {
            let destination = self.new_register(place.span)?;
            let field_index = typed
                .records()
                .get(match typed.arena().get(*record_type) {
                    Type::NominalRecord { definition, .. } => definition,
                    _ => unreachable!("record type collected above"),
                })
                .and_then(|info| {
                    info.fields
                        .iter()
                        .position(|field| field.name == place.fields[index].normalized)
                })
                .ok_or_else(|| {
                    invalid_module(
                        self.module,
                        place.fields[index].span,
                        "assignment field is absent",
                    )
                })?;
            self.push_instruction(
                Instruction::UpdateRecord {
                    destination,
                    base: record_registers[index],
                    updates: vec![RecordUpdate {
                        field: u32::try_from(field_index)
                            .map_err(|_| invalid_without_span("assignment field index overflow"))?,
                        value: updated,
                    }],
                },
                place.fields[index].span,
            )?;
            updated = destination;
        }
        self.write_binding(binding, updated, environment, place.span)?
            .map_or_else(|| self.emit_constant(expression, Constant::Unit), Ok)
    }

    fn lower_match(
        &mut self,
        expression: &hir::Expression,
        scrutinee: &hir::Expression,
        cases: &[hir::MatchCase],
        environment: &mut BindingEnvironment,
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
        let mut scrutinee_environment = environment.clone();
        let scrutinee_register = self.lower_expression(scrutinee, &mut scrutinee_environment)?;
        self.propagate_mutable_bindings(environment, &scrutinee_environment);
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
        let mutable_parameters =
            self.add_mutable_merge_parameters(merge, environment, expression.span)?;

        let original_environment = environment.clone();
        let mut incoming_environment = original_environment.clone();
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
            let failure_parameters = if let Some(failure) = failure {
                Some(
                    self.add_mutable_parameters_for_keys(
                        failure,
                        &mutable_parameters
                            .iter()
                            .map(|(key, _)| *key)
                            .collect::<Vec<_>>(),
                        case.span,
                    )?,
                )
            } else {
                None
            };
            let pattern_failure = failure.map(|block| PatternFailure {
                block,
                parameters: failure_parameters.as_deref(),
            });
            let mut case_environment = incoming_environment.clone();
            self.emit_pattern(
                scrutinee_register,
                scrutinee_type,
                &case.pattern,
                &mut case_environment,
                pattern_success,
                pattern_failure,
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
                        false_arguments: failure_parameters.as_ref().map_or_else(
                            || Ok(Vec::new()),
                            |parameters| {
                                self.mutable_merge_arguments(
                                    parameters,
                                    &case_environment,
                                    guard.span,
                                )
                            },
                        )?,
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
                    arguments: {
                        let mut arguments = vec![value];
                        arguments.extend(self.mutable_merge_arguments(
                            &mutable_parameters,
                            &case_environment,
                            case.body.span,
                        )?);
                        arguments
                    },
                },
                case.body.span,
            )?;

            if let Some(failure) = failure {
                self.set_current_block(failure)?;
                incoming_environment = self.environment_from_parameters(
                    &incoming_environment,
                    failure_parameters.as_deref().unwrap_or_default(),
                );
            }
        }
        self.set_current_block(merge)?;
        for (key, register) in mutable_parameters {
            environment.insert(key, BindingStorage::Direct(register));
        }
        Ok(result_register)
    }

    fn lower_if(
        &mut self,
        expression: &hir::Expression,
        condition: &hir::Expression,
        then_branch: &hir::Expression,
        else_branch: &hir::Expression,
        environment: &mut BindingEnvironment,
    ) -> Result<RegisterIndex, LoweringError> {
        let mut condition_environment = environment.clone();
        let condition_register = self.lower_expression(condition, &mut condition_environment)?;
        self.propagate_mutable_bindings(environment, &condition_environment);
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
        let mutable_parameters =
            self.add_mutable_merge_parameters(merge, environment, expression.span)?;
        let branch_environment = environment.clone();
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
        let mut then_environment = branch_environment.clone();
        let then_value = self.lower_expression(then_branch, &mut then_environment)?;
        self.set_terminator(
            Terminator::Jump {
                target: merge,
                arguments: {
                    let mut arguments = vec![then_value];
                    arguments.extend(self.mutable_merge_arguments(
                        &mutable_parameters,
                        &then_environment,
                        then_branch.span,
                    )?);
                    arguments
                },
            },
            then_branch.span,
        )?;

        self.set_current_block(else_block)?;
        let mut else_environment = branch_environment;
        let else_value = self.lower_expression(else_branch, &mut else_environment)?;
        self.set_terminator(
            Terminator::Jump {
                target: merge,
                arguments: {
                    let mut arguments = vec![else_value];
                    arguments.extend(self.mutable_merge_arguments(
                        &mutable_parameters,
                        &else_environment,
                        else_branch.span,
                    )?);
                    arguments
                },
            },
            else_branch.span,
        )?;
        self.set_current_block(merge)?;
        for (key, register) in mutable_parameters {
            environment.insert(key, BindingStorage::Direct(register));
        }
        Ok(result_register)
    }

    fn lower_binary(
        &mut self,
        expression: &hir::Expression,
        operator: hir::BinaryOperator,
        left: &hir::Expression,
        right: &hir::Expression,
        environment: &mut BindingEnvironment,
    ) -> Result<RegisterIndex, LoweringError> {
        if matches!(
            operator,
            hir::BinaryOperator::BooleanAnd | hir::BinaryOperator::BooleanOr
        ) {
            let mut left_environment = environment.clone();
            let left_register = self.lower_expression(left, &mut left_environment)?;
            self.propagate_mutable_bindings(environment, &left_environment);
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
            let mutable_parameters =
                self.add_mutable_merge_parameters(merge, environment, expression.span)?;
            let branch_environment = environment.clone();
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
            let mut right_environment = branch_environment.clone();
            let right_register = self.lower_expression(right, &mut right_environment)?;
            self.set_terminator(
                Terminator::Jump {
                    target: merge,
                    arguments: {
                        let mut arguments = vec![right_register];
                        arguments.extend(self.mutable_merge_arguments(
                            &mutable_parameters,
                            &right_environment,
                            right.span,
                        )?);
                        arguments
                    },
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
                    arguments: {
                        let mut arguments = vec![short_value];
                        arguments.extend(self.mutable_merge_arguments(
                            &mutable_parameters,
                            &branch_environment,
                            expression.span,
                        )?);
                        arguments
                    },
                },
                expression.span,
            )?;
            self.set_current_block(merge)?;
            for (key, register) in mutable_parameters {
                environment.insert(key, BindingStorage::Direct(register));
            }
            return Ok(result_register);
        }

        let left_register = self.lower_expression(left, environment)?;
        let right_register = self.lower_expression(right, environment)?;
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
        environment: &mut BindingEnvironment,
    ) -> Result<RegisterIndex, LoweringError> {
        let operand_register = self.lower_expression(operand, environment)?;
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
        environment: &mut BindingEnvironment,
        success: BlockIndex,
        failure: Option<PatternFailure<'_>>,
    ) -> Result<(), LoweringError> {
        let typed = self.owner.snapshot.checked().typed();
        match &pattern.kind {
            hir::PatternKind::Binding { id, .. } => {
                let key = BindingKey::new(self.module.id, *id);
                environment.insert(key, self.bind_storage(key, value, pattern.span)?);
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
                        false_target: failure.block,
                        false_arguments: failure.parameters.map_or_else(
                            || Ok(Vec::new()),
                            |parameters| {
                                self.mutable_merge_arguments(parameters, environment, pattern.span)
                            },
                        )?,
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
                                    false_target: failure.block,
                                    false_arguments: failure.parameters.map_or_else(
                                        || Ok(Vec::new()),
                                        |parameters| {
                                            self.mutable_merge_arguments(
                                                parameters,
                                                environment,
                                                pattern.span,
                                            )
                                        },
                                    )?,
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
                                    false_target: failure.block,
                                    false_arguments: failure.parameters.map_or_else(
                                        || Ok(Vec::new()),
                                        |parameters| {
                                            self.mutable_merge_arguments(
                                                parameters,
                                                environment,
                                                pattern.span,
                                            )
                                        },
                                    )?,
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
        environment: &mut BindingEnvironment,
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
        environment: &mut BindingEnvironment,
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
        environment: &mut BindingEnvironment,
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
        environment: &mut BindingEnvironment,
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
        environment: &mut BindingEnvironment,
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
        if let Some(call) = self
            .owner
            .snapshot
            .checked()
            .typed()
            .trait_member_call(ExpressionKey::new(self.module.id, expression.id))
            .cloned()
        {
            let values = self.lower_arguments(arguments, environment)?;
            let signature = self
                .owner
                .named_signatures
                .get(call.implementation())
                .ok_or_else(|| {
                    invalid_module(
                        self.module,
                        expression.span,
                        "Trait implementation member signature is absent",
                    )
                })?;
            if arguments.len() == signature.parameters.len() {
                let destination = self.new_register(expression.span)?;
                self.push_instruction(
                    Instruction::Call {
                        destination,
                        function: self.owner.function_indices[call.implementation()],
                        arguments: values,
                    },
                    expression.span,
                )?;
                return Ok(destination);
            }
            if arguments.len() > signature.parameters.len() {
                return Err(invalid_module(
                    self.module,
                    expression.span,
                    "Trait member call arity exceeds implementation signature",
                ));
            }
            let callee = self.new_register(function.span)?;
            self.push_instruction(
                Instruction::MakeClosure {
                    destination: callee,
                    function: self.owner.function_indices[call.implementation()],
                    captures: Vec::new(),
                },
                function.span,
            )?;
            let destination = self.new_register(expression.span)?;
            self.push_instruction(
                Instruction::CallClosure {
                    destination,
                    callee,
                    arguments: values,
                },
                expression.span,
            )?;
            return Ok(destination);
        }
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
            if self.owner.handler_mode {
                function.span
            } else {
                expression.span
            },
        )?;
        Ok(destination)
    }

    fn lower_arguments(
        &mut self,
        arguments: &[hir::Expression],
        environment: &mut BindingEnvironment,
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
        environment: &mut BindingEnvironment,
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
        environment: &BindingEnvironment,
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
                self.read_binding(binding, environment, expression.span)
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

    fn effects(&self, effects: &[EffectKey]) -> Result<Vec<Effect>, LoweringError> {
        wire_effects(effects, &self.owner.type_indices)
            .ok_or_else(|| invalid_without_span("canonical type table omitted an Effect type"))
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

fn is_trait_member_reference(
    snapshot: &ProgramSnapshot,
    module: ModuleId,
    expression: &hir::Expression,
) -> Result<bool, LoweringError> {
    Ok(matches!(
        simple_reference_target(snapshot, module, expression)?,
        Some(ReferenceTarget::Definition(ref definition))
            if snapshot.checked().typed().resolved().trait_member(definition).is_some()
    ))
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
    if is_trait_member_reference(snapshot, module.id, expression)? {
        // The checked call mapping lowers the member directly; the
        // projection is not an independent runtime value.
        return Ok(());
    }
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
                    hir::SequenceElement::LetAwait(binding) => {
                        return Err(unsupported_module(
                            module,
                            binding.span,
                            "structured task execution (L-TASK-0004)",
                        ));
                    }
                    hir::SequenceElement::Expression(value) => {
                        collect_type_shapes(snapshot, module, value, builder)?;
                    }
                }
            }
        }
        hir::ExpressionKind::TaskScope { .. }
        | hir::ExpressionKind::TaskSpawn { .. }
        | hir::ExpressionKind::TaskAwait { .. }
        | hir::ExpressionKind::TaskReturn { .. } => {
            return Err(unsupported_module(
                module,
                expression.span,
                "structured task execution (L-TASK-0004)",
            ));
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
        hir::ExpressionKind::Handle { body, clauses } => {
            collect_type_shapes(snapshot, module, body, builder)?;
            for clause in clauses {
                collect_type_shapes(snapshot, module, &clause.body, builder)?;
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
    if is_trait_member_reference(snapshot, module.id, expression)? {
        return Ok(());
    }
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
                        if binding.mutable && !aggregate_mode {
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
                    hir::SequenceElement::LetAwait(binding) => {
                        return Err(unsupported_module(
                            module,
                            binding.span,
                            "structured task execution (L-TASK-0004)",
                        ));
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
        hir::ExpressionKind::TaskScope { .. }
        | hir::ExpressionKind::TaskSpawn { .. }
        | hir::ExpressionKind::TaskAwait { .. }
        | hir::ExpressionKind::TaskReturn { .. } => {
            return Err(unsupported_module(
                module,
                expression.span,
                "structured task execution (L-TASK-0004)",
            ));
        }
        hir::ExpressionKind::Handle { body, clauses } => {
            collect_constants_v1_1(snapshot, module, body, strings, constants, aggregate_mode)?;
            for clause in clauses {
                collect_constants_v1_1(
                    snapshot,
                    module,
                    &clause.body,
                    strings,
                    constants,
                    aggregate_mode,
                )?;
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
        hir::ExpressionKind::Assignment { value, .. } if aggregate_mode => {
            collect_constants_v1_1(snapshot, module, value, strings, constants, aggregate_mode)?;
            insert_constant(constants, Constant::Unit)?;
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
        Type::Task { .. } | Type::TaskHandle { .. } => Err(unsupported_module(
            module,
            span,
            "structured task execution (L-TASK-0004)",
        )),
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
