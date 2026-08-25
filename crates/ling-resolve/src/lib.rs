//! Deterministic module and lexical name resolution for Ling Seed.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_hir::{self as hir, BindingId, ExpressionId, PatternId, ReferenceId};
use ling_project::{
    PackageGraph, PackageGraphId, PackageIdentity, PackageName, QualifiedModuleName,
};
use ling_source::Span;

const LANGUAGE_VERSION: &str = "0.0.1-dev";
const SEED_SEMANTIC_SCHEMA: &str = "ling.semantic/0.1";
const PROJECT_SEMANTIC_SCHEMA: &str = "ling.semantic/0.2";

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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PatternKey {
    module: ModuleId,
    local: PatternId,
}

impl PatternKey {
    #[must_use]
    pub const fn new(module: ModuleId, local: PatternId) -> Self {
        Self { module, local }
    }

    #[must_use]
    pub const fn module(self) -> ModuleId {
        self.module
    }

    #[must_use]
    pub const fn local(self) -> PatternId {
        self.local
    }
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
        encode_part(&mut bytes, SEED_SEMANTIC_SCHEMA.as_bytes());
        encode_part(&mut bytes, kind.as_bytes());
        encode_part(&mut bytes, module.as_bytes());
        encode_part(&mut bytes, name.as_bytes());
        Self(format!(
            "experimental:blake3:{}",
            blake3::hash(&bytes).to_hex()
        ))
    }

    fn new_repl(kind: &str, module: &str, name: &str, generation: u64) -> Self {
        let mut bytes = Vec::new();
        encode_part(&mut bytes, b"ling.repl-definition-id/v1");
        encode_part(&mut bytes, LANGUAGE_VERSION.as_bytes());
        encode_part(&mut bytes, SEED_SEMANTIC_SCHEMA.as_bytes());
        encode_part(&mut bytes, kind.as_bytes());
        encode_part(&mut bytes, module.as_bytes());
        encode_part(&mut bytes, name.as_bytes());
        encode_part(&mut bytes, &generation.to_be_bytes());
        Self(format!(
            "experimental:blake3:{}",
            blake3::hash(&bytes).to_hex()
        ))
    }

    fn new_project(
        kind: &str,
        package: Option<&PackageIdentity>,
        module: &str,
        name: &str,
    ) -> Self {
        let mut bytes = Vec::new();
        encode_part(&mut bytes, b"ling.definition-id/v2");
        encode_part(&mut bytes, LANGUAGE_VERSION.as_bytes());
        encode_part(&mut bytes, PROJECT_SEMANTIC_SCHEMA.as_bytes());
        match package {
            Some(package) => {
                encode_part(&mut bytes, b"package");
                encode_package_identity(&mut bytes, package);
            }
            None => encode_part(&mut bytes, b"system"),
        }
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

pub const PRELUDE_MODULE: &str = "Ling.Prelude";

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PreludeDefinition {
    Option,
    Some,
    None,
    Result,
    Ok,
    Error,
}

impl PreludeDefinition {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Option => "Option",
            Self::Some => "Some",
            Self::None => "None",
            Self::Result => "Result",
            Self::Ok => "Ok",
            Self::Error => "Error",
        }
    }

    #[must_use]
    pub const fn kind(self) -> DefinitionKind {
        match self {
            Self::Option | Self::Result => DefinitionKind::Type,
            Self::Some | Self::None | Self::Ok | Self::Error => DefinitionKind::Constructor,
        }
    }
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

/// Primitive types used by the fixed Experimental handler-operation registry.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HandlerValueType {
    Unit,
    Int,
    Text,
}

/// Static continuation cardinality for a checked handler operation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HandlerResumeMode {
    Never,
    Once,
    Many,
}

/// One compiler-owned operation signature accepted by DEC-0260.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResolvedHandlerOperation {
    source_name: &'static str,
    label: &'static str,
    owner: &'static str,
    operation: &'static str,
    inputs: &'static [HandlerValueType],
    output: HandlerValueType,
    resume_mode: HandlerResumeMode,
}

impl ResolvedHandlerOperation {
    #[must_use]
    pub const fn source_name(self) -> &'static str {
        self.source_name
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        self.label
    }

    #[must_use]
    pub const fn owner(self) -> &'static str {
        self.owner
    }

    #[must_use]
    pub const fn operation(self) -> &'static str {
        self.operation
    }

    #[must_use]
    pub const fn inputs(self) -> &'static [HandlerValueType] {
        self.inputs
    }

    #[must_use]
    pub const fn output(self) -> HandlerValueType {
        self.output
    }

    #[must_use]
    pub const fn resume_mode(self) -> HandlerResumeMode {
        self.resume_mode
    }
}

const TEXT_INPUT: &[HandlerValueType] = &[HandlerValueType::Text];
const INT_INPUT: &[HandlerValueType] = &[HandlerValueType::Int];

/// Resolves a DEC-0260 operation without adding it to the value namespace.
#[must_use]
pub fn resolve_handler_operation(name: &str) -> Option<ResolvedHandlerOperation> {
    match name {
        "Console.Write.write" => Some(ResolvedHandlerOperation {
            source_name: "Console.Write.write",
            label: "Console.Write",
            owner: "Console.Write",
            operation: "write",
            inputs: TEXT_INPUT,
            output: HandlerValueType::Unit,
            resume_mode: HandlerResumeMode::Once,
        }),
        "Clock.now" => Some(ResolvedHandlerOperation {
            source_name: "Clock.now",
            label: "Clock",
            owner: "Clock",
            operation: "now",
            inputs: &[],
            output: HandlerValueType::Int,
            resume_mode: HandlerResumeMode::Once,
        }),
        "Random.next" => Some(ResolvedHandlerOperation {
            source_name: "Random.next",
            label: "Random",
            owner: "Random",
            operation: "next",
            inputs: INT_INPUT,
            output: HandlerValueType::Int,
            resume_mode: HandlerResumeMode::Many,
        }),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefinitionKind {
    Value,
    Task,
    Type,
    Constructor,
    Builtin,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefinitionOrigin {
    User { module: ModuleId },
    Builtin(Builtin),
    Prelude(PreludeDefinition),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionInfo {
    pub id: DefinitionId,
    pub package: Option<PackageIdentity>,
    pub module_name: String,
    pub name: String,
    pub name_source: String,
    pub name_skeleton: String,
    pub name_scripts: Vec<String>,
    pub name_suspicious_mixed_script: bool,
    pub kind: DefinitionKind,
    pub origin: DefinitionOrigin,
    pub mutable: bool,
    pub source_name: Option<String>,
    pub span: Option<Span>,
}

/// Resolved identity and signature metadata for a declared Trait member.
///
/// Trait members are not inserted into ordinary module value scope. They are
/// indexed separately so a qualified `Trait.member` expression can retain a
/// stable definition identity without pretending that the member is a free
/// top-level function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraitMemberInfo {
    pub definition: DefinitionId,
    pub module: ModuleId,
    pub trait_name: String,
    pub member_name: String,
    pub ordinal: usize,
    pub signature: hir::TypeSyntax,
    pub source_name: String,
    pub span: Span,
}

/// Resolved identity metadata for an implementation member body.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImplMemberInfo {
    pub definition: DefinitionId,
    pub module: ModuleId,
    pub impl_ordinal: usize,
    pub member_ordinal: usize,
    pub trait_name: String,
    pub member_name: String,
    pub source_name: String,
    pub span: Span,
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
    pub package: Option<PackageIdentity>,
    pub hir: hir::Program,
    pub imports: BTreeMap<String, ModuleId>,
}

/// Lightweight package metadata retained by a package-aware resolved program.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedProjectPackage {
    identity: PackageIdentity,
    entry: QualifiedModuleName,
    exports: Box<[QualifiedModuleName]>,
}

impl ResolvedProjectPackage {
    #[must_use]
    pub const fn identity(&self) -> &PackageIdentity {
        &self.identity
    }

    #[must_use]
    pub const fn entry(&self) -> &QualifiedModuleName {
        &self.entry
    }

    #[must_use]
    pub fn exports(&self) -> &[QualifiedModuleName] {
        &self.exports
    }
}

/// Path-free package context used by package-aware semantic identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedProject {
    graph_id: PackageGraphId,
    root: PackageIdentity,
    packages: Box<[ResolvedProjectPackage]>,
    dependencies: BTreeMap<(PackageIdentity, PackageName), PackageIdentity>,
}

