//! Seed effect inference, capability checking, and checked-program sealing.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_hir as hir;
use ling_resolve::{
    BindingKey, Builtin, DefinitionId, DefinitionOrigin, ExpressionKey, HandlerResumeMode,
    HandlerValueType, ModuleId, ReferenceTarget, ResolvedHandlerOperation,
    resolve_handler_operation,
};
use ling_source::Span;
use ling_types::{DictionaryTable, TraitMemberCall, Type, TypeId, TypedProgram};

mod handler_core;
mod solver;
mod v2;

pub use handler_core::{
    HandlerCore, HandlerCoreClause, HandlerCoreError, HandlerCoreNodeId, ResumeUse,
};
pub use solver::{
    EffectConflictKind, EffectConstraint, EffectConstraintConflict, EffectConstraintError,
    EffectConstraintOrigin, EffectConstraintSolver, EffectInference, EffectInstantiationError,
    EffectRowScheme, EffectSourceSpan, EffectSubstitution, GeneralizationBoundary,
    subtract_handler,
};
pub use v2::{
    EFFECT_GRAPH_EXTENSION_VERSION, EffectGraphProjection, EffectId, EffectIdError, EffectLabel,
    EffectOperation, EffectOperationError, EffectRowModel, EffectRowTail, EffectRowUnionError,
    EffectTypeRef, EffectTypeRefError, HandlerClause, HandlerClauseError, HandlerContract,
    HandlerContractError, ResumeMode, RowVariableId,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Effect {
    ConsoleWrite,
    State { display: String, identity: String },
}

impl Effect {
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Self::ConsoleWrite => "Console.Write".to_owned(),
            Self::State { display, .. } => format!("State<{display}>"),
        }
    }

    #[must_use]
    pub fn canonical_name(&self) -> String {
        match self {
            Self::ConsoleWrite => "Console.Write".to_owned(),
            Self::State { identity, .. } => format!("State<{identity}>"),
        }
    }
}

fn handler_effect(operation: &str) -> Option<Effect> {
    match resolve_handler_operation(operation)?.label() {
        "Console.Write" => Some(Effect::ConsoleWrite),
        "Clock" | "Random" => None,
        _ => None,
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EffectRow(BTreeSet<Effect>);

impl EffectRow {
    #[must_use]
    pub fn is_pure(&self) -> bool {
        self.0.is_empty()
    }

    pub fn effects(&self) -> impl Iterator<Item = &Effect> {
        self.0.iter()
    }

    #[must_use]
    pub fn names(&self) -> Vec<String> {
        self.effects().map(Effect::name).collect()
    }

    #[must_use]
    pub fn canonical_names(&self) -> Vec<String> {
        let mut names = self
            .effects()
            .map(Effect::canonical_name)
            .collect::<Vec<_>>();
        names.sort();
        names
    }

    fn insert(&mut self, effect: Effect) {
        self.0.insert(effect);
    }

    fn extend(&mut self, other: &Self) {
        self.0.extend(other.0.iter().cloned());
    }

    fn without(&self, handled: &BTreeSet<Effect>) -> Self {
        Self(self.0.difference(handled).cloned().collect())
    }

    /// Projects the current Seed closed row into canonical, path-free names.
    ///
    /// This is an in-process observation of the existing row only. It does not
    /// introduce open rows, row variables, handlers, or Effect selection.
    #[must_use]
    pub fn seed_snapshot(&self) -> SeedEffectRowSnapshot {
        SeedEffectRowSnapshot {
            canonical_names: self.canonical_names().into_boxed_slice(),
        }
    }
}

/// Deterministic in-process projection of a Seed closed Effect row.
///
/// The snapshot carries only canonical effect identities. It intentionally
/// omits display spelling, source paths, host state, and future v0.2 row
/// variables or handler metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeedEffectRowSnapshot {
    canonical_names: Box<[String]>,
}

impl SeedEffectRowSnapshot {
    #[must_use]
    pub fn canonical_names(&self) -> &[String] {
        &self.canonical_names
    }

    #[must_use]
    pub fn is_pure(&self) -> bool {
        self.canonical_names.is_empty()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Capability {
    ConsoleWrite,
}

impl Capability {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::ConsoleWrite => "Console.Write",
        }
    }
}

#[derive(Clone, Debug)]
pub struct CheckedProgram {
    typed: TypedProgram,
    definition_effects: BTreeMap<DefinitionId, EffectRow>,
    binding_effects: BTreeMap<BindingKey, EffectRow>,
    expression_effects: BTreeMap<ExpressionKey, EffectRow>,
    handler_cores: BTreeMap<ExpressionKey, HandlerCore>,
    module_capabilities: BTreeMap<ModuleId, BTreeSet<Capability>>,
    warnings: Vec<Diagnostic>,
}

/// A function type after call-graph Effect analysis has completed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedFunctionType {
    type_id: TypeId,
    parameters: Vec<TypeId>,
    result: TypeId,
    effects: EffectRow,
    display: String,
}

impl CheckedFunctionType {
    #[must_use]
    pub const fn type_id(&self) -> TypeId {
        self.type_id
    }

    #[must_use]
    pub fn parameters(&self) -> &[TypeId] {
        &self.parameters
    }

    #[must_use]
    pub const fn result(&self) -> TypeId {
        self.result
    }

    #[must_use]
    pub const fn effects(&self) -> &EffectRow {
        &self.effects
    }

    #[must_use]
    pub fn display(&self) -> &str {
        &self.display
    }
}

impl CheckedProgram {
    #[must_use]
    pub const fn typed(&self) -> &TypedProgram {
        &self.typed
    }

    #[must_use]
    pub const fn dictionary(&self) -> &DictionaryTable {
        self.typed.dictionary()
    }

    #[must_use]
    pub fn trait_member_call(&self, key: ExpressionKey) -> Option<&TraitMemberCall> {
        self.typed.trait_member_call(key)
    }

    #[must_use]
    pub fn definition_effect(&self, definition: &DefinitionId) -> Option<&EffectRow> {
        self.definition_effects.get(definition)
    }

    #[must_use]
    pub fn definition_effects(&self) -> &BTreeMap<DefinitionId, EffectRow> {
        &self.definition_effects
    }

    #[must_use]
    pub fn binding_effect(&self, binding: BindingKey) -> Option<&EffectRow> {
        self.binding_effects.get(&binding)
    }

    /// Returns the checked residual Effect row of one exact HIR expression.
    #[must_use]
    pub fn expression_effect(&self, expression: ExpressionKey) -> Option<&EffectRow> {
        self.expression_effects.get(&expression)
    }

    #[must_use]
    pub fn handler_core(&self, expression: ExpressionKey) -> Option<&HandlerCore> {
        self.handler_cores.get(&expression)
    }

    #[must_use]
    pub fn handler_cores(&self) -> &BTreeMap<ExpressionKey, HandlerCore> {
        &self.handler_cores
    }

