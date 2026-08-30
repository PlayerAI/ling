use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_source::{SourceFile, Span};

use crate::cst::{CstNode, NodeKind, SyntaxTree};
use crate::lexer::{LexError, lex};
use crate::token::{Token, TokenKind, TokenValue};

const MAX_PARSE_DEPTH: usize = 512;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedSource {
    tree: SyntaxTree,
    lexical_errors: Vec<LexError>,
    parse_errors: Vec<ParseError>,
}

impl ParsedSource {
    #[must_use]
    pub const fn tree(&self) -> &SyntaxTree {
        &self.tree
    }

    #[must_use]
    pub fn lexical_errors(&self) -> &[LexError] {
        &self.lexical_errors
    }

    #[must_use]
    pub fn parse_errors(&self) -> &[ParseError] {
        &self.parse_errors
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.lexical_errors.is_empty() && self.parse_errors.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError {
    kind: ParseErrorKind,
    span: Span,
    context: &'static str,
}

impl ParseError {
    #[must_use]
    pub const fn kind(&self) -> &ParseErrorKind {
        &self.kind
    }

    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    #[must_use]
    pub fn to_diagnostic(&self, file: &str) -> Diagnostic {
        let span = DiagnosticSpan::new(file, self.span);
        match &self.kind {
            ParseErrorKind::UnexpectedToken { expected, found } => Diagnostic::new(
                codes::UNEXPECTED_TOKEN,
                Severity::Error,
                format!(
                    "语法错误：在 {} 中需要 {}",
                    self.context,
                    token_list(expected)
                ),
                format!(
                    "syntax error: expected {} in {}",
                    token_list(expected),
                    self.context
                ),
            )
            .with_primary_span(span)
            .with_fact("context", self.context)
            .with_fact("expected", token_list(expected))
            .with_fact("found", format!("{found:?}")),
            ParseErrorKind::RecursionLimit => Diagnostic::new(
                codes::PARSE_RECURSION_LIMIT,
                Severity::Error,
                "语法嵌套超过 512 层",
                "syntax nesting exceeds 512 levels",
            )
            .with_primary_span(span)
            .with_fact("maximum_depth", 512_u64),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseErrorKind {
    UnexpectedToken {
        expected: Vec<TokenKind>,
        found: TokenKind,
    },
    RecursionLimit,
}

#[must_use]
pub fn parse(source: &SourceFile) -> ParsedSource {
    let (tokens, lexical_errors) = lex(source).into_parts();
    if !lexical_errors.is_empty() {
        let root = CstNode::new(NodeKind::Program, 0..tokens.len(), Vec::new());
        return ParsedSource {
            tree: SyntaxTree::new(tokens, root),
            lexical_errors,
            parse_errors: Vec::new(),
        };
    }

    let (root, parse_errors) = Parser::new(&tokens).parse_program();
    ParsedSource {
        tree: SyntaxTree::new(tokens, root),
        lexical_errors,
        parse_errors,
    }
}

struct Parser<'tokens> {
    tokens: &'tokens [Token],
    position: usize,
    errors: Vec<ParseError>,
    depth: usize,
    task_depth: usize,
}

impl<'tokens> Parser<'tokens> {
    fn new(tokens: &'tokens [Token]) -> Self {
        Self {
            tokens,
            position: 0,
            errors: Vec::new(),
            depth: 0,
            task_depth: 0,
        }
    }

    fn parse_program(mut self) -> (CstNode, Vec<ParseError>) {
        let start = self.position;
        let mut children = Vec::new();
        self.eat_newlines(false);
        while !self.at(TokenKind::Eof) {
            let before = self.position;
            let declaration = match self.current_kind() {
                TokenKind::Module => self.parse_module_declaration(),
                TokenKind::Import => self.parse_import_declaration(),
                TokenKind::Let => self.parse_let_declaration(),
                TokenKind::Type => self.parse_type_declaration(),
                TokenKind::Trait => self.parse_trait_declaration(),
                TokenKind::Impl => self.parse_impl_declaration(),
                TokenKind::Identifier if self.is_contextual("task") => {
                    self.parse_task_declaration()
                }
                TokenKind::Identifier if self.is_contextual("actor") => {
                    self.parse_actor_declaration()
                }
                _ => {
                    self.unexpected(
                        &[
                            TokenKind::Module,
                            TokenKind::Import,
                            TokenKind::Let,
                            TokenKind::Type,
                            TokenKind::Trait,
                            TokenKind::Impl,
                            TokenKind::Identifier,
                        ],
                        "program",
                    );
                    self.recover_to_line_boundary();
                    CstNode::new(NodeKind::Error, before..self.position, Vec::new())
                }
            };
            children.push(declaration);
            self.eat_newlines(false);
            if self.position == before {
                self.bump_any();
            }
        }
        self.bump(TokenKind::Eof);
        (
            CstNode::new(NodeKind::Program, start..self.position, children),
            self.errors,
        )
    }

    fn parse_task_declaration(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect_contextual("task", "Task declaration");
        self.expect(TokenKind::Identifier, "Task name");
        while self.starts_pattern() && !self.at(TokenKind::Equals) {
            children.push(self.parse_pattern("Task parameter"));
        }
        self.expect(TokenKind::Equals, "Task declaration");
        if !self.eat_newlines(false) {
            self.unexpected(&[TokenKind::Newline], "Task declaration body");
        }
        self.expect(TokenKind::Indent, "Task declaration body");
        self.task_depth = self.task_depth.saturating_add(1);
        if self.is_contextual("scope") {
            children.push(self.parse_task_scope_expression());
        } else {
            self.unexpected(&[TokenKind::Identifier], "Task outer scope");
            self.recover_to(TokenKind::Dedent);
        }
        self.task_depth = self.task_depth.saturating_sub(1);
        self.eat_newlines(false);
        self.expect(TokenKind::Dedent, "Task declaration body");
        CstNode::new(NodeKind::TaskDeclaration, start..self.position, children)
    }

    fn parse_actor_declaration(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect_contextual("actor", "Actor declaration");
        self.expect(TokenKind::Identifier, "Actor name");
        self.expect(TokenKind::Colon, "Actor message type");
        children.push(self.parse_type_expression(&[TokenKind::Equals]));
        self.expect(TokenKind::Equals, "Actor declaration");
        if !self.eat_newlines(false) {
            self.unexpected(&[TokenKind::Newline], "Actor declaration body");
        }
        self.expect(TokenKind::Indent, "Actor declaration body");
        children.push(self.parse_actor_state_clause());
        self.eat_newlines(false);
        children.push(self.parse_actor_receive_clause());
        self.eat_newlines(false);
        if !self.at(TokenKind::Dedent) {
            self.unexpected(&[TokenKind::Dedent], "Actor declaration body");
            self.recover_to(TokenKind::Dedent);
        }
        self.expect(TokenKind::Dedent, "Actor declaration body");
        CstNode::new(NodeKind::ActorDeclaration, start..self.position, children)
    }

    fn parse_actor_state_clause(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect_contextual("state", "Actor state clause");
        children.push(self.parse_type_expression(&[TokenKind::Equals]));
        self.expect(TokenKind::Equals, "Actor state clause");
        children.push(self.parse_expression());
        CstNode::new(NodeKind::ActorStateClause, start..self.position, children)
    }

    fn parse_actor_receive_clause(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect_contextual("receive", "Actor receive clause");
        children.push(self.parse_pattern("Actor state pattern"));
        children.push(self.parse_pattern("Actor message pattern"));
        self.expect(TokenKind::Equals, "Actor receive clause");
        if !self.at(TokenKind::Newline) {
            self.unexpected(&[TokenKind::Newline], "Actor receive body");
        }
        children.push(self.parse_body_expression("Actor receive body"));
        CstNode::new(NodeKind::ActorReceiveClause, start..self.position, children)
    }

    fn parse_module_declaration(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::Module, "module declaration");
        children.push(self.parse_qualified_name("module name"));

        if self.at(TokenKind::Newline) {
            self.eat_newlines(false);
            if self.eat(TokenKind::Indent) {
                if self.at(TokenKind::Requires) {
                    children.push(self.parse_capability_block());
                } else {
                    self.unexpected(&[TokenKind::Requires], "module capability block");
                    self.recover_to(TokenKind::Dedent);
                }
                self.expect(TokenKind::Dedent, "module capability block");
            }
        }
        CstNode::new(NodeKind::ModuleDeclaration, start..self.position, children)
    }

    fn parse_capability_block(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::Requires, "capability block");
        children.push(self.parse_qualified_name("capability name"));
        while self.eat(TokenKind::Comma) {
            children.push(self.parse_qualified_name("capability name"));
        }
        self.eat_newlines(false);
        CstNode::new(NodeKind::CapabilityBlock, start..self.position, children)
    }

    fn parse_import_declaration(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::Import, "import declaration");
        children.push(self.parse_qualified_name("imported module name"));
        if self.eat(TokenKind::As) {
            self.expect(TokenKind::Identifier, "import alias");
        }
        CstNode::new(NodeKind::ImportDeclaration, start..self.position, children)
    }

    fn parse_qualified_name(&mut self, context: &'static str) -> CstNode {
        let start = self.position;
        self.expect(TokenKind::Identifier, context);
        while self.eat(TokenKind::Dot) {
            self.expect(TokenKind::Identifier, context);
        }
        CstNode::new(NodeKind::QualifiedName, start..self.position, Vec::new())
    }

    fn parse_let_declaration(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::Let, "let declaration");
        self.eat(TokenKind::Rec);
        self.eat(TokenKind::Mutable);
        children.push(self.parse_pattern("binding pattern"));

        if self.at(TokenKind::Less) {
            children.push(self.parse_type_parameter_list());
        }
        if self.at(TokenKind::Requires) {
            children.push(self.parse_constraint_block());
        }

        while self.starts_pattern() && !self.at(TokenKind::Equals) && !self.at(TokenKind::Colon) {
            children.push(self.parse_pattern("parameter pattern"));
        }
        if self.eat(TokenKind::Colon) {
            children.push(self.parse_type_expression(&[TokenKind::Equals]));
        }
        self.expect(TokenKind::Equals, "let declaration");
        children.push(self.parse_body_expression("let body"));
        CstNode::new(NodeKind::LetDeclaration, start..self.position, children)
    }

