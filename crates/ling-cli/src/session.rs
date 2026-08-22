//! Transactional checked-core REPL session.

use std::collections::{BTreeMap, BTreeSet};

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_eval::{Console, RuntimeFault, evaluate_definition};
use ling_hir as hir;
use ling_resolve::DefinitionId;
use ling_source::SourceId;

use crate::{CompileFailure, compile_programs, lower_source_with_counters};

const REPL_MODULE: &str = "Main";
const RESULT_PREFIX: &str = "__ling_repl_result";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubmissionKind {
    Expression,
    ValueDeclaration,
    TypeDeclaration,
}

#[derive(Clone, Debug)]
pub struct SubmissionSuccess {
    pub submission: u64,
    pub kind: SubmissionKind,
    pub committed: bool,
    pub name: Option<String>,
    pub type_name: Option<String>,
    pub value: Option<String>,
    pub effects: Vec<String>,
    pub capabilities: Vec<String>,
    pub definition_id: Option<String>,
    pub warnings: Vec<Diagnostic>,
}

#[derive(Clone, Debug)]
pub enum SubmissionFailure {
    Compile {
        submission: u64,
        diagnostics: Vec<Diagnostic>,
    },
    Runtime {
        submission: u64,
        fault: RuntimeFault,
    },
    Internal {
        submission: u64,
        message: String,
    },
    SnapshotMismatch {
        submission: u64,
        message: String,
    },
}