    #[must_use]
    pub fn definition_function_type(
        &self,
        definition: &DefinitionId,
    ) -> Option<CheckedFunctionType> {
        let type_id = self.typed.definition_type(definition)?;
        self.function_type(type_id, self.definition_effect(definition)?)
    }

    #[must_use]
    pub fn binding_function_type(&self, binding: BindingKey) -> Option<CheckedFunctionType> {
        let type_id = self.typed.binding_type(binding)?;
        self.function_type(type_id, self.binding_effect(binding)?)
    }

    #[must_use]
    pub fn module_capabilities(&self, module: ModuleId) -> Option<&BTreeSet<Capability>> {
        self.module_capabilities.get(&module)
    }

    #[must_use]
    pub fn warnings(&self) -> &[Diagnostic] {
        &self.warnings
    }

    fn function_type(&self, type_id: TypeId, effects: &EffectRow) -> Option<CheckedFunctionType> {
        let Type::Function { parameters, result } = self.typed.arena().get(type_id) else {
            return None;
        };
        Some(CheckedFunctionType {
            type_id,
            parameters: parameters.clone(),
            result: *result,
            effects: effects.clone(),
            display: format!(
                "{} ! {{{}}}",
                self.typed.display_type(type_id),
                effects.names().join(", ")
            ),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectError {
    pub kind: EffectErrorKind,
    pub source_name: String,
    pub span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EffectErrorKind {
    MissingCapability { capability: &'static str },
    UnknownCapability { capability: String },
    InvalidHandlerContract { operation: String, reason: String },
}

impl EffectError {
    #[must_use]
    pub fn to_diagnostic(&self) -> Diagnostic {
        let (code, zh, en) = match &self.kind {
            EffectErrorKind::MissingCapability { capability } => (
                codes::MISSING_CAPABILITY,
                format!("模块缺少 Capability 声明“{capability}”"),
                format!("module is missing required capability `{capability}`"),
            ),
            EffectErrorKind::UnknownCapability { capability } => (
                codes::UNKNOWN_CAPABILITY,
                format!("Seed 不支持 Capability“{capability}”"),
                format!("Ling Seed does not support capability `{capability}`"),
            ),
            EffectErrorKind::InvalidHandlerContract { reason, .. } => (
                codes::INVALID_HANDLER_CONTRACT,
                format!("Handler clause contract 无效：{reason}"),
                format!("handler clause contract is invalid: {reason}"),
            ),
        };
        let diagnostic = Diagnostic::new(code, Severity::Error, zh, en)
            .with_primary_span(DiagnosticSpan::new(&self.source_name, self.span));
        match &self.kind {
            EffectErrorKind::InvalidHandlerContract { operation, reason } => diagnostic
                .with_fact("operation", operation.clone())
                .with_fact("reason", reason.clone()),
            EffectErrorKind::MissingCapability { .. }
            | EffectErrorKind::UnknownCapability { .. } => diagnostic,
        }
    }
}

impl fmt::Display for EffectError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.kind)
    }
}

impl Error for EffectError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryError {
    pub kind: EntryErrorKind,
    pub source_name: String,
    pub span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntryErrorKind {
    EntryModuleMustBeMain { actual: String },
    MissingMain,
    InvalidMainSignature { actual: String },
    MainMustHaveUnitPattern,
}

impl EntryError {
    #[must_use]
    pub fn to_diagnostic(&self) -> Diagnostic {
        let (code, zh, en) = match &self.kind {
            EntryErrorKind::EntryModuleMustBeMain { actual } => (
                codes::INVALID_ENTRY_MODULE,
                format!("run 入口模块必须是 Main，实际为“{actual}”"),
                format!("the run entry module must be `Main`, found `{actual}`"),
            ),
            EntryErrorKind::MissingMain => (
                codes::MISSING_MAIN,
                "Main 模块缺少 main 定义".to_owned(),
                "module `Main` does not define `main`".to_owned(),
            ),
            EntryErrorKind::InvalidMainSignature { actual } => (
                codes::INVALID_MAIN_SIGNATURE,
                format!("main 必须具有类型 Unit -> Unit，实际为 {actual}"),
                format!("`main` must have type Unit -> Unit, found {actual}"),
            ),
            EntryErrorKind::MainMustHaveUnitPattern => (
                codes::INVALID_MAIN_SIGNATURE,
                "main 必须显式声明一个 Unit pattern 参数".to_owned(),
                "`main` must explicitly declare one Unit pattern parameter".to_owned(),
            ),
        };
        Diagnostic::new(code, Severity::Error, zh, en)
            .with_primary_span(DiagnosticSpan::new(&self.source_name, self.span))
    }
}

impl fmt::Display for EntryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.kind)
    }
}

impl Error for EntryError {}

/// Infers effects and verifies every module's declared capability closure.
pub fn check(typed: TypedProgram) -> Result<CheckedProgram, Vec<EffectError>> {
    Checker::new(typed).run()
}

/// Locates and validates the executable Seed entry point.
pub fn locate_main(checked: &CheckedProgram) -> Result<DefinitionId, EntryError> {
    let resolved = checked.typed.resolved();
    let module = resolved.entry_module();
    let module_name = module.hir.module.name.normalized();
    if module_name != "Main" {
        return Err(EntryError {
            kind: EntryErrorKind::EntryModuleMustBeMain {
                actual: module_name,
            },
            source_name: module.hir.source_name.clone(),
            span: module.hir.module.span,
        });
    }
    let Some(main_id) = resolved.definition_id(module.id, "main").cloned() else {
        return Err(EntryError {
            kind: EntryErrorKind::MissingMain,
            source_name: module.hir.source_name.clone(),
            span: module.hir.module.span,
        });
    };
    let Some(main) = module
        .hir
        .definitions
        .iter()
        .find(|definition| definition.name.normalized == "main")
    else {
        return Err(EntryError {
            kind: EntryErrorKind::MissingMain,
            source_name: module.hir.source_name.clone(),
            span: module.hir.module.span,
        });
    };
    if !matches!(
        main.parameters.as_slice(),
        [hir::Pattern {
            kind: hir::PatternKind::Unit,
            ..
        }]
    ) {
        return Err(EntryError {
            kind: EntryErrorKind::MainMustHaveUnitPattern,
            source_name: module.hir.source_name.clone(),
            span: main.span,
        });
    }
    let main_type = checked
        .definition_function_type(&main_id)
        .expect("checked function definitions have checked function types");
    let valid = main_type.parameters().len() == 1
        && matches!(
            checked.typed.arena().get(main_type.parameters()[0]),
            Type::Unit
        )
        && matches!(checked.typed.arena().get(main_type.result()), Type::Unit);
    if !valid {
        return Err(EntryError {
            kind: EntryErrorKind::InvalidMainSignature {
                actual: main_type.display().to_owned(),
            },
            source_name: module.hir.source_name.clone(),
            span: main.span,
        });
    }
    Ok(main_id)
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Callable {
    Definition(DefinitionId),
    Binding(BindingKey),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CallEdge {
    callee: Callable,
    handled: BTreeSet<Effect>,
}

struct Checker {
    typed: TypedProgram,
    direct_effects: BTreeMap<Callable, EffectRow>,
    calls: BTreeMap<Callable, BTreeSet<CallEdge>>,
    returns: BTreeMap<Callable, BTreeSet<Callable>>,
    callable_parameters: BTreeMap<Callable, Vec<Option<BindingKey>>>,
    errors: Vec<EffectError>,
    warnings: Vec<Diagnostic>,
}

impl Checker {
    fn new(typed: TypedProgram) -> Self {
        let warnings = typed.warnings().to_vec();
        Self {
            typed,
            direct_effects: BTreeMap::new(),
            calls: BTreeMap::new(),
            returns: BTreeMap::new(),
            callable_parameters: BTreeMap::new(),
            errors: Vec::new(),
            warnings,
        }
    }

    fn run(mut self) -> Result<CheckedProgram, Vec<EffectError>> {
        self.collect_direct_effects();
        let callable_effects = self.propagate_effects(true);
        let capability_effects = self.propagate_effects(false);
        let definition_effects = callable_effects
            .iter()
            .filter_map(|(callable, effects)| match callable {
                Callable::Definition(definition) => Some((definition.clone(), effects.clone())),
                Callable::Binding(_) => None,
            })
            .collect();
        let binding_effects = callable_effects
            .iter()
            .filter_map(|(callable, effects)| match callable {
                Callable::Binding(binding) => Some((*binding, effects.clone())),
                Callable::Definition(_) => None,
            })
            .collect();
        let expression_effects = self.build_expression_effect_rows(&callable_effects);
        let handler_cores = self.build_handler_cores(&callable_effects);
        let capability_definition_effects = capability_effects
            .iter()
            .filter_map(|(callable, effects)| match callable {
                Callable::Definition(definition) => Some((definition.clone(), effects.clone())),
                Callable::Binding(_) => None,
            })
            .collect();
        let module_capabilities = self.check_capabilities(&capability_definition_effects);
        if self.errors.is_empty() {
            Ok(CheckedProgram {
                typed: self.typed,
                definition_effects,
                binding_effects,
                expression_effects,
                handler_cores,
                module_capabilities,
                warnings: self.warnings,
            })
        } else {
            self.errors.sort_by(|left, right| {
                (
                    &left.source_name,
                    left.span.start(),
                    format!("{:?}", left.kind),
                )
                    .cmp(&(
                        &right.source_name,
                        right.span.start(),
                        format!("{:?}", right.kind),
                    ))
            });
            Err(self.errors)
        }
    }

    fn collect_direct_effects(&mut self) {
        let modules = self.typed.resolved().modules().to_vec();
        for module in &modules {
            for definition in &module.hir.definitions {
                let Some(id) = self
                    .typed
                    .resolved()
                    .definition_id(module.id, &definition.name.normalized)
                    .cloned()
                else {
                    continue;
                };
                self.register_parameters(
                    Callable::Definition(id),
                    module.id,
                    &definition.parameters,
                );
            }
        }
        for module in &modules {
            for definition in &module.hir.definitions {
                let Some(id) = self
                    .typed
                    .resolved()
                    .definition_id(module.id, &definition.name.normalized)
                    .cloned()
                else {
                    continue;
                };
                let callable = Callable::Definition(id);
                self.collect_callable_body(callable.clone(), module.id, &definition.value);
                if definition.parameters.is_empty() {
                    self.connect_alias(callable, module.id, &definition.value);
                }
            }
            for (impl_ordinal, implementation) in module.hir.impls.iter().enumerate() {
                for (member_ordinal, definition) in implementation.members.iter().enumerate() {
                    let Some(id) = self
                        .typed
                        .resolved()
                        .impl_members()
                        .values()
                        .find(|member| {
                            member.module == module.id
                                && member.impl_ordinal == impl_ordinal
                                && member.member_ordinal == member_ordinal
                        })
                        .map(|member| member.definition.clone())
                    else {
                        continue;
                    };
                    let callable = Callable::Definition(id);
                    self.register_parameters(callable.clone(), module.id, &definition.parameters);
                    self.collect_callable_body(callable.clone(), module.id, &definition.value);
                    if definition.parameters.is_empty() {
                        self.connect_alias(callable, module.id, &definition.value);
                    }
                }
            }
        }
        for definition in self.typed.resolved().definitions().values() {
            let mut effects = EffectRow::default();
            match definition.origin {
                DefinitionOrigin::Builtin(Builtin::ConsoleWrite) => {
                    effects.insert(Effect::ConsoleWrite);
                }
                DefinitionOrigin::Builtin(_) | DefinitionOrigin::Prelude(_) => {}
                DefinitionOrigin::User { .. } => continue,
            }
            let callable = Callable::Definition(definition.id.clone());
            self.direct_effects.insert(callable.clone(), effects);
            self.calls.entry(callable).or_default();
        }
    }

    fn collect_callable_body(
        &mut self,
        callable: Callable,
        module: ModuleId,
        expression: &hir::Expression,
    ) {
        let mut effects = EffectRow::default();
        let mut calls = BTreeSet::new();
        self.visit_expression(
            module,
            expression,
            &BTreeSet::new(),
            &mut effects,
            &mut calls,
        );
        self.direct_effects.insert(callable.clone(), effects);
        self.calls
            .entry(callable.clone())
            .or_default()
            .extend(calls);
        if let Some(target) = self.value_callable(module, expression) {
            self.returns.entry(callable).or_default().insert(target);
        }
    }

    fn register_parameters(
        &mut self,
        callable: Callable,
        module: ModuleId,
        parameters: &[hir::Pattern],
    ) {
        let parameters = parameters
            .iter()
            .map(|pattern| match pattern.kind {
                hir::PatternKind::Binding { id, .. } => Some(BindingKey::new(module, id)),
                _ => None,
            })
            .collect();
        self.callable_parameters.insert(callable, parameters);
    }

    fn connect_alias(&mut self, alias: Callable, module: ModuleId, expression: &hir::Expression) {
        if let Some(target) = self.value_callable(module, expression) {
            self.calls.entry(alias).or_default().insert(CallEdge {
                callee: target,
                handled: BTreeSet::new(),
            });
        }
    }

    fn visit_expression(
        &mut self,
        module: ModuleId,
        expression: &hir::Expression,
        handled: &BTreeSet<Effect>,
        effects: &mut EffectRow,
        calls: &mut BTreeSet<CallEdge>,
    ) {
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            let binding_callable =
                                Callable::Binding(BindingKey::new(module, binding.id));
                            if binding.parameters.is_empty() {
                                self.visit_expression(
                                    module,
                                    &binding.value,
                                    handled,
                                    effects,
                                    calls,
                                );
                                self.connect_alias(binding_callable, module, &binding.value);
                            } else {
                                self.register_parameters(
                                    binding_callable.clone(),
                                    module,
                                    &binding.parameters,
                                );
                                self.collect_callable_body(
                                    binding_callable,
                                    module,
                                    &binding.value,
                                );
                            }
                        }
                        hir::SequenceElement::Expression(expression) => {
                            self.visit_expression(module, expression, handled, effects, calls);
                        }
                    }
                }
            }
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expression(module, condition, handled, effects, calls);
                self.visit_expression(module, then_branch, handled, effects, calls);
                self.visit_expression(module, else_branch, handled, effects, calls);
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                self.visit_expression(module, scrutinee, handled, effects, calls);
                for case in cases {
                    if let Some(guard) = &case.guard {
                        self.visit_expression(module, guard, handled, effects, calls);
                    }
                    self.visit_expression(module, &case.body, handled, effects, calls);
                }
            }
            hir::ExpressionKind::Assignment { value, .. } => {
                let key = ExpressionKey::new(module, expression.id);
                let state_type = self
                    .typed
                    .place_root_type(key)
                    .map(|type_id| Effect::State {
                        display: self.typed.display_type(type_id),
                        identity: self.typed.arena().display(type_id),
                    })
                    .expect("checked assignment has a root place type");
                if !handled.contains(&state_type) {
                    effects.insert(state_type);
                }
                self.visit_expression(module, value, handled, effects, calls);
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                self.visit_expression(module, function, handled, effects, calls);
                if let Some(call) = self
                    .typed
                    .trait_member_call(ExpressionKey::new(module, expression.id))
                {
                    calls.insert(CallEdge {
                        callee: Callable::Definition(call.implementation().clone()),
                        handled: handled.clone(),
                    });
                }
                if let Some((callee, applied)) = self.expression_callable_state(module, function) {
                    let target = if applied > 0 {
                        self.value_callable(module, function)
                            .unwrap_or_else(|| callee.clone())
                    } else {
                        callee.clone()
                    };
                    let target_applied = if target == callee { applied } else { 0 };
                    let complete = self.callable_arity(&target).is_none_or(|arity| {
                        target_applied.saturating_add(arguments.len()) >= arity
                    });
                    if complete {
                        let is_map = matches!(
                            &target,
                            Callable::Definition(definition)
                                if definition == self.typed.resolved().builtin_id(Builtin::Map)
                        );
                        calls.insert(CallEdge {
                            callee: target.clone(),
                            handled: handled.clone(),
                        });
                        self.connect_arguments(&target, module, arguments);
                        if is_map {
                            if let Some(callback) = arguments
                                .first()
                                .and_then(|argument| self.expression_callable(module, argument))
                            {
                                calls.insert(CallEdge {
                                    callee: callback,
                                    handled: handled.clone(),
                                });
                            }
                        }
                    }
                }
                for argument in arguments {
                    self.visit_expression(module, argument, handled, effects, calls);
                }
            }
            hir::ExpressionKind::Projection { target, .. } => {
                if self.expression_callable(module, expression).is_none() {
                    self.visit_expression(module, target, handled, effects, calls);
                }
            }
            hir::ExpressionKind::Binary { left, right, .. } => {
                self.visit_expression(module, left, handled, effects, calls);
                self.visit_expression(module, right, handled, effects, calls);
            }
            hir::ExpressionKind::Unary { operand, .. } => {
                self.visit_expression(module, operand, handled, effects, calls);
            }
            hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
                for element in elements {
                    self.visit_expression(module, element, handled, effects, calls);
                }
            }
            hir::ExpressionKind::Record(fields) => {
                for field in fields {
                    self.visit_expression(module, &field.value, handled, effects, calls);
                }
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                self.visit_expression(module, base, handled, effects, calls);
                for field in fields {
                    self.visit_expression(module, &field.value, handled, effects, calls);
                }
            }
            hir::ExpressionKind::Handle { body, clauses } => {
                let mut inner = handled.clone();
                for clause in clauses {
                    if let Some(effect) = handler_effect(&clause.operation.normalized()) {
                        inner.insert(effect);
                    }
                }
                self.visit_expression(module, body, &inner, effects, calls);
                for clause in clauses {
                    self.visit_expression(module, &clause.body, handled, effects, calls);
                }
            }
            hir::ExpressionKind::Name { .. }
            | hir::ExpressionKind::Literal(_)
            | hir::ExpressionKind::Unit => {}
        }
    }

    fn build_expression_effect_rows(
        &self,
        callable_effects: &BTreeMap<Callable, EffectRow>,
    ) -> BTreeMap<ExpressionKey, EffectRow> {
        let mut output = BTreeMap::new();
        for module in self.typed.resolved().modules() {
            for definition in &module.hir.definitions {
                self.collect_expression_effect_rows(
                    module.id,
                    &definition.value,
                    callable_effects,
                    &mut output,
                );
            }
            for implementation in &module.hir.impls {
                for definition in &implementation.members {
                    self.collect_expression_effect_rows(
                        module.id,
                        &definition.value,
                        callable_effects,
                        &mut output,
                    );
                }
            }
        }
        output
    }

    fn collect_expression_effect_rows(
        &self,
        module: ModuleId,
        expression: &hir::Expression,
        callable_effects: &BTreeMap<Callable, EffectRow>,
        output: &mut BTreeMap<ExpressionKey, EffectRow>,
    ) {
        output.insert(
            ExpressionKey::new(module, expression.id),
            self.expression_effects(module, expression, callable_effects),
        );
        let mut visit = |value: &hir::Expression| {
            self.collect_expression_effect_rows(module, value, callable_effects, output);
        };
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                for element in elements {
                    visit(match element {
                        hir::SequenceElement::Let(binding) => &binding.value,
                        hir::SequenceElement::Expression(value) => value,
                    });
                }
            }
            hir::ExpressionKind::Handle { body, clauses } => {
                visit(body);
                for clause in clauses {
                    visit(&clause.body);
                }
            }
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                visit(condition);
                visit(then_branch);
                visit(else_branch);
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                visit(scrutinee);
                for case in cases {
                    if let Some(guard) = &case.guard {
                        visit(guard);
                    }
                    visit(&case.body);
                }
            }
            hir::ExpressionKind::Assignment { value, .. }
            | hir::ExpressionKind::Unary { operand: value, .. }
            | hir::ExpressionKind::Projection { target: value, .. } => visit(value),
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                visit(function);
                for argument in arguments {
                    visit(argument);
                }
            }
            hir::ExpressionKind::Binary { left, right, .. } => {
                visit(left);
                visit(right);
            }
            hir::ExpressionKind::Tuple(values) | hir::ExpressionKind::List(values) => {
                for value in values {
                    visit(value);
                }
            }
            hir::ExpressionKind::Record(fields) => {
                for field in fields {
                    visit(&field.value);
                }
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                visit(base);
                for field in fields {
                    visit(&field.value);
                }
            }
            hir::ExpressionKind::Name { .. }
            | hir::ExpressionKind::Literal(_)
            | hir::ExpressionKind::Unit => {}
        }
    }

    fn expression_effects(
        &self,
        module: ModuleId,
        expression: &hir::Expression,
        callable_effects: &BTreeMap<Callable, EffectRow>,
    ) -> EffectRow {
        let mut output = EffectRow::default();
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) if binding.parameters.is_empty() => {
                            output.extend(&self.expression_effects(
                                module,
                                &binding.value,
                                callable_effects,
                            ));
                        }
                        hir::SequenceElement::Expression(value) => {
                            output.extend(&self.expression_effects(module, value, callable_effects))
                        }
                        hir::SequenceElement::Let(_) => {}
                    }
                }
            }
            hir::ExpressionKind::Handle { body, clauses } => {
                let input = self.expression_effects(module, body, callable_effects);
                let handled = clauses
                    .iter()
                    .filter_map(|clause| handler_effect(&clause.operation.normalized()))
                    .collect();
                output.extend(&input.without(&handled));
                for clause in clauses {
                    output.extend(&self.expression_effects(module, &clause.body, callable_effects));
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
                    output.extend(&self.expression_effects(module, value, callable_effects));
                }
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                output.extend(&self.expression_effects(module, scrutinee, callable_effects));
                for case in cases {
                    if let Some(guard) = &case.guard {
                        output.extend(&self.expression_effects(module, guard, callable_effects));
                    }
                    output.extend(&self.expression_effects(module, &case.body, callable_effects));
                }
            }
            hir::ExpressionKind::Assignment { value, .. } => {
                let key = ExpressionKey::new(module, expression.id);
                if let Some(type_id) = self.typed.place_root_type(key) {
                    output.insert(Effect::State {
                        display: self.typed.display_type(type_id),
                        identity: self.typed.arena().display(type_id),
                    });
                }
                output.extend(&self.expression_effects(module, value, callable_effects));
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                output.extend(&self.expression_effects(module, function, callable_effects));
                for argument in arguments {
                    output.extend(&self.expression_effects(module, argument, callable_effects));
                }
                if let Some(call) = self
                    .typed
                    .trait_member_call(ExpressionKey::new(module, expression.id))
                    .and_then(|call| {
                        callable_effects.get(&Callable::Definition(call.implementation().clone()))
                    })
                {
                    output.extend(call);
                }
                if let Some((callee, applied)) = self.expression_callable_state(module, function) {
                    let target = if applied > 0 {
                        self.value_callable(module, function)
                            .unwrap_or_else(|| callee.clone())
                    } else {
                        callee.clone()
                    };
                    let target_applied = if target == callee { applied } else { 0 };
                    let complete = self.callable_arity(&target).is_none_or(|arity| {
                        target_applied.saturating_add(arguments.len()) >= arity
                    });
                    if complete {
                        if let Some(effects) = callable_effects.get(&target) {
                            output.extend(effects);
                        }
                        if matches!(
                            &target,
                            Callable::Definition(definition)
                                if definition == self.typed.resolved().builtin_id(Builtin::Map)
                        ) {
                            if let Some(callback) = arguments
                                .first()
                                .and_then(|argument| self.expression_callable(module, argument))
                                .and_then(|callback| callable_effects.get(&callback))
                            {
                                output.extend(callback);
                            }
                        }
                    }
                }
            }
            hir::ExpressionKind::Projection { target, .. } => {
                if self.expression_callable(module, expression).is_none() {
                    output.extend(&self.expression_effects(module, target, callable_effects));
                }
            }
            hir::ExpressionKind::Binary { left, right, .. } => {
                output.extend(&self.expression_effects(module, left, callable_effects));
                output.extend(&self.expression_effects(module, right, callable_effects));
            }
            hir::ExpressionKind::Unary { operand, .. } => {
                output.extend(&self.expression_effects(module, operand, callable_effects));
            }
            hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
                for element in elements {
                    output.extend(&self.expression_effects(module, element, callable_effects));
                }
            }
            hir::ExpressionKind::Record(fields) => {
                for field in fields {
                    output.extend(&self.expression_effects(module, &field.value, callable_effects));
                }
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                output.extend(&self.expression_effects(module, base, callable_effects));
                for field in fields {
                    output.extend(&self.expression_effects(module, &field.value, callable_effects));
                }
            }
            hir::ExpressionKind::Name { .. }
            | hir::ExpressionKind::Literal(_)
            | hir::ExpressionKind::Unit => {}
        }
        output
    }

    fn build_handler_cores(
        &mut self,
        callable_effects: &BTreeMap<Callable, EffectRow>,
    ) -> BTreeMap<ExpressionKey, HandlerCore> {
        let modules = self.typed.resolved().modules().to_vec();
        let mut output = BTreeMap::new();
        for module in modules {
            for definition in &module.hir.definitions {
                self.collect_handler_cores(
                    module.id,
                    &module.hir.source_name,
                    &definition.value,
                    callable_effects,
                    &mut output,
                );
            }
            for implementation in &module.hir.impls {
                for definition in &implementation.members {
                    self.collect_handler_cores(
                        module.id,
                        &module.hir.source_name,
                        &definition.value,
                        callable_effects,
                        &mut output,
                    );
                }
            }
        }
        output
    }

    fn collect_handler_cores(
        &mut self,
        module: ModuleId,
        source_name: &str,
        expression: &hir::Expression,
        callable_effects: &BTreeMap<Callable, EffectRow>,
        output: &mut BTreeMap<ExpressionKey, HandlerCore>,
    ) {
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                for element in elements {
                    let value = match element {
                        hir::SequenceElement::Let(binding) => &binding.value,
                        hir::SequenceElement::Expression(value) => value,
                    };
                    self.collect_handler_cores(
                        module,
                        source_name,
                        value,
                        callable_effects,
                        output,
                    );
                }
            }
            hir::ExpressionKind::Handle { body, clauses } => {
                self.collect_handler_cores(module, source_name, body, callable_effects, output);
                for clause in clauses {
                    self.collect_handler_cores(
                        module,
                        source_name,
                        &clause.body,
                        callable_effects,
                        output,
                    );
                }
                let built = (|| {
                    let input =
                        effect_row_model(&self.expression_effects(module, body, callable_effects))?;
                    let mut core_clauses = Vec::with_capacity(clauses.len());
                    for clause in clauses {
                        let operation_name = clause.operation.normalized();
                        let operation = resolve_handler_operation(&operation_name)
                            .ok_or_else(|| format!("unknown operation `{operation_name}`"))?;
                        let clause_contract = handler_clause(operation)?;
                        let uses = clause.resume.as_ref().map_or(0, |resume| {
                            self.typed
                                .resolved()
                                .handler_resume_uses(BindingKey::new(module, resume.id))
                                .unwrap_or(0)
                        });
                        let resume_use = match uses {
                            0 => ResumeUse::Never,
                            1 => ResumeUse::Once,
                            _ => ResumeUse::Many,
                        };
                        core_clauses.push(HandlerCoreClause::new(
                            clause_contract,
                            handler_node_id(clause.body.id),
                            resume_use,
                        ));
                    }
                    let key = ExpressionKey::new(module, expression.id);
                    let return_type = self
                        .typed
                        .expression_type(key)
                        .ok_or_else(|| "missing checked handler result type".to_owned())?;
                    HandlerCore::new(
                        input,
                        handler_node_id(body.id),
                        canonical_type_ref(&self.typed.display_type(return_type))?,
                        core_clauses,
                        Some(EffectSourceSpan::new(
                            source_name,
                            u64::from(expression.span.start().get()),
                            u64::from(expression.span.end().get()),
                        )),
                    )
                    .map_err(|error| error.to_string())
                    .map(|core| (key, core))
                })();
                match built {
                    Ok((key, core)) => {
                        output.insert(key, core);
                    }
                    Err(reason) => self.errors.push(EffectError {
                        kind: EffectErrorKind::InvalidHandlerContract {
                            operation: clauses
                                .first()
                                .map_or_else(String::new, |clause| clause.operation.normalized()),
                            reason,
                        },
                        source_name: source_name.to_owned(),
                        span: expression.span,
                    }),
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
                    self.collect_handler_cores(
                        module,
                        source_name,
                        value,
                        callable_effects,
                        output,
                    );
                }
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                self.collect_handler_cores(
                    module,
                    source_name,
                    scrutinee,
                    callable_effects,
                    output,
                );
                for case in cases {
                    if let Some(guard) = &case.guard {
                        self.collect_handler_cores(
                            module,
                            source_name,
                            guard,
                            callable_effects,
                            output,
                        );
                    }
                    self.collect_handler_cores(
                        module,
                        source_name,
                        &case.body,
                        callable_effects,
                        output,
                    );
                }
            }
            hir::ExpressionKind::Assignment { value, .. } => {
                self.collect_handler_cores(module, source_name, value, callable_effects, output)
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                self.collect_handler_cores(module, source_name, function, callable_effects, output);
                for argument in arguments {
                    self.collect_handler_cores(
                        module,
                        source_name,
                        argument,
                        callable_effects,
                        output,
                    );
                }
            }
            hir::ExpressionKind::Projection { target, .. }
            | hir::ExpressionKind::Unary {
                operand: target, ..
            } => self.collect_handler_cores(module, source_name, target, callable_effects, output),
            hir::ExpressionKind::Binary { left, right, .. } => {
                for value in [left.as_ref(), right.as_ref()] {
                    self.collect_handler_cores(
                        module,
                        source_name,
                        value,
                        callable_effects,
                        output,
                    );
                }
            }
            hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
                for element in elements {
                    self.collect_handler_cores(
                        module,
                        source_name,
                        element,
                        callable_effects,
                        output,
                    );
                }
            }
            hir::ExpressionKind::Record(fields) => {
                for field in fields {
                    self.collect_handler_cores(
                        module,
                        source_name,
                        &field.value,
                        callable_effects,
                        output,
                    );
                }
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                self.collect_handler_cores(module, source_name, base, callable_effects, output);
                for field in fields {
                    self.collect_handler_cores(
                        module,
                        source_name,
                        &field.value,
                        callable_effects,
                        output,
                    );
                }
            }
            hir::ExpressionKind::Name { .. }
            | hir::ExpressionKind::Literal(_)
            | hir::ExpressionKind::Unit => {}
        }
    }

    fn connect_arguments(
        &mut self,
        callee: &Callable,
        module: ModuleId,
        arguments: &[hir::Expression],
    ) {
        let parameters = self
            .callable_parameters
            .get(callee)
            .cloned()
            .unwrap_or_default();
        for (parameter, argument) in parameters.into_iter().zip(arguments) {
            let Some(parameter) = parameter else {
                continue;
            };
            let Some(argument) = self
                .value_callable(module, argument)
                .or_else(|| self.expression_callable(module, argument))
            else {
                continue;
            };
            self.calls
                .entry(Callable::Binding(parameter))
                .or_default()
                .insert(CallEdge {
                    callee: argument,
                    handled: BTreeSet::new(),
                });
        }
    }

    fn expression_callable(
        &self,
        module: ModuleId,
        expression: &hir::Expression,
    ) -> Option<Callable> {
        self.expression_callable_state(module, expression)
            .map(|(callable, _)| callable)
    }

    fn expression_callable_state(
        &self,
        module: ModuleId,
        expression: &hir::Expression,
    ) -> Option<(Callable, usize)> {
        if let hir::ExpressionKind::Application {
            function,
            arguments,
        } = &expression.kind
        {
            let (callable, applied) = self.expression_callable_state(module, function)?;
            return Some((callable, applied.saturating_add(arguments.len())));
        }
        let reference = match &expression.kind {
            hir::ExpressionKind::Name { reference, .. }
            | hir::ExpressionKind::Projection { reference, .. } => *reference,
            _ => return None,
        };
        match self.typed.resolved().reference(module, reference) {
            Some(ReferenceTarget::Definition(definition)) => {
                Some((Callable::Definition(definition.clone()), 0))
            }
            Some(ReferenceTarget::Binding(binding)) => Some((Callable::Binding(*binding), 0)),
            None => None,
        }
    }

    fn value_callable(&self, module: ModuleId, expression: &hir::Expression) -> Option<Callable> {
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                elements.iter().rev().find_map(|element| match element {
                    hir::SequenceElement::Expression(value) => self.value_callable(module, value),
                    hir::SequenceElement::Let(_) => None,
                })
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                let (callee, applied) = self.expression_callable_state(module, function)?;
                let arity = self.callable_arity(&callee)?;
                let total = applied.saturating_add(arguments.len());
                if total < arity {
                    return Some(callee);
                }
                self.returns.get(&callee)?.iter().next().cloned()
            }
            hir::ExpressionKind::Name { .. } | hir::ExpressionKind::Projection { .. } => {
                self.expression_callable(module, expression)
            }
            _ => None,
        }
    }

    fn callable_arity(&self, callable: &Callable) -> Option<usize> {
        let type_id = match callable {
            Callable::Definition(definition) => self.typed.definition_type(definition)?,
            Callable::Binding(binding) => self.typed.binding_type(*binding)?,
        };
        match self.typed.arena().get(type_id) {
            Type::Function { parameters, .. } => Some(parameters.len()),
            _ => None,
        }
    }

    fn propagate_effects(&self, apply_handlers: bool) -> BTreeMap<Callable, EffectRow> {
        let mut effects = self.direct_effects.clone();
        let mut changed = true;
        while changed {
            changed = false;
            for (caller, callees) in &self.calls {
                let mut next = effects.get(caller).cloned().unwrap_or_default();
                for edge in callees {
                    if let Some(callee_effects) = effects.get(&edge.callee) {
                        if apply_handlers {
                            next.extend(&callee_effects.without(&edge.handled));
                        } else {
                            next.extend(callee_effects);
                        }
                    }
                }
                if effects.get(caller) != Some(&next) {
                    effects.insert(caller.clone(), next);
                    changed = true;
                }
            }
        }
        effects
    }

    fn check_capabilities(
        &mut self,
        definition_effects: &BTreeMap<DefinitionId, EffectRow>,
    ) -> BTreeMap<ModuleId, BTreeSet<Capability>> {
        let modules = self.typed.resolved().modules().to_vec();
        let mut output = BTreeMap::new();
        for module in &modules {
            let mut declared = BTreeSet::new();
            for capability in &module.hir.module.requires {
                let normalized = capability.normalized();
                if normalized == Capability::ConsoleWrite.name() {
                    declared.insert(Capability::ConsoleWrite);
                } else {
                    self.errors.push(EffectError {
                        kind: EffectErrorKind::UnknownCapability {
                            capability: normalized,
                        },
                        source_name: module.hir.source_name.clone(),
                        span: capability.span,
                    });
                }
            }
            let uses_console = self
                .typed
                .resolved()
                .definitions()
                .values()
                .filter(|definition| {
                    matches!(
                        definition.origin,
                        DefinitionOrigin::User { module: owner } if owner == module.id
                    )
                })
                .filter_map(|definition| definition_effects.get(&definition.id))
                .any(|row| row.0.contains(&Effect::ConsoleWrite));
            if uses_console && !declared.contains(&Capability::ConsoleWrite) {
                self.errors.push(EffectError {
                    kind: EffectErrorKind::MissingCapability {
                        capability: Capability::ConsoleWrite.name(),
                    },
                    source_name: module.hir.source_name.clone(),
                    span: module.hir.module.span,
                });
            }
            if !uses_console && declared.contains(&Capability::ConsoleWrite) {
                self.warnings.push(
                    Diagnostic::new(
                        codes::UNUSED_CAPABILITY,
                        Severity::Warning,
                        "模块声明了未使用的 Capability“Console.Write”",
                        "module declares unused capability `Console.Write`",
                    )
                    .with_primary_span(DiagnosticSpan::new(
                        &module.hir.source_name,
                        module.hir.module.span,
                    )),
                );
            }
            output.insert(module.id, declared);
        }
        output
    }
}