    fn parse_type_parameter_list(&mut self) -> CstNode {
        let start = self.position;
        self.expect(TokenKind::Less, "type parameter list");
        self.expect(TokenKind::Apostrophe, "type parameter");
        self.expect(TokenKind::Identifier, "type parameter");
        while self.eat(TokenKind::Comma) {
            self.expect(TokenKind::Apostrophe, "type parameter");
            self.expect(TokenKind::Identifier, "type parameter");
        }
        self.expect(TokenKind::Greater, "type parameter list");
        CstNode::new(
            NodeKind::TypeParameterList,
            start..self.position,
            Vec::new(),
        )
    }

    fn parse_constraint_block(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::Requires, "Trait constraint block");
        self.expect(TokenKind::LeftBrace, "Trait constraint block");
        self.eat_member_separators();
        while !self.at(TokenKind::RightBrace) && !self.at(TokenKind::Eof) {
            children.push(self.parse_type_expression(&[
                TokenKind::Comma,
                TokenKind::Semicolon,
                TokenKind::SoftNewline,
                TokenKind::RightBrace,
            ]));
            let comma = self.eat(TokenKind::Comma);
            let layout_separator = self.eat_member_separators();
            if !comma && !layout_separator && !self.at(TokenKind::RightBrace) {
                self.unexpected(
                    &[
                        TokenKind::Comma,
                        TokenKind::Semicolon,
                        TokenKind::SoftNewline,
                    ],
                    "Trait constraint separator",
                );
                self.recover_to(TokenKind::RightBrace);
            }
        }
        self.expect(TokenKind::RightBrace, "Trait constraint block");
        CstNode::new(NodeKind::ConstraintBlock, start..self.position, children)
    }

    fn parse_trait_declaration(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::Trait, "Trait declaration");
        self.expect(TokenKind::Identifier, "Trait name");
        if self.at(TokenKind::Less) {
            children.push(self.parse_type_parameter_list());
        }
        self.expect(TokenKind::Equals, "Trait declaration");
        if !self.eat_newlines(false) {
            self.unexpected(&[TokenKind::Newline], "Trait declaration body");
        }
        self.expect(TokenKind::Indent, "Trait declaration body");
        while !self.at(TokenKind::Dedent) && !self.at(TokenKind::Eof) {
            let member_start = self.position;
            self.expect(TokenKind::Identifier, "Trait member name");
            self.expect(TokenKind::Colon, "Trait member signature");
            let signature = self.parse_type_expression(&[
                TokenKind::Newline,
                TokenKind::SoftNewline,
                TokenKind::Dedent,
            ]);
            children.push(CstNode::new(
                NodeKind::TraitMember,
                member_start..self.position,
                vec![signature],
            ));
            self.eat_newlines(false);
        }
        self.expect(TokenKind::Dedent, "Trait declaration body");
        CstNode::new(NodeKind::TraitDeclaration, start..self.position, children)
    }

    fn parse_impl_declaration(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::Impl, "impl declaration");
        children.push(self.parse_qualified_name("Trait name"));
        children.push(self.parse_type_expression(&[TokenKind::Equals]));
        self.expect(TokenKind::Equals, "impl declaration");
        if !self.eat_newlines(false) {
            self.unexpected(&[TokenKind::Newline], "impl declaration body");
        }
        self.expect(TokenKind::Indent, "impl declaration body");
        while !self.at(TokenKind::Dedent) && !self.at(TokenKind::Eof) {
            let member_start = self.position;
            let member = self.parse_let_declaration();
            children.push(CstNode::new(
                NodeKind::ImplMember,
                member_start..self.position,
                vec![member],
            ));
            self.eat_newlines(false);
        }
        self.expect(TokenKind::Dedent, "impl declaration body");
        CstNode::new(NodeKind::ImplDeclaration, start..self.position, children)
    }

    fn parse_type_declaration(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::Type, "type declaration");
        self.expect(TokenKind::Identifier, "type name");
        if self.eat(TokenKind::Less) {
            self.expect(TokenKind::Apostrophe, "type parameter");
            self.expect(TokenKind::Identifier, "type parameter");
            while self.eat(TokenKind::Comma) {
                self.expect(TokenKind::Apostrophe, "type parameter");
                self.expect(TokenKind::Identifier, "type parameter");
            }
            self.expect(TokenKind::Greater, "type parameter list");
        }
        self.expect(TokenKind::Equals, "type declaration");

        let had_block = if self.at(TokenKind::Newline) {
            self.eat_newlines(false);
            self.expect(TokenKind::Indent, "type declaration body");
            true
        } else {
            false
        };
        let body = match self.current_kind() {
            TokenKind::LeftBrace => self.parse_record_type(),
            TokenKind::Pipe => self.parse_variant_type(),
            _ => {
                self.parse_type_expression(&[TokenKind::Newline, TokenKind::Dedent, TokenKind::Eof])
            }
        };
        children.push(body);
        if had_block {
            self.eat_newlines(false);
            self.expect(TokenKind::Dedent, "type declaration body");
        }
        CstNode::new(NodeKind::TypeDeclaration, start..self.position, children)
    }

    fn parse_record_type(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::LeftBrace, "record type");
        self.eat_member_separators();
        while !self.at(TokenKind::RightBrace) && !self.at(TokenKind::Eof) {
            let field_start = self.position;
            self.eat(TokenKind::Mutable);
            self.expect(TokenKind::Identifier, "record field name");
            self.expect(TokenKind::Colon, "record field");
            let field_type = self.parse_type_expression(&[
                TokenKind::Semicolon,
                TokenKind::SoftNewline,
                TokenKind::RightBrace,
            ]);
            children.push(CstNode::new(
                NodeKind::FieldDeclaration,
                field_start..self.position,
                vec![field_type],
            ));
            if !self.eat_member_separators() && !self.at(TokenKind::RightBrace) {
                self.unexpected(
                    &[TokenKind::Semicolon, TokenKind::SoftNewline],
                    "record field separator",
                );
                self.recover_to(TokenKind::RightBrace);
            }
        }
        self.expect(TokenKind::RightBrace, "record type");
        CstNode::new(NodeKind::RecordType, start..self.position, children)
    }

    fn parse_variant_type(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        while self.at(TokenKind::Pipe) {
            let case_start = self.position;
            self.bump(TokenKind::Pipe);
            self.expect(TokenKind::Identifier, "variant case");
            let mut case_children = Vec::new();
            if self.eat(TokenKind::Of) {
                case_children.push(self.parse_type_expression(&[
                    TokenKind::Newline,
                    TokenKind::Dedent,
                    TokenKind::Pipe,
                ]));
            }
            children.push(CstNode::new(
                NodeKind::VariantCase,
                case_start..self.position,
                case_children,
            ));
            self.eat_newlines(false);
        }
        if children.is_empty() {
            self.unexpected(&[TokenKind::Pipe], "variant type");
        }
        CstNode::new(NodeKind::VariantType, start..self.position, children)
    }

    fn parse_type_expression(&mut self, terminators: &[TokenKind]) -> CstNode {
        let start = self.position;
        self.parse_type(terminators);
        if !terminators.contains(&self.current_kind()) && !self.at(TokenKind::Eof) {
            self.unexpected(terminators, "type expression terminator");
            while !terminators.contains(&self.current_kind()) && !self.at(TokenKind::Eof) {
                self.bump_any();
            }
        }
        CstNode::new(NodeKind::TypeExpression, start..self.position, Vec::new())
    }

    fn parse_type(&mut self, terminators: &[TokenKind]) {
        if self.depth >= MAX_PARSE_DEPTH {
            self.errors.push(ParseError {
                kind: ParseErrorKind::RecursionLimit,
                span: self.current_token().span(),
                context: "type expression",
            });
            if !terminators.contains(&self.current_kind()) && !self.at(TokenKind::Eof) {
                self.bump_any();
            }
            return;
        }
        self.depth += 1;
        self.parse_function_type(terminators);
        self.depth -= 1;
    }

    fn parse_function_type(&mut self, terminators: &[TokenKind]) {
        self.parse_product_type(terminators);
        if self.eat(TokenKind::RightArrow) {
            self.parse_type(terminators);
        }
    }

    fn parse_product_type(&mut self, terminators: &[TokenKind]) {
        self.parse_type_atom(terminators);
        while self.eat(TokenKind::Star) {
            self.parse_type_atom(terminators);
        }
    }

    fn parse_type_atom(&mut self, terminators: &[TokenKind]) {
        match self.current_kind() {
            TokenKind::Apostrophe => {
                self.bump(TokenKind::Apostrophe);
                self.expect(TokenKind::Identifier, "type variable");
            }
            TokenKind::Identifier => {
                self.bump(TokenKind::Identifier);
                while self.eat(TokenKind::Dot) {
                    self.expect(TokenKind::Identifier, "qualified type name");
                }
                if self.eat(TokenKind::Less) {
                    self.parse_type_arguments();
                }
            }
            TokenKind::LeftParen => {
                self.bump(TokenKind::LeftParen);
                self.parse_type(&[TokenKind::Comma, TokenKind::RightParen]);
                while self.eat(TokenKind::Comma) {
                    self.parse_type(&[TokenKind::Comma, TokenKind::RightParen]);
                }
                self.expect(TokenKind::RightParen, "parenthesized or tuple type");
            }
            _ => {
                self.unexpected(
                    &[
                        TokenKind::Identifier,
                        TokenKind::Apostrophe,
                        TokenKind::LeftParen,
                    ],
                    "type expression",
                );
                if !terminators.contains(&self.current_kind()) && !self.at(TokenKind::Eof) {
                    self.bump_any();
                }
            }
        }
    }

    fn parse_type_arguments(&mut self) {
        self.parse_type(&[TokenKind::Comma, TokenKind::Greater]);
        while self.eat(TokenKind::Comma) {
            self.parse_type(&[TokenKind::Comma, TokenKind::Greater]);
        }
        self.expect(TokenKind::Greater, "type argument list");
    }

    fn parse_pattern(&mut self, context: &'static str) -> CstNode {
        let start = self.position;
        if self.depth >= MAX_PARSE_DEPTH {
            self.errors.push(ParseError {
                kind: ParseErrorKind::RecursionLimit,
                span: self.current_token().span(),
                context,
            });
            self.bump_any();
            return CstNode::new(NodeKind::Pattern, start..self.position, Vec::new());
        }
        self.depth += 1;
        match self.current_kind() {
            TokenKind::Identifier => {
                self.bump_any();
                while self.eat(TokenKind::Dot) {
                    self.expect(TokenKind::Identifier, "qualified constructor pattern");
                }
            }
            TokenKind::Integer
            | TokenKind::Float
            | TokenKind::Text
            | TokenKind::True
            | TokenKind::False => self.bump_any(),
            TokenKind::LeftParen => {
                self.bump_any();
                if !self.at(TokenKind::RightParen) {
                    self.parse_pattern_terms(context);
                    while self.eat(TokenKind::Comma) {
                        if self.at(TokenKind::RightParen) {
                            self.unexpected(&[TokenKind::Identifier], "tuple pattern element");
                            break;
                        }
                        self.parse_pattern_terms(context);
                    }
                }
                self.expect(TokenKind::RightParen, context);
            }
            TokenKind::LeftBrace => {
                self.bump_any();
                self.eat_member_separators();
                if self.at(TokenKind::RightBrace) {
                    self.unexpected(&[TokenKind::Identifier], "record pattern field");
                }
                while !self.at(TokenKind::RightBrace) && !self.at(TokenKind::Eof) {
                    self.expect(TokenKind::Identifier, "record pattern field");
                    self.expect(TokenKind::Equals, "record pattern field");
                    if self.parse_pattern_terms(context) == 0 {
                        self.unexpected(&[TokenKind::Identifier], "record field pattern");
                    }
                    self.eat_member_separators();
                }
                self.expect(TokenKind::RightBrace, context);
            }
            _ => self.unexpected(&[TokenKind::Identifier], context),
        }
        self.depth -= 1;
        CstNode::new(NodeKind::Pattern, start..self.position, Vec::new())
    }

    fn parse_body_expression(&mut self, context: &'static str) -> CstNode {
        if !self.at(TokenKind::Newline) {
            return self.parse_expression();
        }
        self.eat_newlines(false);
        if !self.eat(TokenKind::Indent) {
            self.unexpected(&[TokenKind::Indent], context);
            return CstNode::new(NodeKind::Error, self.position..self.position, Vec::new());
        }
        let sequence = self.parse_sequence();
        self.expect(TokenKind::Dedent, context);
        sequence
    }

    fn parse_sequence(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.eat_newlines(false);
        while !self.at(TokenKind::Dedent) && !self.at(TokenKind::Eof) {
            let before = self.position;
            let child = if self.task_depth > 0 && self.at_adjacent_let_bang() {
                self.parse_task_let_await()
            } else if self.at(TokenKind::Let) {
                self.parse_let_declaration()
            } else {
                self.parse_expression()
            };
            children.push(child);
            if self.at(TokenKind::Newline) {
                self.eat_newlines(false);
            } else if self.previous_significant_kind() == Some(TokenKind::Dedent) {
                // A nested body consumes its closing Dedent before returning.
                // That token is also the separator between the nested form and
                // the next expression in this enclosing sequence.
            } else if !self.at(TokenKind::Dedent) {
                self.unexpected(&[TokenKind::Newline, TokenKind::Dedent], "sequence");
                self.recover_to_line_boundary();
                self.eat_newlines(false);
            }
            if self.position == before {
                self.bump_any();
            }
        }
        CstNode::new(NodeKind::Sequence, start..self.position, children)
    }

    fn parse_expression(&mut self) -> CstNode {
        if self.depth >= MAX_PARSE_DEPTH {
            let start = self.position;
            self.errors.push(ParseError {
                kind: ParseErrorKind::RecursionLimit,
                span: self.current_token().span(),
                context: "expression",
            });
            self.bump_any();
            return CstNode::new(NodeKind::Error, start..self.position, Vec::new());
        }
        self.depth += 1;
        let expression = if self.task_depth > 0 && self.is_contextual("scope") {
            self.parse_task_scope_expression()
        } else if self.task_depth > 0 && self.is_contextual("spawn") {
            self.parse_task_spawn_expression()
        } else if self.task_depth > 0 && self.is_contextual("await") {
            self.parse_task_await_expression()
        } else if self.task_depth > 0 && self.is_contextual("return") {
            self.parse_task_return_expression()
        } else {
            match self.current_kind() {
                TokenKind::If => self.parse_if_expression(),
                TokenKind::Match => self.parse_match_expression(),
                _ => self.parse_assignment_expression(),
            }
        };
        self.depth -= 1;
        expression
    }

    fn parse_task_scope_expression(&mut self) -> CstNode {
        let start = self.position;
        self.expect_contextual("scope", "Task scope");
        if !self.eat_newlines(false) {
            self.unexpected(&[TokenKind::Newline], "Task scope body");
        }
        self.expect(TokenKind::Indent, "Task scope body");
        let body = self.parse_sequence();
        self.expect(TokenKind::Dedent, "Task scope body");
        CstNode::new(
            NodeKind::TaskScopeExpression,
            start..self.position,
            vec![body],
        )
    }

    fn parse_task_spawn_expression(&mut self) -> CstNode {
        let start = self.position;
        self.expect_contextual("spawn", "Task spawn");
        let call = self.parse_application_expression();
        CstNode::new(
            NodeKind::TaskSpawnExpression,
            start..self.position,
            vec![call],
        )
    }

    fn parse_task_await_expression(&mut self) -> CstNode {
        let start = self.position;
        self.expect_contextual("await", "Task await");
        let handle = self.parse_postfix_expression();
        CstNode::new(
            NodeKind::TaskAwaitExpression,
            start..self.position,
            vec![handle],
        )
    }

    fn parse_task_return_expression(&mut self) -> CstNode {
        let start = self.position;
        self.expect_contextual("return", "Task return");
        let value = self.parse_expression();
        CstNode::new(
            NodeKind::TaskReturnExpression,
            start..self.position,
            vec![value],
        )
    }

    fn parse_task_let_await(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::Let, "Task let-await");
        self.expect(TokenKind::Bang, "Task let-await");
        children.push(self.parse_pattern("Task let-await pattern"));
        self.expect(TokenKind::Equals, "Task let-await");
        children.push(self.parse_application_expression());
        CstNode::new(NodeKind::TaskLetAwait, start..self.position, children)
    }

    fn at_adjacent_let_bang(&self) -> bool {
        if !self.at(TokenKind::Let) {
            return false;
        }
        let let_index = self.next_nontrivia_index(self.position);
        let bang_index = self.next_nontrivia_index(let_index + 1);
        self.tokens.get(bang_index).is_some_and(|bang| {
            bang.kind() == TokenKind::Bang
                && self.tokens[let_index].span().end() == bang.span().start()
        })
    }

    fn parse_non_assignment_expression(&mut self) -> CstNode {
        match self.current_kind() {
            TokenKind::If => self.parse_if_expression(),
            TokenKind::Match => self.parse_match_expression(),
            _ => self.parse_pipeline_expression(),
        }
    }

    fn parse_if_expression(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::If, "if expression");
        children.push(self.parse_expression());
        self.expect(TokenKind::Then, "if expression");
        children.push(self.parse_body_expression("then branch"));
        self.expect(TokenKind::Else, "if expression");
        children.push(self.parse_body_expression("else branch"));
        CstNode::new(NodeKind::IfExpression, start..self.position, children)
    }

    fn parse_match_expression(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::Match, "match expression");
        children.push(self.parse_expression());
        self.expect(TokenKind::With, "match expression");

        let had_block = if self.at(TokenKind::Newline) {
            self.eat_newlines(false);
            self.eat(TokenKind::Indent)
        } else {
            false
        };
        self.eat_newlines(false);
        while self.at(TokenKind::Pipe) {
            children.push(self.parse_match_case());
            self.eat_newlines(false);
        }
        if had_block {
            self.expect(TokenKind::Dedent, "match cases");
        }
        CstNode::new(NodeKind::MatchExpression, start..self.position, children)
    }

    fn parse_match_case(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::Pipe, "match case");
        children.push(self.parse_pattern_sequence());
        if self.eat(TokenKind::When) {
            children.push(self.parse_expression());
        }
        self.expect(TokenKind::RightArrow, "match case");
        children.push(self.parse_body_expression("match case body"));
        CstNode::new(NodeKind::MatchCase, start..self.position, children)
    }

    fn parse_pattern_sequence(&mut self) -> CstNode {
        let start = self.position;
        self.parse_pattern_terms("match pattern");
        CstNode::new(NodeKind::Pattern, start..self.position, Vec::new())
    }

    fn parse_pattern_terms(&mut self, context: &'static str) -> usize {
        let mut count = 0;
        while self.starts_pattern() {
            let before = self.position;
            self.parse_pattern(context);
            if self.position == before {
                break;
            }
            count += 1;
        }
        count
    }

    fn parse_assignment_expression(&mut self) -> CstNode {
        let left = self.parse_pipeline_expression();
        if !self.eat(TokenKind::LeftArrow) {
            return left;
        }
        let start = left.token_range().start;
        let indented = self.begin_operator_continuation("assignment right-hand side");
        let right = self.parse_non_assignment_expression();
        if indented {
            self.eat_newlines(false);
            self.expect(TokenKind::Dedent, "assignment continuation");
        }
        CstNode::new(
            NodeKind::AssignmentExpression,
            start..self.position,
            vec![left, right],
        )
    }

    fn parse_pipeline_expression(&mut self) -> CstNode {
        let mut left = self.parse_boolean_or_expression();
        while let Some(operator_index) = self.pipeline_operator_index() {
            self.position = operator_index;
            self.bump(TokenKind::PipeGreater);
            let indented = self.begin_operator_continuation("pipeline right-hand side");
            let right = self.parse_boolean_or_expression();
            if indented {
                self.eat_newlines(false);
                self.expect(TokenKind::Dedent, "pipeline continuation");
            }
            let start = left.token_range().start;
            left = CstNode::new(
                NodeKind::PipelineExpression,
                start..self.position,
                vec![left, right],
            );
        }
        left
    }

    fn parse_boolean_or_expression(&mut self) -> CstNode {
        self.parse_binary(Self::parse_boolean_and_expression, &[TokenKind::PipePipe])
    }

    fn parse_boolean_and_expression(&mut self) -> CstNode {
        self.parse_binary(Self::parse_equality_expression, &[TokenKind::AmpAmp])
    }

    fn parse_equality_expression(&mut self) -> CstNode {
        self.parse_binary(
            Self::parse_comparison_expression,
            &[TokenKind::EqualEqual, TokenKind::BangEqual],
        )
    }

    fn parse_comparison_expression(&mut self) -> CstNode {
        self.parse_binary(
            Self::parse_additive_expression,
            &[
                TokenKind::Less,
                TokenKind::LessEqual,
                TokenKind::Greater,
                TokenKind::GreaterEqual,
            ],
        )
    }

    fn parse_additive_expression(&mut self) -> CstNode {
        self.parse_binary(
            Self::parse_multiplicative_expression,
            &[TokenKind::Plus, TokenKind::Minus],
        )
    }

    fn parse_multiplicative_expression(&mut self) -> CstNode {
        self.parse_binary(
            Self::parse_application_expression,
            &[TokenKind::Star, TokenKind::Slash, TokenKind::Percent],
        )
    }

    fn parse_binary(
        &mut self,
        operand: fn(&mut Self) -> CstNode,
        operators: &[TokenKind],
    ) -> CstNode {
        let mut left = operand(self);
        while operators.contains(&self.current_kind()) {
            self.bump_any();
            let indented = self.begin_operator_continuation("binary expression");
            let right = operand(self);
            if indented {
                self.eat_newlines(false);
                self.expect(TokenKind::Dedent, "binary continuation");
            }
            let start = left.token_range().start;
            left = CstNode::new(
                NodeKind::BinaryExpression,
                start..self.position,
                vec![left, right],
            );
        }
        left
    }

    fn parse_application_expression(&mut self) -> CstNode {
        let mut function = self.parse_postfix_expression();
        while self.starts_primary() {
            let argument = self.parse_postfix_expression();
            let start = function.token_range().start;
            function = CstNode::new(
                NodeKind::ApplicationExpression,
                start..self.position,
                vec![function, argument],
            );
        }
        function
    }

    fn parse_postfix_expression(&mut self) -> CstNode {
        let mut expression = self.parse_primary_expression();
        while self.eat(TokenKind::Dot) {
            self.expect(TokenKind::Identifier, "field projection");
            let start = expression.token_range().start;
            expression = CstNode::new(
                NodeKind::ProjectionExpression,
                start..self.position,
                vec![expression],
            );
        }
        expression
    }

    fn parse_primary_expression(&mut self) -> CstNode {
        let start = self.position;
        match self.current_kind() {
            TokenKind::Identifier => {
                if self.is_contextual("handle") {
                    if let Some(handler) = self.parse_handle_expression() {
                        return handler;
                    }
                }
                self.bump_any();
                CstNode::new(NodeKind::NameExpression, start..self.position, Vec::new())
            }
            TokenKind::Integer
            | TokenKind::Float
            | TokenKind::Text
            | TokenKind::True
            | TokenKind::False => {
                self.bump_any();
                CstNode::new(
                    NodeKind::LiteralExpression,
                    start..self.position,
                    Vec::new(),
                )
            }
            TokenKind::Plus | TokenKind::Minus => {
                self.bump_any();
                let operand = self.parse_primary_expression();
                CstNode::new(
                    NodeKind::UnaryExpression,
                    start..self.position,
                    vec![operand],
                )
            }
            TokenKind::LeftParen => self.parse_parenthesized_expression(),
            TokenKind::LeftBrace => self.parse_record_expression(),
            TokenKind::LeftBracket => self.parse_list_expression(),
            TokenKind::If => self.parse_if_expression(),
            TokenKind::Match => self.parse_match_expression(),
            _ => {
                self.unexpected(
                    &[
                        TokenKind::Identifier,
                        TokenKind::Integer,
                        TokenKind::Text,
                        TokenKind::LeftParen,
                    ],
                    "expression",
                );
                if !self.is_expression_terminator(self.current_kind()) {
                    self.bump_any();
                }
                CstNode::new(NodeKind::Error, start..self.position, Vec::new())
            }
        }
    }

    fn parse_parenthesized_expression(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::LeftParen, "parenthesized expression");
        self.eat_newlines(true);
        if self.eat(TokenKind::RightParen) {
            return CstNode::new(NodeKind::UnitExpression, start..self.position, children);
        }
        children.push(self.parse_expression());
        if self.eat(TokenKind::Comma) {
            loop {
                children.push(self.parse_expression());
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
            self.eat_newlines(true);
            self.expect(TokenKind::RightParen, "tuple expression");
            CstNode::new(NodeKind::TupleExpression, start..self.position, children)
        } else {
            self.eat_newlines(true);
            self.expect(TokenKind::RightParen, "group expression");
            CstNode::new(NodeKind::GroupExpression, start..self.position, children)
        }
    }

    fn parse_record_expression(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::LeftBrace, "record expression");
        self.eat_member_separators();

        let update = self.at(TokenKind::Identifier)
            && self.kind_after_significant(1) == Some(TokenKind::With);
        if update {
            children.push(self.parse_primary_expression());
            self.expect(TokenKind::With, "record update");
            self.eat_member_separators();
        }
        while !self.at(TokenKind::RightBrace) && !self.at(TokenKind::Eof) {
            let field_start = self.position;
            self.expect(TokenKind::Identifier, "record field name");
            self.expect(TokenKind::Equals, "record field");
            let value = self.parse_expression();
            children.push(CstNode::new(
                NodeKind::RecordField,
                field_start..self.position,
                vec![value],
            ));
            if !self.eat_member_separators() && !self.at(TokenKind::RightBrace) {
                self.unexpected(
                    &[TokenKind::Semicolon, TokenKind::SoftNewline],
                    "record field separator",
                );
                self.recover_to(TokenKind::RightBrace);
            }
        }
        self.expect(TokenKind::RightBrace, "record expression");
        CstNode::new(
            if update {
                NodeKind::RecordUpdate
            } else {
                NodeKind::RecordExpression
            },
            start..self.position,
            children,
        )
    }

    fn parse_list_expression(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect(TokenKind::LeftBracket, "list expression");
        self.eat_member_separators();
        while !self.at(TokenKind::RightBracket) && !self.at(TokenKind::Eof) {
            children.push(self.parse_expression());
            if !self.eat_member_separators() && !self.at(TokenKind::RightBracket) {
                self.unexpected(
                    &[TokenKind::Semicolon, TokenKind::SoftNewline],
                    "list element separator",
                );
                self.recover_to(TokenKind::RightBracket);
            }
        }
        self.expect(TokenKind::RightBracket, "list expression");
        CstNode::new(NodeKind::ListExpression, start..self.position, children)
    }

    /// Parses the experimental, parser-only handler projection authorized by
    /// DEC-0064. A failed shape probe restores both cursor and diagnostics so
    /// contextual identifiers remain ordinary Seed names.
    fn parse_handle_expression(&mut self) -> Option<CstNode> {
        let saved_position = self.position;
        let saved_errors = self.errors.len();
        let start = self.position;
        if !self.eat_contextual("handle") {
            return None;
        }

        let body = self.parse_non_assignment_expression();
        if !self.at(TokenKind::With) {
            self.position = saved_position;
            self.errors.truncate(saved_errors);
            return None;
        }
        self.bump(TokenKind::With);
        if !self.eat_newlines(false) {
            self.unexpected(&[TokenKind::Newline], "handler clauses");
        }
        if !self.eat(TokenKind::Indent) {
            self.unexpected(&[TokenKind::Indent], "handler clauses");
        }
        self.eat_newlines(false);

        let mut children = vec![body];
        let mut clause_count = 0;
        while self.is_contextual("operation") && !self.at(TokenKind::Dedent) {
            children.push(self.parse_handler_clause());
            clause_count += 1;
            self.eat_newlines(false);
        }
        if clause_count == 0 {
            self.unexpected(&[TokenKind::Identifier], "handler clauses");
        }
        self.expect(TokenKind::Dedent, "handler clauses");
        Some(CstNode::new(
            NodeKind::HandleExpression,
            start..self.position,
            children,
        ))
    }

    fn parse_handler_clause(&mut self) -> CstNode {
        let start = self.position;
        let mut children = Vec::new();
        self.expect_contextual("operation", "handler operation clause");
        children.push(self.parse_qualified_name("handler operation name"));
        self.expect(TokenKind::LeftParen, "handler operation parameters");
        if !self.at(TokenKind::RightParen) {
            loop {
                children.push(self.parse_pattern("handler operation parameter"));
                if !self.eat(TokenKind::Comma) {
                    break;
                }
                if self.at(TokenKind::RightParen) {
                    break;
                }
                if self.is_contextual("resume") {
                    self.bump_any();
                    break;
                }
            }
        }
        self.expect(TokenKind::RightParen, "handler operation parameters");
        self.expect(TokenKind::RightArrow, "handler operation clause");
        children.push(self.parse_body_expression("handler operation body"));
        CstNode::new(NodeKind::HandlerClause, start..self.position, children)
    }

    fn begin_operator_continuation(&mut self, context: &'static str) -> bool {
        if !self.at(TokenKind::Newline) && !self.at(TokenKind::SoftNewline) {
            return false;
        }
        self.eat_newlines(true);
        if self.eat(TokenKind::Indent) {
            true
        } else {
            self.unexpected(&[TokenKind::Indent], context);
            false
        }
    }

    fn pipeline_operator_index(&self) -> Option<usize> {
        let direct = self.next_nontrivia_index(self.position);
        if self.tokens[direct].kind() == TokenKind::PipeGreater {
            return Some(direct);
        }
        if !matches!(
            self.tokens[direct].kind(),
            TokenKind::Newline | TokenKind::SoftNewline
        ) {
            return None;
        }

        let mut index = direct;
        while matches!(
            self.tokens[index].kind(),
            TokenKind::Newline | TokenKind::SoftNewline
        ) {
            index = self.next_nontrivia_index(index + 1);
        }
        (self.tokens[index].kind() == TokenKind::PipeGreater).then_some(index)
    }

    fn starts_pattern(&self) -> bool {
        matches!(
            self.current_kind(),
            TokenKind::Identifier
                | TokenKind::Integer
                | TokenKind::Float
                | TokenKind::Text
                | TokenKind::True
                | TokenKind::False
                | TokenKind::LeftParen
                | TokenKind::LeftBrace
        )
    }

    fn starts_primary(&self) -> bool {
        matches!(
            self.current_kind(),
            TokenKind::Identifier
                | TokenKind::Integer
                | TokenKind::Float
                | TokenKind::Text
                | TokenKind::True
                | TokenKind::False
                | TokenKind::LeftParen
                | TokenKind::LeftBrace
                | TokenKind::LeftBracket
                | TokenKind::If
                | TokenKind::Match
        )
    }

    fn is_expression_terminator(&self, kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Newline
                | TokenKind::SoftNewline
                | TokenKind::Dedent
                | TokenKind::Eof
                | TokenKind::Semicolon
                | TokenKind::Comma
                | TokenKind::RightParen
                | TokenKind::RightBracket
                | TokenKind::RightBrace
                | TokenKind::Then
                | TokenKind::Else
                | TokenKind::With
                | TokenKind::When
                | TokenKind::RightArrow
                | TokenKind::Pipe
        )
    }

    fn eat_member_separators(&mut self) -> bool {
        let mut consumed = false;
        while self.eat(TokenKind::Semicolon) || self.eat(TokenKind::SoftNewline) {
            consumed = true;
        }
        consumed
    }

    fn eat_newlines(&mut self, include_soft: bool) -> bool {
        let mut consumed = false;
        loop {
            if self.eat(TokenKind::Newline) || (include_soft && self.eat(TokenKind::SoftNewline)) {
                consumed = true;
            } else {
                break;
            }
        }
        consumed
    }

    fn current_kind(&self) -> TokenKind {
        self.current_token().kind()
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.next_nontrivia_index(self.position)]
    }

    fn is_contextual(&self, spelling: &str) -> bool {
        matches!(
            self.current_token().value(),
            Some(TokenValue::Identifier(identifier))
                if identifier.identifier().normalized() == spelling
        )
    }

    fn eat_contextual(&mut self, spelling: &str) -> bool {
        if self.current_kind() == TokenKind::Identifier && self.is_contextual(spelling) {
            self.bump_any();
            true
        } else {
            false
        }
    }

    fn expect_contextual(&mut self, spelling: &'static str, context: &'static str) {
        if !self.eat_contextual(spelling) {
            self.unexpected(&[TokenKind::Identifier], context);
        }
    }

    fn next_nontrivia_index(&self, mut index: usize) -> usize {
        while self
            .tokens
            .get(index)
            .is_some_and(|token| token.kind().is_trivia())
        {
            index += 1;
        }
        index.min(self.tokens.len() - 1)
    }

    fn kind_after_significant(&self, count: usize) -> Option<TokenKind> {
        let mut index = self.next_nontrivia_index(self.position);
        for _ in 0..count {
            index = self.next_nontrivia_index(index + 1);
        }
        self.tokens.get(index).map(Token::kind)
    }

    fn previous_significant_kind(&self) -> Option<TokenKind> {
        self.tokens[..self.position.min(self.tokens.len())]
            .iter()
            .rev()
            .find(|token| !token.kind().is_trivia())
            .map(Token::kind)
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.current_kind() == kind
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.bump(kind);
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind, context: &'static str) {
        if !self.eat(kind) {
            self.unexpected(&[kind], context);
        }
    }

    fn bump(&mut self, kind: TokenKind) {
        debug_assert!(self.at(kind));
        self.position = self.next_nontrivia_index(self.position) + 1;
    }

    fn bump_any(&mut self) {
        let index = self.next_nontrivia_index(self.position);
        if self.tokens[index].kind() != TokenKind::Eof {
            self.position = index + 1;
        }
    }

    fn unexpected(&mut self, expected: &[TokenKind], context: &'static str) {
        let token = self.current_token();
        self.errors.push(ParseError {
            kind: ParseErrorKind::UnexpectedToken {
                expected: expected.to_vec(),
                found: token.kind(),
            },
            span: token.span(),
            context,
        });
    }

    fn recover_to_line_boundary(&mut self) {
        while !matches!(
            self.current_kind(),
            TokenKind::Newline | TokenKind::Dedent | TokenKind::Eof
        ) {
            self.bump_any();
        }
    }

    fn recover_to(&mut self, kind: TokenKind) {
        while !self.at(kind) && !self.at(TokenKind::Eof) {
            self.bump_any();
        }
    }
}

