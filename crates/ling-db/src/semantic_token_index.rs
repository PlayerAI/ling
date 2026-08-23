//! Snapshot-bound semantic-token classification authorized by RFC-0047.
//!
//! The index is compiler-owned and LSP-independent. It retains original UTF-8
//! spans and abstract taxonomy values only; transport, positions, legends,
//! document versions, and result identities belong to later protocol work.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use ling_effects::CheckedProgram;
use ling_hir::{
    self as hir, Expression, ExpressionKind, Pattern, PatternKind, SequenceElement, TypeAtom,
    TypeDefinition, TypeSyntax,
};
use ling_resolve::{
    BindingKey, DefinitionInfo, DefinitionKind, DefinitionOrigin, ModuleId, ReferenceTarget,
    ResolvedProgram,
};
use ling_source::{ByteOffset, Revision, SourceId, Span};
use ling_syntax::TokenKind;

use crate::TokenSourceIndex;

/// Accepted identity of the in-process generation contract.
pub const SEMANTIC_TOKEN_GENERATION_VERSION: &str = "ling.semantic-token-generation/0.1";

/// RFC-0046 canonical token-type order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SemanticTokenKind {
    Namespace,
    Type,
    Enum,
    Interface,
    Struct,
    TypeParameter,
    Parameter,
    Variable,
    Property,
    EnumMember,
    Function,
    Method,
    Keyword,
    Comment,
    String,
    Number,
    Operator,
}

impl SemanticTokenKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Namespace => "namespace",
            Self::Type => "type",
            Self::Enum => "enum",
            Self::Interface => "interface",
            Self::Struct => "struct",
            Self::TypeParameter => "typeParameter",
            Self::Parameter => "parameter",
            Self::Variable => "variable",
            Self::Property => "property",
            Self::EnumMember => "enumMember",
            Self::Function => "function",
            Self::Method => "method",
            Self::Keyword => "keyword",
            Self::Comment => "comment",
            Self::String => "string",
            Self::Number => "number",
            Self::Operator => "operator",
        }
    }
}

/// RFC-0046 canonical modifier order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SemanticTokenModifier {
    Declaration,
    Definition,
    Readonly,
    Modification,
    Documentation,
    DefaultLibrary,
    Mutable,
}

impl SemanticTokenModifier {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Declaration => "declaration",
            Self::Definition => "definition",
            Self::Readonly => "readonly",
            Self::Modification => "modification",
            Self::Documentation => "documentation",
            Self::DefaultLibrary => "defaultLibrary",
            Self::Mutable => "mutable",
        }
    }
}

/// Compiler evidence selected for one entry.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SemanticTokenEvidence {
    CheckedIdentity,
    CheckedStructure,
    LexicalFallback,
}

impl SemanticTokenEvidence {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheckedIdentity => "checked-identity",
            Self::CheckedStructure => "checked-structure",
            Self::LexicalFallback => "lexical-fallback",
        }
    }
}

/// Whether complete checking succeeded for the generated source.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SemanticTokenGenerationMode {
    Typed,
    LexicalFallback,
}

impl SemanticTokenGenerationMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Typed => "typed",
            Self::LexicalFallback => "lexical-fallback",
        }
    }
}

/// One abstract semantic token over an original UTF-8 source span.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticTokenEntry {
    span: Span,
    kind: SemanticTokenKind,
    modifiers: Box<[SemanticTokenModifier]>,
    evidence: SemanticTokenEvidence,
}

impl SemanticTokenEntry {
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    #[must_use]
    pub const fn kind(&self) -> SemanticTokenKind {
        self.kind
    }

    #[must_use]
    pub fn modifiers(&self) -> &[SemanticTokenModifier] {
        &self.modifiers
    }

    #[must_use]
    pub const fn evidence(&self) -> SemanticTokenEvidence {
        self.evidence
    }
}

/// Immutable typed or conservative-fallback token generation for one source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticTokenIndex {
    source: SourceId,
    revision: Revision,
    source_name: String,
    mode: SemanticTokenGenerationMode,
    entries: Box<[SemanticTokenEntry]>,
}

impl SemanticTokenIndex {
    #[must_use]
    pub const fn source(&self) -> SourceId {
        self.source
    }

    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn mode(&self) -> SemanticTokenGenerationMode {
        self.mode
    }

    #[must_use]
    pub fn entries(&self) -> &[SemanticTokenEntry] {
        &self.entries
    }