impl ResolvedProject {
    #[must_use]
    pub const fn graph_id(&self) -> &PackageGraphId {
        &self.graph_id
    }

    #[must_use]
    pub const fn root(&self) -> &PackageIdentity {
        &self.root
    }

    #[must_use]
    pub fn packages(&self) -> &[ResolvedProjectPackage] {
        &self.packages
    }

    #[must_use]
    pub fn dependency(
        &self,
        package: &PackageIdentity,
        name: &PackageName,
    ) -> Option<&PackageIdentity> {
        self.dependencies.get(&(package.clone(), name.clone()))
    }
}

/// HIR modules belonging to one exact package identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackagePrograms {
    identity: PackageIdentity,
    programs: Vec<hir::Program>,
}

impl PackagePrograms {
    #[must_use]
    pub fn new(identity: PackageIdentity, programs: Vec<hir::Program>) -> Self {
        Self { identity, programs }
    }

    #[must_use]
    pub const fn identity(&self) -> &PackageIdentity {
        &self.identity
    }

    #[must_use]
    pub fn programs(&self) -> &[hir::Program] {
        &self.programs
    }
}

/// Structural mismatch between a validated package graph and supplied HIR.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectInputError {
    pub reason: &'static str,
    pub package: Option<String>,
    pub module: Option<String>,
}

impl fmt::Display for ProjectInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid project resolver input: {}", self.reason)
    }
}

impl Error for ProjectInputError {}

#[derive(Debug)]
pub enum ProjectResolveFailure {
    Input(ProjectInputError),
    Resolution(Vec<ResolveError>),
}

impl fmt::Display for ProjectResolveFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(error) => error.fmt(formatter),
            Self::Resolution(errors) => {
                write!(
                    formatter,
                    "project resolution produced {} error(s)",
                    errors.len()
                )
            }
        }
    }
}

impl Error for ProjectResolveFailure {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedProgram {
    modules: Vec<ResolvedModule>,
    entry: ModuleId,
    definitions: BTreeMap<DefinitionId, DefinitionInfo>,
    references: BTreeMap<ReferenceKey, ReferenceTarget>,
    bindings: BTreeMap<BindingKey, BindingInfo>,
    handler_resume_uses: BTreeMap<BindingKey, usize>,
    pattern_constructors: BTreeMap<PatternKey, DefinitionId>,
    builtins: BTreeMap<Builtin, DefinitionId>,
    prelude: BTreeMap<PreludeDefinition, DefinitionId>,
    trait_members: BTreeMap<DefinitionId, TraitMemberInfo>,
    impl_members: BTreeMap<DefinitionId, ImplMemberInfo>,
    project: Option<ResolvedProject>,
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
    pub fn trait_member(&self, id: &DefinitionId) -> Option<&TraitMemberInfo> {
        self.trait_members.get(id)
    }

    #[must_use]
    pub fn trait_members(&self) -> &BTreeMap<DefinitionId, TraitMemberInfo> {
        &self.trait_members
    }

    #[must_use]
    pub fn impl_member(&self, id: &DefinitionId) -> Option<&ImplMemberInfo> {
        self.impl_members.get(id)
    }