fn token_list(tokens: &[TokenKind]) -> String {
    tokens
        .iter()
        .map(|token| format!("`{token:?}`"))
        .collect::<Vec<_>>()
        .join(" or ")
}

#[cfg(test)]
mod tests {
    use ling_source::SourceId;

    use super::*;

    fn parse_text(input: &str) -> ParsedSource {
        let source =
            SourceFile::from_bytes(SourceId::new(0), "test.ling", input.as_bytes().to_vec())
                .unwrap();
        parse(&source)
    }

    #[test]
    fn parses_hello_world() {
        let parsed = parse_text(
            "module Main\n    requires Console.Write\n\nlet main () =\n    Console.write \"你好，零\"\n",
        );

        assert!(parsed.lexical_errors().is_empty());
        assert!(
            parsed.parse_errors().is_empty(),
            "{:?}",
            parsed.parse_errors()
        );
        assert_eq!(parsed.tree().root().kind(), NodeKind::Program);
        assert_eq!(parsed.tree().root().children().len(), 2);
    }

    #[test]
    fn parses_module_imports_with_optional_aliases() {
        let parsed = parse_text(
            "module Main\n\nimport Game.Math\nimport Game.Text as 文本\n\nlet main () = ()\n",
        );

        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let children = parsed.tree().root().children();
        assert_eq!(children.len(), 4);
        assert_eq!(children[1].kind(), NodeKind::ImportDeclaration);
        assert_eq!(children[2].kind(), NodeKind::ImportDeclaration);
    }