    pub(crate) fn from_checked(
        lexical: &TokenSourceIndex,
        checked: &CheckedProgram,
        revision: Revision,
        original: &str,
    ) -> Result<Self, SemanticTokenIndexError> {
        let mut builder = TokenBuilder::new(lexical.source(), original);
        builder.add_lexical(lexical, false)?;

        let resolved = checked.typed().resolved();
        let module = resolved
            .modules()
            .iter()
            .find(|module| {
                module.hir.source_name == lexical.source_name()
                    && module.hir.span.source() == lexical.source()
            })
            .ok_or(SemanticTokenIndexError::MissingCheckedSource)?;
        TypedClassifier::new(resolved, module.id, &mut builder).program(&module.hir)?;

        Ok(Self {
            source: lexical.source(),
            revision,
            source_name: lexical.source_name().to_owned(),
            mode: SemanticTokenGenerationMode::Typed,
            entries: builder.finish()?.into_boxed_slice(),
        })
    }

    pub(crate) fn from_lexical_fallback(
        lexical: &TokenSourceIndex,
        revision: Revision,
        original: &str,
    ) -> Result<Self, SemanticTokenIndexError> {
        let mut builder = TokenBuilder::new(lexical.source(), original);
        builder.add_lexical(lexical, true)?;
        Ok(Self {
            source: lexical.source(),
            revision,
            source_name: lexical.source_name().to_owned(),
            mode: SemanticTokenGenerationMode::LexicalFallback,
            entries: builder.finish()?.into_boxed_slice(),
        })
    }
}

impl fmt::Display for SemanticTokenIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} semantic token index for {} ({} entries)",
            self.mode.as_str(),
            self.source_name,
            self.entries.len()
        )
    }
}

/// Failure to construct a complete, nonoverlapping token index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemanticTokenIndexError {
    MissingCheckedSource,
    WrongSource,
    InvalidSpan,
    ConflictingRole { span: Span },
    OverlappingSpans { first: Span, second: Span },
}

impl fmt::Display for SemanticTokenIndexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCheckedSource => {
                formatter.write_str("checked workspace does not contain the requested source")
            }
            Self::WrongSource => {
                formatter.write_str("semantic token span belongs to another source")
            }
            Self::InvalidSpan => formatter.write_str("semantic token span is invalid source text"),
            Self::ConflictingRole { span } => write!(
                formatter,
                "semantic token span {}..{} has incompatible equal-precedence roles",
                span.start().get(),
                span.end().get()
            ),
            Self::OverlappingSpans { first, second } => write!(
                formatter,
                "semantic token spans {}..{} and {}..{} overlap",
                first.start().get(),
                first.end().get(),
                second.start().get(),
                second.end().get()
            ),
        }
    }
}

impl Error for SemanticTokenIndexError {}

#[derive(Clone)]
struct Candidate {
    entry: SemanticTokenEntry,
    precedence: u8,
}

struct TokenBuilder<'source> {
    source: SourceId,
    original: &'source str,
    candidates: BTreeMap<(u32, u32), Candidate>,
}

impl<'source> TokenBuilder<'source> {
    fn new(source: SourceId, original: &'source str) -> Self {
        Self {
            source,
            original,
            candidates: BTreeMap::new(),
        }
    }

    fn add_lexical(
        &mut self,
        lexical: &TokenSourceIndex,
        fallback: bool,
    ) -> Result<(), SemanticTokenIndexError> {
        let evidence = if fallback {
            SemanticTokenEvidence::LexicalFallback
        } else {
            SemanticTokenEvidence::CheckedStructure
        };
        for token in lexical.tokens() {
            let Some(kind) = lexical_kind(token.kind()) else {
                continue;
            };
            let modifiers = if !fallback && token.kind() == TokenKind::DocComment {
                vec![SemanticTokenModifier::Documentation]
            } else {
                Vec::new()
            };
            self.add(token.span(), kind, modifiers, evidence, 1)?;
        }
        Ok(())
    }

    fn add(
        &mut self,
        span: Span,
        kind: SemanticTokenKind,
        mut modifiers: Vec<SemanticTokenModifier>,
        evidence: SemanticTokenEvidence,
        precedence: u8,
    ) -> Result<(), SemanticTokenIndexError> {
        if span.start() == span.end() {
            return Ok(());
        }
        self.validate_span(span)?;
        modifiers.sort_unstable();
        modifiers.dedup();
        if modifiers.contains(&SemanticTokenModifier::Declaration)
            && modifiers.contains(&SemanticTokenModifier::Definition)
            || modifiers.contains(&SemanticTokenModifier::Readonly)
                && modifiers.contains(&SemanticTokenModifier::Mutable)
        {
            return Err(SemanticTokenIndexError::ConflictingRole { span });
        }
        let candidate = Candidate {
            entry: SemanticTokenEntry {
                span,
                kind,
                modifiers: modifiers.into_boxed_slice(),
                evidence,
            },
            precedence,
        };
        let key = (span.start().get(), span.end().get());
        match self.candidates.get(&key) {
            Some(existing) if existing.precedence > precedence => return Ok(()),
            Some(existing) if existing.precedence == precedence => {
                if existing.entry != candidate.entry {
                    return Err(SemanticTokenIndexError::ConflictingRole { span });
                }
                return Ok(());
            }
            _ => {}
        }
        self.candidates.insert(key, candidate);
        Ok(())
    }

