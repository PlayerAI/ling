use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_source::{SourceFile, Span};

use crate::cst::{CstNode, NodeKind, SyntaxTree};
use crate::lexer::{LexError, lex};
use crate::token::{Token, TokenKind};

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
}

impl<'tokens> Parser<'tokens> {
    fn new(tokens: &'tokens [Token]) -> Self {
        Self {
            tokens,
            position: 0,
            errors: Vec::new(),
            depth: 0,
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
                _ => {
                    self.unexpected(
                        &[
                            TokenKind::Module,
                            TokenKind::Import,
                            TokenKind::Let,
                            TokenKind::Type,
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
            TokenKind::Identifier
            | TokenKind::Integer
            | TokenKind::Float
            | TokenKind::Text
            | TokenKind::True
            | TokenKind::False => self.bump_any(),
            TokenKind::LeftParen => {
                self.bump_any();
                while !self.at(TokenKind::RightParen) && !self.at(TokenKind::Eof) {
                    self.parse_pattern(context);
                    if !self.eat(TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(TokenKind::RightParen, context);
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
            let child = if self.at(TokenKind::Let) {
                self.parse_let_declaration()
            } else {
                self.parse_expression()
            };
            children.push(child);
            if self.at(TokenKind::Newline) {
                self.eat_newlines(false);
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
        let expression = match self.current_kind() {
            TokenKind::If => self.parse_if_expression(),
            TokenKind::Match => self.parse_match_expression(),
            _ => self.parse_assignment_expression(),
        };
        self.depth -= 1;
        expression
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
        while self.starts_pattern()
            && !matches!(self.current_kind(), TokenKind::When | TokenKind::RightArrow)
        {
            let before = self.position;
            self.parse_pattern("match pattern");
            if self.position == before {
                break;
            }
        }
        CstNode::new(NodeKind::Pattern, start..self.position, Vec::new())
    }

    fn parse_assignment_expression(&mut self) -> CstNode {
        let left = self.parse_pipeline_expression();
        if !self.eat(TokenKind::LeftArrow) {
            return left;
        }
        let start = left.token_range().start;
        let indented = self.begin_operator_continuation("assignment right-hand side");
        let right = self.parse_expression();
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
        let mut left = self.parse_equality_expression();
        while let Some(operator_index) = self.pipeline_operator_index() {
            self.position = operator_index;
            self.bump(TokenKind::PipeGreater);
            let indented = self.begin_operator_continuation("pipeline right-hand side");
            let right = self.parse_equality_expression();
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