fn handler_node_id(expression: hir::ExpressionId) -> HandlerCoreNodeId {
    HandlerCoreNodeId::new(expression.get().saturating_add(1))
}

fn handler_clause(operation: ResolvedHandlerOperation) -> Result<HandlerClause, String> {
    let owner = EffectId::new(operation.owner()).map_err(|error| error.to_string())?;
    let label = EffectLabel::new(owner.clone(), []);
    let inputs = operation
        .inputs()
        .iter()
        .copied()
        .map(handler_type_ref)
        .collect::<Result<Vec<_>, _>>()?;
    let operation = EffectOperation::new(
        owner,
        operation.operation(),
        inputs,
        handler_type_ref(operation.output())?,
        match operation.resume_mode() {
            HandlerResumeMode::Never => ResumeMode::Never,
            HandlerResumeMode::Once => ResumeMode::Once,
            HandlerResumeMode::Many => ResumeMode::Many,
        },
    )
    .map_err(|error| error.to_string())?;
    HandlerClause::new(label, operation).map_err(|error| error.to_string())
}

fn handler_type_ref(value: HandlerValueType) -> Result<EffectTypeRef, String> {
    EffectTypeRef::new(match value {
        HandlerValueType::Unit => "Unit",
        HandlerValueType::Int => "Int",
        HandlerValueType::Text => "Text",
    })
    .map_err(|error| error.to_string())
}