    fn validate_span(&self, span: Span) -> Result<(), SemanticTokenIndexError> {
        if span.source() != self.source {
            return Err(SemanticTokenIndexError::WrongSource);
        }
        let start = usize::try_from(span.start().get())
            .map_err(|_| SemanticTokenIndexError::InvalidSpan)?;
        let end =
            usize::try_from(span.end().get()).map_err(|_| SemanticTokenIndexError::InvalidSpan)?;
        if end > self.original.len()
            || start >= end
            || !self.original.is_char_boundary(start)
            || !self.original.is_char_boundary(end)
        {
            return Err(SemanticTokenIndexError::InvalidSpan);
        }
        Ok(())
    }

    fn finish(self) -> Result<Vec<SemanticTokenEntry>, SemanticTokenIndexError> {
        let mut entries = Vec::new();
        for candidate in self.candidates.into_values() {
            entries.extend(split_entry(candidate.entry, self.original)?);
        }
        entries.sort_by(|left, right| {
            left.span
                .start()
                .cmp(&right.span.start())
                .then_with(|| left.span.end().cmp(&right.span.end()))
                .then_with(|| left.kind.cmp(&right.kind))
                .then_with(|| left.modifiers.cmp(&right.modifiers))
                .then_with(|| left.evidence.cmp(&right.evidence))
        });
        for pair in entries.windows(2) {
            if pair[0].span.end() > pair[1].span.start() {
                return Err(SemanticTokenIndexError::OverlappingSpans {
                    first: pair[0].span,
                    second: pair[1].span,
                });
            }
        }
        Ok(entries)
    }
}

fn split_entry(
    entry: SemanticTokenEntry,
    original: &str,
) -> Result<Vec<SemanticTokenEntry>, SemanticTokenIndexError> {
    let start = usize::try_from(entry.span.start().get())
        .map_err(|_| SemanticTokenIndexError::InvalidSpan)?;
    let end = usize::try_from(entry.span.end().get())
        .map_err(|_| SemanticTokenIndexError::InvalidSpan)?;
    let bytes = original.as_bytes();
    let mut segments = Vec::new();
    let mut segment_start = start;
    let mut index = start;
    while index < end {
        if bytes[index] == b'\n' {
            let segment_end = if index > segment_start && bytes[index - 1] == b'\r' {
                index - 1
            } else {
                index
            };
            push_segment(&entry, segment_start, segment_end, &mut segments)?;
            segment_start = index + 1;
        }
        index += 1;
    }
    push_segment(&entry, segment_start, end, &mut segments)?;
    Ok(segments)
}

fn push_segment(
    entry: &SemanticTokenEntry,
    start: usize,
    end: usize,
    output: &mut Vec<SemanticTokenEntry>,
) -> Result<(), SemanticTokenIndexError> {
    if start == end {
        return Ok(());
    }
    let span = Span::new(
        entry.span.source(),
        ByteOffset::new(u32::try_from(start).map_err(|_| SemanticTokenIndexError::InvalidSpan)?),
        ByteOffset::new(u32::try_from(end).map_err(|_| SemanticTokenIndexError::InvalidSpan)?),
    )
    .map_err(|_| SemanticTokenIndexError::InvalidSpan)?;
    output.push(SemanticTokenEntry {
        span,
        kind: entry.kind,
        modifiers: entry.modifiers.clone(),
        evidence: entry.evidence,
    });
    Ok(())
}

