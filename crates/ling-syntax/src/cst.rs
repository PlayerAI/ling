use std::ops::Range;

use crate::Token;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NodeKind {
    Program,
    ModuleDeclaration,
    ImportDeclaration,
    CapabilityBlock,
    QualifiedName,
    LetDeclaration,
    TypeDeclaration,
    RecordType,
    FieldDeclaration,
    VariantType,
    VariantCase,
    TypeExpression,
    Pattern,
    Sequence,
    Expression,
    IfExpression,
    MatchExpression,
    MatchCase,
    AssignmentExpression,
    PipelineExpression,
    BinaryExpression,
    UnaryExpression,
    ApplicationExpression,
    ProjectionExpression,
    NameExpression,
    LiteralExpression,
    UnitExpression,
    GroupExpression,
    TupleExpression,
    RecordExpression,
    RecordField,
    RecordUpdate,
    ListExpression,
    Error,
}

impl NodeKind {
    /// Stable spelling used by deterministic syntax-tree projections.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Program => "program",
            Self::ModuleDeclaration => "module_declaration",
            Self::ImportDeclaration => "import_declaration",
            Self::CapabilityBlock => "capability_block",
            Self::QualifiedName => "qualified_name",
            Self::LetDeclaration => "let_declaration",
            Self::TypeDeclaration => "type_declaration",
            Self::RecordType => "record_type",
            Self::FieldDeclaration => "field_declaration",
            Self::VariantType => "variant_type",
            Self::VariantCase => "variant_case",
            Self::TypeExpression => "type_expression",
            Self::Pattern => "pattern",
            Self::Sequence => "sequence",
            Self::Expression => "expression",
            Self::IfExpression => "if_expression",
            Self::MatchExpression => "match_expression",
            Self::MatchCase => "match_case",
            Self::AssignmentExpression => "assignment_expression",
            Self::PipelineExpression => "pipeline_expression",
            Self::BinaryExpression => "binary_expression",
            Self::UnaryExpression => "unary_expression",
            Self::ApplicationExpression => "application_expression",
            Self::ProjectionExpression => "projection_expression",
            Self::NameExpression => "name_expression",
            Self::LiteralExpression => "literal_expression",
            Self::UnitExpression => "unit_expression",
            Self::GroupExpression => "group_expression",
            Self::TupleExpression => "tuple_expression",
            Self::RecordExpression => "record_expression",
            Self::RecordField => "record_field",
            Self::RecordUpdate => "record_update",
            Self::ListExpression => "list_expression",
            Self::Error => "error",
        }
    }
}

/// A lossless CST node referencing a contiguous range in `SyntaxTree::tokens`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CstNode {
    kind: NodeKind,
    token_range: Range<usize>,
    children: Vec<CstNode>,
}

impl CstNode {
    pub(crate) fn new(kind: NodeKind, token_range: Range<usize>, children: Vec<Self>) -> Self {
        Self {
            kind,
            token_range,
            children,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> NodeKind {
        self.kind
    }

    #[must_use]
    pub fn token_range(&self) -> Range<usize> {
        self.token_range.clone()
    }

    #[must_use]
    pub fn children(&self) -> &[Self] {
        &self.children
    }

    #[must_use]
    pub fn tokens<'tree>(&self, tree: &'tree SyntaxTree) -> &'tree [Token] {
        &tree.tokens[self.token_range.clone()]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxTree {
    tokens: Vec<Token>,
    root: CstNode,
}

impl SyntaxTree {
    pub(crate) fn new(tokens: Vec<Token>, root: CstNode) -> Self {
        Self { tokens, root }
    }

    #[must_use]
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    #[must_use]
    pub const fn root(&self) -> &CstNode {
        &self.root
    }
}