impl SubmissionFailure {
    #[must_use]
    pub const fn submission(&self) -> u64 {
        match self {
            Self::Compile { submission, .. }
            | Self::Runtime { submission, .. }
            | Self::Internal { submission, .. }
            | Self::SnapshotMismatch { submission, .. } => *submission,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum VisibleKind {
    Value,
    Type,
    Constructor,
}

#[derive(Clone, Debug)]
struct VisibleName {
    normalized: String,
    source: String,
    skeleton: String,
    internal: Option<String>,
    kind: VisibleKind,
}

/// Immutable-on-failure state for a single logical `Main` module.
pub struct Session {
    capabilities: Vec<String>,
    definitions: Vec<hir::Definition>,
    types: Vec<hir::TypeDeclaration>,
    visible: BTreeMap<String, VisibleName>,
    counters: hir::IdCounters,
    next_submission: u64,
}

impl Session {
    #[must_use]
    pub fn new(mut capabilities: Vec<String>) -> Self {
        capabilities.sort();
        capabilities.dedup();
        Self {
            capabilities,
            definitions: Vec::new(),
            types: Vec::new(),
            visible: BTreeMap::new(),
            counters: hir::IdCounters::default(),
            next_submission: 1,
        }
    }

    pub fn submit(
        &mut self,
        source: &str,
        console: &mut dyn Console,
    ) -> Result<SubmissionSuccess, SubmissionFailure> {
        let submission = self.next_submission;
        self.next_submission = self.next_submission.saturating_add(1);
        let source_name = format!("<repl:{submission}>");
        let classified = classify(source);
        if matches!(classified, Classified::Forbidden) {
            return Err(SubmissionFailure::Compile {
                submission,
                diagnostics: vec![repl_structure_diagnostic(
                    &source_name,
                    "REPL submissions cannot declare modules, imports, or capabilities",
                )],
            });
        }

        let (wrapper, kind) = match classified {
            Classified::Declaration => (self.wrap_declaration(source), None),
            Classified::Expression => (
                self.wrap_expression(source, submission),
                Some(SubmissionKind::Expression),
            ),
            Classified::Forbidden => unreachable!("handled above"),
        };
        let (mut candidate, next_counters) = lower_source_with_counters(
            SourceId::new(u32::try_from(submission).unwrap_or(u32::MAX)),
            source_name.clone(),
            wrapper.text.as_bytes().to_vec(),
            self.counters,
        )
        .map_err(|failure| {
            map_compile_failure(submission, failure, &wrapper.mapping, &source_name)
        })?;

        if !candidate.imports.is_empty()
            || candidate
                .definitions
                .len()
                .saturating_add(candidate.types.len())
                != 1
        {
            return Err(SubmissionFailure::Compile {
                submission,
                diagnostics: vec![repl_structure_diagnostic(
                    &source_name,
                    "each REPL submission must contain exactly one declaration or expression",
                )],
            });
        }

        let mut tentative_visible = self.visible.clone();
        let (kind, public_name, internal_name) = if let Some(kind) = kind {
            let definition = candidate
                .definitions
                .first_mut()
                .expect("expression wrapper creates one definition");
            let internal = internal_name(RESULT_PREFIX, submission);
            definition.name.source.clone_from(&internal);
            definition.name.normalized.clone_from(&internal);
            definition.name.skeleton.clone_from(&internal);
            definition.session_generation = Some(submission);
            rewrite_definition_references(definition, &visible_value_map(&tentative_visible));
            (kind, None, Some(internal))
        } else if let Some(definition) = candidate.definitions.first_mut() {
            let public = definition.name.normalized.clone();
            let public_source = definition.name.source.clone();
            let public_skeleton = definition.name.skeleton.clone();
            validate_visible_name(
                &tentative_visible,
                &definition.name,
                VisibleKind::Value,
                &source_name,
                &wrapper.mapping,
            )?;
            let internal = internal_name(&public, submission);
            let mut mapping = visible_value_map(&tentative_visible);
            mapping.insert(public.clone(), internal.clone());
            definition.name.source.clone_from(&public);
            definition.name.normalized.clone_from(&internal);
            definition.name.skeleton.clone_from(&internal);
            definition.session_generation = Some(submission);
            rewrite_definition_references(definition, &mapping);
            tentative_visible.insert(
                public.clone(),
                VisibleName {
                    normalized: public.clone(),
                    source: public_source,
                    skeleton: public_skeleton,
                    internal: Some(internal.clone()),
                    kind: VisibleKind::Value,
                },
            );
            (
                SubmissionKind::ValueDeclaration,
                Some(public),
                Some(internal),
            )
        } else {
            let declaration = candidate
                .types
                .first()
                .expect("one declaration was checked above");
            let names = type_names(declaration);
            for (name, kind) in &names {
                validate_visible_name(
                    &tentative_visible,
                    name,
                    *kind,
                    &source_name,
                    &wrapper.mapping,
                )?;
                tentative_visible.insert(
                    name.normalized.clone(),
                    VisibleName {
                        normalized: name.normalized.clone(),
                        source: name.source.clone(),
                        skeleton: name.skeleton.clone(),
                        internal: None,
                        kind: *kind,
                    },
                );
            }
            (
                SubmissionKind::TypeDeclaration,
                Some(declaration.name.normalized.clone()),
                None,
            )
        };

        let mut definitions = self.definitions.clone();
        definitions.extend(candidate.definitions.iter().cloned());
        let mut types = self.types.clone();
        types.extend(candidate.types.iter().cloned());
        let program = hir::Program {
            source_name: source_name.clone(),
            span: candidate.span,
            module: candidate.module,
            imports: Vec::new(),
            definitions,
            types,
            traits: Vec::new(),
            impls: Vec::new(),
        };
        let compiled = compile_programs(vec![program], REPL_MODULE).map_err(|failure| {
            map_compile_failure(submission, failure, &wrapper.mapping, &source_name)
        })?;

        let warnings = compiled
            .snapshot
            .checked()
            .warnings()
            .iter()
            .filter(|warning| warning.code() != codes::UNUSED_CAPABILITY)
            .cloned()
            .map(|warning| remap_diagnostic(warning, &wrapper.mapping, &source_name))
            .collect::<Vec<_>>();
        let type_name;
        let mut value = None;
        let mut effects = Vec::new();
        let mut capabilities = Vec::new();
        let mut definition_id = None;

        if let Some(internal) = &internal_name {
            let module = compiled.snapshot.checked().typed().resolved().entry();
            let id = compiled
                .snapshot
                .checked()
                .typed()
                .resolved()
                .definition_id(module, internal)
                .cloned()
                .ok_or_else(|| SubmissionFailure::Internal {
                    submission,
                    message: "REPL result definition is absent after checking".to_owned(),
                })?;
            let evaluated =
                evaluate_definition(&compiled.snapshot, &id, console).map_err(|mut fault| {
                    fault.source_name.clone_from(&source_name);
                    fault.span = wrapper.mapping.map_span(fault.span);
                    SubmissionFailure::Runtime { submission, fault }
                })?;
            let semantic = semantic_definition(&compiled.snapshot, &id).ok_or_else(|| {
                SubmissionFailure::Internal {
                    submission,
                    message: "REPL result semantic definition is absent".to_owned(),
                }
            })?;
            type_name = Some(semantic.type_name.clone());
            effects.clone_from(&semantic.effects);
            capabilities.clone_from(&semantic.capabilities);
            definition_id = Some(id.to_string());
            if kind == SubmissionKind::Expression && !evaluated.is_unit() {
                value = Some(evaluated.rendered().to_owned());
            }
        } else {
            type_name = Some("type".to_owned());
        }

        if kind != SubmissionKind::Expression {
            self.definitions.extend(candidate.definitions);
            self.types.extend(candidate.types);
            self.visible = tentative_visible;
            self.counters = next_counters;
        }

        Ok(SubmissionSuccess {
            submission,
            kind,
            committed: true,
            name: public_name,
            type_name,
            value,
            effects,
            capabilities,
            definition_id,
            warnings,
        })
    }

    fn module_header(&self) -> String {
        if self.capabilities.is_empty() {
            "module Main\n\n".to_owned()
        } else {
            format!(
                "module Main\n    requires {}\n\n",
                self.capabilities.join(", ")
            )
        }
    }

    fn wrap_declaration(&self, source: &str) -> Wrapper {
        let mut text = self.module_header();
        let start = text.len();
        text.push_str(source);
        let end = text.len();
        if !text.ends_with('\n') {
            text.push('\n');
        }
        Wrapper {
            text,
            mapping: WrapperMapping {
                source_id: SourceId::new(
                    u32::try_from(self.next_submission.saturating_sub(1)).unwrap_or(u32::MAX),
                ),
                source_len: source.len(),
                segments: vec![MappingSegment {
                    wrapped_start: start,
                    wrapped_end: end,
                    source_start: 0,
                }],
            },
        }
    }

    fn wrap_expression(&self, source: &str, submission: u64) -> Wrapper {
        let mut text = self.module_header();
        text.push_str(&format!("let {RESULT_PREFIX}_{submission} =\n"));
        let mut segments = Vec::new();
        let mut source_start = 0;
        for line in source.split_inclusive('\n') {
            text.push_str("    ");
            let wrapped_start = text.len();
            text.push_str(line);
            let wrapped_end = text.len();
            segments.push(MappingSegment {
                wrapped_start,
                wrapped_end,
                source_start,
            });
            source_start += line.len();
        }
        if !text.ends_with('\n') {
            text.push('\n');
        }
        Wrapper {
            text,
            mapping: WrapperMapping {
                source_id: SourceId::new(u32::try_from(submission).unwrap_or(u32::MAX)),
                source_len: source.len(),
                segments,
            },
        }
    }
}

struct Wrapper {
    text: String,
    mapping: WrapperMapping,
}

struct WrapperMapping {
    source_id: SourceId,
    source_len: usize,
    segments: Vec<MappingSegment>,
}

struct MappingSegment {
    wrapped_start: usize,
    wrapped_end: usize,
    source_start: usize,
}

impl WrapperMapping {
    fn map_offset(&self, wrapped: u32) -> u32 {
        let wrapped = usize::try_from(wrapped).unwrap_or(usize::MAX);
        for segment in &self.segments {
            if (segment.wrapped_start..=segment.wrapped_end).contains(&wrapped) {
                return u32::try_from(
                    segment
                        .source_start
                        .saturating_add(wrapped.saturating_sub(segment.wrapped_start))
                        .min(self.source_len),
                )
                .unwrap_or(u32::MAX);
            }
        }
        u32::try_from(self.source_len).unwrap_or(u32::MAX)
    }

    fn map_span(&self, span: ling_source::Span) -> ling_source::Span {
        let start = self.map_offset(span.start().get());
        let end = self.map_offset(span.end().get()).max(start);
        ling_source::Span::new(
            self.source_id,
            ling_source::ByteOffset::new(start),
            ling_source::ByteOffset::new(end),
        )
        .expect("mapped REPL span is ordered")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Classified {
    Declaration,
    Expression,
    Forbidden,
}

fn classify(source: &str) -> Classified {
    let first = source.trim_start();
    if starts_keyword(first, "module") || starts_keyword(first, "import") {
        Classified::Forbidden
    } else if starts_keyword(first, "let") || starts_keyword(first, "type") {
        Classified::Declaration
    } else {
        Classified::Expression
    }
}

fn starts_keyword(source: &str, keyword: &str) -> bool {
    source
        .strip_prefix(keyword)
        .is_some_and(|tail| tail.is_empty() || tail.starts_with(char::is_whitespace))
}

fn internal_name(public: &str, submission: u64) -> String {
    format!("__ling_repl_{submission}_{public}")
}

fn visible_value_map(visible: &BTreeMap<String, VisibleName>) -> BTreeMap<String, String> {
    visible
        .values()
        .filter_map(|name| {
            name.internal
                .as_ref()
                .map(|internal| (name.normalized.clone(), internal.clone()))
        })
        .collect()
}

fn validate_visible_name(
    visible: &BTreeMap<String, VisibleName>,
    name: &hir::Name,
    kind: VisibleKind,
    source_name: &str,
    mapping: &WrapperMapping,
) -> Result<(), SubmissionFailure> {
    if let Some(previous) = visible.get(&name.normalized) {
        if previous.kind == VisibleKind::Value && kind == VisibleKind::Value {
            return Ok(());
        }
        return Err(SubmissionFailure::Compile {
            submission: submission_from_source(source_name),
            diagnostics: vec![
                Diagnostic::new(
                    codes::DUPLICATE_DEFINITION,
                    Severity::Error,
                    format!("名称“{}”已在当前 REPL 会话中定义", name.source),
                    format!(
                        "name `{}` is already defined in this REPL session",
                        name.source
                    ),
                )
                .with_primary_span(DiagnosticSpan::new(
                    source_name,
                    mapping.map_span(name.span),
                ))
                .with_fact("previous_name", previous.source.clone()),
            ],
        });
    }
    if let Some(previous) = visible.values().find(|previous| {
        previous.skeleton == name.skeleton && previous.normalized != name.normalized
    }) {
        return Err(SubmissionFailure::Compile {
            submission: submission_from_source(source_name),
            diagnostics: vec![
                Diagnostic::new(
                    codes::CONFUSABLE_COLLISION,
                    Severity::Error,
                    format!(
                        "名称“{}”与当前 REPL 会话中的“{}”视觉混淆",
                        name.source, previous.source
                    ),
                    format!(
                        "name `{}` is confusable with `{}` in this REPL session",
                        name.source, previous.source
                    ),
                )
                .with_primary_span(DiagnosticSpan::new(
                    source_name,
                    mapping.map_span(name.span),
                ))
                .with_fact("first", previous.source.clone())
                .with_fact("second", name.source.clone()),
            ],
        });
    }
    Ok(())
}

fn submission_from_source(source_name: &str) -> u64 {
    source_name
        .strip_prefix("<repl:")
        .and_then(|value| value.strip_suffix('>'))
        .and_then(|value| value.parse().ok())
        .unwrap_or(0)
}

fn type_names(declaration: &hir::TypeDeclaration) -> Vec<(&hir::Name, VisibleKind)> {
    let mut names = vec![(&declaration.name, VisibleKind::Type)];
    if let hir::TypeDefinition::Variant(cases) = &declaration.definition {
        names.extend(
            cases
                .iter()
                .map(|case| (&case.name, VisibleKind::Constructor)),
        );
    }
    names
}

fn semantic_definition<'snapshot>(
    snapshot: &'snapshot ling_semantic::ProgramSnapshot,
    id: &DefinitionId,
) -> Option<&'snapshot ling_semantic::SemanticDefinition> {
    snapshot
        .graph()
        .definitions
        .iter()
        .find(|definition| definition.definition_id == id.as_str())
}

fn map_compile_failure(
    submission: u64,
    failure: CompileFailure,
    mapping: &WrapperMapping,
    source_name: &str,
) -> SubmissionFailure {
    match failure {
        CompileFailure::Diagnostics(diagnostics) => SubmissionFailure::Compile {
            submission,
            diagnostics: diagnostics
                .into_iter()
                .map(|diagnostic| remap_diagnostic(diagnostic, mapping, source_name))
                .collect(),
        },
        CompileFailure::Internal(message) => SubmissionFailure::Internal {
            submission,
            message,
        },
        CompileFailure::SnapshotMismatch(message) => SubmissionFailure::SnapshotMismatch {
            submission,
            message,
        },
    }
}

fn remap_diagnostic(
    diagnostic: Diagnostic,
    mapping: &WrapperMapping,
    source_name: &str,
) -> Diagnostic {
    let Some(span) = diagnostic.primary_span() else {
        return diagnostic;
    };
    if span.file() != source_name {
        return diagnostic;
    }
    let (Ok(start), Ok(end)) = (
        u32::try_from(span.start_byte()),
        u32::try_from(span.end_byte()),
    ) else {
        return diagnostic;
    };
    let start = mapping.map_offset(start);
    let end = mapping.map_offset(end).max(start);
    diagnostic.with_primary_span(DiagnosticSpan::at(source_name, start, end))
}

fn repl_structure_diagnostic(source_name: &str, message: &str) -> Diagnostic {
    Diagnostic::new(
        codes::INVALID_MODULE,
        Severity::Error,
        "REPL submission 结构无效",
        message,
    )
    .with_primary_span(DiagnosticSpan::at(source_name, 0, 0))
    .with_fact("committed", false)
}

fn rewrite_definition_references(
    definition: &mut hir::Definition,
    mapping: &BTreeMap<String, String>,
) {
    let mut local = BTreeSet::new();
    for parameter in &definition.parameters {
        collect_pattern_bindings(parameter, &mut local);
    }
    rewrite_expression(&mut definition.value, mapping, &mut vec![local]);
}

fn rewrite_expression(
    expression: &mut hir::Expression,
    mapping: &BTreeMap<String, String>,
    scopes: &mut Vec<BTreeSet<String>>,
) {
    match &mut expression.kind {
        hir::ExpressionKind::Sequence(elements) => {
            scopes.push(BTreeSet::new());
            for element in elements {
                match element {
                    hir::SequenceElement::Let(binding) => {
                        if binding.recursive {
                            scopes
                                .last_mut()
                                .expect("sequence scope exists")
                                .insert(binding.name.normalized.clone());
                        }
                        let mut parameters = BTreeSet::new();
                        for parameter in &binding.parameters {
                            collect_pattern_bindings(parameter, &mut parameters);
                        }
                        scopes.push(parameters);
                        rewrite_expression(&mut binding.value, mapping, scopes);
                        scopes.pop();
                        if !binding.recursive {
                            scopes
                                .last_mut()
                                .expect("sequence scope exists")
                                .insert(binding.name.normalized.clone());
                        }
                    }
                    hir::SequenceElement::Expression(expression) => {
                        rewrite_expression(expression, mapping, scopes);
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
            rewrite_expression(condition, mapping, scopes);
            rewrite_expression(then_branch, mapping, scopes);
            rewrite_expression(else_branch, mapping, scopes);
        }
        hir::ExpressionKind::Match { scrutinee, cases } => {
            rewrite_expression(scrutinee, mapping, scopes);
            for case in cases {
                let mut bindings = BTreeSet::new();
                collect_pattern_bindings(&case.pattern, &mut bindings);
                scopes.push(bindings);
                if let Some(guard) = &mut case.guard {
                    rewrite_expression(guard, mapping, scopes);
                }
                rewrite_expression(&mut case.body, mapping, scopes);
                scopes.pop();
            }
        }
        hir::ExpressionKind::Assignment { place, value } => {
            rewrite_name(&mut place.root, mapping, scopes);
            rewrite_expression(value, mapping, scopes);
        }
        hir::ExpressionKind::Application {
            function,
            arguments,
        } => {
            rewrite_expression(function, mapping, scopes);
            for argument in arguments {
                rewrite_expression(argument, mapping, scopes);
            }
        }
        hir::ExpressionKind::Projection { target, .. } => {
            rewrite_expression(target, mapping, scopes);
        }
        hir::ExpressionKind::Name { name, .. } => rewrite_name(name, mapping, scopes),
        hir::ExpressionKind::Binary { left, right, .. } => {
            rewrite_expression(left, mapping, scopes);
            rewrite_expression(right, mapping, scopes);
        }
        hir::ExpressionKind::Unary { operand, .. } => rewrite_expression(operand, mapping, scopes),
        hir::ExpressionKind::Tuple(elements) | hir::ExpressionKind::List(elements) => {
            for element in elements {
                rewrite_expression(element, mapping, scopes);
            }
        }
        hir::ExpressionKind::Record(fields) => {
            for field in fields {
                rewrite_expression(&mut field.value, mapping, scopes);
            }
        }
        hir::ExpressionKind::RecordUpdate { base, fields } => {
            rewrite_expression(base, mapping, scopes);
            for field in fields {
                rewrite_expression(&mut field.value, mapping, scopes);
            }
        }
        hir::ExpressionKind::Handle { .. } => {}
        hir::ExpressionKind::Literal(_) | hir::ExpressionKind::Unit => {}
    }
}

fn rewrite_name(
    name: &mut hir::Name,
    mapping: &BTreeMap<String, String>,
    scopes: &[BTreeSet<String>],
) {
    if scopes
        .iter()
        .rev()
        .any(|scope| scope.contains(&name.normalized))
    {
        return;
    }
    if let Some(internal) = mapping.get(&name.normalized) {
        name.normalized.clone_from(internal);
        name.skeleton.clone_from(internal);
    }
}

fn collect_pattern_bindings(pattern: &hir::Pattern, bindings: &mut BTreeSet<String>) {
    match &pattern.kind {
        hir::PatternKind::Binding { name, .. } => {
            bindings.insert(name.normalized.clone());
        }
        hir::PatternKind::Tuple(patterns) => {
            for pattern in patterns {
                collect_pattern_bindings(pattern, bindings);
            }
        }
        hir::PatternKind::Record(fields) => {
            for field in fields {
                collect_pattern_bindings(&field.pattern, bindings);
            }
        }
        hir::PatternKind::Constructor { arguments, .. } => {
            for argument in arguments {
                collect_pattern_bindings(argument, bindings);
            }
        }
        hir::PatternKind::Wildcard | hir::PatternKind::Unit | hir::PatternKind::Literal(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use ling_eval::MemoryConsole;

    use super::*;

    fn submit(session: &mut Session, source: &str) -> SubmissionSuccess {
        session
            .submit(source, &mut MemoryConsole::default())
            .unwrap_or_else(|failure| panic!("submission failed: {failure:?}"))
    }

    #[test]
    fn bindings_are_visible_and_failed_submissions_roll_back() {
        let mut session = Session::new(Vec::new());
        submit(&mut session, "let answer = 41");
        let failure = session
            .submit("let broken = missing", &mut MemoryConsole::default())
            .expect_err("undefined name fails");
        let SubmissionFailure::Compile { diagnostics, .. } = failure else {
            panic!("expected compile failure");
        };
        let span = diagnostics[0]
            .primary_span()
            .expect("name error has a span");
        assert_eq!((span.start_byte(), span.end_byte()), (13, 20));

        let result = submit(&mut session, "answer + 1");
        assert_eq!(result.value.as_deref(), Some("42"));
        assert_eq!(result.type_name.as_deref(), Some("Int"));
        let missing = session
            .submit("broken", &mut MemoryConsole::default())
            .expect_err("failed binding did not commit");
        assert!(matches!(missing, SubmissionFailure::Compile { .. }));
    }

    #[test]
    fn runtime_failure_rolls_back_the_declaration_and_remaps_its_span() {
        let mut session = Session::new(Vec::new());
        submit(&mut session, "let stable = 9");
        let failure = session
            .submit("let failed = 1 / 0", &mut MemoryConsole::default())
            .expect_err("division by zero fails at runtime");
        let SubmissionFailure::Runtime { fault, .. } = failure else {
            panic!("expected runtime failure");
        };
        assert_eq!(fault.source_name, "<repl:2>");
        assert_eq!((fault.span.start().get(), fault.span.end().get()), (13, 18));

        assert_eq!(submit(&mut session, "stable").value.as_deref(), Some("9"));
        assert!(matches!(
            session.submit("failed", &mut MemoryConsole::default()),
            Err(SubmissionFailure::Compile { .. })
        ));
    }

    #[test]
    fn redefinition_preserves_old_closure_generation() {
        let mut session = Session::new(Vec::new());
        submit(&mut session, "let value = 1");
        submit(&mut session, "let old () = value");
        submit(&mut session, "let value = 2");

        assert_eq!(submit(&mut session, "value").value.as_deref(), Some("2"));
        assert_eq!(submit(&mut session, "old ()").value.as_deref(), Some("1"));
    }

    #[test]
    fn local_bindings_shadow_session_values() {
        let mut session = Session::new(Vec::new());
        submit(&mut session, "let value = 7");
        submit(&mut session, "let identity value = value");
        assert_eq!(
            submit(&mut session, "identity 3").value.as_deref(),
            Some("3")
        );
    }

    #[test]
    fn rejects_cross_submission_confusables_and_type_redefinition() {
        let mut session = Session::new(Vec::new());
        submit(&mut session, "let a = 1");
        let confusable = session
            .submit("let а = 2", &mut MemoryConsole::default())
            .expect_err("Cyrillic a collides with Latin a");
        let SubmissionFailure::Compile { diagnostics, .. } = confusable else {
            panic!("expected confusable diagnostic");
        };
        assert_eq!(diagnostics[0].code(), codes::CONFUSABLE_COLLISION);

        submit(&mut session, "type State =\n    | Ready");
        let duplicate = session
            .submit("type State =\n    | Waiting", &mut MemoryConsole::default())
            .expect_err("types cannot be redefined");
        let SubmissionFailure::Compile { diagnostics, .. } = duplicate else {
            panic!("expected duplicate diagnostic");
        };
        assert_eq!(diagnostics[0].code(), codes::DUPLICATE_DEFINITION);
    }

    #[test]
    fn file_and_session_use_equivalent_checked_type_effect_and_value() {
        let mut session = Session::new(Vec::new());
        let session_result = submit(&mut session, "sum [1; 2; 3]");

        let compiled = crate::compile_source(
            "pair.ling",
            b"module Main\n\nlet result = sum [1; 2; 3]\n".to_vec(),
        )
        .expect("file source compiles");
        let module = compiled.snapshot.checked().typed().resolved().entry();
        let id = compiled
            .snapshot
            .checked()
            .typed()
            .resolved()
            .definition_id(module, "result")
            .expect("file result definition exists");
        let file_value = evaluate_definition(&compiled.snapshot, id, &mut MemoryConsole::default())
            .expect("file result evaluates");
        let file_semantic = semantic_definition(&compiled.snapshot, id)
            .expect("file result semantic definition exists");

        assert_eq!(session_result.value.as_deref(), Some(file_value.rendered()));
        assert_eq!(
            session_result.type_name.as_deref(),
            Some(file_semantic.type_name.as_str())
        );
        assert_eq!(session_result.effects, file_semantic.effects);
        assert_eq!(session_result.capabilities, file_semantic.capabilities);
    }
}