    #[test]
    fn parses_record_adt_match_and_assignment() {
        let parsed = parse_text(
            "type 人物 =\n    { 姓名: Text\n      mutable 血量: Int }\n\ntype 状态 =\n    | 健康\n    | 受伤 of Int\n\nlet 描述 状态 =\n    match 状态 with\n    | 健康 ->\n        \"健康\"\n    | 受伤 程度 ->\n        程度\n\nlet 受伤 人物 =\n    人物.血量 <-\n        max 0 (人物.血量 - 1)\n",
        );

        assert!(parsed.lexical_errors().is_empty());
        assert!(
            parsed.parse_errors().is_empty(),
            "{:?}",
            parsed.parse_errors()
        );
    }

    #[test]
    fn parses_multiline_pipeline_and_single_line_records() {
        let parsed = parse_text(
            "let 总伤害 人物列表 =\n    人物列表\n    |> map 计算伤害\n    |> sum\n\nlet 人物 = { 姓名 = \"关羽\"; 血量 = 100; }\n",
        );

        assert!(parsed.lexical_errors().is_empty());
        assert!(
            parsed.parse_errors().is_empty(),
            "{:?}",
            parsed.parse_errors()
        );
    }

    #[test]
    fn parses_nested_bodies_followed_by_sibling_expressions() {
        let parsed = parse_text(concat!(
            "type 人物 =\n",
            "    { 血量: Int }\n",
            "\n",
            "let 更新 人物 =\n",
            "    { 人物 with\n",
            "        血量 = 70 }\n",
            "\n",
            "let main () =\n",
            "    let 总和 =\n",
            "        [1; 2; 3]\n",
            "        |> map 更新\n",
            "\n",
            "    Console.write \"done\"\n",
        ));

        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    }