    #[must_use]
    pub fn impl_members(&self) -> &BTreeMap<DefinitionId, ImplMemberInfo> {
        &self.impl_members
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
    pub fn handler_resume_uses(&self, binding: BindingKey) -> Option<usize> {
        self.handler_resume_uses.get(&binding).copied()
    }

    #[must_use]
    pub fn pattern_constructor(
        &self,
        module: ModuleId,
        pattern: PatternId,
    ) -> Option<&DefinitionId> {
        self.pattern_constructors
            .get(&PatternKey::new(module, pattern))
    }

    #[must_use]
    pub fn pattern_constructors(&self) -> &BTreeMap<PatternKey, DefinitionId> {
        &self.pattern_constructors
    }

    #[must_use]
    pub fn builtin_id(&self, builtin: Builtin) -> &DefinitionId {
        self.builtins
            .get(&builtin)
            .expect("all Seed builtins are injected")
    }

    #[must_use]
    pub fn prelude_id(&self, definition: PreludeDefinition) -> &DefinitionId {
        self.prelude
            .get(&definition)
            .expect("all Seed Prelude definitions are injected")
    }

    #[must_use]
    pub fn prelude_definition(&self, name: &str) -> Option<&DefinitionId> {
        self.prelude
            .iter()
            .find_map(|(definition, id)| (definition.name() == name).then_some(id))
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

    #[must_use]
    pub const fn project(&self) -> Option<&ResolvedProject> {
        self.project.as_ref()
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
    UndefinedName {
        name: String,
    },
    DuplicateDefinition {
        name: String,
    },
    DuplicateModule {
        module: String,
    },
    DuplicateImportAlias {
        alias: String,
    },
    MissingModule {
        module: String,
    },
    ImportedModuleMustBeExplicit {
        module: String,
    },
    ImportCycle {
        modules: Vec<String>,
    },
    ConfusableCollision {
        first: String,
        second: String,
    },
    SuspiciousMixedScript {
        name: String,
        scripts: Vec<String>,
    },
    ReservedName {
        name: String,
    },
    MutableTopLevel {
        name: String,
    },
    UnsupportedHandler,
    InvalidHandlerContract {
        operation: String,
        reason: &'static str,
        expected: Option<String>,
        actual: Option<String>,
    },
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
            ResolveErrorKind::SuspiciousMixedScript { name, scripts } => (
                codes::SUSPICIOUS_MIXED_SCRIPT,
                format!(
                    "标识符“{name}”包含 Seed 默认拒绝的可疑混合文字：{}",
                    scripts.join(", ")
                ),
                format!(
                    "identifier `{name}` contains a suspicious script mix rejected by Ling Seed: {}",
                    scripts.join(", ")
                ),
            ),
            ResolveErrorKind::ReservedName { name } => (
                codes::RESERVED_NAME,
                format!("模块作用域不能重定义保留名称“{name}”"),
                format!("module scope cannot redefine reserved name `{name}`"),
            ),
            ResolveErrorKind::MutableTopLevel { name } => (
                codes::INVALID_MODULE,
                format!("Seed 不允许顶层 mutable binding“{name}”"),
                format!("Ling Seed does not allow mutable top-level binding `{name}`"),
            ),
            ResolveErrorKind::UnsupportedHandler => (
                codes::UNSUPPORTED_HANDLER,
                "Handler 尚未具备已检查语义".to_owned(),
                "handler does not yet have checked semantics".to_owned(),
            ),
            ResolveErrorKind::InvalidHandlerContract { reason, .. } => (
                codes::INVALID_HANDLER_CONTRACT,
                format!("Handler clause contract 无效：{reason}"),
                format!("handler clause contract is invalid: {reason}"),
            ),
        };
        let diagnostic = Diagnostic::new(code, Severity::Error, zh, en)
            .with_primary_span(DiagnosticSpan::new(&self.source_name, self.span));
        match &self.kind {
            ResolveErrorKind::SuspiciousMixedScript { name, scripts } => diagnostic
                .with_fact("name", name.clone())
                .with_fact("scripts", scripts.clone()),
            ResolveErrorKind::InvalidHandlerContract {
                operation,
                reason,
                expected,
                actual,
            } => {
                let mut diagnostic = diagnostic
                    .with_fact("operation", operation.clone())
                    .with_fact("reason", (*reason).to_owned());
                if let Some(expected) = expected {
                    diagnostic = diagnostic.with_fact("expected", expected.clone());
                }
                if let Some(actual) = actual {
                    diagnostic = diagnostic.with_fact("actual", actual.clone());
                }
                diagnostic
            }
            _ => diagnostic,
        }
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
    Resolver::new_seed(programs, entry_module_name).run()
}

/// Resolves exact HIR modules against a validated local package graph.
///
/// The supplied package/module set must exactly match `graph`. This keeps
/// parsing and HIR lowering in their owning pipeline layers while ensuring
/// package-aware resolution cannot silently omit or inject a module.
pub fn resolve_project(
    graph: &PackageGraph,
    mut packages: Vec<PackagePrograms>,
) -> Result<ResolvedProgram, ProjectResolveFailure> {
    let expected = graph
        .packages()
        .iter()
        .map(|package| (package.identity().clone(), package))
        .collect::<BTreeMap<_, _>>();
    let mut supplied = BTreeMap::<PackageIdentity, Vec<hir::Program>>::new();
    packages.sort_by(|left, right| left.identity.cmp(&right.identity));
    for package in packages {
        if !expected.contains_key(&package.identity) {
            return Err(project_input_error(
                "unexpected_package",
                Some(package.identity.name().as_str()),
                None,
            ));
        }
        let package_name = package.identity.name().as_str().to_owned();
        if supplied
            .insert(package.identity, package.programs)
            .is_some()
        {
            return Err(project_input_error(
                "duplicate_package",
                Some(&package_name),
                None,
            ));
        }
    }

    let mut inputs = Vec::new();
    for (identity, resolved_package) in &expected {
        let Some(mut programs) = supplied.remove(identity) else {
            return Err(project_input_error(
                "missing_package",
                Some(identity.name().as_str()),
                None,
            ));
        };
        programs.sort_by_key(|program| program.module.name.normalized());
        let expected_modules = resolved_package
            .modules()
            .nodes()
            .iter()
            .map(|node| node.name().as_str().to_owned())
            .collect::<BTreeSet<_>>();
        let mut actual_modules = BTreeSet::new();
        for program in programs {
            let module = program.module.name.normalized();
            if !actual_modules.insert(module.clone()) {
                return Err(project_input_error(
                    "duplicate_module",
                    Some(identity.name().as_str()),
                    Some(&module),
                ));
            }
            if !expected_modules.contains(&module) {
                return Err(project_input_error(
                    "unexpected_module",
                    Some(identity.name().as_str()),
                    Some(&module),
                ));
            }
            inputs.push(ModuleInput {
                package: Some(identity.clone()),
                hir: program,
            });
        }
        if let Some(missing) = expected_modules.difference(&actual_modules).next() {
            return Err(project_input_error(
                "missing_module",
                Some(identity.name().as_str()),
                Some(missing),
            ));
        }
    }

    Resolver::new_project(inputs, ResolvedProject::from_graph(graph))
        .run()
        .map_err(ProjectResolveFailure::Resolution)
}

fn project_input_error(
    reason: &'static str,
    package: Option<&str>,
    module: Option<&str>,
) -> ProjectResolveFailure {
    ProjectResolveFailure::Input(ProjectInputError {
        reason,
        package: package.map(str::to_owned),
        module: module.map(str::to_owned),
    })
}

impl ResolvedProject {
    fn from_graph(graph: &PackageGraph) -> Self {
        Self {
            graph_id: graph.id().clone(),
            root: graph.root().clone(),
            packages: graph
                .packages()
                .iter()
                .map(|package| ResolvedProjectPackage {
                    identity: package.identity().clone(),
                    entry: package.entry().clone(),
                    exports: package.exports().to_vec().into_boxed_slice(),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            dependencies: graph
                .edges()
                .iter()
                .map(|edge| {
                    (
                        (edge.from().clone(), edge.dependency().clone()),
                        edge.to().clone(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ModuleKey {
    package: Option<PackageIdentity>,
    name: String,
}

struct ModuleInput {
    package: Option<PackageIdentity>,
    hir: hir::Program,
}

struct Resolver {
    modules: Vec<ResolvedModule>,
    entry: ModuleKey,
    project: Option<ResolvedProject>,
    module_names: BTreeMap<ModuleKey, ModuleId>,
    definitions: BTreeMap<DefinitionId, DefinitionInfo>,
    module_definitions: BTreeMap<ModuleId, BTreeMap<String, DefinitionId>>,
    references: BTreeMap<ReferenceKey, ReferenceTarget>,
    bindings: BTreeMap<BindingKey, BindingInfo>,
    handler_resume_uses: BTreeMap<BindingKey, usize>,
    pattern_constructors: BTreeMap<PatternKey, DefinitionId>,
    builtins: BTreeMap<Builtin, DefinitionId>,
    prelude: BTreeMap<PreludeDefinition, DefinitionId>,
    trait_members: BTreeMap<DefinitionId, TraitMemberInfo>,
    impl_members: BTreeMap<DefinitionId, ImplMemberInfo>,
    errors: Vec<ResolveError>,
}

impl Resolver {
    fn new_seed(programs: Vec<hir::Program>, entry_name: &str) -> Self {
        let inputs = programs
            .into_iter()
            .map(|hir| ModuleInput { package: None, hir })
            .collect();
        Self::new(
            inputs,
            ModuleKey {
                package: None,
                name: entry_name.to_owned(),
            },
            None,
        )
    }

    fn new_project(inputs: Vec<ModuleInput>, project: ResolvedProject) -> Self {
        let root = project.root.clone();
        let entry = project
            .packages
            .iter()
            .find(|package| package.identity == root)
            .expect("validated package graph contains its root")
            .entry
            .as_str()
            .to_owned();
        Self::new(
            inputs,
            ModuleKey {
                package: Some(root),
                name: entry,
            },
            Some(project),
        )
    }

    fn new(
        mut inputs: Vec<ModuleInput>,
        entry: ModuleKey,
        project: Option<ResolvedProject>,
    ) -> Self {
        inputs.sort_by(|left, right| {
            ModuleKey {
                package: left.package.clone(),
                name: left.hir.module.name.normalized(),
            }
            .cmp(&ModuleKey {
                package: right.package.clone(),
                name: right.hir.module.name.normalized(),
            })
        });
        let modules = inputs
            .into_iter()
            .enumerate()
            .map(|(index, input)| ResolvedModule {
                id: ModuleId(u32::try_from(index).unwrap_or(u32::MAX)),
                package: input.package,
                hir: input.hir,
                imports: BTreeMap::new(),
            })
            .collect();
        Self {
            modules,
            entry,
            project,
            module_names: BTreeMap::new(),
            definitions: BTreeMap::new(),
            module_definitions: BTreeMap::new(),
            references: BTreeMap::new(),
            bindings: BTreeMap::new(),
            handler_resume_uses: BTreeMap::new(),
            pattern_constructors: BTreeMap::new(),
            builtins: BTreeMap::new(),
            prelude: BTreeMap::new(),
            trait_members: BTreeMap::new(),
            impl_members: BTreeMap::new(),
            errors: Vec::new(),
        }
    }

    fn run(mut self) -> Result<ResolvedProgram, Vec<ResolveError>> {
        self.inject_builtins();
        self.inject_prelude();
        self.index_modules();
        self.resolve_imports();
        self.detect_import_cycles();
        self.index_definitions();
        self.resolve_bodies();

        let entry = self.module_names.get(&self.entry).copied();
        if entry.is_none() {
            let (source_name, span) = self.modules.first().map_or_else(
                || (String::new(), empty_span()),
                |module| (module.hir.source_name.clone(), module.hir.span),
            );
            self.errors.push(ResolveError {
                kind: ResolveErrorKind::MissingModule {
                    module: self.entry.name.clone(),
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
                handler_resume_uses: self.handler_resume_uses,
                pattern_constructors: self.pattern_constructors,
                builtins: self.builtins,
                prelude: self.prelude,
                trait_members: self.trait_members,
                impl_members: self.impl_members,
                project: self.project,
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
            let id = self.make_definition_id("builtin", None, "<builtin>", qualified);
            self.definitions.insert(
                id.clone(),
                DefinitionInfo {
                    id: id.clone(),
                    package: None,
                    module_name: "<builtin>".to_owned(),
                    name: qualified.to_owned(),
                    name_source: qualified.to_owned(),
                    name_skeleton: qualified.to_owned(),
                    name_scripts: Vec::new(),
                    name_suspicious_mixed_script: false,
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

    fn inject_prelude(&mut self) {
        for definition in [
            PreludeDefinition::Option,
            PreludeDefinition::Some,
            PreludeDefinition::None,
            PreludeDefinition::Result,
            PreludeDefinition::Ok,
            PreludeDefinition::Error,
        ] {
            let kind = definition.kind();
            let kind_name = match kind {
                DefinitionKind::Type => "type",
                DefinitionKind::Constructor => "constructor",
                DefinitionKind::Value | DefinitionKind::Task | DefinitionKind::Builtin => {
                    unreachable!("Prelude only contains types and constructors")
                }
            };
            let id = self.make_definition_id(kind_name, None, PRELUDE_MODULE, definition.name());
            self.definitions.insert(
                id.clone(),
                DefinitionInfo {
                    id: id.clone(),
                    package: None,
                    module_name: PRELUDE_MODULE.to_owned(),
                    name: definition.name().to_owned(),
                    name_source: definition.name().to_owned(),
                    name_skeleton: definition.name().to_owned(),
                    name_scripts: vec!["Latn".to_owned()],
                    name_suspicious_mixed_script: false,
                    kind,
                    origin: DefinitionOrigin::Prelude(definition),
                    mutable: false,
                    source_name: None,
                    span: None,
                },
            );
            self.prelude.insert(definition, id);
        }
    }

    fn prelude_definition(&self, name: &str) -> Option<&DefinitionId> {
        self.prelude
            .iter()
            .find_map(|(definition, id)| (definition.name() == name).then_some(id))
    }

    fn index_modules(&mut self) {
        let mut skeletons = BTreeMap::<Option<PackageIdentity>, BTreeMap<String, String>>::new();
        for module in &self.modules {
            let name = module.hir.module.name.normalized();
            let key = ModuleKey {
                package: module.package.clone(),
                name: name.clone(),
            };
            if self.module_names.insert(key, module.id).is_some() {
                self.errors.push(ResolveError {
                    kind: ResolveErrorKind::DuplicateModule { module: name },
                    source_name: module.hir.source_name.clone(),
                    span: module.hir.module.span,
                });
            }
            check_qualified_name_security(
                skeletons.entry(module.package.clone()).or_default(),
                &module.hir.module.name,
                &module.hir.source_name,
                &mut self.errors,
            );
        }
    }

    fn resolve_imports(&mut self) {
        let names = self.module_names.clone();
        let project = self.project.clone();
        let explicit = self
            .modules
            .iter()
            .map(|module| (module.id, module.hir.module.explicit))
            .collect::<BTreeMap<_, _>>();
        for module in &mut self.modules {
            let mut skeletons = BTreeMap::<String, String>::new();
            for import in &module.hir.imports {
                let imported_name = import.module.normalized();
                let key = imported_module_key(project.as_ref(), module, &imported_name);
                let Some(imported_id) = names.get(&key).copied() else {
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
                check_name_security(
                    &mut skeletons,
                    &import.alias,
                    &module.hir.source_name,
                    &mut self.errors,
                );
            }
        }
    }

    fn make_definition_id(
        &self,
        kind: &str,
        package: Option<&PackageIdentity>,
        module: &str,
        name: &str,
    ) -> DefinitionId {
        if self.project.is_some() {
            DefinitionId::new_project(kind, package, module, name)
        } else {
            DefinitionId::new(kind, module, name)
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
                    definition.session_generation,
                    &mut scope,
                    &mut skeletons,
                );
            }
            for declaration in &module.hir.tasks {
                self.insert_user_definition(
                    module,
                    &module_name,
                    &declaration.name,
                    DefinitionKind::Task,
                    false,
                    None,
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
                    None,
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
                            None,
                            &mut scope,
                            &mut skeletons,
                        );
                    }
                }
            }
            for declaration in &module.hir.traits {
                for (ordinal, member) in declaration.members.iter().enumerate() {
                    let qualified_name =
                        format!("{}.{}", declaration.name.normalized, member.name.normalized);
                    let id = self.make_definition_id(
                        "trait-member",
                        module.package.as_ref(),
                        &module_name,
                        &qualified_name,
                    );
                    self.definitions
                        .entry(id.clone())
                        .or_insert_with(|| DefinitionInfo {
                            id: id.clone(),
                            package: module.package.clone(),
                            module_name: module_name.clone(),
                            name: qualified_name,
                            name_source: format!(
                                "{}.{}",
                                declaration.name.source, member.name.source
                            ),
                            name_skeleton: format!(
                                "{}.{}",
                                declaration.name.skeleton, member.name.skeleton
                            ),
                            name_scripts: declaration
                                .name
                                .scripts
                                .iter()
                                .chain(member.name.scripts.iter())
                                .cloned()
                                .collect(),
                            name_suspicious_mixed_script: declaration.name.suspicious_mixed_script
                                || member.name.suspicious_mixed_script,
                            kind: DefinitionKind::Value,
                            origin: DefinitionOrigin::User { module: module.id },
                            mutable: false,
                            source_name: Some(module.hir.source_name.clone()),
                            span: Some(member.span),
                        });
                    self.trait_members.insert(
                        id.clone(),
                        TraitMemberInfo {
                            definition: id,
                            module: module.id,
                            trait_name: declaration.name.normalized.clone(),
                            member_name: member.name.normalized.clone(),
                            ordinal,
                            signature: member.signature.clone(),
                            source_name: module.hir.source_name.clone(),
                            span: member.span,
                        },
                    );
                }
            }
            for (impl_ordinal, implementation) in module.hir.impls.iter().enumerate() {
                let trait_name = implementation.trait_name.normalized();
                for (member_ordinal, member) in implementation.members.iter().enumerate() {
                    let qualified_name = format!(
                        "{}#{}::{}",
                        trait_name, impl_ordinal, member.name.normalized
                    );
                    let id = self.make_definition_id(
                        "impl-member",
                        module.package.as_ref(),
                        &module_name,
                        &qualified_name,
                    );
                    self.definitions
                        .entry(id.clone())
                        .or_insert_with(|| DefinitionInfo {
                            id: id.clone(),
                            package: module.package.clone(),
                            module_name: module_name.clone(),
                            name: qualified_name,
                            name_source: member.name.source.clone(),
                            name_skeleton: member.name.skeleton.clone(),
                            name_scripts: member.name.scripts.clone(),
                            name_suspicious_mixed_script: member.name.suspicious_mixed_script,
                            kind: DefinitionKind::Value,
                            origin: DefinitionOrigin::User { module: module.id },
                            mutable: member.mutable,
                            source_name: Some(module.hir.source_name.clone()),
                            span: Some(member.span),
                        });
                    self.impl_members.insert(
                        id.clone(),
                        ImplMemberInfo {
                            definition: id,
                            module: module.id,
                            impl_ordinal,
                            member_ordinal,
                            trait_name: trait_name.clone(),
                            member_name: member.name.normalized.clone(),
                            source_name: module.hir.source_name.clone(),
                            span: member.span,
                        },
                    );
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
        session_generation: Option<u64>,
        scope: &mut BTreeMap<String, DefinitionId>,
        skeletons: &mut BTreeMap<String, String>,
    ) {
        if matches!(
            name.normalized.as_str(),
            "Console"
                | "Text"
                | "Option"
                | "Some"
                | "None"
                | "Result"
                | "Ok"
                | "Error"
                | "max"
                | "min"
                | "map"
                | "sum"
        ) {
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
            DefinitionKind::Task => "task",
            DefinitionKind::Type => "type",
            DefinitionKind::Constructor => "constructor",
            DefinitionKind::Builtin => "builtin",
        };
        let id = if self.project.is_some() {
            self.make_definition_id(
                kind_name,
                module.package.as_ref(),
                module_name,
                &name.normalized,
            )
        } else {
            session_generation.map_or_else(
                || DefinitionId::new(kind_name, module_name, &name.normalized),
                |generation| {
                    DefinitionId::new_repl(kind_name, module_name, &name.source, generation)
                },
            )
        };
        if scope.insert(name.normalized.clone(), id.clone()).is_some() {
            self.errors.push(ResolveError {
                kind: ResolveErrorKind::DuplicateDefinition {
                    name: name.normalized.clone(),
                },
                source_name: module.hir.source_name.clone(),
                span: name.span,
            });
        }
        check_name_security(skeletons, name, &module.hir.source_name, &mut self.errors);
        self.definitions.insert(
            id.clone(),
            DefinitionInfo {
                id,
                package: module.package.clone(),
                module_name: module_name.to_owned(),
                name: name.normalized.clone(),
                name_source: name.source.clone(),
                name_skeleton: name.skeleton.clone(),
                name_scripts: name.scripts.clone(),
                name_suspicious_mixed_script: name.suspicious_mixed_script,
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
                self.resolve_definition_body(module_id, definition);
            }
            let tasks = self.modules[index].hir.tasks.clone();
            for task in &tasks {
                self.resolve_task_body(module_id, task);
            }
            let impls = self.modules[index].hir.impls.clone();
            for implementation in &impls {
                for definition in &implementation.members {
                    self.resolve_definition_body(module_id, definition);
                }
            }
        }
    }

    fn resolve_task_body(&mut self, module: ModuleId, task: &hir::TaskDeclaration) {
        let mut scopes = vec![Scope::default()];
        for parameter in &task.parameters {
            self.bind_pattern(module, parameter, true, &mut scopes);
        }
        self.resolve_expression(module, &task.body, &mut scopes);
    }

    fn resolve_definition_body(&mut self, module: ModuleId, definition: &hir::Definition) {
        let mut scopes = vec![Scope::default()];
        for parameter in &definition.parameters {
            self.bind_pattern(module, parameter, true, &mut scopes);
        }
        self.resolve_expression(module, &definition.value, &mut scopes);
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
                        hir::SequenceElement::LetAwait(binding) => {
                            self.resolve_expression(module, &binding.call, scopes);
                            self.bind_pattern(module, &binding.pattern, false, scopes);
                        }
                        hir::SequenceElement::Expression(expression) => {
                            self.resolve_expression(module, expression, scopes);
                        }
                    }
                }
                scopes.pop();
            }
            hir::ExpressionKind::TaskScope { body, .. } => {
                scopes.push(Scope::default());
                self.resolve_expression(module, body, scopes);
                scopes.pop();
            }
            hir::ExpressionKind::TaskSpawn { call, .. } => {
                self.resolve_expression(module, call, scopes);
            }
            hir::ExpressionKind::TaskAwait { handle, .. } => {
                self.resolve_expression(module, handle, scopes);
            }
            hir::ExpressionKind::TaskReturn { value, .. } => {
                self.resolve_expression(module, value, scopes);
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
            hir::ExpressionKind::Handle { body, clauses } => {
                self.resolve_expression(module, body, scopes);
                let mut handled_labels = BTreeSet::new();
                for clause in clauses {
                    let operation_name = clause.operation.normalized();
                    let operation = resolve_handler_operation(&operation_name);
                    let inputs_are_checkable = operation.is_some_and(|operation| {
                        clause.parameters.len() == operation.inputs().len()
                    });
                    if let Some(operation) = operation {
                        if !handled_labels.insert(operation.label()) {
                            self.invalid_handler_contract(
                                module,
                                clause.operation.span,
                                &operation_name,
                                "duplicate_handled_label",
                                None,
                                Some(operation.label().to_owned()),
                            );
                        }
                        if clause.parameters.len() != operation.inputs().len() {
                            self.invalid_handler_contract(
                                module,
                                clause.span,
                                &operation_name,
                                "parameter_arity",
                                Some(operation.inputs().len().to_string()),
                                Some(clause.parameters.len().to_string()),
                            );
                        }
                    } else {
                        self.invalid_handler_contract(
                            module,
                            clause.operation.span,
                            &operation_name,
                            "unknown_operation",
                            None,
                            None,
                        );
                    }

                    scopes.push(Scope::default());
                    for parameter in &clause.parameters {
                        self.bind_pattern(module, parameter, true, scopes);
                        if inputs_are_checkable
                            && !self.is_handler_input_irrefutable(module, parameter)
                        {
                            self.invalid_handler_contract(
                                module,
                                parameter.span,
                                &operation_name,
                                "refutable_parameter",
                                None,
                                None,
                            );
                        }
                    }
                    if let Some(resume) = &clause.resume {
                        self.bind_name(module, resume.id, &resume.name, false, true, scopes);
                    }
                    self.resolve_expression(module, &clause.body, scopes);
                    scopes.pop();

                    if let (Some(operation), Some(resume)) = (operation, &clause.resume) {
                        let uses = self.count_binding_references(
                            module,
                            &clause.body,
                            BindingKey::new(module, resume.id),
                        );
                        self.handler_resume_uses
                            .insert(BindingKey::new(module, resume.id), uses);
                        match operation.resume_mode() {
                            HandlerResumeMode::Never => self.invalid_handler_contract(
                                module,
                                resume.name.span,
                                &operation_name,
                                "resume_forbidden",
                                Some("0".to_owned()),
                                Some(uses.to_string()),
                            ),
                            HandlerResumeMode::Once if uses > 1 => self.invalid_handler_contract(
                                module,
                                resume.name.span,
                                &operation_name,
                                "resume_uses",
                                Some("at-most-one".to_owned()),
                                Some(uses.to_string()),
                            ),
                            HandlerResumeMode::Once | HandlerResumeMode::Many => {}
                        }
                    }
                }
            }
            hir::ExpressionKind::Literal(_) | hir::ExpressionKind::Unit => {}
        }
    }

    fn invalid_handler_contract(
        &mut self,
        module: ModuleId,
        span: Span,
        operation: &str,
        reason: &'static str,
        expected: Option<String>,
        actual: Option<String>,
    ) {
        self.errors.push(ResolveError {
            kind: ResolveErrorKind::InvalidHandlerContract {
                operation: operation.to_owned(),
                reason,
                expected,
                actual,
            },
            source_name: self.modules[module.0 as usize].hir.source_name.clone(),
            span,
        });
    }

    fn is_handler_input_irrefutable(&self, module: ModuleId, pattern: &hir::Pattern) -> bool {
        match &pattern.kind {
            hir::PatternKind::Wildcard => true,
            hir::PatternKind::Binding { .. } => !self
                .pattern_constructors
                .contains_key(&PatternKey::new(module, pattern.id)),
            hir::PatternKind::Unit
            | hir::PatternKind::Literal(_)
            | hir::PatternKind::Tuple(_)
            | hir::PatternKind::Record(_)
            | hir::PatternKind::Constructor { .. } => false,
        }
    }

    fn count_binding_references(
        &self,
        module: ModuleId,
        expression: &hir::Expression,
        binding: BindingKey,
    ) -> usize {
        let reference_count = |reference: ReferenceId| {
            usize::from(matches!(
                self.references.get(&ReferenceKey::new(module, reference)),
                Some(ReferenceTarget::Binding(target)) if *target == binding
            ))
        };
        match &expression.kind {
            hir::ExpressionKind::Sequence(elements) => elements
                .iter()
                .map(|element| match element {
                    hir::SequenceElement::Let(local) => {
                        self.count_binding_references(module, &local.value, binding)
                    }
                    hir::SequenceElement::LetAwait(local) => {
                        self.count_binding_references(module, &local.call, binding)
                    }
                    hir::SequenceElement::Expression(value) => {
                        self.count_binding_references(module, value, binding)
                    }
                })
                .fold(0usize, usize::saturating_add),
            hir::ExpressionKind::TaskScope { body, .. } => {
                self.count_binding_references(module, body, binding)
            }
            hir::ExpressionKind::TaskSpawn { call, .. } => {
                self.count_binding_references(module, call, binding)
            }
            hir::ExpressionKind::TaskAwait { handle, .. } => {
                self.count_binding_references(module, handle, binding)
            }
            hir::ExpressionKind::TaskReturn { value, .. } => {
                self.count_binding_references(module, value, binding)
            }
            hir::ExpressionKind::Handle { body, clauses } => clauses
                .iter()
                .map(|clause| self.count_binding_references(module, &clause.body, binding))
                .fold(
                    self.count_binding_references(module, body, binding),
                    usize::saturating_add,
                ),
            hir::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => self
                .count_binding_references(module, condition, binding)
                .saturating_add(self.count_binding_references(module, then_branch, binding))
                .saturating_add(self.count_binding_references(module, else_branch, binding)),
            hir::ExpressionKind::Match { scrutinee, cases } => cases
                .iter()
                .map(|case| {
                    case.guard
                        .as_ref()
                        .map_or(0, |guard| {
                            self.count_binding_references(module, guard, binding)
                        })
                        .saturating_add(self.count_binding_references(module, &case.body, binding))
                })
                .fold(
                    self.count_binding_references(module, scrutinee, binding),
                    usize::saturating_add,
                ),
            hir::ExpressionKind::Assignment { place, value } => {
                reference_count(place.root_reference)
                    .saturating_add(self.count_binding_references(module, value, binding))
            }
            hir::ExpressionKind::Application {
                function,
                arguments,
            } => arguments
                .iter()
                .map(|argument| self.count_binding_references(module, argument, binding))
                .fold(
                    self.count_binding_references(module, function, binding),
                    usize::saturating_add,
                ),
            hir::ExpressionKind::Projection {
                reference, target, ..
            } => reference_count(*reference)
                .saturating_add(self.count_binding_references(module, target, binding)),
            hir::ExpressionKind::Name { reference, .. } => reference_count(*reference),
            hir::ExpressionKind::Binary { left, right, .. } => self
                .count_binding_references(module, left, binding)
                .saturating_add(self.count_binding_references(module, right, binding)),
            hir::ExpressionKind::Unary { operand, .. } => {
                self.count_binding_references(module, operand, binding)
            }
            hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => elements
                .iter()
                .map(|element| self.count_binding_references(module, element, binding))
                .fold(0usize, usize::saturating_add),
            hir::ExpressionKind::Record(fields) => fields
                .iter()
                .map(|field| self.count_binding_references(module, &field.value, binding))
                .fold(0usize, usize::saturating_add),
            hir::ExpressionKind::RecordUpdate { base, fields } => fields
                .iter()
                .map(|field| self.count_binding_references(module, &field.value, binding))
                .fold(
                    self.count_binding_references(module, base, binding),
                    usize::saturating_add,
                ),
            hir::ExpressionKind::Literal(_) | hir::ExpressionKind::Unit => 0,
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
        if let Some(id) = self
            .prelude_definition(&name.normalized)
            .filter(|id| {
                self.definitions
                    .get(*id)
                    .is_some_and(|definition| definition.kind == DefinitionKind::Constructor)
            })
            .cloned()
        {
            self.references.insert(
                ReferenceKey::new(module, reference),
                ReferenceTarget::Definition(id),
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
            if let Some(id) = self
                .trait_members
                .values()
                .find(|member| {
                    member.module == module
                        && member.trait_name == segments[0].normalized
                        && member.member_name == segments[1].normalized
                })
                .map(|member| member.definition.clone())
            {
                self.references.insert(
                    ReferenceKey::new(module, reference),
                    ReferenceTarget::Definition(id),
                );
                return true;
            }
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
        if segments.len() == 3 {
            let current = &self.modules[module.0 as usize];
            if let Some(imported) = current.imports.get(&segments[0].normalized) {
                if let Some(id) = self
                    .trait_members
                    .values()
                    .find(|member| {
                        member.module == *imported
                            && member.trait_name == segments[1].normalized
                            && member.member_name == segments[2].normalized
                    })
                    .map(|member| member.definition.clone())
                {
                    self.references.insert(
                        ReferenceKey::new(module, reference),
                        ReferenceTarget::Definition(id),
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
                let shadowed = scopes
                    .iter()
                    .rev()
                    .any(|scope| scope.names.contains_key(&name.normalized));
                if let Some(constructor) = (!shadowed)
                    .then(|| self.constructor_definition(module, None, &name.normalized))
                    .flatten()
                {
                    self.pattern_constructors
                        .insert(PatternKey::new(module, pattern.id), constructor);
                } else {
                    self.bind_name(module, *id, name, false, parameter, scopes);
                }
            }
            hir::PatternKind::Tuple(elements) => {
                for element in elements {
                    self.bind_pattern(module, element, parameter, scopes);
                }
            }
            hir::PatternKind::Record(fields) => {
                for field in fields {
                    self.bind_pattern(module, &field.pattern, parameter, scopes);
                }
            }
            hir::PatternKind::Constructor {
                qualifier,
                name,
                arguments,
            } => {
                if let Some(constructor) =
                    self.constructor_definition(module, qualifier.as_ref(), &name.normalized)
                {
                    self.pattern_constructors
                        .insert(PatternKey::new(module, pattern.id), constructor);
                } else {
                    let display_name = qualifier.as_ref().map_or_else(
                        || name.normalized.clone(),
                        |qualifier| format!("{}.{}", qualifier.normalized, name.normalized),
                    );
                    self.undefined(module, name.span, display_name);
                }
                for argument in arguments {
                    self.bind_pattern(module, argument, parameter, scopes);
                }
            }
            hir::PatternKind::Wildcard | hir::PatternKind::Unit | hir::PatternKind::Literal(_) => {}
        }
    }

    fn constructor_definition(
        &self,
        module: ModuleId,
        qualifier: Option<&hir::Name>,
        name: &str,
    ) -> Option<DefinitionId> {
        let owner = qualifier
            .and_then(|qualifier| {
                self.modules[module.0 as usize]
                    .imports
                    .get(&qualifier.normalized)
            })
            .copied()
            .unwrap_or(module);
        if qualifier.is_some() && owner == module {
            return None;
        }
        let definition = self
            .module_definitions
            .get(&owner)
            .and_then(|definitions| definitions.get(name))
            .or_else(|| {
                qualifier
                    .is_none()
                    .then(|| self.prelude_definition(name))
                    .flatten()
            })?;
        self.definitions
            .get(definition)
            .filter(|info| info.kind == DefinitionKind::Constructor)
            .map(|info| info.id.clone())
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
        check_name_security(
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

fn check_name_security(
    skeletons: &mut BTreeMap<String, String>,
    name: &hir::Name,
    source_name: &str,
    errors: &mut Vec<ResolveError>,
) {
    if name.suspicious_mixed_script {
        errors.push(ResolveError {
            kind: ResolveErrorKind::SuspiciousMixedScript {
                name: name.normalized.clone(),
                scripts: name.scripts.clone(),
            },
            source_name: source_name.to_owned(),
            span: name.span,
        });
    }
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

fn check_qualified_name_security(
    skeletons: &mut BTreeMap<String, String>,
    name: &hir::QualifiedName,
    source_name: &str,
    errors: &mut Vec<ResolveError>,
) {
    for segment in &name.segments {
        if segment.suspicious_mixed_script {
            errors.push(ResolveError {
                kind: ResolveErrorKind::SuspiciousMixedScript {
                    name: segment.normalized.clone(),
                    scripts: segment.scripts.clone(),
                },
                source_name: source_name.to_owned(),
                span: segment.span,
            });
        }
    }

    let normalized = name.normalized();
    let skeleton = name
        .segments
        .iter()
        .map(|segment| segment.skeleton.as_str())
        .collect::<Vec<_>>()
        .join(".");
    if let Some(first) = skeletons.get(&skeleton) {
        if first != &normalized {
            errors.push(ResolveError {
                kind: ResolveErrorKind::ConfusableCollision {
                    first: first.clone(),
                    second: normalized,
                },
                source_name: source_name.to_owned(),
                span: name.span,
            });
        }
    } else {
        skeletons.insert(skeleton, normalized);
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

fn imported_module_key(
    project: Option<&ResolvedProject>,
    importer: &ResolvedModule,
    imported_name: &str,
) -> ModuleKey {
    let Some(project) = project else {
        return ModuleKey {
            package: None,
            name: imported_name.to_owned(),
        };
    };
    let package = importer
        .package
        .as_ref()
        .expect("package-aware resolver modules carry a package identity");
    let mut segments = imported_name.split('.');
    let first = segments.next().unwrap_or(imported_name);
    let dependency = project
        .dependencies
        .iter()
        .find_map(|((from, name), target)| {
            (from == package && name.as_str() == first).then_some(target)
        });
    match dependency {
        Some(target) => ModuleKey {
            package: Some(target.clone()),
            name: segments.collect::<Vec<_>>().join("."),
        },
        None => ModuleKey {
            package: Some(package.clone()),
            name: imported_name.to_owned(),
        },
    }
}

fn encode_part(output: &mut Vec<u8>, bytes: &[u8]) {
    output.extend_from_slice(&u32::try_from(bytes.len()).unwrap_or(u32::MAX).to_be_bytes());
    output.extend_from_slice(bytes);
}

fn encode_package_identity(output: &mut Vec<u8>, package: &PackageIdentity) {
    encode_part(output, package.name().as_str().as_bytes());
    encode_part(output, package.version().to_string().as_bytes());
    encode_part(output, package.source().as_str().as_bytes());
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
    fn injects_prelude_definitions_and_allows_local_constructor_shadowing() {
        let program = hir_program(
            0,
            "prelude.ling",
            concat!(
                "module Main\n\n",
                "let wrapped = Some 1\n",
                "let unwrap option =\n",
                "    match option with\n",
                "    | Some value -> value\n",
                "    | None -> 0\n\n",
                "let shadow () =\n",
                "    let Some = 40\n",
                "    Some + 2\n",
            ),
        );
        let resolved = resolve(vec![program], "Main").expect("Prelude names resolve");

        for definition in [
            PreludeDefinition::Option,
            PreludeDefinition::Some,
            PreludeDefinition::None,
            PreludeDefinition::Result,
            PreludeDefinition::Ok,
            PreludeDefinition::Error,
        ] {
            let id = resolved.prelude_id(definition);
            assert!(matches!(
                resolved.definition(id).map(|info| info.origin),
                Some(DefinitionOrigin::Prelude(actual)) if actual == definition
            ));
        }
        assert_eq!(resolved.pattern_constructors().len(), 2);
        let local_some = resolved
            .bindings()
            .values()
            .find(|binding| binding.name == "Some")
            .expect("local Some binding")
            .key;
        assert!(resolved.references().values().any(
            |target| matches!(target, ReferenceTarget::Binding(binding) if binding == &local_some)
        ));
    }

    #[test]
    fn seed_builtin_and_prelude_inventories_are_exact() {
        let program = hir_program(0, "inventory.ling", "module Main\n\nlet main () = ()\n");
        let resolved = resolve(vec![program], "Main").expect("inventory resolves");

        let builtins = [
            Builtin::ConsoleWrite,
            Builtin::TextFormat,
            Builtin::Max,
            Builtin::Min,
            Builtin::Map,
            Builtin::Sum,
        ];
        assert_eq!(
            builtins.map(Builtin::qualified_name),
            ["Console.write", "Text.format", "max", "min", "map", "sum"]
        );
        let builtin_definitions = resolved
            .definitions()
            .values()
            .filter(|definition| matches!(definition.origin, DefinitionOrigin::Builtin(_)))
            .collect::<Vec<_>>();
        assert_eq!(builtin_definitions.len(), builtins.len());
        for builtin in builtins {
            let definition = resolved
                .definition(resolved.builtin_id(builtin))
                .expect("builtin definition exists");
            assert_eq!(definition.name, builtin.qualified_name());
            assert_eq!(definition.kind, DefinitionKind::Builtin);
            assert_eq!(definition.origin, DefinitionOrigin::Builtin(builtin));
            assert_eq!(definition.module_name, "<builtin>");
            assert!(definition.source_name.is_none());
            assert!(definition.span.is_none());
        }

        let prelude = [
            PreludeDefinition::Option,
            PreludeDefinition::Some,
            PreludeDefinition::None,
            PreludeDefinition::Result,
            PreludeDefinition::Ok,
            PreludeDefinition::Error,
        ];
        assert_eq!(
            prelude.map(PreludeDefinition::name),
            ["Option", "Some", "None", "Result", "Ok", "Error"]
        );
        let prelude_definitions = resolved
            .definitions()
            .values()
            .filter(|definition| matches!(definition.origin, DefinitionOrigin::Prelude(_)))
            .collect::<Vec<_>>();
        assert_eq!(prelude_definitions.len(), prelude.len());
        for definition in prelude {
            let info = resolved
                .definition(resolved.prelude_id(definition))
                .expect("Prelude definition exists");
            assert_eq!(info.name, definition.name());
            assert_eq!(info.kind, definition.kind());
            assert_eq!(info.origin, DefinitionOrigin::Prelude(definition));
            assert_eq!(info.module_name, PRELUDE_MODULE);
            assert!(info.source_name.is_none());
            assert!(info.span.is_none());
        }
    }

    #[test]
    fn seed_injected_surface_has_no_plan_only_convenience_names() {
        let program = hir_program(0, "surface.ling", "module Main\n\nlet main () = ()\n");
        let resolved = resolve(vec![program], "Main").expect("surface resolves");
        let injected_names = resolved
            .definitions()
            .values()
            .filter(|definition| {
                matches!(
                    definition.origin,
                    DefinitionOrigin::Builtin(_) | DefinitionOrigin::Prelude(_)
                )
            })
            .map(|definition| definition.name.as_str())
            .collect::<BTreeSet<_>>();
        for plan_only in [
            "Clock.now",
            "Random.next",
            "Network.get",
            "Network.retry",
            "Runtime.global",
            "Reflect.dynamic",
            "Ffi.call",
            "Collection.unbounded",
        ] {
            assert!(
                !injected_names.contains(plan_only),
                "plan-only convenience API entered the Seed surface: {plan_only}"
            );
        }
        assert_eq!(injected_names.len(), 12);
    }

    #[test]
    fn rejects_module_scope_prelude_redefinition() {
        let program = hir_program(
            0,
            "redefine.ling",
            "module Main\n\ntype Option<'a> =\n    | Some of 'a\n    | None\n",
        );
        let errors = resolve(vec![program], "Main").expect_err("Prelude names are reserved");
        for name in ["Option", "Some", "None"] {
            assert!(errors.iter().any(|error| matches!(
                &error.kind,
                ResolveErrorKind::ReservedName { name: actual } if actual == name
            )));
        }
    }

    #[test]
    fn rejects_module_builtin_redefinition_but_allows_local_shadowing() {
        for name in ["max", "min", "map", "sum"] {
            let program = hir_program(
                0,
                "builtin-redefinition.ling",
                &format!("module Main\n\nlet {name} value = value\n"),
            );
            let errors = resolve(vec![program], "Main")
                .expect_err("module-scope builtin names are reserved");
            assert!(errors.iter().any(|error| matches!(
                &error.kind,
                ResolveErrorKind::ReservedName { name: actual } if actual == name
            )));
        }

        let local = hir_program(
            0,
            "local-shadow.ling",
            concat!(
                "module Main\n\n",
                "let useLocal () =\n",
                "    let max value = value\n",
                "    max 1\n",
            ),
        );
        resolve(vec![local], "Main")
            .expect("local lexical bindings may shadow unqualified builtins");
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

    #[test]
    fn resolves_checked_handler_scopes_and_rejects_unknown_operations() {
        let program = hir_program(
            0,
            "Main.ling",
            concat!(
                "module Main\n\n",
                "let input = 1\n",
                "let value =\n",
                "    handle input with\n",
                "        operation Random.next(seed, resume) -> resume seed\n",
            ),
        );
        let resolved = resolve(vec![program], "Main").expect("handler scope resolves");
        let module = resolved.entry_module();
        let hir::ExpressionKind::Handle { clauses, .. } = &module.hir.definitions[1].value.kind
        else {
            panic!("expected handler");
        };
        let resume = clauses[0].resume.as_ref().expect("resume binding");
        let key = BindingKey::new(module.id, resume.id);
        assert_eq!(resolved.handler_resume_uses(key), Some(1));
        assert!(resolved.bindings().contains_key(&key));

        let unknown = hir_program(
            0,
            "Main.ling",
            concat!(
                "module Main\n\n",
                "let value =\n",
                "    handle 1 with\n",
                "        operation Missing.run() -> 1\n",
            ),
        );
        let errors = resolve(vec![unknown], "Main").expect_err("unknown operation is rejected");
        let error = errors
            .iter()
            .find(|error| {
                matches!(
                    error.kind,
                    ResolveErrorKind::InvalidHandlerContract {
                        reason: "unknown_operation",
                        ..
                    }
                )
            })
            .expect("unknown-operation rejection");
        assert_eq!(
            error.to_diagnostic().code(),
            codes::INVALID_HANDLER_CONTRACT
        );
        assert!(
            error
                .to_diagnostic()
                .render_json()
                .expect("diagnostic JSON")
                .contains("Missing.run")
        );
        assert!(error.span.start() < error.span.end());
    }

    #[test]
    fn resolves_qualified_constructor_patterns_across_modules() {
        let main = hir_program(
            0,
            "Main.ling",
            concat!(
                "module Main\n\n",
                "import Domain.State as State\n\n",
                "let describe value =\n",
                "    match value with\n",
                "    | State.Hurt amount -> amount\n",
                "    | State.Healthy -> 0\n",
            ),
        );
        let state = hir_program(
            1,
            "Domain/State.ling",
            concat!(
                "module Domain.State\n\n",
                "type State =\n",
                "    | Healthy\n",
                "    | Hurt of Int\n",
            ),
        );
        let resolved = resolve(vec![main, state], "Main").expect("constructors resolve");
        assert_eq!(resolved.pattern_constructors().len(), 2);
        assert!(resolved.pattern_constructors().values().all(|definition| {
            resolved
                .definition(definition)
                .is_some_and(|info| info.module_name == "Domain.State")
        }));
    }

    #[test]
    fn rejects_duplicate_binders_inside_tuple_patterns() {
        let program = hir_program(
            0,
            "Main.ling",
            "module Main\n\nlet duplicate (value, value) = value\n",
        );
        let errors = resolve(vec![program], "Main").expect_err("binder names must be unique");
        assert!(errors.iter().any(|error| matches!(
            &error.kind,
            ResolveErrorKind::DuplicateDefinition { name } if name == "value"
        )));
    }

    #[test]
    fn rejects_suspicious_mixed_script_names_without_a_collision() {
        let top_level = hir_program(0, "Main.ling", "module Main\n\nlet pаypal = 1\n");
        let errors = resolve(vec![top_level], "Main")
            .expect_err("a suspicious mixed-script name is rejected on its own");
        let error = errors
            .iter()
            .find(|error| matches!(error.kind, ResolveErrorKind::SuspiciousMixedScript { .. }))
            .expect("mixed-script diagnostic");
        assert_eq!(error.to_diagnostic().code(), codes::SUSPICIOUS_MIXED_SCRIPT);
        assert!(
            error
                .to_diagnostic()
                .render_json()
                .expect("diagnostic JSON")
                .contains("\"scripts\":[\"Cyrl\",\"Latn\"]")
        );

        let local = hir_program(
            1,
            "Main.ling",
            "module Main\n\nlet use value =\n    let pаypal = value\n    pаypal\n",
        );
        assert!(resolve(vec![local], "Main").is_err());

        let mixed_module = hir_program(2, "Mаin.ling", "module Mаin\n\nlet value = 1\n");
        assert!(resolve(vec![mixed_module], "Mаin").is_err());

        let main = hir_program(
            3,
            "Main.ling",
            "module Main\n\nimport Library as pаypal\n\nlet value = 1\n",
        );
        let library = hir_program(4, "Library.ling", "module Library\n\nlet value = 1\n");
        assert!(resolve(vec![main, library], "Main").is_err());
    }

    #[test]
    fn rejects_confusable_module_names() {
        let latin = hir_program(0, "Paypal.ling", "module Paypal\n\nlet value = 1\n");
        let spoofed = hir_program(1, "Pаypal.ling", "module Pаypal\n\nlet value = 2\n");
        let errors = resolve(vec![latin, spoofed], "Paypal")
            .expect_err("the global module scope rejects confusable names");
        assert!(
            errors
                .iter()
                .any(|error| matches!(error.kind, ResolveErrorKind::ConfusableCollision { .. }))
        );
    }

    #[test]
    fn allows_seed_script_combinations() {
        for name in ["人物ID", "日本かなカナID", "韓國한글ID"] {
            let program = hir_program(0, "Main.ling", &format!("module Main\n\nlet {name} = 1\n"));
            resolve(vec![program], "Main").expect("allowed Seed script combination");
        }
    }
}