fn lexical_kind(kind: TokenKind) -> Option<SemanticTokenKind> {
    match kind {
        TokenKind::Let
        | TokenKind::Mutable
        | TokenKind::Rec
        | TokenKind::And
        | TokenKind::Type
        | TokenKind::Of
        | TokenKind::Match
        | TokenKind::With
        | TokenKind::When
        | TokenKind::If
        | TokenKind::Then
        | TokenKind::Else
        | TokenKind::True
        | TokenKind::False
        | TokenKind::Module
        | TokenKind::Import
        | TokenKind::As
        | TokenKind::Requires
        | TokenKind::Trait
        | TokenKind::Impl => Some(SemanticTokenKind::Keyword),
        TokenKind::LineComment | TokenKind::DocComment | TokenKind::BlockComment => {
            Some(SemanticTokenKind::Comment)
        }
        TokenKind::Text => Some(SemanticTokenKind::String),
        TokenKind::Integer | TokenKind::Float => Some(SemanticTokenKind::Number),
        TokenKind::Equals
        | TokenKind::EqualEqual
        | TokenKind::BangEqual
        | TokenKind::Less
        | TokenKind::LessEqual
        | TokenKind::Greater
        | TokenKind::GreaterEqual
        | TokenKind::LeftArrow
        | TokenKind::RightArrow
        | TokenKind::Pipe
        | TokenKind::PipeGreater
        | TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::Percent
        | TokenKind::AmpAmp
        | TokenKind::PipePipe => Some(SemanticTokenKind::Operator),
        TokenKind::Whitespace
        | TokenKind::Newline
        | TokenKind::SoftNewline
        | TokenKind::Indent
        | TokenKind::Dedent
        | TokenKind::Identifier
        | TokenKind::LeftParen
        | TokenKind::RightParen
        | TokenKind::LeftBracket
        | TokenKind::RightBracket
        | TokenKind::LeftBrace
        | TokenKind::RightBrace
        | TokenKind::Comma
        | TokenKind::Semicolon
        | TokenKind::Colon
        | TokenKind::Dot
        | TokenKind::Apostrophe
        | TokenKind::Error
        | TokenKind::Eof => None,
    }
}

struct TypedClassifier<'checked, 'builder, 'source> {
    resolved: &'checked ResolvedProgram,
    module: ModuleId,
    builder: &'builder mut TokenBuilder<'source>,
    callable_bindings: BTreeSet<BindingKey>,
}

impl<'checked, 'builder, 'source> TypedClassifier<'checked, 'builder, 'source> {
    fn new(
        resolved: &'checked ResolvedProgram,
        module: ModuleId,
        builder: &'builder mut TokenBuilder<'source>,
    ) -> Self {
        Self {
            resolved,
            module,
            builder,
            callable_bindings: BTreeSet::new(),
        }
    }

    fn program(&mut self, program: &hir::Program) -> Result<(), SemanticTokenIndexError> {
        if program.module.explicit {
            for segment in &program.module.name.segments {
                self.structure(
                    segment.span,
                    SemanticTokenKind::Namespace,
                    vec![SemanticTokenModifier::Definition],
                )?;
            }
        }
        for import in &program.imports {
            for segment in &import.module.segments {
                self.structure(segment.span, SemanticTokenKind::Namespace, Vec::new())?;
            }
            self.identity(
                import.alias.span,
                SemanticTokenKind::Namespace,
                vec![SemanticTokenModifier::Declaration],
            )?;
        }
        for declaration in &program.types {
            self.type_declaration(declaration)?;
        }
        for declaration in &program.traits {
            self.trait_declaration(declaration)?;
        }
        for definition in &program.definitions {
            self.definition(definition, false)?;
        }
        for implementation in &program.impls {
            self.qualified(&implementation.trait_name, SemanticTokenKind::Interface)?;
            self.type_syntax(&implementation.receiver)?;
            for member in &implementation.members {
                self.definition(member, true)?;
            }
        }
        Ok(())
    }

    fn type_declaration(
        &mut self,
        declaration: &hir::TypeDeclaration,
    ) -> Result<(), SemanticTokenIndexError> {
        let kind = match &declaration.definition {
            TypeDefinition::Record(_) => SemanticTokenKind::Struct,
            TypeDefinition::Variant(_) => SemanticTokenKind::Enum,
            TypeDefinition::Alias(_) => SemanticTokenKind::Type,
        };
        self.checked_declaration(
            declaration.name.span,
            kind,
            vec![SemanticTokenModifier::Definition],
        )?;
        for parameter in &declaration.parameters {
            self.structure(
                parameter.span,
                SemanticTokenKind::TypeParameter,
                vec![SemanticTokenModifier::Definition],
            )?;
        }
        match &declaration.definition {
            TypeDefinition::Record(fields) => {
                for field in fields {
                    let mut modifiers = vec![SemanticTokenModifier::Definition];
                    modifiers.push(mutability(field.mutable));
                    self.structure(field.name.span, SemanticTokenKind::Property, modifiers)?;
                    self.type_syntax(&field.field_type)?;
                }
            }
            TypeDefinition::Variant(cases) => {
                for case in cases {
                    self.checked_declaration(
                        case.name.span,
                        SemanticTokenKind::EnumMember,
                        vec![SemanticTokenModifier::Definition],
                    )?;
                    if let Some(payload) = &case.payload {
                        self.type_syntax(payload)?;
                    }
                }
            }
            TypeDefinition::Alias(value) => self.type_syntax(value)?,
        }
        Ok(())
    }

