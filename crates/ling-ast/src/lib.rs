//! Trivia-free, span-preserving abstract syntax for Ling Seed.

use std::error::Error;
use std::fmt;

use ling_source::{ByteOffset, SourceFile, Span};
use ling_syntax::{CstNode, NodeKind, ParsedSource, SyntaxTree, Token, TokenKind, TokenValue};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub span: Span,
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Item {
    Module(ModuleDeclaration),
    Import(ImportDeclaration),
    Let(LetDeclaration),
    Type(TypeDeclaration),
    Trait(TraitDeclaration),
    Impl(ImplDeclaration),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleDeclaration {
    pub span: Span,
    pub name: QualifiedName,
    pub requires: Vec<QualifiedName>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportDeclaration {
    pub span: Span,
    pub module: QualifiedName,
    pub alias: Option<Name>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedName {
    pub span: Span,
    pub segments: Vec<Name>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Name {
    pub span: Span,
    pub source: String,
    pub normalized: String,
    pub skeleton: String,
    pub scripts: Vec<String>,
    pub suspicious_mixed_script: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LetDeclaration {
    pub span: Span,
    pub recursive: bool,
    pub mutable: bool,
    pub binding: Pattern,
    pub type_parameters: Vec<Name>,
    pub constraints: Vec<TypeExpression>,
    pub parameters: Vec<Pattern>,
    pub annotation: Option<TypeExpression>,
    pub value: Expression,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraitDeclaration {
    pub span: Span,
    pub name: Name,
    pub parameters: Vec<Name>,
    pub members: Vec<TraitMember>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraitMember {
    pub span: Span,
    pub name: Name,
    pub signature: TypeExpression,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImplDeclaration {
    pub span: Span,
    pub trait_name: QualifiedName,
    pub receiver: TypeExpression,
    pub members: Vec<LetDeclaration>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeDeclaration {
    pub span: Span,
    pub name: Name,
    pub parameters: Vec<Name>,
    pub definition: TypeDefinition,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeDefinition {
    Record(Vec<TypeField>),
    Variant(Vec<VariantCase>),
    Alias(TypeExpression),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeField {
    pub span: Span,
    pub name: Name,
    pub mutable: bool,
    pub field_type: TypeExpression,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariantCase {
    pub span: Span,
    pub name: Name,
    pub payload: Option<TypeExpression>,
}

/// A syntax-preserving type expression with trivia and layout removed.
///
/// Type precedence is intentionally lowered in M3, where the accepted type
/// grammar and namespace rules are available together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeExpression {
    pub span: Span,
    pub atoms: Vec<TypeAtom>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeAtom {
    Name(Name),
    Variable(Name),
    Arrow,
    Product,
    LeftParen,
    RightParen,
    LeftAngle,
    RightAngle,
    Comma,
    Dot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pattern {
    pub span: Span,
    pub atoms: Vec<PatternAtom>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PatternAtom {
    Name(Name),
    Literal(Literal),
    Dot,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Equals,
    Semicolon,
    Comma,
}

/// An unresolved operation clause preserved by the experimental handler AST
/// projection. Operation lookup and resume typing belong to a later checked
/// Core boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandlerClause {
    pub span: Span,
    pub operation: QualifiedName,
    pub parameters: Vec<Pattern>,
    pub resume: Option<Name>,
    pub body: Expression,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expression {
    pub span: Span,
    pub kind: ExpressionKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpressionKind {
    Sequence(Vec<SequenceElement>),
    Handle {
        body: Box<Expression>,
        clauses: Vec<HandlerClause>,
    },
    If {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Box<Expression>,
    },
    Match {
        scrutinee: Box<Expression>,
        cases: Vec<MatchCase>,
    },
    Assignment {
        place: Box<Expression>,
        value: Box<Expression>,
    },
    /// Preserved until HIR applies DEC-0004 final-argument lowering.
    Pipeline {
        input: Box<Expression>,
        target: Box<Expression>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Application {
        function: Box<Expression>,
        argument: Box<Expression>,
    },
    Projection {
        target: Box<Expression>,
        field: Name,
    },
    Name(Name),
    Literal(Literal),
    Unit,
    Tuple(Vec<Expression>),
    Record(Vec<RecordField>),
    RecordUpdate {
        base: Box<Expression>,
        fields: Vec<RecordField>,
    },
    List(Vec<Expression>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SequenceElement {
    Let(LetDeclaration),
    Expression(Expression),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchCase {
    pub span: Span,
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Expression,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordField {
    pub span: Span,
    pub name: Name,
    pub value: Expression,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    Integer { radix: u32, digits: String },
    Float(String),
    Text(String),
    Boolean(bool),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnaryOperator {
    Positive,
    Negative,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryOperator {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    BooleanAnd,
    BooleanOr,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerError {
    kind: LowerErrorKind,
    span: Option<Span>,
}

impl LowerError {
    #[must_use]
    pub const fn kind(&self) -> &LowerErrorKind {
        &self.kind
    }

    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        self.span
    }
}

impl fmt::Display for LowerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            LowerErrorKind::InvalidCst => formatter.write_str("cannot lower an invalid CST"),
            LowerErrorKind::UnexpectedNode(kind) => {
                write!(
                    formatter,
                    "unexpected CST node during AST lowering: {kind:?}"
                )
            }
            LowerErrorKind::MissingChild(context) => {
                write!(formatter, "CST is missing a child for {context}")
            }
            LowerErrorKind::MissingToken(context) => {
                write!(formatter, "CST is missing a token for {context}")
            }
        }
    }
}

impl Error for LowerError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LowerErrorKind {
    InvalidCst,
    UnexpectedNode(NodeKind),
    MissingChild(&'static str),
    MissingToken(&'static str),
}

/// Lowers a valid lossless CST into the public Ling AST.
pub fn lower(source: &SourceFile, parsed: &ParsedSource) -> Result<Program, LowerError> {
    if !parsed.is_valid() {
        return Err(LowerError {
            kind: LowerErrorKind::InvalidCst,
            span: first_error_span(parsed),
        });
    }
    Lowerer::new(source, parsed.tree()).program(parsed.tree().root())
}

struct Lowerer<'input> {
    source: &'input SourceFile,
    tree: &'input SyntaxTree,
}

impl<'input> Lowerer<'input> {
    const fn new(source: &'input SourceFile, tree: &'input SyntaxTree) -> Self {
        Self { source, tree }
    }

    fn program(&self, node: &CstNode) -> Result<Program, LowerError> {
        self.expect_kind(node, NodeKind::Program)?;
        let items = node
            .children()
            .iter()
            .map(|child| self.item(child))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Program {
            span: self.node_span(node)?,
            items,
        })
    }

    fn item(&self, node: &CstNode) -> Result<Item, LowerError> {
        match node.kind() {
            NodeKind::ModuleDeclaration => self.module_declaration(node).map(Item::Module),
            NodeKind::ImportDeclaration => self.import_declaration(node).map(Item::Import),
            NodeKind::LetDeclaration => self.let_declaration(node).map(Item::Let),
            NodeKind::TypeDeclaration => self.type_declaration(node).map(Item::Type),
            NodeKind::TraitDeclaration => self.trait_declaration(node).map(Item::Trait),
            NodeKind::ImplDeclaration => self.impl_declaration(node).map(Item::Impl),
            kind => Err(self.unexpected(node, kind)),
        }
    }

    fn module_declaration(&self, node: &CstNode) -> Result<ModuleDeclaration, LowerError> {
        let mut children = node.children().iter();
        let name = self.qualified_name(self.child(&mut children, node, "module name")?)?;
        let mut requires = Vec::new();
        if let Some(capabilities) = children.next() {
            for capability in capabilities.children() {
                requires.push(self.qualified_name(capability)?);
            }
        }
        Ok(ModuleDeclaration {
            span: self.node_span(node)?,
            name,
            requires,
        })
    }

    fn qualified_name(&self, node: &CstNode) -> Result<QualifiedName, LowerError> {
        self.expect_kind(node, NodeKind::QualifiedName)?;
        let segments = self
            .significant_tokens(node)
            .filter(|token| token.kind() == TokenKind::Identifier)
            .map(|token| self.name(token))
            .collect::<Result<Vec<_>, _>>()?;
        if segments.is_empty() {
            return Err(self.missing_token(node, "qualified name"));
        }
        Ok(QualifiedName {
            span: self.node_span(node)?,
            segments,
        })
    }

    fn import_declaration(&self, node: &CstNode) -> Result<ImportDeclaration, LowerError> {
        self.expect_kind(node, NodeKind::ImportDeclaration)?;
        let module_node = node
            .children()
            .first()
            .ok_or_else(|| self.missing_child(node, "imported module name"))?;
        let alias = self
            .tokens_after(node, module_node.token_range().end)
            .into_iter()
            .find(|token| token.kind() == TokenKind::Identifier)
            .map(|token| self.name(token))
            .transpose()?;
        Ok(ImportDeclaration {
            span: self.node_span(node)?,
            module: self.qualified_name(module_node)?,
            alias,
        })
    }

    fn let_declaration(&self, node: &CstNode) -> Result<LetDeclaration, LowerError> {
        self.expect_kind(node, NodeKind::LetDeclaration)?;
        let header = self.direct_tokens_before_children(node);
        let recursive = header.iter().any(|token| token.kind() == TokenKind::Rec);
        let mutable = header
            .iter()
            .any(|token| token.kind() == TokenKind::Mutable);
        let mut children = node.children().iter();
        let binding = self.pattern(self.child(&mut children, node, "binding pattern")?)?;
        let remaining = children.collect::<Vec<_>>();
        let value_node = remaining
            .last()
            .copied()
            .ok_or_else(|| self.missing_child(node, "let value"))?;
        let type_parameters = remaining
            .iter()
            .find(|child| child.kind() == NodeKind::TypeParameterList)
            .map(|child| {
                self.significant_tokens(child)
                    .filter(|token| token.kind() == TokenKind::Identifier)
                    .map(|token| self.name(token))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?
            .unwrap_or_default();
        let constraints = remaining
            .iter()
            .find(|child| child.kind() == NodeKind::ConstraintBlock)
            .map(|child| {
                child
                    .children()
                    .iter()
                    .map(|constraint| self.type_expression(constraint))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?
            .unwrap_or_default();
        let annotation_index = remaining
            .iter()
            .position(|child| child.kind() == NodeKind::TypeExpression);
        let parameters = remaining
            .iter()
            .filter(|child| child.kind() == NodeKind::Pattern)
            .map(|child| self.pattern(child))
            .collect::<Result<Vec<_>, _>>()?;
        let annotation = annotation_index
            .map(|index| self.type_expression(remaining[index]))
            .transpose()?;
        Ok(LetDeclaration {
            span: self.node_span(node)?,
            recursive,
            mutable,
            binding,
            type_parameters,
            constraints,
            parameters,
            annotation,
            value: self.expression(value_node)?,
        })
    }

    fn trait_declaration(&self, node: &CstNode) -> Result<TraitDeclaration, LowerError> {
        self.expect_kind(node, NodeKind::TraitDeclaration)?;
        let body_start = node
            .children()
            .iter()
            .position(|child| child.kind() == NodeKind::TraitMember)
            .unwrap_or(node.children().len());
        let header = self.tokens_before(
            node,
            node.children()
                .get(body_start)
                .map_or(node.token_range().end, |child| child.token_range().start),
        );
        let identifiers = header
            .iter()
            .filter(|token| token.kind() == TokenKind::Identifier)
            .collect::<Vec<_>>();
        let name = identifiers
            .first()
            .copied()
            .ok_or_else(|| self.missing_token(node, "Trait name"))
            .and_then(|token| self.name(token))?;
        let parameters = node
            .children()
            .iter()
            .find(|child| child.kind() == NodeKind::TypeParameterList)
            .map(|child| {
                self.significant_tokens(child)
                    .filter(|token| token.kind() == TokenKind::Identifier)
                    .map(|token| self.name(token))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?
            .unwrap_or_default();
        let members = node
            .children()
            .iter()
            .filter(|child| child.kind() == NodeKind::TraitMember)
            .map(|child| self.trait_member(child))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(TraitDeclaration {
            span: self.node_span(node)?,
            name,
            parameters,
            members,
        })
    }

    fn trait_member(&self, node: &CstNode) -> Result<TraitMember, LowerError> {
        self.expect_kind(node, NodeKind::TraitMember)?;
        let signature = node
            .children()
            .first()
            .ok_or_else(|| self.missing_child(node, "Trait member signature"))?;
        let name = self
            .tokens_before(node, signature.token_range().start)
            .into_iter()
            .find(|token| token.kind() == TokenKind::Identifier)
            .ok_or_else(|| self.missing_token(node, "Trait member name"))
            .and_then(|token| self.name(token))?;
        Ok(TraitMember {
            span: self.node_span(node)?,
            name,
            signature: self.type_expression(signature)?,
        })
    }

    fn impl_declaration(&self, node: &CstNode) -> Result<ImplDeclaration, LowerError> {
        self.expect_kind(node, NodeKind::ImplDeclaration)?;
        let headers = node
            .children()
            .iter()
            .filter(|child| child.kind() != NodeKind::ImplMember)
            .collect::<Vec<_>>();
        if headers.len() != 2 {
            return Err(self.missing_child(node, "impl header"));
        }
        let trait_name = headers[0];
        let receiver = headers[1];
        let members = node
            .children()
            .iter()
            .filter(|child| child.kind() == NodeKind::ImplMember)
            .map(|child| {
                let declaration = child
                    .children()
                    .first()
                    .ok_or_else(|| self.missing_child(child, "impl member"))?;
                self.let_declaration(declaration)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ImplDeclaration {
            span: self.node_span(node)?,
            trait_name: self.qualified_name(trait_name)?,
            receiver: self.type_expression(receiver)?,
            members,
        })
    }

    fn type_declaration(&self, node: &CstNode) -> Result<TypeDeclaration, LowerError> {
        self.expect_kind(node, NodeKind::TypeDeclaration)?;
        let body = node
            .children()
            .first()
            .ok_or_else(|| self.missing_child(node, "type definition"))?;
        let header = self.tokens_before(node, body.token_range().start);
        let identifiers = header
            .iter()
            .filter(|token| token.kind() == TokenKind::Identifier)
            .collect::<Vec<_>>();
        let name = identifiers
            .first()
            .copied()
            .ok_or_else(|| self.missing_token(node, "type name"))
            .and_then(|token| self.name(token))?;
        let parameters = identifiers[1..]
            .iter()
            .map(|token| self.name(token))
            .collect::<Result<Vec<_>, _>>()?;
        let definition = match body.kind() {
            NodeKind::RecordType => TypeDefinition::Record(
                body.children()
                    .iter()
                    .map(|child| self.type_field(child))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            NodeKind::VariantType => TypeDefinition::Variant(
                body.children()
                    .iter()
                    .map(|child| self.variant_case(child))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            NodeKind::TypeExpression => TypeDefinition::Alias(self.type_expression(body)?),
            kind => return Err(self.unexpected(body, kind)),
        };
        Ok(TypeDeclaration {
            span: self.node_span(node)?,
            name,
            parameters,
            definition,
        })
    }

    fn type_field(&self, node: &CstNode) -> Result<TypeField, LowerError> {
        self.expect_kind(node, NodeKind::FieldDeclaration)?;
        let field_type = node
            .children()
            .first()
            .ok_or_else(|| self.missing_child(node, "record field type"))?;
        let header = self.tokens_before(node, field_type.token_range().start);
        let name = header
            .iter()
            .find(|token| token.kind() == TokenKind::Identifier)
            .ok_or_else(|| self.missing_token(node, "record field name"))
            .and_then(|token| self.name(token))?;
        Ok(TypeField {
            span: self.node_span(node)?,
            name,
            mutable: header
                .iter()
                .any(|token| token.kind() == TokenKind::Mutable),
            field_type: self.type_expression(field_type)?,
        })
    }

    fn variant_case(&self, node: &CstNode) -> Result<VariantCase, LowerError> {
        self.expect_kind(node, NodeKind::VariantCase)?;
        let name = self
            .significant_tokens(node)
            .find(|token| token.kind() == TokenKind::Identifier)
            .ok_or_else(|| self.missing_token(node, "variant case name"))
            .and_then(|token| self.name(token))?;
        let payload = node
            .children()
            .first()
            .map(|child| self.type_expression(child))
            .transpose()?;
        Ok(VariantCase {
            span: self.node_span(node)?,
            name,
            payload,
        })
    }

    fn type_expression(&self, node: &CstNode) -> Result<TypeExpression, LowerError> {
        self.expect_kind(node, NodeKind::TypeExpression)?;
        let tokens = self.significant_tokens(node).collect::<Vec<_>>();
        let mut atoms = Vec::new();
        let mut variable = false;
        for token in tokens {
            let atom = match token.kind() {
                TokenKind::Apostrophe => {
                    variable = true;
                    continue;
                }
                TokenKind::Identifier if variable => {
                    variable = false;
                    TypeAtom::Variable(self.name(token)?)
                }
                TokenKind::Identifier => TypeAtom::Name(self.name(token)?),
                TokenKind::RightArrow => TypeAtom::Arrow,
                TokenKind::Star => TypeAtom::Product,
                TokenKind::LeftParen => TypeAtom::LeftParen,
                TokenKind::RightParen => TypeAtom::RightParen,
                TokenKind::Less => TypeAtom::LeftAngle,
                TokenKind::Greater => TypeAtom::RightAngle,
                TokenKind::Comma => TypeAtom::Comma,
                TokenKind::Dot => TypeAtom::Dot,
                kind => return Err(self.unexpected(node, token_kind_node(kind))),
            };
            atoms.push(atom);
        }
        Ok(TypeExpression {
            span: self.node_span(node)?,
            atoms,
        })
    }

    fn pattern(&self, node: &CstNode) -> Result<Pattern, LowerError> {
        self.expect_kind(node, NodeKind::Pattern)?;
        let atoms = self
            .significant_tokens(node)
            .map(|token| match token.kind() {
                TokenKind::Identifier => self.name(token).map(PatternAtom::Name),
                TokenKind::Integer
                | TokenKind::Float
                | TokenKind::Text
                | TokenKind::True
                | TokenKind::False => self.literal(token).map(PatternAtom::Literal),
                TokenKind::Dot => Ok(PatternAtom::Dot),
                TokenKind::LeftParen => Ok(PatternAtom::LeftParen),
                TokenKind::RightParen => Ok(PatternAtom::RightParen),
                TokenKind::LeftBrace => Ok(PatternAtom::LeftBrace),
                TokenKind::RightBrace => Ok(PatternAtom::RightBrace),
                TokenKind::Equals => Ok(PatternAtom::Equals),
                TokenKind::Semicolon => Ok(PatternAtom::Semicolon),
                TokenKind::Comma => Ok(PatternAtom::Comma),
                kind => Err(self.unexpected(node, token_kind_node(kind))),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Pattern {
            span: self.node_span(node)?,
            atoms,
        })
    }

    fn expression(&self, node: &CstNode) -> Result<Expression, LowerError> {
        let span = self.node_span(node)?;
        let kind = match node.kind() {
            NodeKind::Sequence => ExpressionKind::Sequence(
                node.children()
                    .iter()
                    .map(|child| match child.kind() {
                        NodeKind::LetDeclaration => {
                            self.let_declaration(child).map(SequenceElement::Let)
                        }
                        _ => self.expression(child).map(SequenceElement::Expression),
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            NodeKind::HandleExpression => {
                let (body, clauses) = node
                    .children()
                    .split_first()
                    .ok_or_else(|| self.missing_child(node, "handler body"))?;
                if clauses.is_empty() {
                    return Err(self.missing_child(node, "handler operation clause"));
                }
                ExpressionKind::Handle {
                    body: Box::new(self.expression(body)?),
                    clauses: clauses
                        .iter()
                        .map(|clause| self.handler_clause(clause))
                        .collect::<Result<Vec<_>, _>>()?,
                }
            }
            NodeKind::IfExpression => {
                let [condition, then_branch, else_branch] = self.three_children(node, "if")?;
                ExpressionKind::If {
                    condition: Box::new(self.expression(condition)?),
                    then_branch: Box::new(self.expression(then_branch)?),
                    else_branch: Box::new(self.expression(else_branch)?),
                }
            }
            NodeKind::MatchExpression => {
                let (scrutinee, cases) = node
                    .children()
                    .split_first()
                    .ok_or_else(|| self.missing_child(node, "match scrutinee"))?;
                ExpressionKind::Match {
                    scrutinee: Box::new(self.expression(scrutinee)?),
                    cases: cases
                        .iter()
                        .map(|case| self.match_case(case))
                        .collect::<Result<Vec<_>, _>>()?,
                }
            }
            NodeKind::AssignmentExpression => {
                let [place, value] = self.two_children(node, "assignment")?;
                ExpressionKind::Assignment {
                    place: Box::new(self.expression(place)?),
                    value: Box::new(self.expression(value)?),
                }
            }
            NodeKind::PipelineExpression => {
                let [input, target] = self.two_children(node, "pipeline")?;
                ExpressionKind::Pipeline {
                    input: Box::new(self.expression(input)?),
                    target: Box::new(self.expression(target)?),
                }
            }
            NodeKind::BinaryExpression => {
                let [left, right] = self.two_children(node, "binary expression")?;
                ExpressionKind::Binary {
                    operator: self.binary_operator(node, left, right)?,
                    left: Box::new(self.expression(left)?),
                    right: Box::new(self.expression(right)?),
                }
            }
            NodeKind::UnaryExpression => {
                let operand = node
                    .children()
                    .first()
                    .ok_or_else(|| self.missing_child(node, "unary operand"))?;
                ExpressionKind::Unary {
                    operator: self.unary_operator(node, operand)?,
                    operand: Box::new(self.expression(operand)?),
                }
            }
            NodeKind::ApplicationExpression => {
                let [function, argument] = self.two_children(node, "application")?;
                ExpressionKind::Application {
                    function: Box::new(self.expression(function)?),
                    argument: Box::new(self.expression(argument)?),
                }
            }
            NodeKind::ProjectionExpression => {
                let target = node
                    .children()
                    .first()
                    .ok_or_else(|| self.missing_child(node, "projection target"))?;
                let field = self
                    .tokens_after(node, target.token_range().end)
                    .into_iter()
                    .find(|token| token.kind() == TokenKind::Identifier)
                    .ok_or_else(|| self.missing_token(node, "projection field"))
                    .and_then(|token| self.name(token))?;
                ExpressionKind::Projection {
                    target: Box::new(self.expression(target)?),
                    field,
                }
            }
            NodeKind::NameExpression => ExpressionKind::Name(
                self.significant_tokens(node)
                    .find(|token| token.kind() == TokenKind::Identifier)
                    .ok_or_else(|| self.missing_token(node, "name expression"))
                    .and_then(|token| self.name(token))?,
            ),
            NodeKind::LiteralExpression => ExpressionKind::Literal(
                self.significant_tokens(node)
                    .next()
                    .ok_or_else(|| self.missing_token(node, "literal expression"))
                    .and_then(|token| self.literal(token))?,
            ),
            NodeKind::UnitExpression => ExpressionKind::Unit,
            NodeKind::GroupExpression => {
                let inner = node
                    .children()
                    .first()
                    .ok_or_else(|| self.missing_child(node, "group expression"))?;
                return self.expression(inner).map(|mut expression| {
                    expression.span = span;
                    expression
                });
            }
            NodeKind::TupleExpression => ExpressionKind::Tuple(
                node.children()
                    .iter()
                    .map(|child| self.expression(child))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            NodeKind::RecordExpression => ExpressionKind::Record(
                node.children()
                    .iter()
                    .map(|child| self.record_field(child))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            NodeKind::RecordUpdate => {
                let (base, fields) = node
                    .children()
                    .split_first()
                    .ok_or_else(|| self.missing_child(node, "record update base"))?;
                ExpressionKind::RecordUpdate {
                    base: Box::new(self.expression(base)?),
                    fields: fields
                        .iter()
                        .map(|child| self.record_field(child))
                        .collect::<Result<Vec<_>, _>>()?,
                }
            }
            NodeKind::ListExpression => ExpressionKind::List(
                node.children()
                    .iter()
                    .map(|child| self.expression(child))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            kind => return Err(self.unexpected(node, kind)),
        };
        Ok(Expression { span, kind })
    }

    fn handler_clause(&self, node: &CstNode) -> Result<HandlerClause, LowerError> {
        self.expect_kind(node, NodeKind::HandlerClause)?;
        let children = node.children();
        if children.len() < 2 {
            return Err(self.missing_child(node, "handler operation clause"));
        }
        let operation = self.qualified_name(&children[0])?;
        let body_node = children
            .last()
            .ok_or_else(|| self.missing_child(node, "handler operation body"))?;
        let body = self.expression(body_node)?;
        let parameter_nodes = &children[1..children.len() - 1];
        if parameter_nodes
            .iter()
            .any(|child| child.kind() != NodeKind::Pattern)
        {
            return Err(self.unexpected(node, NodeKind::HandlerClause));
        }
        let parameters = parameter_nodes
            .iter()
            .map(|parameter| self.pattern(parameter))
            .collect::<Result<Vec<_>, _>>()?;
        let resume = self
            .unclaimed_contextual_token(node, body_node, "resume")
            .map(|token| self.name(token))
            .transpose()?;
        Ok(HandlerClause {
            span: self.node_span(node)?,
            operation,
            parameters,
            resume,
            body,
        })
    }

    fn match_case(&self, node: &CstNode) -> Result<MatchCase, LowerError> {
        self.expect_kind(node, NodeKind::MatchCase)?;
        let children = node.children();
        if children.len() < 2 {
            return Err(self.missing_child(node, "match case"));
        }
        let pattern = self.pattern(&children[0])?;
        let (guard, body_index) = if children.len() == 3 {
            (Some(self.expression(&children[1])?), 2)
        } else {
            (None, 1)
        };
        Ok(MatchCase {
            span: self.node_span(node)?,
            pattern,
            guard,
            body: self.expression(&children[body_index])?,
        })
    }

    fn record_field(&self, node: &CstNode) -> Result<RecordField, LowerError> {
        self.expect_kind(node, NodeKind::RecordField)?;
        let value_node = node
            .children()
            .first()
            .ok_or_else(|| self.missing_child(node, "record field value"))?;
        let name = self
            .tokens_before(node, value_node.token_range().start)
            .into_iter()
            .find(|token| token.kind() == TokenKind::Identifier)
            .ok_or_else(|| self.missing_token(node, "record field name"))
            .and_then(|token| self.name(token))?;
        Ok(RecordField {
            span: self.node_span(node)?,
            name,
            value: self.expression(value_node)?,
        })
    }

    fn name(&self, token: &Token) -> Result<Name, LowerError> {
        let Some(TokenValue::Identifier(security)) = token.value() else {
            return Err(LowerError {
                kind: LowerErrorKind::MissingToken("identifier metadata"),
                span: Some(token.span()),
            });
        };
        Ok(Name {
            span: token.span(),
            source: security.identifier().original().to_owned(),
            normalized: security.identifier().normalized().to_owned(),
            skeleton: security.skeleton().to_owned(),
            scripts: security
                .scripts()
                .iter()
                .map(|script| script.as_str().to_owned())
                .collect(),
            suspicious_mixed_script: security.has_suspicious_mixed_script(),
        })
    }

    fn literal(&self, token: &Token) -> Result<Literal, LowerError> {
        match (token.kind(), token.value()) {
            (TokenKind::Integer, Some(TokenValue::Integer(integer))) => Ok(Literal::Integer {
                radix: integer.radix(),
                digits: integer.digits().to_owned(),
            }),
            (TokenKind::Float, Some(TokenValue::Float(float))) => {
                Ok(Literal::Float(float.normalized().to_owned()))
            }
            (TokenKind::Text, Some(TokenValue::Text(text))) => Ok(Literal::Text(text.clone())),
            (TokenKind::True, None) => Ok(Literal::Boolean(true)),
            (TokenKind::False, None) => Ok(Literal::Boolean(false)),
            _ => Err(LowerError {
                kind: LowerErrorKind::MissingToken("literal metadata"),
                span: Some(token.span()),
            }),
        }
    }

    fn binary_operator(
        &self,
        node: &CstNode,
        left: &CstNode,
        right: &CstNode,
    ) -> Result<BinaryOperator, LowerError> {
        let tokens = &self.tree.tokens()[left.token_range().end..right.token_range().start];
        tokens
            .iter()
            .find_map(|token| match token.kind() {
                TokenKind::EqualEqual => Some(BinaryOperator::Equal),
                TokenKind::BangEqual => Some(BinaryOperator::NotEqual),
                TokenKind::Less => Some(BinaryOperator::Less),
                TokenKind::LessEqual => Some(BinaryOperator::LessEqual),
                TokenKind::Greater => Some(BinaryOperator::Greater),
                TokenKind::GreaterEqual => Some(BinaryOperator::GreaterEqual),
                TokenKind::Plus => Some(BinaryOperator::Add),
                TokenKind::Minus => Some(BinaryOperator::Subtract),
                TokenKind::Star => Some(BinaryOperator::Multiply),
                TokenKind::Slash => Some(BinaryOperator::Divide),
                TokenKind::Percent => Some(BinaryOperator::Remainder),
                TokenKind::AmpAmp => Some(BinaryOperator::BooleanAnd),
                TokenKind::PipePipe => Some(BinaryOperator::BooleanOr),
                _ => None,
            })
            .ok_or_else(|| self.missing_token(node, "binary operator"))
    }

    fn unary_operator(
        &self,
        node: &CstNode,
        operand: &CstNode,
    ) -> Result<UnaryOperator, LowerError> {
        self.tree.tokens()[node.token_range().start..operand.token_range().start]
            .iter()
            .find_map(|token| match token.kind() {
                TokenKind::Plus => Some(UnaryOperator::Positive),
                TokenKind::Minus => Some(UnaryOperator::Negative),
                _ => None,
            })
            .ok_or_else(|| self.missing_token(node, "unary operator"))
    }

    fn node_span(&self, node: &CstNode) -> Result<Span, LowerError> {
        self.raw_node_span(node)
            .ok_or_else(|| self.error_without_span(LowerErrorKind::MissingToken("node span")))
    }

    fn raw_node_span(&self, node: &CstNode) -> Option<Span> {
        let mut tokens = self.significant_tokens(node);
        let first = tokens.next();
        let last = tokens.last().or(first);
        match (first, last) {
            (Some(first), Some(last)) => Span::new(
                first.span().source(),
                first.span().start(),
                last.span().end(),
            )
            .ok(),
            _ if node.kind() == NodeKind::Program => {
                Span::new(self.source.id(), ByteOffset::new(0), ByteOffset::new(0)).ok()
            }
            _ => None,
        }
    }

    fn significant_tokens<'node>(
        &'node self,
        node: &'node CstNode,
    ) -> impl Iterator<Item = &'input Token> + 'node {
        node.tokens(self.tree)
            .iter()
            .filter(|token| !token.kind().is_trivia() && !token.kind().is_layout())
            .filter(|token| token.kind() != TokenKind::Eof)
    }

    fn direct_tokens_before_children(&self, node: &CstNode) -> Vec<&'input Token> {
        let end = node
            .children()
            .first()
            .map_or(node.token_range().end, |child| child.token_range().start);
        self.tokens_before(node, end)
    }

    fn tokens_before(&self, node: &CstNode, end: usize) -> Vec<&'input Token> {
        self.tree.tokens()[node.token_range().start..end]
            .iter()
            .filter(|token| !token.kind().is_trivia() && !token.kind().is_layout())
            .collect()
    }

    fn tokens_after(&self, node: &CstNode, start: usize) -> Vec<&'input Token> {
        self.tree.tokens()[start..node.token_range().end]
            .iter()
            .filter(|token| !token.kind().is_trivia() && !token.kind().is_layout())
            .collect()
    }

    fn unclaimed_contextual_token(
        &self,
        node: &CstNode,
        body: &CstNode,
        spelling: &str,
    ) -> Option<&'input Token> {
        let child_ranges = node
            .children()
            .iter()
            .map(CstNode::token_range)
            .collect::<Vec<_>>();
        let range = node.token_range();
        let body_start = body.token_range().start;
        self.tree.tokens()[range.start..body_start]
            .iter()
            .enumerate()
            .map(|(offset, token)| (range.start + offset, token))
            .filter(|(index, _)| {
                !child_ranges
                    .iter()
                    .any(|child| child.start <= *index && *index < child.end)
            })
            .map(|(_, token)| token)
            .find(|token| {
                token.kind() == TokenKind::Identifier
                    && matches!(
                        token.value(),
                        Some(TokenValue::Identifier(identifier))
                            if identifier.identifier().normalized() == spelling
                    )
            })
    }

    fn two_children<'node>(
        &self,
        node: &'node CstNode,
        context: &'static str,
    ) -> Result<[&'node CstNode; 2], LowerError> {
        match node.children() {
            [first, second] => Ok([first, second]),
            _ => Err(self.missing_child(node, context)),
        }
    }

    fn three_children<'node>(
        &self,
        node: &'node CstNode,
        context: &'static str,
    ) -> Result<[&'node CstNode; 3], LowerError> {
        match node.children() {
            [first, second, third] => Ok([first, second, third]),
            _ => Err(self.missing_child(node, context)),
        }
    }

    fn child<'node, I>(
        &self,
        children: &mut I,
        parent: &CstNode,
        context: &'static str,
    ) -> Result<&'node CstNode, LowerError>
    where
        I: Iterator<Item = &'node CstNode>,
    {
        children
            .next()
            .ok_or_else(|| self.missing_child(parent, context))
    }

    fn expect_kind(&self, node: &CstNode, expected: NodeKind) -> Result<(), LowerError> {
        if node.kind() == expected {
            Ok(())
        } else {
            Err(self.unexpected(node, node.kind()))
        }
    }

    fn unexpected(&self, node: &CstNode, kind: NodeKind) -> LowerError {
        LowerError {
            kind: LowerErrorKind::UnexpectedNode(kind),
            span: self.raw_node_span(node),
        }
    }

    fn missing_child(&self, node: &CstNode, context: &'static str) -> LowerError {
        LowerError {
            kind: LowerErrorKind::MissingChild(context),
            span: self.raw_node_span(node),
        }
    }

    fn missing_token(&self, node: &CstNode, context: &'static str) -> LowerError {
        LowerError {
            kind: LowerErrorKind::MissingToken(context),
            span: self.raw_node_span(node),
        }
    }

    const fn error_without_span(&self, kind: LowerErrorKind) -> LowerError {
        LowerError { kind, span: None }
    }
}

fn first_error_span(parsed: &ParsedSource) -> Option<Span> {
    parsed
        .lexical_errors()
        .first()
        .map(ling_syntax::LexError::span)
        .or_else(|| {
            parsed
                .parse_errors()
                .first()
                .map(ling_syntax::ParseError::span)
        })
}

// Only used to report an impossible token in a structurally valid CST without
// adding a second error enum solely for internal corruption.
const fn token_kind_node(_kind: TokenKind) -> NodeKind {
    NodeKind::Error
}

#[cfg(test)]
mod tests {
    use super::*;
    use ling_source::SourceId;
    use ling_syntax::parse;

    fn lower_text(text: &str) -> Program {
        let source =
            SourceFile::from_bytes(SourceId::new(1), "test.ling", text.as_bytes().to_vec())
                .expect("valid source");
        let parsed = parse(&source);
        assert!(
            parsed.is_valid(),
            "lexical={:?}, parse={:?}",
            parsed.lexical_errors(),
            parsed.parse_errors()
        );
        lower(&source, &parsed).expect("valid CST lowers")
    }

    #[test]
    fn lowers_module_types_and_unicode_name_metadata() {
        let program = lower_text(
            "module 游戏\n    requires Console.Write, Random\n\n\
             type 人物<'a> = { mutable 血量: Int; 数据: 'a }\n\
             type 结果<'a> =\n    | 成功 of 'a\n    | 失败 of Text\n",
        );

        assert_eq!(program.items.len(), 3);
        let Item::Module(module) = &program.items[0] else {
            panic!("expected module");
        };
        assert_eq!(module.name.segments[0].normalized, "游戏");
        assert_eq!(module.requires.len(), 2);

        let Item::Type(character) = &program.items[1] else {
            panic!("expected record type");
        };
        assert_eq!(character.name.normalized, "人物");
        assert_eq!(character.parameters[0].normalized, "a");
        let TypeDefinition::Record(fields) = &character.definition else {
            panic!("expected record definition");
        };
        assert!(fields[0].mutable);
        assert_eq!(fields[0].name.normalized, "血量");
    }

    #[test]
    fn lowers_import_module_and_alias() {
        let program = lower_text(
            "module Main\n\nimport Game.Math\nimport Game.Text as 文本\n\nlet main () = ()\n",
        );

        let Item::Import(math) = &program.items[1] else {
            panic!("expected import");
        };
        assert_eq!(math.module.segments[1].normalized, "Math");
        assert!(math.alias.is_none());

        let Item::Import(text) = &program.items[2] else {
            panic!("expected aliased import");
        };
        assert_eq!(
            text.alias.as_ref().map(|name| name.normalized.as_str()),
            Some("文本")
        );
    }

    #[test]
    fn removes_grouping_but_preserves_pipeline_and_assignment() {
        let program = lower_text(
            "let mutable 总数 = 0\n\
             let 计算 xs = (xs |> map transform)\n\
             let 更新 value = 总数 <- value\n",
        );

        let Item::Let(pipeline) = &program.items[1] else {
            panic!("expected let");
        };
        assert!(matches!(
            pipeline.value.kind,
            ExpressionKind::Pipeline { .. }
        ));
        let Item::Let(assignment) = &program.items[2] else {
            panic!("expected let");
        };
        assert!(matches!(
            assignment.value.kind,
            ExpressionKind::Assignment { .. }
        ));
    }

    #[test]
    fn extracts_the_operator_between_nested_operands() {
        let program = lower_text("let value = 1 * 2 + 3\n");
        let Item::Let(declaration) = &program.items[0] else {
            panic!("expected let");
        };
        let ExpressionKind::Binary { operator, left, .. } = &declaration.value.kind else {
            panic!("expected binary expression");
        };
        assert_eq!(*operator, BinaryOperator::Add);
        assert!(matches!(
            left.kind,
            ExpressionKind::Binary {
                operator: BinaryOperator::Multiply,
                ..
            }
        ));
    }

    #[test]
    fn rejects_invalid_syntax_instead_of_forging_an_ast() {
        let source = SourceFile::from_bytes(SourceId::new(1), "bad.ling", b"let =".to_vec())
            .expect("valid UTF-8");
        let parsed = parse(&source);

        assert_eq!(
            lower(&source, &parsed).unwrap_err().kind(),
            &LowerErrorKind::InvalidCst
        );
    }

    #[test]
    fn lowers_handler_cst_into_unresolved_ast_without_hir_publication() {
        let source = SourceFile::from_bytes(
            SourceId::new(1),
            "handler.ling",
            b"let value =\n    handle value with\n        operation Clock.now() -> 1\n".to_vec(),
        )
        .expect("valid UTF-8");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());

        let program = lower(&source, &parsed).expect("accepted AST projection");
        let Item::Let(declaration) = &program.items[0] else {
            panic!("expected let declaration");
        };
        let ExpressionKind::Sequence(elements) = &declaration.value.kind else {
            panic!("expected layout sequence");
        };
        let SequenceElement::Expression(expression) = &elements[0] else {
            panic!("expected handler expression");
        };
        let ExpressionKind::Handle { body, clauses } = &expression.kind else {
            panic!("expected handler AST");
        };
        assert!(matches!(body.kind, ExpressionKind::Name(_)));
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].operation.segments[0].normalized, "Clock");
        assert_eq!(clauses[0].operation.segments[1].normalized, "now");
        assert!(clauses[0].parameters.is_empty());
        assert!(clauses[0].resume.is_none());
        assert!(clauses[0].span.start() < clauses[0].span.end());
    }

    #[test]
    fn lowers_trait_impl_and_generic_constraint_items() {
        let program = lower_text(
            r#"trait Renderable<'a> =
    render: 'a -> Text

impl Renderable Item =
    let render item = item.name

let show<'a> requires { Renderable<'a> } value =
    value

let phantom<'a> requires { Renderable<'a> } =
    0
"#,
        );

        let Item::Trait(trait_declaration) = &program.items[0] else {
            panic!("expected trait declaration");
        };
        assert!(trait_declaration.span.start() < trait_declaration.span.end());
        assert_eq!(trait_declaration.name.normalized, "Renderable");
        assert_eq!(trait_declaration.parameters[0].normalized, "a");
        assert_eq!(trait_declaration.members[0].name.normalized, "render");

        let Item::Impl(impl_declaration) = &program.items[1] else {
            panic!("expected impl declaration");
        };
        assert!(impl_declaration.span.start() < impl_declaration.span.end());
        assert_eq!(
            impl_declaration.trait_name.segments[0].normalized,
            "Renderable"
        );
        let [PatternAtom::Name(member_name)] = impl_declaration.members[0].binding.atoms.as_slice()
        else {
            panic!("expected impl member binding");
        };
        assert_eq!(member_name.normalized, "render");

        let Item::Let(generic) = &program.items[2] else {
            panic!("expected constrained generic declaration");
        };
        assert_eq!(generic.type_parameters[0].normalized, "a");
        assert_eq!(generic.constraints.len(), 1);
        assert_eq!(generic.parameters.len(), 1);

        let Item::Let(no_parameters) = &program.items[3] else {
            panic!("expected generic declaration without value parameters");
        };
        assert_eq!(no_parameters.type_parameters.len(), 1);
        assert!(no_parameters.parameters.is_empty());
    }

    #[test]
    fn lowers_an_empty_program_with_a_zero_width_span() {
        let program = lower_text("");

        assert!(program.items.is_empty());
        assert_eq!(program.span.start().get(), 0);
        assert_eq!(program.span.end().get(), 0);
    }
}
