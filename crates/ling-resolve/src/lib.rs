//! Deterministic module and lexical name resolution for Ling Seed.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_hir::{self as hir, BindingId, ExpressionId, ReferenceId};
use ling_source::Span;

const LANGUAGE_VERSION: &str = "0.0.1-dev";
const SEMANTIC_SCHEMA: &str = "ling.semantic/0.1";

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModuleId(u32);

impl ModuleId {
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExpressionKey {
    module: ModuleId,
    local: ExpressionId,
}

impl ExpressionKey {
    #[must_use]
    pub const fn new(module: ModuleId, local: ExpressionId) -> Self {
        Self { module, local }
    }

    #[must_use]
    pub const fn module(self) -> ModuleId {
        self.module
    }

    #[must_use]
    pub const fn local(self) -> ExpressionId {
        self.local
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReferenceKey {
    module: ModuleId,
    local: ReferenceId,
}

impl ReferenceKey {
    #[must_use]
    pub const fn new(module: ModuleId, local: ReferenceId) -> Self {
        Self { module, local }
    }

    #[must_use]
    pub const fn module(self) -> ModuleId {
        self.module
    }

    #[must_use]
    pub const fn local(self) -> ReferenceId {
        self.local
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BindingKey {
    module: ModuleId,
    local: BindingId,
}

impl BindingKey {
    #[must_use]
    pub const fn new(module: ModuleId, local: BindingId) -> Self {
        Self { module, local }
    }

    #[must_use]
    pub const fn module(self) -> ModuleId {
        self.module
    }

    #[must_use]
    pub const fn local(self) -> BindingId {
        self.local
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DefinitionId(String);

impl DefinitionId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn new(kind: &str, module: &str, name: &str) -> Self {
        let mut bytes = Vec::new();
        encode_part(&mut bytes, b"ling.definition-id/v1");
        encode_part(&mut bytes, LANGUAGE_VERSION.as_bytes());
        encode_part(&mut bytes, SEMANTIC_SCHEMA.as_bytes());
        encode_part(&mut bytes, kind.as_bytes());
        encode_part(&mut bytes, module.as_bytes());
        encode_part(&mut bytes, name.as_bytes());
        Self(format!(
            "experimental:blake3:{}",
            blake3::hash(&bytes).to_hex()
        ))
    }
}

impl fmt::Display for DefinitionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Builtin {
    ConsoleWrite,
    TextFormat,
    Max,
    Min,
    Map,
    Sum,
}

impl Builtin {
    #[must_use]
    pub const fn qualified_name(self) -> &'static str {
        match self {
            Self::ConsoleWrite => "Console.write",
            Self::TextFormat => "Text.format",
            Self::Max => "max",
            Self::Min => "min",
            Self::Map => "map",
            Self::Sum => "sum",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefinitionKind {
    Value,
    Type,
    Constructor,
    Builtin,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DefinitionOrigin {
    User { module: ModuleId },
    Builtin(Builtin),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionInfo {
    pub id: DefinitionId,
    pub module_name: String,
    pub name: String,
    pub kind: DefinitionKind,
    pub origin: DefinitionOrigin,
    pub mutable: bool,
    pub source_name: Option<String>,
    pub span: Option<Span>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingInfo {
    pub key: BindingKey,
    pub name: String,
    pub mutable: bool,
    pub parameter: bool,
    pub span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceTarget {
    Definition(DefinitionId),
    Binding(BindingKey),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedModule {
    pub id: ModuleId,
    pub hir: hir::Program,
    pub imports: BTreeMap<String, ModuleId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedProgram {
    modules: Vec<ResolvedModule>,
    entry: ModuleId,
    definitions: BTreeMap<DefinitionId, DefinitionInfo>,
    references: BTreeMap<ReferenceKey, ReferenceTarget>,
    bindings: BTreeMap<BindingKey, BindingInfo>,
    builtins: BTreeMap<Builtin, DefinitionId>,
}

impl ResolvedProgram {
    #[must_use]
    pub fn modules(&self) -> &[ResolvedModule] {
        &self.modules
    }

    #[must_use]
    pub const fn entry(&self) -> ModuleId {
        self.entry
    }

    #[must_use]
    pub fn entry_module(&self) -> &ResolvedModule {
        self.module(self.entry)
            .expect("resolved entry module must exist")
    }

    #[must_use]
    pub fn module(&self, id: ModuleId) -> Option<&ResolvedModule> {
        self.modules.iter().find(|module| module.id == id)
    }

    #[must_use]
    pub fn definitions(&self) -> &BTreeMap<DefinitionId, DefinitionInfo> {
        &self.definitions
    }

    #[must_use]
    pub fn definition(&self, id: &DefinitionId) -> Option<&DefinitionInfo> {
        self.definitions.get(id)
    }

    #[must_use]
    pub fn references(&self) -> &BTreeMap<ReferenceKey, ReferenceTarget> {
        &self.references
    }

    #[must_use]
    pub fn reference(&self, module: ModuleId, reference: ReferenceId) -> Option<&ReferenceTarget> {
        self.references.get(&ReferenceKey::new(module, reference))
    }

    #[must_use]
    pub fn bindings(&self) -> &BTreeMap<BindingKey, BindingInfo> {
        &self.bindings
    }

    #[must_use]
    pub fn builtin_id(&self, builtin: Builtin) -> &DefinitionId {
        self.builtins
            .get(&builtin)
            .expect("all Seed builtins are injected")
    }

    #[must_use]
    pub fn definition_id(&self, module: ModuleId, normalized_name: &str) -> Option<&DefinitionId> {
        self.definitions
            .values()
            .find_map(|definition| match definition.origin {
                DefinitionOrigin::User {
                    module: definition_module,
                } if definition_module == module && definition.name == normalized_name => {
                    Some(&definition.id)
                }
                _ => None,
            })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolveError {
    pub kind: ResolveErrorKind,
    pub source_name: String,
    pub span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResolveErrorKind {
    UndefinedName { name: String },
    DuplicateDefinition { name: String },
    DuplicateModule { module: String },
    DuplicateImportAlias { alias: String },
    MissingModule { module: String },
    ImportedModuleMustBeExplicit { module: String },
    ImportCycle { modules: Vec<String> },
    ConfusableCollision { first: String, second: String },
    ReservedName { name: String },
    MutableTopLevel { name: String },
}

impl ResolveError {
    #[must_use]
    pub fn to_diagnostic(&self) -> Diagnostic {
        let (code, zh, en) = match &self.kind {
            ResolveErrorKind::UndefinedName { name } => (
                codes::UNDEFINED_NAME,
                format!("未定义名称“{name}”"),
                format!("undefined name `{name}`"),
            ),
            ResolveErrorKind::DuplicateDefinition { name } => (
                codes::DUPLICATE_DEFINITION,
                format!("名称“{name}”在同一作用域中重复定义"),
                format!("name `{name}` is defined more than once in the same scope"),
            ),
            ResolveErrorKind::DuplicateModule { module } => (
                codes::INVALID_MODULE,
                format!("模块“{module}”重复"),
                format!("module `{module}` is duplicated"),
            ),
            ResolveErrorKind::DuplicateImportAlias { alias } => (
                codes::DUPLICATE_IMPORT_ALIAS,
                format!("import 别名“{alias}”重复"),
                format!("import alias `{alias}` is duplicated"),
            ),
            ResolveErrorKind::MissingModule { module } => (
                codes::MODULE_NOT_FOUND,
                format!("找不到 import 模块“{module}”"),
                format!("imported module `{module}` was not provided"),
            ),
            ResolveErrorKind::ImportedModuleMustBeExplicit { module } => (
                codes::INVALID_MODULE,
                format!("被 import 的模块“{module}”必须显式声明 module"),
                format!("imported module `{module}` must declare its module name"),
            ),
            ResolveErrorKind::ImportCycle { modules } => (
                codes::IMPORT_CYCLE,
                format!("Seed 不允许 import cycle：{}", modules.join(" -> ")),
                format!("Ling Seed rejects import cycles: {}", modules.join(" -> ")),
            ),
            ResolveErrorKind::ConfusableCollision { first, second } => (
                codes::CONFUSABLE_COLLISION,
                format!("同一作用域中的名称“{first}”与“{second}”视觉混淆"),
                format!("names `{first}` and `{second}` are confusable in the same scope"),
            ),
            ResolveErrorKind::ReservedName { name } => (
                codes::RESERVED_NAME,
                format!("模块作用域不能重定义内置名称“{name}”"),
                format!("module scope cannot redefine built-in name `{name}`"),
            ),
            ResolveErrorKind::MutableTopLevel { name } => (
                codes::INVALID_MODULE,
                format!("Seed 不允许顶层 mutable binding“{name}”"),
                format!("Ling Seed does not allow mutable top-level binding `{name}`"),
            ),
        };
        Diagnostic::new(code, Severity::Error, zh, en)
            .with_primary_span(DiagnosticSpan::new(&self.source_name, self.span))
    }
}

impl fmt::Display for ResolveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.kind)
    }
}

impl Error for ResolveError {}

/// Resolves a deterministic, complete module set. The entry is identified by
/// its normalized module name.
pub fn resolve(
    programs: Vec<hir::Program>,
    entry_module_name: &str,
) -> Result<ResolvedProgram, Vec<ResolveError>> {
    Resolver::new(programs, entry_module_name).run()
}

struct Resolver {
    modules: Vec<ResolvedModule>,
    entry_name: String,
    module_names: BTreeMap<String, ModuleId>,
    definitions: BTreeMap<DefinitionId, DefinitionInfo>,
    module_definitions: BTreeMap<ModuleId, BTreeMap<String, DefinitionId>>,
    references: BTreeMap<ReferenceKey, ReferenceTarget>,
    bindings: BTreeMap<BindingKey, BindingInfo>,
    builtins: BTreeMap<Builtin, DefinitionId>,
    errors: Vec<ResolveError>,
}

impl Resolver {
    fn new(mut programs: Vec<hir::Program>, entry_name: &str) -> Self {
        programs.sort_by_key(|program| program.module.name.normalized());
        let modules = programs
            .into_iter()
            .enumerate()
            .map(|(index, hir)| ResolvedModule {
                id: ModuleId(u32::try_from(index).unwrap_or(u32::MAX)),
                hir,
                imports: BTreeMap::new(),
            })
            .collect();
        Self {
            modules,
            entry_name: entry_name.to_owned(),
            module_names: BTreeMap::new(),
            definitions: BTreeMap::new(),
            module_definitions: BTreeMap::new(),
            references: BTreeMap::new(),
            bindings: BTreeMap::new(),
            builtins: BTreeMap::new(),
            errors: Vec::new(),
        }
    }

    fn run(mut self) -> Result<ResolvedProgram, Vec<ResolveError>> {
        self.inject_builtins();
        self.index_modules();
        self.resolve_imports();
        self.detect_import_cycles();
        self.index_definitions();
        self.resolve_bodies();

        let entry = self.module_names.get(&self.entry_name).copied();
        if entry.is_none() {
            let (source_name, span) = self.modules.first().map_or_else(
                || (String::new(), empty_span()),
                |module| (module.hir.source_name.clone(), module.hir.span),
            );
            self.errors.push(ResolveError {
                kind: ResolveErrorKind::MissingModule {
                    module: self.entry_name.clone(),
                },
                source_name,
                span,
            });
        }

        if self.errors.is_empty() {
            Ok(ResolvedProgram {
                modules: self.modules,
                entry: entry.expect("entry checked above"),
                definitions: self.definitions,
                references: self.references,
                bindings: self.bindings,
                builtins: self.builtins,
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

    fn inject_builtins(&mut self) {
        for builtin in [
            Builtin::ConsoleWrite,
            Builtin::TextFormat,
            Builtin::Max,
            Builtin::Min,
            Builtin::Map,
            Builtin::Sum,
        ] {
            let qualified = builtin.qualified_name();
            let id = DefinitionId::new("builtin", "<builtin>", qualified);
            self.definitions.insert(
                id.clone(),
                DefinitionInfo {
                    id: id.clone(),
                    module_name: "<builtin>".to_owned(),
                    name: qualified.to_owned(),
                    kind: DefinitionKind::Builtin,
                    origin: DefinitionOrigin::Builtin(builtin),
                    mutable: false,
                    source_name: None,
                    span: None,
                },
            );
            self.builtins.insert(builtin, id);
        }
    }

    fn index_modules(&mut self) {
        for module in &self.modules {
            let name = module.hir.module.name.normalized();
            if self.module_names.insert(name.clone(), module.id).is_some() {
                self.errors.push(ResolveError {
                    kind: ResolveErrorKind::DuplicateModule { module: name },
                    source_name: module.hir.source_name.clone(),
                    span: module.hir.module.span,
                });
            }
        }
    }

    fn resolve_imports(&mut self) {
        let names = self.module_names.clone();
        let explicit = self
            .modules
            .iter()
            .map(|module| (module.id, module.hir.module.explicit))
            .collect::<BTreeMap<_, _>>();
        for module in &mut self.modules {
            let mut skeletons = BTreeMap::<String, String>::new();
            for import in &module.hir.imports {
                let imported_name = import.module.normalized();
                let Some(imported_id) = names.get(&imported_name).copied() else {
                    self.errors.push(ResolveError {
                        kind: ResolveErrorKind::MissingModule {
                            module: imported_name,
                        },
                        source_name: module.hir.source_name.clone(),
                        span: import.span,
                    });
                    continue;
                };
                if !explicit[&imported_id] {
                    self.errors.push(ResolveError {
                        kind: ResolveErrorKind::ImportedModuleMustBeExplicit {
                            module: imported_name,
                        },
                        source_name: module.hir.source_name.clone(),
                        span: import.span,
                    });
                }
                if module
                    .imports
                    .insert(import.alias.normalized.clone(), imported_id)
                    .is_some()
                {
                    self.errors.push(ResolveError {
                        kind: ResolveErrorKind::DuplicateImportAlias {
                            alias: import.alias.normalized.clone(),
                        },
                        source_name: module.hir.source_name.clone(),
                        span: import.alias.span,
                    });
                }
                check_confusable(
                    &mut skeletons,
                    &import.alias,
                    &module.hir.source_name,
                    &mut self.errors,
                );
            }
        }
    }

    fn detect_import_cycles(&mut self) {
        let mut state = vec![0_u8; self.modules.len()];
        let mut stack = Vec::new();
        for module in 0..self.modules.len() {
            self.visit_module(ModuleId(module as u32), &mut state, &mut stack);
        }
    }

    fn visit_module(&mut self, module: ModuleId, state: &mut [u8], stack: &mut Vec<ModuleId>) {
        let index = module.0 as usize;
        if state[index] == 2 {
            return;
        }
        if state[index] == 1 {
            if let Some(start) = stack.iter().position(|candidate| *candidate == module) {
                let mut cycle = stack[start..]
                    .iter()
                    .map(|id| self.modules[id.0 as usize].hir.module.name.normalized())
                    .collect::<Vec<_>>();
                cycle.push(self.modules[index].hir.module.name.normalized());
                self.errors.push(ResolveError {
                    kind: ResolveErrorKind::ImportCycle { modules: cycle },
                    source_name: self.modules[index].hir.source_name.clone(),
                    span: self.modules[index].hir.module.span,
                });
            }
            return;
        }
        state[index] = 1;
        stack.push(module);
        let dependencies = self.modules[index]
            .imports
            .values()
            .copied()
            .collect::<BTreeSet<_>>();
        for dependency in dependencies {
            self.visit_module(dependency, state, stack);
        }
        stack.pop();
        state[index] = 2;
    }

    fn index_definitions(&mut self) {
        let modules = self.modules.clone();
        for module in &modules {
            let module_name = module.hir.module.name.normalized();
            let mut scope = BTreeMap::new();
            let mut skeletons = BTreeMap::<String, String>::new();
            for definition in &module.hir.definitions {
                if definition.mutable {
                    self.errors.push(ResolveError {
                        kind: ResolveErrorKind::MutableTopLevel {
                            name: definition.name.normalized.clone(),
                        },
                        source_name: module.hir.source_name.clone(),
                        span: definition.span,
                    });
                }
                self.insert_user_definition(
                    module,
                    &module_name,
                    &definition.name,
                    DefinitionKind::Value,
                    definition.mutable,
                    &mut scope,
                    &mut skeletons,
                );
            }
            for declaration in &module.hir.types {
                self.insert_user_definition(
                    module,
                    &module_name,
                    &declaration.name,
                    DefinitionKind::Type,
                    false,
                    &mut scope,
                    &mut skeletons,
                );
                if let hir::TypeDefinition::Variant(cases) = &declaration.definition {
                    for case in cases {
                        self.insert_user_definition(
                            module,
                            &module_name,
                            &case.name,
                            DefinitionKind::Constructor,
                            false,
                            &mut scope,
                            &mut skeletons,
                        );
                    }
                }
            }
            self.module_definitions.insert(module.id, scope);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn insert_user_definition(
        &mut self,
        module: &ResolvedModule,
        module_name: &str,
        name: &hir::Name,
        kind: DefinitionKind,
        mutable: bool,
        scope: &mut BTreeMap<String, DefinitionId>,
        skeletons: &mut BTreeMap<String, String>,
    ) {
        if matches!(name.normalized.as_str(), "Console" | "Text") {
            self.errors.push(ResolveError {
                kind: ResolveErrorKind::ReservedName {
                    name: name.normalized.clone(),
                },
                source_name: module.hir.source_name.clone(),
                span: name.span,
            });
        }
        let kind_name = match kind {
            DefinitionKind::Value => "value",
            DefinitionKind::Type => "type",
            DefinitionKind::Constructor => "constructor",
            DefinitionKind::Builtin => "builtin",
        };
        let id = DefinitionId::new(kind_name, module_name, &name.normalized);
        if scope.insert(name.normalized.clone(), id.clone()).is_some() {
            self.errors.push(ResolveError {
                kind: ResolveErrorKind::DuplicateDefinition {
                    name: name.normalized.clone(),
                },
                source_name: module.hir.source_name.clone(),
                span: name.span,
            });
        }
        check_confusable(skeletons, name, &module.hir.source_name, &mut self.errors);
        self.definitions.insert(
            id.clone(),
            DefinitionInfo {
                id,
                module_name: module_name.to_owned(),
                name: name.normalized.clone(),
                kind,
                origin: DefinitionOrigin::User { module: module.id },
                mutable,
                source_name: Some(module.hir.source_name.clone()),
                span: Some(name.span),
            },
        );
    }

    fn resolve_bodies(&mut self) {
        for index in 0..self.modules.len() {
            let module_id = self.modules[index].id;
            let definitions = self.modules[index].hir.definitions.clone();
            for definition in &definitions {
                let mut scopes = vec![Scope::default()];
                for parameter in &definition.parameters {
                    self.bind_pattern(module_id, parameter, true, &mut scopes);
                }
                self.resolve_expression(module_id, &definition.value, &mut scopes);
            }
        }
    }

    fn resolve_expression(
        &mut self,
        module: ModuleId,
        expression: &hir::Expression,
        scopes: &mut Vec<Scope>,
    ) {
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => {
                scopes.push(Scope::default());
                for element in elements {
                    match element {
                        hir::SequenceElement::Let(binding) => {
                            if binding.recursive {
                                self.bind_local(module, binding, scopes);
                            }
                            scopes.push(Scope::default());
                            for parameter in &binding.parameters {
                                self.bind_pattern(module, parameter, true, scopes);
                            }
                            self.resolve_expression(module, &binding.value, scopes);
                            scopes.pop();
                            if !binding.recursive {
                                self.bind_local(module, binding, scopes);
                            }
                        }
                        hir::SequenceElement::Expression(expression) => {
                            self.resolve_expression(module, expression, scopes);
                        }
                    }
                }
                scopes.pop();
            }
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expression(module, condition, scopes);
                self.resolve_expression(module, then_branch, scopes);
                self.resolve_expression(module, else_branch, scopes);
            }
            hir::ExpressionKind::Match { scrutinee, cases } => {
                self.resolve_expression(module, scrutinee, scopes);
                for case in cases {
                    scopes.push(Scope::default());
                    self.bind_pattern(module, &case.pattern, true, scopes);
                    if let Some(guard) = &case.guard {
                        self.resolve_expression(module, guard, scopes);
                    }
                    self.resolve_expression(module, &case.body, scopes);
                    scopes.pop();
                }
            }
            hir::ExpressionKind::Assignment { place, value } => {
                self.resolve_reference(module, place.root_reference, &place.root, scopes);
                self.resolve_expression(module, value, scopes);
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => {
                self.resolve_expression(module, function, scopes);
                for argument in arguments {
                    self.resolve_expression(module, argument, scopes);
                }
            }
            hir::ExpressionKind::Projection {
                reference,
                target,
                field,
            } => {
                if let Some(segments) = qualified_segments(expression) {
                    if self.resolve_qualified(module, *reference, &segments) {
                        return;
                    }
                    if is_namespace_root(&segments[0].normalized, &self.modules[module.0 as usize])
                    {
                        self.undefined(
                            module,
                            expression.span,
                            segments
                                .iter()
                                .map(|segment| segment.normalized.as_str())
                                .collect::<Vec<_>>()
                                .join("."),
                        );
                        return;
                    }
                }
                self.resolve_expression(module, target, scopes);
                let _ = field;
            }
            hir::ExpressionKind::Name { reference, name } => {
                self.resolve_reference(module, *reference, name, scopes);
            }
            hir::ExpressionKind::Binary { left, right, .. } => {
                self.resolve_expression(module, left, scopes);
                self.resolve_expression(module, right, scopes);
            }
            hir::ExpressionKind::Unary { operand, .. } => {
                self.resolve_expression(module, operand, scopes);
            }
            hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
                for element in elements {
                    self.resolve_expression(module, element, scopes);
                }
            }
            hir::ExpressionKind::Record(fields) => {
                for field in fields {
                    self.resolve_expression(module, &field.value, scopes);
                }
            }
            hir::ExpressionKind::RecordUpdate { base, fields } => {
                self.resolve_expression(module, base, scopes);
                for field in fields {
                    self.resolve_expression(module, &field.value, scopes);
                }
            }
            hir::ExpressionKind::Literal(_) | hir::ExpressionKind::Unit => {}
        }
    }

    fn resolve_reference(
        &mut self,
        module: ModuleId,
        reference: ReferenceId,
        name: &hir::Name,
        scopes: &[Scope],
    ) {
        for scope in scopes.iter().rev() {
            if let Some(binding) = scope.names.get(&name.normalized) {
                self.references.insert(
                    ReferenceKey::new(module, reference),
                    ReferenceTarget::Binding(*binding),
                );
                return;
            }
        }
        if let Some(id) = self
            .module_definitions
            .get(&module)
            .and_then(|definitions| definitions.get(&name.normalized))
        {
            self.references.insert(
                ReferenceKey::new(module, reference),
                ReferenceTarget::Definition(id.clone()),
            );
            return;
        }
        let builtin = match name.normalized.as_str() {
            "max" => Some(Builtin::Max),
            "min" => Some(Builtin::Min),
            "map" => Some(Builtin::Map),
            "sum" => Some(Builtin::Sum),
            _ => None,
        };
        if let Some(builtin) = builtin {
            self.references.insert(
                ReferenceKey::new(module, reference),
                ReferenceTarget::Definition(self.builtins[&builtin].clone()),
            );
        } else {
            self.undefined(module, name.span, name.normalized.clone());
        }
    }

    fn resolve_qualified(
        &mut self,
        module: ModuleId,
        reference: ReferenceId,
        segments: &[&hir::Name],
    ) -> bool {
        let qualified = segments
            .iter()
            .map(|segment| segment.normalized.as_str())
            .collect::<Vec<_>>()
            .join(".");
        let builtin = match qualified.as_str() {
            "Console.write" => Some(Builtin::ConsoleWrite),
            "Text.format" => Some(Builtin::TextFormat),
            _ => None,
        };
        if let Some(builtin) = builtin {
            self.references.insert(
                ReferenceKey::new(module, reference),
                ReferenceTarget::Definition(self.builtins[&builtin].clone()),
            );
            return true;
        }
        if segments.len() == 2 {
            let current = &self.modules[module.0 as usize];
            if let Some(imported) = current.imports.get(&segments[0].normalized) {
                if let Some(id) = self
                    .module_definitions
                    .get(imported)
                    .and_then(|definitions| definitions.get(&segments[1].normalized))
                {
                    self.references.insert(
                        ReferenceKey::new(module, reference),
                        ReferenceTarget::Definition(id.clone()),
                    );
                    return true;
                }
            }
        }
        false
    }

    fn bind_pattern(
        &mut self,
        module: ModuleId,
        pattern: &hir::Pattern,
        parameter: bool,
        scopes: &mut [Scope],
    ) {
        match &pattern.kind {
            hir::PatternKind::Binding { id, name } => {
                self.bind_name(module, *id, name, false, parameter, scopes);
            }
            hir::PatternKind::Tuple(elements) => {
                for element in elements {
                    self.bind_pattern(module, element, parameter, scopes);
                }
            }
            hir::PatternKind::Constructor { arguments, .. } => {
                for argument in arguments {
                    self.bind_pattern(module, argument, parameter, scopes);
                }
            }
            hir::PatternKind::Unit | hir::PatternKind::Literal(_) => {}
        }
    }

    fn bind_local(&mut self, module: ModuleId, binding: &hir::LocalBinding, scopes: &mut [Scope]) {
        self.bind_name(
            module,
            binding.id,
            &binding.name,
            binding.mutable,
            false,
            scopes,
        );
    }

    fn bind_name(
        &mut self,
        module: ModuleId,
        id: BindingId,
        name: &hir::Name,
        mutable: bool,
        parameter: bool,
        scopes: &mut [Scope],
    ) {
        let scope = scopes.last_mut().expect("resolver always has a scope");
        let key = BindingKey::new(module, id);
        if scope.names.insert(name.normalized.clone(), key).is_some() {
            self.errors.push(ResolveError {
                kind: ResolveErrorKind::DuplicateDefinition {
                    name: name.normalized.clone(),
                },
                source_name: self.modules[module.0 as usize].hir.source_name.clone(),
                span: name.span,
            });
        }
        check_confusable(
            &mut scope.skeletons,
            name,
            &self.modules[module.0 as usize].hir.source_name,
            &mut self.errors,
        );
        self.bindings.insert(
            key,
            BindingInfo {
                key,
                name: name.normalized.clone(),
                mutable,
                parameter,
                span: name.span,
            },
        );
    }

    fn undefined(&mut self, module: ModuleId, span: Span, name: String) {
        self.errors.push(ResolveError {
            kind: ResolveErrorKind::UndefinedName { name },
            source_name: self.modules[module.0 as usize].hir.source_name.clone(),
            span,
        });
    }
}

#[derive(Default)]
struct Scope {
    names: BTreeMap<String, BindingKey>,
    skeletons: BTreeMap<String, String>,
}

fn check_confusable(
    skeletons: &mut BTreeMap<String, String>,
    name: &hir::Name,
    source_name: &str,
    errors: &mut Vec<ResolveError>,
) {
    if let Some(first) = skeletons.get(&name.skeleton) {
        if first != &name.normalized {
            errors.push(ResolveError {
                kind: ResolveErrorKind::ConfusableCollision {
                    first: first.clone(),
                    second: name.normalized.clone(),
                },
                source_name: source_name.to_owned(),
                span: name.span,
            });
        }
    } else {
        skeletons.insert(name.skeleton.clone(), name.normalized.clone());
    }
}

fn qualified_segments(expression: &hir::Expression) -> Option<Vec<&hir::Name>> {
    fn visit<'expression>(
        expression: &'expression hir::Expression,
        output: &mut Vec<&'expression hir::Name>,
    ) -> bool {
        match &expression.kind {
            hir::ExpressionKind::Name { name, .. } => {
                output.push(name);
                true
            }
            hir::ExpressionKind::Projection { target, field, .. } => {
                let qualified = visit(target, output);
                if qualified {
                    output.push(field);
                }
                qualified
            }
            _ => false,
        }
    }

    let mut segments = Vec::new();
    visit(expression, &mut segments).then_some(segments)
}

fn is_namespace_root(root: &str, module: &ResolvedModule) -> bool {
    matches!(root, "Console" | "Text") || module.imports.contains_key(root)
}

fn encode_part(output: &mut Vec<u8>, bytes: &[u8]) {
    output.extend_from_slice(&u32::try_from(bytes.len()).unwrap_or(u32::MAX).to_be_bytes());
    output.extend_from_slice(bytes);
}

fn empty_span() -> Span {
    Span::new(
        ling_source::SourceId::new(0),
        ling_source::ByteOffset::new(0),
        ling_source::ByteOffset::new(0),
    )
    .expect("zero-width span is valid")
}

#[cfg(test)]
mod tests {
    use ling_ast::lower as lower_ast;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;

    use super::*;

    fn hir_program(source_id: u32, name: &str, text: &str) -> hir::Program {
        let source =
            SourceFile::from_bytes(SourceId::new(source_id), name, text.as_bytes().to_vec())
                .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        hir::lower(source.name(), &ast).expect("valid HIR")
    }

    #[test]
    fn resolves_hello_world_to_builtin_and_parameter_bindings() {
        let program = hir_program(
            0,
            "hello.ling",
            "module Main\n    requires Console.Write\n\nlet main () =\n    Console.write \"你好，零\"\n",
        );
        let resolved = resolve(vec![program], "Main").expect("hello resolves");
        assert_eq!(resolved.entry_module().hir.module.name.normalized(), "Main");
        assert!(resolved.references().values().any(|target| {
            matches!(target, ReferenceTarget::Definition(id) if id == resolved.builtin_id(Builtin::ConsoleWrite))
        }));
    }

    #[test]
    fn resolves_import_alias_members() {
        let main = hir_program(
            0,
            "Main.ling",
            "module Main\n\nimport Game.Math as M\n\nlet value = M.answer\n",
        );
        let math = hir_program(1, "Game/Math.ling", "module Game.Math\n\nlet answer = 42\n");
        let resolved = resolve(vec![main, math], "Main").expect("import resolves");
        assert_eq!(resolved.modules().len(), 2);
    }

    #[test]
    fn rejects_undefined_names_and_import_cycles() {
        let undefined = hir_program(0, "Main.ling", "module Main\n\nlet value = missing\n");
        let errors = resolve(vec![undefined], "Main").expect_err("name must be rejected");
        assert!(matches!(
            errors[0].kind,
            ResolveErrorKind::UndefinedName { .. }
        ));

        let first = hir_program(1, "A.ling", "module A\n\nimport B\n\nlet a = 1\n");
        let second = hir_program(2, "B.ling", "module B\n\nimport A\n\nlet b = 2\n");
        let errors = resolve(vec![first, second], "A").expect_err("cycle must be rejected");
        assert!(
            errors
                .iter()
                .any(|error| matches!(error.kind, ResolveErrorKind::ImportCycle { .. }))
        );
    }
}