    fn trait_declaration(
        &mut self,
        declaration: &hir::TraitDeclaration,
    ) -> Result<(), SemanticTokenIndexError> {
        self.checked_declaration(
            declaration.name.span,
            SemanticTokenKind::Interface,
            vec![SemanticTokenModifier::Definition],
        )?;
        for parameter in &declaration.parameters {
            self.structure(
                parameter.span,
                SemanticTokenKind::TypeParameter,
                vec![SemanticTokenModifier::Definition],
            )?;
        }
        for member in &declaration.members {
            self.checked_declaration(
                member.name.span,
                SemanticTokenKind::Method,
                vec![SemanticTokenModifier::Declaration],
            )?;
            self.type_syntax(&member.signature)?;
        }
        Ok(())
    }

    fn definition(
        &mut self,
        definition: &hir::Definition,
        implementation_member: bool,
    ) -> Result<(), SemanticTokenIndexError> {
        let kind = if implementation_member {
            SemanticTokenKind::Method
        } else if definition.parameters.is_empty() {
            SemanticTokenKind::Variable
        } else {
            SemanticTokenKind::Function
        };
        self.checked_declaration(
            definition.name.span,
            kind,
            vec![
                SemanticTokenModifier::Definition,
                mutability(definition.mutable),
            ],
        )?;
        for parameter in &definition.type_parameters {
            self.structure(
                parameter.span,
                SemanticTokenKind::TypeParameter,
                vec![SemanticTokenModifier::Definition],
            )?;
        }
        for constraint in &definition.constraints {
            self.constraint_syntax(constraint)?;
        }
        for parameter in &definition.parameters {
            self.pattern(parameter, true)?;
        }
        if let Some(annotation) = &definition.annotation {
            self.type_syntax(annotation)?;
        }
        self.expression(&definition.value, None)
    }

    fn local_binding(
        &mut self,
        binding: &hir::LocalBinding,
    ) -> Result<(), SemanticTokenIndexError> {
        let key = BindingKey::new(self.module, binding.id);
        if !binding.parameters.is_empty() {
            self.callable_bindings.insert(key);
        }
        let info = self.resolved.bindings().get(&key);
        let mutable = info.map_or(binding.mutable, |value| value.mutable);
        let kind = if binding.parameters.is_empty() {
            SemanticTokenKind::Variable
        } else {
            SemanticTokenKind::Function
        };
        self.builder.add(
            binding.name.span,
            kind,
            vec![SemanticTokenModifier::Definition, mutability(mutable)],
            if info.is_some() {
                SemanticTokenEvidence::CheckedIdentity
            } else {
                SemanticTokenEvidence::CheckedStructure
            },
            if info.is_some() { 3 } else { 2 },
        )?;
        for parameter in &binding.type_parameters {
            self.structure(
                parameter.span,
                SemanticTokenKind::TypeParameter,
                vec![SemanticTokenModifier::Definition],
            )?;
        }
        for constraint in &binding.constraints {
            self.constraint_syntax(constraint)?;
        }
        for parameter in &binding.parameters {
            self.pattern(parameter, true)?;
        }
        if let Some(annotation) = &binding.annotation {
            self.type_syntax(annotation)?;
        }
        self.expression(&binding.value, None)
    }

    fn pattern(
        &mut self,
        pattern: &Pattern,
        parameter: bool,
    ) -> Result<(), SemanticTokenIndexError> {
        match &pattern.kind {
            PatternKind::Binding { id, name } => {
                if self
                    .resolved
                    .pattern_constructor(self.module, pattern.id)
                    .is_some()
                {
                    return self.identity(name.span, SemanticTokenKind::EnumMember, Vec::new());
                }
                let key = BindingKey::new(self.module, *id);
                let info = self.resolved.bindings().get(&key);
                let is_parameter = info.map_or(parameter, |value| value.parameter);
                let mutable = info.is_some_and(|value| value.mutable);
                self.builder.add(
                    name.span,
                    if is_parameter {
                        SemanticTokenKind::Parameter
                    } else {
                        SemanticTokenKind::Variable
                    },
                    vec![SemanticTokenModifier::Definition, mutability(mutable)],
                    if info.is_some() {
                        SemanticTokenEvidence::CheckedIdentity
                    } else {
                        SemanticTokenEvidence::CheckedStructure
                    },
                    if info.is_some() { 3 } else { 2 },
                )?;
            }
            PatternKind::Tuple(values) => {
                for value in values {
                    self.pattern(value, parameter)?;
                }
            }
            PatternKind::Record(fields) => {
                for field in fields {
                    self.structure(field.name.span, SemanticTokenKind::Property, Vec::new())?;
                    self.pattern(&field.pattern, parameter)?;
                }
            }
            PatternKind::Constructor {
                qualifier,
                name,
                arguments,
            } => {
                if self
                    .resolved
                    .pattern_constructor(self.module, pattern.id)
                    .is_some()
                {
                    if let Some(qualifier) = qualifier {
                        self.structure(qualifier.span, SemanticTokenKind::Namespace, Vec::new())?;
                    }
                    self.identity(name.span, SemanticTokenKind::EnumMember, Vec::new())?;
                }
                for argument in arguments {
                    self.pattern(argument, parameter)?;
                }
            }
            PatternKind::Wildcard | PatternKind::Unit | PatternKind::Literal(_) => {}
        }
        Ok(())
    }