    #[test]
    fn parses_experimental_handler_cst_without_reserving_contextual_names() {
        let parsed = parse_text(
            "let main value =\n    handle value with\n        operation Clock.now() -> 1\n        operation Random.next(seed, resume) -> seed\n\nlet handle = 1\nlet use_handle = handle\n",
        );

        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let declarations = parsed.tree().root().children();
        let let_body = declarations[0].children().last().expect("let body");
        let handler = let_body.children().first().expect("handler expression");
        assert_eq!(handler.kind(), NodeKind::HandleExpression);
        assert_eq!(handler.children().len(), 3);
        assert_eq!(handler.children()[1].kind(), NodeKind::HandlerClause);
        assert_eq!(handler.children()[2].kind(), NodeKind::HandlerClause);
        assert_eq!(declarations[1].kind(), NodeKind::LetDeclaration);
        assert_eq!(declarations[2].kind(), NodeKind::LetDeclaration);
    }

    #[test]
    fn rejects_incomplete_experimental_handler_shapes() {
        for source in [
            "let value =\n    handle value with\n",
            "let value =\n    handle value with\n        operation Clock.now() 1\n",
        ] {
            let parsed = parse_text(source);
            assert!(!parsed.is_valid(), "unexpectedly accepted `{source}`");
            assert!(parsed.parse_errors().len() < 8);
        }
    }

