use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

use ling_hir::{self, TypeDefinition};
use ling_resolve::ResolvedModule;
use ling_source::{SourceId, Span};

/// Maximum structural nodes published by one resolved-outline query.
pub const MAX_RESOLVED_OUTLINE_NODES: usize = 4_096;

/// Compiler-owned structural classification for one resolved source outline.
///
/// This taxonomy contains no LSP `SymbolKind` values or wire behavior.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResolvedOutlineKind {
    Module,
    Record,
    Variant,
    Alias,
    Field,
    VariantCase,
    Function,
    Constant,
    Trait,
    TraitMember,
    Implementation,
    ImplementationMember,
}

/// One immutable resolved structural node with original UTF-8 byte spans.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedOutlineNode {
    name: String,
    kind: ResolvedOutlineKind,
    span: Span,
    selection_span: Span,
    children: Box<[ResolvedOutlineNode]>,
}

impl ResolvedOutlineNode {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn kind(&self) -> ResolvedOutlineKind {
        self.kind
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    #[must_use]
    pub const fn selection_span(&self) -> Span {
        self.selection_span
    }

    #[must_use]
    pub fn children(&self) -> &[ResolvedOutlineNode] {
        &self.children
    }
}

/// One complete module-rooted structural outline.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedOutline {
    root: ResolvedOutlineNode,
    node_count: usize,
}

impl ResolvedOutline {
    #[must_use]
    pub const fn root(&self) -> &ResolvedOutlineNode {
        &self.root
    }

    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }

    pub(crate) fn from_module(
        module: &ResolvedModule,
        original: &str,
    ) -> Result<Self, ResolvedOutlineError> {
        let source = module.hir.span.source();
        let mut builder = OutlineBuilder {
            source,
            original,
            count: 0,
        };
        let mut children = Vec::new();
        children.extend(
            module
                .hir
                .definitions
                .iter()
                .map(|definition| builder.definition(definition, false))
                .collect::<Result<Vec<_>, _>>()?,
        );
        children.extend(
            module
                .hir
                .types
                .iter()
                .map(|declaration| builder.type_declaration(declaration))
                .collect::<Result<Vec<_>, _>>()?,
        );
        children.extend(
            module
                .hir
                .traits
                .iter()
                .map(|declaration| builder.trait_declaration(declaration))
                .collect::<Result<Vec<_>, _>>()?,
        );
        children.extend(
            module
                .hir
                .impls
                .iter()
                .map(|declaration| builder.impl_declaration(declaration))
                .collect::<Result<Vec<_>, _>>()?,
        );
        children.sort_by(node_order);

        let module_name = module
            .hir
            .module
            .name
            .segments
            .iter()
            .map(|segment| segment.source.as_str())
            .collect::<Vec<_>>()
            .join(".");
        let root = builder.node(
            module_name,
            ResolvedOutlineKind::Module,
            module.hir.span,
            module.hir.module.name.span,
            children,
        )?;
        Ok(Self {
            root,
            node_count: builder.count,
        })
    }
}

/// Failure to construct a bounded structural outline from validated HIR.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResolvedOutlineError {
    TooManyNodes { maximum: usize },
    InvalidSpan,
    InvalidReceiverText,
    MutableTopLevel,
}

impl fmt::Display for ResolvedOutlineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyNodes { maximum } => {
                write!(formatter, "resolved outline exceeds {maximum} nodes")
            }
            Self::InvalidSpan => formatter.write_str("resolved outline contains an invalid span"),
            Self::InvalidReceiverText => {
                formatter.write_str("implementation receiver span is not valid source text")
            }
            Self::MutableTopLevel => {
                formatter.write_str("resolved outline contains a mutable top-level definition")
            }
        }
    }
}

impl Error for ResolvedOutlineError {}

struct OutlineBuilder<'a> {
    source: SourceId,
    original: &'a str,
    count: usize,
}

