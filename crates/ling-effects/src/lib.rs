//! Seed effect inference, capability checking, and checked-program sealing.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_hir as hir;
use ling_resolve::{
    Builtin, DefinitionId, DefinitionOrigin, ExpressionKey, ModuleId, ReferenceTarget,
};
use ling_source::Span;
use ling_types::{Type, TypedProgram};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Effect {
    ConsoleWrite,
    State(String),
}

impl Effect {
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Self::ConsoleWrite => "Console.Write".to_owned(),
            Self::State(value) => format!("State<{value}>"),
        }
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

    fn insert(&mut self, effect: Effect) {
        self.0.insert(effect);
    }

    fn extend(&mut self, other: &Self) {
        self.0.extend(other.0.iter().cloned());
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
    module_capabilities: BTreeMap<ModuleId, BTreeSet<Capability>>,
    warnings: Vec<Diagnostic>,
}

impl CheckedProgram {
    #[must_use]
    pub const fn typed(&self) -> &TypedProgram {
        &self.typed
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
    pub fn module_capabilities(&self, module: ModuleId) -> Option<&BTreeSet<Capability>> {
        self.module_capabilities.get(&module)
    }

    #[must_use]
    pub fn warnings(&self) -> &[Diagnostic] {
        &self.warnings
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
        };
        Diagnostic::new(code, Severity::Error, zh, en)
            .with_primary_span(DiagnosticSpan::new(&self.source_name, self.span))
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
        .typed
        .definition_type(&main_id)
        .expect("checked user definitions have types");
    let valid = match checked.typed.arena().get(main_type) {
        Type::Function { parameters, result } if parameters.len() == 1 => {
            matches!(checked.typed.arena().get(parameters[0]), Type::Unit)
                && matches!(checked.typed.arena().get(*result), Type::Unit)
        }
        _ => false,
    };
    if !valid {
        return Err(EntryError {
            kind: EntryErrorKind::InvalidMainSignature {
                actual: checked.typed.arena().display(main_type),
            },
            source_name: module.hir.source_name.clone(),
            span: main.span,
        });
    }
    Ok(main_id)
}

struct Checker {
    typed: TypedProgram,
    direct_effects: BTreeMap<DefinitionId, EffectRow>,
    calls: BTreeMap<DefinitionId, BTreeSet<DefinitionId>>,
    errors: Vec<EffectError>,
    warnings: Vec<Diagnostic>,
}

impl Checker {
    fn new(typed: TypedProgram) -> Self {
        Self {
            typed,
            direct_effects: BTreeMap::new(),
            calls: BTreeMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn run(mut self) -> Result<CheckedProgram, Vec<EffectError>> {
        self.collect_direct_effects();
        let definition_effects = self.propagate_effects();
        let module_capabilities = self.check_capabilities(&definition_effects);
        if self.errors.is_empty() {
            Ok(CheckedProgram {
                typed: self.typed,
                definition_effects,
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
                let mut effects = EffectRow::default();
                let mut calls = BTreeSet::new();
                self.visit_expression(module.id, &definition.value, &mut effects, &mut calls);
                self.direct_effects.insert(id.clone(), effects);
                self.calls.insert(id, calls);
            }
        }
        for definition in self.typed.resolved().definitions().values() {
            if let DefinitionOrigin::Builtin(builtin) = definition.origin {
                let mut effects = EffectRow::default();
                if builtin == Builtin::ConsoleWrite {
                    effects.insert(Effect::ConsoleWrite);
                }
                self.direct_effects.insert(definition.id.clone(), effects);
                self.calls.entry(definition.id.clone()).or_default();
            }
        }
    }

    fn visit_expression(
        &self,
        module: ModuleId,
        expression: &hir::Expression,
        effects: &mut EffectRow,
        calls: &mut BTreeSet<DefinitionId>,
    ) {
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            self.visit_expression(module, &binding.value, effects, calls);
                        }
                        hir::SequenceElement::Expression(expression) => {
                            self.visit_expression(module, expression, effects, calls);
                        }
                    }
                }
            }
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expression(module, condition, effects, calls);
                self.visit_expression(module, then_branch, effects, calls);
                self.visit_expression(module, else_branch, effects, calls);
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                self.visit_expression(module, scrutinee, effects, calls);
                for case in cases {
                    if let Some(guard) = &case.guard {
                        self.visit_expression(module, guard, effects, calls);
                    }
                    self.visit_expression(module, &case.body, effects, calls);
                }
            }
            hir::ExpressionKind::Assignment { value, .. } => {
                let key = ExpressionKey::new(module, expression.id);
                let value_type = self
                    .typed
                    .place_type(key)
                    .map(|type_id| self.typed.arena().display(type_id))
                    .unwrap_or_else(|| "unknown".to_owned());
                effects.insert(Effect::State(value_type));
                self.visit_expression(module, value, effects, calls);
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                if let Some(definition) = self.expression_definition(module, function) {
                    calls.insert(definition);
                } else {
                    self.visit_expression(module, function, effects, calls);
                }
                for argument in arguments {
                    self.visit_expression(module, argument, effects, calls);
                }
            }
            hir::ExpressionKind::Projection { target, .. } => {
                if self.expression_definition(module, expression).is_none() {
                    self.visit_expression(module, target, effects, calls);
                }
            }
            hir::ExpressionKind::Binary { left, right, .. } => {
                self.visit_expression(module, left, effects, calls);
                self.visit_expression(module, right, effects, calls);
            }
            hir::ExpressionKind::Unary { operand, .. } => {
                self.visit_expression(module, operand, effects, calls);
            }
            hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
                for element in elements {
                    self.visit_expression(module, element, effects, calls);
                }
            }
            hir::ExpressionKind::Record(fields) => {
                for field in fields {
                    self.visit_expression(module, &field.value, effects, calls);
                }
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                self.visit_expression(module, base, effects, calls);
                for field in fields {
                    self.visit_expression(module, &field.value, effects, calls);
                }
            }
            hir::ExpressionKind::Name { .. }
            | hir::ExpressionKind::Literal(_)
            | hir::ExpressionKind::Unit => {}
        }
    }

    fn expression_definition(
        &self,
        module: ModuleId,
        expression: &hir::Expression,
    ) -> Option<DefinitionId> {
        let reference = match &expression.kind {
            hir::ExpressionKind::Name { reference, .. }
            | hir::ExpressionKind::Projection { reference, .. } => *reference,
            _ => return None,
        };
        match self.typed.resolved().reference(module, reference) {
            Some(ReferenceTarget::Definition(definition)) => Some(definition.clone()),
            _ => None,
        }
    }

    fn propagate_effects(&self) -> BTreeMap<DefinitionId, EffectRow> {
        let mut effects = self.direct_effects.clone();
        let mut changed = true;
        while changed {
            changed = false;
            for (caller, callees) in &self.calls {
                let mut next = effects.get(caller).cloned().unwrap_or_default();
                for callee in callees {
                    if let Some(callee_effects) = effects.get(callee) {
                        next.extend(callee_effects);
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
            let uses_console = module.hir.definitions.iter().any(|definition| {
                self.typed
                    .resolved()
                    .definition_id(module.id, &definition.name.normalized)
                    .and_then(|id| definition_effects.get(id))
                    .is_some_and(|row| row.0.contains(&Effect::ConsoleWrite))
            });
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

#[cfg(test)]
mod tests {
    use ling_ast::lower as lower_ast;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;

    use super::*;

    fn typed(text: &str) -> TypedProgram {
        let source =
            SourceFile::from_bytes(SourceId::new(0), "test.ling", text.as_bytes().to_vec())
                .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        let hir = hir::lower(source.name(), &ast).expect("valid HIR");
        let resolved = ling_resolve::resolve(vec![hir], "Main").expect("resolves");
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
    fn value_binding_is_not_a_main_entry() {
        let checked = check(typed("module Main\n\nlet main = ()\n")).expect("pure module checks");
        assert!(matches!(
            locate_main(&checked)
                .expect_err("main needs Unit pattern")
                .kind,
            EntryErrorKind::MainMustHaveUnitPattern
        ));
    }
}