    #[test]
    fn parses_record_wildcard_and_tuple_patterns() {
        let parsed = parse_text(concat!(
            "module Main\n\n",
            "type Point = { x: Int; y: Int }\n\n",
            "let describe point =\n",
            "    match point with\n",
            "    | { x = value; y = _ } -> value\n\n",
            "let first pair =\n",
            "    match pair with\n",
            "    | (left, _) -> left\n",
        ));
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
    }

    #[test]
    fn parses_generic_function_product_and_tuple_types() {
        let parsed = parse_text(
            "type Option<'a> =\n    | Some of 'a\n    | None\n\n\
             type Result<'a, 'e> =\n    | Ok of 'a\n    | Error of 'e\n\n\
             type Mapper<'a, 'b> = ('a -> 'b) * List<'a>\n\
             let identity value: 'a -> 'a = value\n",
        );

        assert!(parsed.lexical_errors().is_empty());
        assert!(
            parsed.parse_errors().is_empty(),
            "{:?}",
            parsed.parse_errors()
        );
    }

    #[test]
    fn parses_trait_and_impl_declarations_with_spans_and_members() {
        let parsed = parse_text(
            r#"trait Renderable<'a> =
    render: 'a -> Text
    measure: 'a -> Int

impl Renderable Item =
    let render item = item.name
    let measure item = 1
"#,
        );

        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let children = parsed.tree().root().children();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].kind(), NodeKind::TraitDeclaration);
        let trait_range = children[0].token_range();
        assert!(trait_range.start < trait_range.end);
        assert_eq!(children[0].children().len(), 3);
        assert_eq!(
            children[0].children()[0].kind(),
            NodeKind::TypeParameterList
        );
        assert_eq!(children[0].children()[1].kind(), NodeKind::TraitMember);
        assert_eq!(children[1].kind(), NodeKind::ImplDeclaration);
        let impl_range = children[1].token_range();
        assert!(impl_range.start < impl_range.end);
        assert_eq!(children[1].children().len(), 4);
        assert!(
            children[1]
                .children()
                .iter()
                .skip(2)
                .all(|child| child.kind() == NodeKind::ImplMember)
        );
    }

    #[test]
    fn parses_requires_constraints_after_generic_parameters() {
        let parsed = parse_text(
            "let render<'a> requires { Renderable<'a>,\n Bounded<'a> } value =\n    value\n",
        );

        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let declaration = &parsed.tree().root().children()[0];
        assert_eq!(declaration.kind(), NodeKind::LetDeclaration);
        assert_eq!(declaration.children().len(), 5);
        assert_eq!(
            declaration.children()[1].kind(),
            NodeKind::TypeParameterList
        );
        assert_eq!(declaration.children()[2].kind(), NodeKind::ConstraintBlock);
        assert_eq!(declaration.children()[2].children().len(), 2);
    }

    #[test]
    fn rejects_structurally_incomplete_type_syntax() {
        for source in [
            "type Broken<'a,> = Int\n",
            "type Broken = Result<Int,>\n",
            "type Broken = Int ->\n",
            "let value: ' = 1\n",
        ] {
            let parsed = parse_text(source);
            assert!(!parsed.is_valid(), "unexpectedly accepted `{source}`");
        }
    }

    #[test]
    fn enforces_the_documented_parser_recursion_boundary() {
        fn nested_if(depth: usize) -> String {
            format!(
                "let value = {}true{}\n",
                "if ".repeat(depth),
                " then 0 else 0".repeat(depth)
            )
        }

        let at_limit = parse_text(&nested_if(511));
        assert!(at_limit.is_valid(), "{:?}", at_limit.parse_errors());

        let beyond_limit = parse_text(&nested_if(512));
        assert!(
            beyond_limit
                .parse_errors()
                .iter()
                .any(|error| error.kind() == &ParseErrorKind::RecursionLimit)
        );
    }

    #[test]
    fn reports_bounded_syntax_errors() {
        let parsed = parse_text("let =\n    )\n");

        assert!(!parsed.lexical_errors().is_empty() || !parsed.parse_errors().is_empty());
        assert!(parsed.lexical_errors().len() + parsed.parse_errors().len() < 8);
    }
}