    fn expression(
        &mut self,
        expression: &Expression,
        relation: Option<ReferenceUse>,
    ) -> Result<(), SemanticTokenIndexError> {
        match &expression.kind {
            ExpressionKind::Sequence(elements) => {
                for element in elements {
                    match element {
                        SequenceElement::Let(binding) => self.local_binding(binding)?,
                        SequenceElement::Expression(value) => self.expression(value, None)?,
                    }
                }
            }
            ExpressionKind::Handle { body, clauses } => {
                self.expression(body, None)?;
                for clause in clauses {
                    for parameter in &clause.parameters {
                        self.pattern(parameter, true)?;
                    }
                    self.expression(&clause.body, None)?;
                }
            }
            ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.expression(condition, None)?;
                self.expression(then_branch, None)?;
                self.expression(else_branch, None)?;
            }
            ExpressionKind::Match { scrutinee, cases } => {
                self.expression(scrutinee, None)?;
                for case in cases {
                    self.pattern(&case.pattern, false)?;
                    if let Some(guard) = &case.guard {
                        self.expression(guard, None)?;
                    }
                    self.expression(&case.body, None)?;
                }
            }
            ExpressionKind::Assignment { place, value } => {
                self.reference(
                    place.root_reference,
                    place.root.span,
                    ReferenceUse::Write,
                    None,
                )?;
                for field in &place.fields {
                    self.structure(field.span, SemanticTokenKind::Property, Vec::new())?;
                }
                self.expression(value, None)?;
            }
            ExpressionKind::Application {
                function,
                arguments,
            } => {
                self.expression(function, Some(ReferenceUse::Call))?;
                for argument in arguments {
                    self.expression(argument, None)?;
                }
            }
            ExpressionKind::Projection {
                reference,
                target,
                field,
            } => {
                if let ExpressionKind::Name {
                    reference: target_reference,
                    name,
                } = &target.kind
                    && self
                        .resolved
                        .reference(self.module, *target_reference)
                        .is_none()
                {
                    if self
                        .resolved
                        .module(self.module)
                        .is_some_and(|module| module.imports.contains_key(&name.normalized))
                    {
                        self.identity(name.span, SemanticTokenKind::Namespace, Vec::new())?;
                    } else if self
                        .resolved
                        .reference(self.module, *reference)
                        .and_then(|target| match target {
                            ReferenceTarget::Definition(id) => Some(id),
                            ReferenceTarget::Binding(_) => None,
                        })
                        .is_some_and(|id| self.resolved.trait_member(id).is_some())
                    {
                        self.structure(name.span, SemanticTokenKind::Interface, Vec::new())?;
                    }
                } else {
                    self.expression(target, None)?;
                }
                if self.resolved.reference(self.module, *reference).is_some() {
                    self.reference(
                        *reference,
                        field.span,
                        relation.unwrap_or(ReferenceUse::Read),
                        None,
                    )?;
                } else {
                    self.structure(field.span, SemanticTokenKind::Property, Vec::new())?;
                }
            }
            ExpressionKind::Name { reference, name } => self.reference(
                *reference,
                name.span,
                relation.unwrap_or(ReferenceUse::Read),
                None,
            )?,
            ExpressionKind::Binary { left, right, .. } => {
                self.expression(left, None)?;
                self.expression(right, None)?;
            }
            ExpressionKind::Unary { operand, .. } => self.expression(operand, None)?,
            ExpressionKind::Tuple(values) | ExpressionKind::List(values) => {
                for value in values {
                    self.expression(value, None)?;
                }
            }
            ExpressionKind::Record(fields) => {
                for field in fields {
                    self.structure(field.name.span, SemanticTokenKind::Property, Vec::new())?;
                    self.expression(&field.value, None)?;
                }
            }
            ExpressionKind::RecordUpdate { base, fields } => {
                self.expression(base, None)?;
                for field in fields {
                    self.structure(field.name.span, SemanticTokenKind::Property, Vec::new())?;
                    self.expression(&field.value, None)?;
                }
            }
            ExpressionKind::Literal(_) | ExpressionKind::Unit => {}
        }
        Ok(())
    }

    fn reference(
        &mut self,
        reference: hir::ReferenceId,
        span: Span,
        relation: ReferenceUse,
        forced_kind: Option<SemanticTokenKind>,
    ) -> Result<(), SemanticTokenIndexError> {
        let Some(target) = self.resolved.reference(self.module, reference) else {
            return Ok(());
        };
        let (kind, mut modifiers) = match target {
            ReferenceTarget::Definition(id) => {
                let Some(definition) = self.resolved.definition(id) else {
                    return Ok(());
                };
                let kind = forced_kind
                    .unwrap_or_else(|| self.definition_reference_kind(definition, relation));
                let mut modifiers = Vec::new();
                if matches!(
                    kind,
                    SemanticTokenKind::Variable | SemanticTokenKind::Function
                ) || kind == SemanticTokenKind::Property
                {
                    modifiers.push(mutability(definition.mutable));
                }
                if !matches!(definition.origin, DefinitionOrigin::User { .. }) {
                    modifiers.push(SemanticTokenModifier::DefaultLibrary);
                }
                (kind, modifiers)
            }
            ReferenceTarget::Binding(key) => {
                let Some(binding) = self.resolved.bindings().get(key) else {
                    return Ok(());
                };
                let kind = forced_kind.unwrap_or_else(|| {
                    if binding.parameter {
                        SemanticTokenKind::Parameter
                    } else if self.callable_bindings.contains(key) || relation == ReferenceUse::Call
                    {
                        SemanticTokenKind::Function
                    } else {
                        SemanticTokenKind::Variable
                    }
                });
                (kind, vec![mutability(binding.mutable)])
            }
        };
        if relation == ReferenceUse::Write {
            modifiers.push(SemanticTokenModifier::Modification);
        }
        self.identity(span, kind, modifiers)
    }

    fn definition_reference_kind(
        &self,
        definition: &DefinitionInfo,
        relation: ReferenceUse,
    ) -> SemanticTokenKind {
        if self.resolved.trait_member(&definition.id).is_some()
            || self.resolved.impl_member(&definition.id).is_some()
        {
            return SemanticTokenKind::Method;
        }
        match definition.kind {
            DefinitionKind::Type => SemanticTokenKind::Type,
            DefinitionKind::Constructor => SemanticTokenKind::EnumMember,
            DefinitionKind::Builtin => SemanticTokenKind::Function,
            DefinitionKind::Value => {
                if relation == ReferenceUse::Call || self.definition_is_callable(definition) {
                    SemanticTokenKind::Function
                } else {
                    SemanticTokenKind::Variable
                }
            }
        }
    }

    fn definition_is_callable(&self, definition: &DefinitionInfo) -> bool {
        let DefinitionOrigin::User { module } = definition.origin else {
            return definition.kind == DefinitionKind::Builtin;
        };
        self.resolved.module(module).is_some_and(|resolved_module| {
            resolved_module.hir.definitions.iter().any(|candidate| {
                candidate.name.span == definition.span.unwrap_or(candidate.span)
                    && !candidate.parameters.is_empty()
            }) || resolved_module.hir.impls.iter().any(|implementation| {
                implementation.members.iter().any(|candidate| {
                    candidate.name.span == definition.span.unwrap_or(candidate.span)
                        && !candidate.parameters.is_empty()
                })
            })
        })
    }

    fn type_syntax(&mut self, syntax: &TypeSyntax) -> Result<(), SemanticTokenIndexError> {
        self.type_syntax_with_head(syntax, None)
    }

    fn constraint_syntax(&mut self, syntax: &TypeSyntax) -> Result<(), SemanticTokenIndexError> {
        self.type_syntax_with_head(syntax, Some(SemanticTokenKind::Interface))
    }

    fn type_syntax_with_head(
        &mut self,
        syntax: &TypeSyntax,
        first_kind: Option<SemanticTokenKind>,
    ) -> Result<(), SemanticTokenIndexError> {
        let mut position = 0;
        let mut named_group = 0;
        while position < syntax.atoms.len() {
            match &syntax.atoms[position] {
                TypeAtom::Name(_) => {
                    let start = position;
                    let mut names = Vec::new();
                    let mut name_positions = Vec::new();
                    while let Some(TypeAtom::Name(name)) = syntax.atoms.get(position) {
                        names.push(name.normalized.as_str());
                        name_positions.push(position);
                        position += 1;
                        if !syntax
                            .atoms
                            .get(position)
                            .is_some_and(|atom| matches!(atom, TypeAtom::Dot))
                        {
                            break;
                        }
                        position += 1;
                    }
                    let last = name_positions.len().saturating_sub(1);
                    for (index, atom_position) in name_positions.into_iter().enumerate() {
                        let TypeAtom::Name(name) = &syntax.atoms[atom_position] else {
                            unreachable!("name positions contain only names")
                        };
                        let kind = if index < last {
                            SemanticTokenKind::Namespace
                        } else {
                            first_kind
                                .filter(|_| named_group == 0)
                                .unwrap_or(SemanticTokenKind::Type)
                        };
                        let modifiers = if index == last
                            && kind == SemanticTokenKind::Type
                            && self.type_reference_is_default_library(&names)
                        {
                            vec![SemanticTokenModifier::DefaultLibrary]
                        } else {
                            Vec::new()
                        };
                        self.structure(name.span, kind, modifiers)?;
                    }
                    named_group += 1;
                    debug_assert!(position > start);
                }
                TypeAtom::Variable(name) => {
                    self.structure(name.span, SemanticTokenKind::TypeParameter, Vec::new())?;
                    position += 1;
                }
                TypeAtom::Arrow
                | TypeAtom::Product
                | TypeAtom::LeftParen
                | TypeAtom::RightParen
                | TypeAtom::LeftAngle
                | TypeAtom::RightAngle
                | TypeAtom::Comma
                | TypeAtom::Dot => position += 1,
            }
        }
        Ok(())
    }

    fn type_reference_is_default_library(&self, names: &[&str]) -> bool {
        match names {
            ["Unit" | "Bool" | "Int" | "f64" | "Text" | "List"] => true,
            [name] => self
                .resolved
                .definition_id(self.module, name)
                .or_else(|| self.resolved.prelude_definition(name))
                .and_then(|id| self.resolved.definition(id))
                .is_some_and(|definition| {
                    definition.kind == DefinitionKind::Type
                        && !matches!(definition.origin, DefinitionOrigin::User { .. })
                }),
            [alias, name] => self
                .resolved
                .module(self.module)
                .and_then(|module| module.imports.get(*alias))
                .and_then(|module| self.resolved.definition_id(*module, name))
                .and_then(|id| self.resolved.definition(id))
                .is_some_and(|definition| {
                    definition.kind == DefinitionKind::Type
                        && !matches!(definition.origin, DefinitionOrigin::User { .. })
                }),
            _ => false,
        }
    }

    fn qualified(
        &mut self,
        name: &hir::QualifiedName,
        final_kind: SemanticTokenKind,
    ) -> Result<(), SemanticTokenIndexError> {
        let last = name.segments.len().saturating_sub(1);
        for (index, segment) in name.segments.iter().enumerate() {
            self.structure(
                segment.span,
                if index == last {
                    final_kind
                } else {
                    SemanticTokenKind::Namespace
                },
                Vec::new(),
            )?;
        }
        Ok(())
    }

    fn checked_declaration(
        &mut self,
        span: Span,
        kind: SemanticTokenKind,
        modifiers: Vec<SemanticTokenModifier>,
    ) -> Result<(), SemanticTokenIndexError> {
        if self.resolved.definitions().values().any(|definition| {
            definition.span == Some(span)
                && matches!(definition.origin, DefinitionOrigin::User { .. })
        }) {
            self.identity(span, kind, modifiers)
        } else {
            self.structure(span, kind, modifiers)
        }
    }

    fn identity(
        &mut self,
        span: Span,
        kind: SemanticTokenKind,
        modifiers: Vec<SemanticTokenModifier>,
    ) -> Result<(), SemanticTokenIndexError> {
        self.builder.add(
            span,
            kind,
            modifiers,
            SemanticTokenEvidence::CheckedIdentity,
            3,
        )
    }

    fn structure(
        &mut self,
        span: Span,
        kind: SemanticTokenKind,
        modifiers: Vec<SemanticTokenModifier>,
    ) -> Result<(), SemanticTokenIndexError> {
        self.builder.add(
            span,
            kind,
            modifiers,
            SemanticTokenEvidence::CheckedStructure,
            2,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReferenceUse {
    Read,
    Write,
    Call,
}

const fn mutability(mutable: bool) -> SemanticTokenModifier {
    if mutable {
        SemanticTokenModifier::Mutable
    } else {
        SemanticTokenModifier::Readonly
    }
}