impl OutlineBuilder<'_> {
    fn definition(
        &mut self,
        definition: &ling_hir::Definition,
        implementation_member: bool,
    ) -> Result<ResolvedOutlineNode, ResolvedOutlineError> {
        let kind = if implementation_member {
            ResolvedOutlineKind::ImplementationMember
        } else if !definition.parameters.is_empty() {
            ResolvedOutlineKind::Function
        } else {
            if definition.mutable {
                return Err(ResolvedOutlineError::MutableTopLevel);
            }
            ResolvedOutlineKind::Constant
        };
        self.node(
            definition.name.source.clone(),
            kind,
            definition.span,
            definition.name.span,
            Vec::new(),
        )
    }

    fn type_declaration(
        &mut self,
        declaration: &ling_hir::TypeDeclaration,
    ) -> Result<ResolvedOutlineNode, ResolvedOutlineError> {
        let (kind, children) = match &declaration.definition {
            TypeDefinition::Record(fields) => (
                ResolvedOutlineKind::Record,
                fields
                    .iter()
                    .map(|field| {
                        self.node(
                            field.name.source.clone(),
                            ResolvedOutlineKind::Field,
                            field.span,
                            field.name.span,
                            Vec::new(),
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            TypeDefinition::Variant(cases) => (
                ResolvedOutlineKind::Variant,
                cases
                    .iter()
                    .map(|case| {
                        self.node(
                            case.name.source.clone(),
                            ResolvedOutlineKind::VariantCase,
                            case.span,
                            case.name.span,
                            Vec::new(),
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            TypeDefinition::Alias(_) => (ResolvedOutlineKind::Alias, Vec::new()),
        };
        self.node(
            declaration.name.source.clone(),
            kind,
            declaration.span,
            declaration.name.span,
            children,
        )
    }

    fn trait_declaration(
        &mut self,
        declaration: &ling_hir::TraitDeclaration,
    ) -> Result<ResolvedOutlineNode, ResolvedOutlineError> {
        let children = declaration
            .members
            .iter()
            .map(|member| {
                self.node(
                    member.name.source.clone(),
                    ResolvedOutlineKind::TraitMember,
                    member.span,
                    member.name.span,
                    Vec::new(),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.node(
            declaration.name.source.clone(),
            ResolvedOutlineKind::Trait,
            declaration.span,
            declaration.name.span,
            children,
        )
    }

    fn impl_declaration(
        &mut self,
        declaration: &ling_hir::ImplDeclaration,
    ) -> Result<ResolvedOutlineNode, ResolvedOutlineError> {
        let receiver = self.source_text(declaration.receiver.span)?;
        let name = format!(
            "impl {} {}",
            declaration.trait_name.normalized(),
            receiver.trim()
        );
        let children = declaration
            .members
            .iter()
            .map(|member| self.definition(member, true))
            .collect::<Result<Vec<_>, _>>()?;
        self.node(
            name,
            ResolvedOutlineKind::Implementation,
            declaration.span,
            declaration.trait_name.span,
            children,
        )
    }

    fn source_text(&self, span: Span) -> Result<&str, ResolvedOutlineError> {
        if span.source() != self.source {
            return Err(ResolvedOutlineError::InvalidReceiverText);
        }
        let start = usize::try_from(span.start().get())
            .map_err(|_| ResolvedOutlineError::InvalidReceiverText)?;
        let end = usize::try_from(span.end().get())
            .map_err(|_| ResolvedOutlineError::InvalidReceiverText)?;
        self.original
            .get(start..end)
            .filter(|text| !text.trim().is_empty())
            .ok_or(ResolvedOutlineError::InvalidReceiverText)
    }

    fn node(
        &mut self,
        name: String,
        kind: ResolvedOutlineKind,
        span: Span,
        selection_span: Span,
        mut children: Vec<ResolvedOutlineNode>,
    ) -> Result<ResolvedOutlineNode, ResolvedOutlineError> {
        let span_end =
            usize::try_from(span.end().get()).map_err(|_| ResolvedOutlineError::InvalidSpan)?;
        if span.source() != self.source
            || selection_span.source() != self.source
            || selection_span.start() < span.start()
            || selection_span.end() > span.end()
            || span_end > self.original.len()
        {
            return Err(ResolvedOutlineError::InvalidSpan);
        }
        children.sort_by(node_order);
        if children.iter().any(|child| {
            child.span.start() < span.start()
                || child.span.end() > span.end()
                || child.span.source() != span.source()
        }) {
            return Err(ResolvedOutlineError::InvalidSpan);
        }
        self.count = self
            .count
            .checked_add(1)
            .ok_or(ResolvedOutlineError::TooManyNodes {
                maximum: MAX_RESOLVED_OUTLINE_NODES,
            })?;
        if self.count > MAX_RESOLVED_OUTLINE_NODES {
            return Err(ResolvedOutlineError::TooManyNodes {
                maximum: MAX_RESOLVED_OUTLINE_NODES,
            });
        }
        Ok(ResolvedOutlineNode {
            name,
            kind,
            span,
            selection_span,
            children: children.into_boxed_slice(),
        })
    }
}

fn node_order(left: &ResolvedOutlineNode, right: &ResolvedOutlineNode) -> Ordering {
    left.span
        .start()
        .cmp(&right.span.start())
        .then_with(|| left.span.end().cmp(&right.span.end()))
        .then_with(|| left.kind.cmp(&right.kind))
        .then_with(|| left.name.cmp(&right.name))
}