fn canonical_type_ref(display: &str) -> Result<EffectTypeRef, String> {
    let canonical = display
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    EffectTypeRef::new(&canonical).map_err(|error| error.to_string())
}

fn effect_row_model(row: &EffectRow) -> Result<EffectRowModel, String> {
    let labels = row
        .effects()
        .map(|effect| match effect {
            Effect::ConsoleWrite => Ok(EffectLabel::console_write()),
            Effect::State { identity, .. } => canonical_type_ref(identity).map(EffectLabel::state),
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(EffectRowModel::closed(labels))
}

#[cfg(test)]
mod tests {
    use ling_ast::lower as lower_ast;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;

    use super::*;

    fn typed(text: &str) -> TypedProgram {
        typed_entry(text, "Main")
    }

    fn typed_entry(text: &str, entry: &str) -> TypedProgram {
        let source =
            SourceFile::from_bytes(SourceId::new(0), "test.ling", text.as_bytes().to_vec())
                .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = hir::lower(source.name(), &ast).expect("valid HIR");
        let resolved = ling_resolve::resolve(vec![hir], entry).expect("resolves");
        ling_types::check(resolved).expect("type-checks")
    }

    #[test]
    fn hello_has_console_effect_and_valid_entry() {
        let checked = check(typed(
            "module Main\n    requires Console.Write\n\nlet main () = Console.write \"你好，零\"\n",
        ))
        .expect("capability is declared");
        let main = locate_main(&checked).expect("valid main");
        assert_eq!(
            checked
                .definition_effect(&main)
                .expect("main effects")
                .names(),
            vec!["Console.Write"]
        );
        assert_eq!(
            checked
                .definition_function_type(&main)
                .expect("checked main function type")
                .display(),
            "Unit -> Unit ! {Console.Write}"
        );
    }

    #[test]
    fn missing_console_capability_is_rejected() {
        let errors = check(typed("module Main\n\nlet main () = Console.write \"x\"\n"))
            .expect_err("missing capability must fail");
        assert!(matches!(
            errors[0].kind,
            EffectErrorKind::MissingCapability { .. }
        ));
    }

    #[test]
    fn boolean_rhs_effects_are_checked_even_when_runtime_would_skip_them() {
        let source = concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let writeBool () =\n",
            "    Console.write \"rhs\"\n",
            "    true\n\n",
            "let main () =\n",
            "    false && writeBool ()\n",
            "    true || writeBool ()\n",
            "    ()\n",
        );
        let checked = check(typed(source)).expect("capability is declared");
        let main = locate_main(&checked).expect("valid main");
        assert_eq!(
            checked
                .definition_effect(&main)
                .expect("main effects")
                .names(),
            vec!["Console.Write"]
        );

        let without_capability = source.replace("    requires Console.Write\n", "");
        let errors = check(typed(&without_capability))
            .expect_err("statically visited boolean RHS requires capability");
        assert!(errors.iter().any(|error| matches!(
            error.kind,
            EffectErrorKind::MissingCapability {
                capability: "Console.Write"
            }
        )));
    }

    #[test]
    fn value_binding_is_not_a_main_entry() {
        let checked = check(typed("module Main\n\nlet main = ()\n")).expect("pure module checks");
        assert!(matches!(
            locate_main(&checked)
                .expect_err("main needs Unit pattern")
                .kind,
            EntryErrorKind::MainMustHaveUnitPattern
        ));
    }

    #[test]
    fn validates_explicit_implicit_missing_module_and_signature_entry_rules() {
        let implicit = check(typed("let main () = ()\n")).expect("implicit Main checks");
        locate_main(&implicit).expect("implicit Main is executable");

        let library = check(typed_entry(
            "module Library\n\nlet main () = ()\n",
            "Library",
        ))
        .expect("library module checks");
        assert!(matches!(
            locate_main(&library).expect_err("run requires Main").kind,
            EntryErrorKind::EntryModuleMustBeMain { actual } if actual == "Library"
        ));

        let missing = check(typed("module Main\n\nlet value = 1\n")).expect("module checks");
        assert!(matches!(
            locate_main(&missing).expect_err("main is required").kind,
            EntryErrorKind::MissingMain
        ));

        let invalid =
            check(typed("module Main\n\nlet main () = 1\n")).expect("typed function checks");
        assert!(matches!(
            locate_main(&invalid)
                .expect_err("main must be Unit -> Unit")
                .kind,
            EntryErrorKind::InvalidMainSignature { .. }
        ));
    }

    #[test]
    fn map_propagates_its_callback_effect_and_capability() {
        let source = concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "let write value = Console.write (Text.format \"{}\" value)\n\n",
            "let main () =\n",
            "    map write [1; 2]\n",
            "    ()\n",
        );
        let checked = check(typed(source)).expect("callback capability is declared");
        let main = locate_main(&checked).expect("valid main");
        assert_eq!(
            checked
                .definition_effect(&main)
                .expect("main effects")
                .names(),
            vec!["Console.Write"]
        );

        let without_capability = source.replace("    requires Console.Write\n", "");
        let errors = check(typed(&without_capability))
            .expect_err("callback effect requires the module capability");
        assert!(errors.iter().any(|error| matches!(
            error.kind,
            EffectErrorKind::MissingCapability {
                capability: "Console.Write"
            }
        )));
    }

    #[test]
    fn local_functions_are_latent_and_higher_order_wrappers_propagate_effects() {
        let unused = check(typed(concat!(
            "module Main\n\n",
            "let main () =\n",
            "    let write value = Console.write value\n",
            "    ()\n",
        )))
        .expect("creating an unused local closure is Pure");
        let unused_main = locate_main(&unused).expect("valid main");
        assert!(
            unused
                .definition_effect(&unused_main)
                .expect("main effects")
                .is_pure()
        );

        let wrapper = concat!(
            "module Main\n\n",
            "let apply callback values = map callback values\n",
            "let write value = Console.write value\n\n",
            "let main () =\n",
            "    apply write [\"a\"; \"b\"]\n",
            "    ()\n",
        );
        let errors = check(typed(wrapper))
            .expect_err("a callback passed through a wrapper still requires Console.Write");
        assert!(errors.iter().any(|error| matches!(
            error.kind,
            EffectErrorKind::MissingCapability {
                capability: "Console.Write"
            }
        )));

        let declared = wrapper.replace(
            "module Main\n\n",
            "module Main\n    requires Console.Write\n\n",
        );
        let checked = check(typed(&declared)).expect("wrapper callback capability is declared");
        let main = locate_main(&checked).expect("valid main");
        assert_eq!(
            checked
                .definition_effect(&main)
                .expect("main effects")
                .names(),
            vec!["Console.Write"]
        );

        let partial = concat!(
            "module Main\n    requires Console.Write\n\n",
            "let write prefix value = Console.write value\n\n",
            "let main () =\n",
            "    let partial = write \"prefix\"\n",
            "    ()\n",
        );
        let checked = check(typed(partial))
            .expect("partial application is pure until the returned function is called");
        let main = locate_main(&checked).expect("valid main");
        assert!(
            checked
                .definition_effect(&main)
                .expect("main effects")
                .is_pure()
        );

        let complete = partial.replace("    ()\n", "    partial \"value\"\n");
        let checked = check(typed(&complete))
            .expect("calling the partially applied function uses its latent Effect");
        let main = locate_main(&checked).expect("valid main");
        assert_eq!(
            checked
                .definition_effect(&main)
                .expect("main effects")
                .names(),
            vec!["Console.Write"]
        );
    }

    #[test]
    fn state_effects_need_no_capability_and_unused_capabilities_warn() {
        let checked = check(typed(concat!(
            "module Main\n",
            "    requires Console.Write\n\n",
            "type Counter = { mutable value: Int }\n\n",
            "let main () =\n",
            "    let mutable count = 0\n",
            "    count <- 1\n",
            "    let mutable counter = { value = 0 }\n",
            "    counter.value <- 1\n",
        )))
        .expect("local State does not require a host capability");
        let main = locate_main(&checked).expect("valid main");
        assert_eq!(
            checked
                .definition_effect(&main)
                .expect("main effects")
                .names(),
            vec!["State<Counter>", "State<Int>"]
        );
        assert_eq!(
            checked
                .definition_function_type(&main)
                .expect("checked main function type")
                .display(),
            "Unit -> Unit ! {State<Counter>, State<Int>}"
        );
        assert!(
            checked
                .warnings()
                .iter()
                .any(|warning| warning.code() == codes::UNUSED_CAPABILITY)
        );
    }

    #[test]
    fn effect_rows_are_idempotent_commutative_sets() {
        let state = Effect::State {
            display: "Counter".to_owned(),
            identity: "counter-definition".to_owned(),
        };
        let console = Effect::ConsoleWrite;

        let mut left = EffectRow::default();
        left.insert(state.clone());
        left.insert(console.clone());
        left.insert(state.clone());

        let mut right = EffectRow::default();
        right.insert(console);
        right.insert(state);

        assert_eq!(left, right);
        assert_eq!(left.names(), vec!["Console.Write", "State<Counter>"]);

        let unchanged = left.clone();
        left.extend(&right);
        assert_eq!(left, unchanged);

        let mut canonical = EffectRow::default();
        canonical.insert(Effect::State {
            display: "A".to_owned(),
            identity: "z".to_owned(),
        });
        canonical.insert(Effect::State {
            display: "Z".to_owned(),
            identity: "a".to_owned(),
        });
        assert_eq!(canonical.names(), ["State<A>", "State<Z>"]);
        assert_eq!(canonical.canonical_names(), ["State<a>", "State<z>"]);
    }

    #[test]
    fn seed_effect_row_snapshot_is_canonical_and_path_free() {
        let mut row = EffectRow::default();
        row.insert(Effect::State {
            display: "Readable display".to_owned(),
            identity: "z-state".to_owned(),
        });
        row.insert(Effect::ConsoleWrite);
        row.insert(Effect::State {
            display: "Other display".to_owned(),
            identity: "a-state".to_owned(),
        });

        let snapshot = row.seed_snapshot();
        assert_eq!(
            snapshot.canonical_names(),
            ["Console.Write", "State<a-state>", "State<z-state>"]
        );
        assert!(!snapshot.is_pure());
        assert_eq!(row.seed_snapshot(), snapshot);

        let pure = EffectRow::default().seed_snapshot();
        assert!(pure.is_pure());
        assert!(pure.canonical_names().is_empty());
    }
}
