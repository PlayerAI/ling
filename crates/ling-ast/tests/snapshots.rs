use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use ling_ast::{
    BinaryOperator, Expression, ExpressionKind, Item, LetDeclaration, Literal, MatchCase, Name,
    Pattern, PatternAtom, Program, QualifiedName, RecordField, SequenceElement, TypeAtom,
    TypeDefinition, TypeExpression, UnaryOperator, lower,
};
use ling_source::{SourceFile, SourceId, Span};
use ling_syntax::{CstNode, ParsedSource, SyntaxTree, parse};

const CASES: &[&str] = &["hello", "record-match", "pipeline-assignment"];

#[test]
fn cst_and_ast_snapshots_are_stable() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/snapshots");
    for (index, case) in CASES.iter().enumerate() {
        let directory = root.join(case);
        let source_text = fs::read_to_string(directory.join("case.ling"))
            .map(|text| normalize_newlines(&text))
            .unwrap_or_else(|error| panic!("failed to read snapshot source `{case}`: {error}"));
        let source = SourceFile::from_bytes(
            SourceId::new(u32::try_from(index).expect("three snapshot cases")),
            format!("{case}/case.ling"),
            source_text.into_bytes(),
        )
        .expect("snapshot source is valid UTF-8");
        let parsed = parse(&source);
        assert!(
            parsed.is_valid(),
            "snapshot source `{case}` failed to parse: lexical={:?}, parse={:?}",
            parsed.lexical_errors(),
            parsed.parse_errors()
        );
        let program = lower(&source, &parsed).expect("valid snapshot CST lowers");
        let actual = format!(
            "CST\n{}\nAST\n{}",
            render_cst(&source, &parsed),
            render_ast(&program)
        );
        let expected = fs::read_to_string(directory.join("syntax.snap"))
            .map(|text| normalize_newlines(&text))
            .unwrap_or_else(|error| panic!("failed to read snapshot `{case}`: {error}"));
        if expected == "pending\n" {
            panic!("generated syntax snapshot `{case}`:\n{actual}");
        }
        assert_eq!(
            actual, expected,
            "syntax snapshot `{case}` changed; review semantic impact before updating it"
        );
    }
}

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}

fn render_cst(source: &SourceFile, parsed: &ParsedSource) -> String {
    let tree = parsed.tree();
    let mut output = String::from("tokens\n");
    for (index, token) in tree.tokens().iter().enumerate() {
        let span = token.span();
        let text = &source.original_text()[span.start().get() as usize..span.end().get() as usize];
        writeln!(
            output,
            "  {index}: {} @{}..{} \"{}\"",
            token.kind().name(),
            span.start().get(),
            span.end().get(),
            escape(text)
        )
        .expect("writing to String cannot fail");
    }
    output.push_str("tree\n");
    render_cst_node(&mut output, tree, tree.root(), 1);
    output
}

fn render_cst_node(output: &mut String, tree: &SyntaxTree, node: &CstNode, depth: usize) {
    let range = node.token_range();
    writeln!(
        output,
        "{}{} tokens={}..{}",
        "  ".repeat(depth),
        node.kind().name(),
        range.start,
        range.end
    )
    .expect("writing to String cannot fail");
    for child in node.children() {
        render_cst_node(output, tree, child, depth + 1);
    }
    let _ = tree;
}

fn render_ast(program: &Program) -> String {
    let mut output = String::new();
    line(&mut output, 0, "program", program.span);
    for item in &program.items {
        render_item(&mut output, item, 1);
    }
    output
}

fn render_item(output: &mut String, item: &Item, depth: usize) {
    match item {
        Item::Module(module) => {
            line_with(
                output,
                depth,
                "module",
                module.span,
                &qualified_text(&module.name),
            );
            for capability in &module.requires {
                line_with(
                    output,
                    depth + 1,
                    "requires",
                    capability.span,
                    &qualified_text(capability),
                );
            }
        }
        Item::Import(import) => {
            let value = match &import.alias {
                Some(alias) => {
                    format!("{} as {}", qualified_text(&import.module), alias.normalized)
                }
                None => qualified_text(&import.module),
            };
            line_with(output, depth, "import", import.span, &value);
        }
        Item::Let(declaration) => render_let(output, declaration, depth),
        Item::Type(declaration) => {
            line_with(
                output,
                depth,
                "type",
                declaration.span,
                &declaration.name.normalized,
            );
            for parameter in &declaration.parameters {
                render_name(output, parameter, depth + 1, "type_parameter");
            }
            match &declaration.definition {
                TypeDefinition::Record(fields) => {
                    plain_line(output, depth + 1, "record");
                    for field in fields {
                        line_with(
                            output,
                            depth + 2,
                            if field.mutable {
                                "mutable_field"
                            } else {
                                "field"
                            },
                            field.span,
                            &field.name.normalized,
                        );
                        render_type_expression(output, &field.field_type, depth + 3);
                    }
                }
                TypeDefinition::Variant(cases) => {
                    plain_line(output, depth + 1, "variant");
                    for case in cases {
                        line_with(output, depth + 2, "case", case.span, &case.name.normalized);
                        if let Some(payload) = &case.payload {
                            render_type_expression(output, payload, depth + 3);
                        }
                    }
                }
                TypeDefinition::Alias(alias) => {
                    plain_line(output, depth + 1, "alias");
                    render_type_expression(output, alias, depth + 2);
                }
            }
        }
        Item::Trait(declaration) => {
            line_with(
                output,
                depth,
                "trait",
                declaration.span,
                &declaration.name.normalized,
            );
            for parameter in &declaration.parameters {
                render_name(output, parameter, depth + 1, "type_parameter");
            }
            for member in &declaration.members {
                line_with(
                    output,
                    depth + 1,
                    "member",
                    member.span,
                    &member.name.normalized,
                );
                render_type_expression(output, &member.signature, depth + 2);
            }
        }
        Item::Impl(declaration) => {
            line_with(
                output,
                depth,
                "impl",
                declaration.span,
                &qualified_text(&declaration.trait_name),
            );
            render_type_expression(output, &declaration.receiver, depth + 1);
            for member in &declaration.members {
                render_let(output, member, depth + 1);
            }
        }
    }
}

fn render_let(output: &mut String, declaration: &LetDeclaration, depth: usize) {
    let flags = match (declaration.recursive, declaration.mutable) {
        (true, true) => "rec mutable",
        (true, false) => "rec",
        (false, true) => "mutable",
        (false, false) => "",
    };
    line_with(output, depth, "let", declaration.span, flags);
    render_pattern(output, &declaration.binding, depth + 1, "binding");
    for parameter in &declaration.type_parameters {
        render_name(output, parameter, depth + 1, "type_parameter");
    }
    for constraint in &declaration.constraints {
        render_type_expression(output, constraint, depth + 1);
    }
    for parameter in &declaration.parameters {
        render_pattern(output, parameter, depth + 1, "parameter");
    }
    if let Some(annotation) = &declaration.annotation {
        render_type_expression(output, annotation, depth + 1);
    }
    render_expression(output, &declaration.value, depth + 1);
}

fn render_type_expression(output: &mut String, expression: &TypeExpression, depth: usize) {
    let mut atoms = Vec::new();
    for atom in &expression.atoms {
        atoms.push(match atom {
            TypeAtom::Name(name) => name.normalized.clone(),
            TypeAtom::Variable(name) => format!("'{}", name.normalized),
            TypeAtom::Arrow => "->".to_owned(),
            TypeAtom::Product => "*".to_owned(),
            TypeAtom::LeftParen => "(".to_owned(),
            TypeAtom::RightParen => ")".to_owned(),
            TypeAtom::LeftAngle => "<".to_owned(),
            TypeAtom::RightAngle => ">".to_owned(),
            TypeAtom::Comma => ",".to_owned(),
            TypeAtom::Dot => ".".to_owned(),
        });
    }
    line_with(
        output,
        depth,
        "type_expression",
        expression.span,
        &atoms.join(" "),
    );
}

fn render_pattern(output: &mut String, pattern: &Pattern, depth: usize, label: &str) {
    let mut atoms = Vec::new();
    for atom in &pattern.atoms {
        atoms.push(match atom {
            PatternAtom::Name(name) => name.normalized.clone(),
            PatternAtom::Literal(literal) => literal_text(literal),
            PatternAtom::Dot => ".".to_owned(),
            PatternAtom::LeftParen => "(".to_owned(),
            PatternAtom::RightParen => ")".to_owned(),
            PatternAtom::LeftBrace => "{".to_owned(),
            PatternAtom::RightBrace => "}".to_owned(),
            PatternAtom::Equals => "=".to_owned(),
            PatternAtom::Semicolon => ";".to_owned(),
            PatternAtom::Comma => ",".to_owned(),
        });
    }
    line_with(output, depth, label, pattern.span, &atoms.join(" "));
}

fn render_expression(output: &mut String, expression: &Expression, depth: usize) {
    match &expression.kind {
        ExpressionKind::Sequence(elements) => {
            line(output, depth, "sequence", expression.span);
            for element in elements {
                match element {
                    SequenceElement::Let(declaration) => render_let(output, declaration, depth + 1),
                    SequenceElement::Expression(expression) => {
                        render_expression(output, expression, depth + 1);
                    }
                }
            }
        }
        ExpressionKind::Handle { body, clauses } => {
            line(output, depth, "handle", expression.span);
            render_expression(output, body, depth + 1);
            for clause in clauses {
                let operation = clause
                    .operation
                    .segments
                    .iter()
                    .map(|segment| segment.normalized.as_str())
                    .collect::<Vec<_>>()
                    .join(".");
                line_with(output, depth + 1, "handler_clause", clause.span, &operation);
                for parameter in &clause.parameters {
                    render_pattern(output, parameter, depth + 2, "parameter");
                }
                if let Some(resume) = &clause.resume {
                    render_name(output, resume, depth + 2, "resume");
                }
                render_expression(output, &clause.body, depth + 2);
            }
        }
        ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            line(output, depth, "if", expression.span);
            render_expression(output, condition, depth + 1);
            render_expression(output, then_branch, depth + 1);
            render_expression(output, else_branch, depth + 1);
        }
        ExpressionKind::Match { scrutinee, cases } => {
            line(output, depth, "match", expression.span);
            render_expression(output, scrutinee, depth + 1);
            for case in cases {
                render_match_case(output, case, depth + 1);
            }
        }
        ExpressionKind::Assignment { place, value } => {
            line(output, depth, "assignment", expression.span);
            render_expression(output, place, depth + 1);
            render_expression(output, value, depth + 1);
        }
        ExpressionKind::Pipeline { input, target } => {
            line(output, depth, "pipeline", expression.span);
            render_expression(output, input, depth + 1);
            render_expression(output, target, depth + 1);
        }
        ExpressionKind::Binary {
            operator,
            left,
            right,
        } => {
            line_with(
                output,
                depth,
                "binary",
                expression.span,
                binary_name(*operator),
            );
            render_expression(output, left, depth + 1);
            render_expression(output, right, depth + 1);
        }
        ExpressionKind::Unary { operator, operand } => {
            line_with(
                output,
                depth,
                "unary",
                expression.span,
                unary_name(*operator),
            );
            render_expression(output, operand, depth + 1);
        }
        ExpressionKind::Application { function, argument } => {
            line(output, depth, "application", expression.span);
            render_expression(output, function, depth + 1);
            render_expression(output, argument, depth + 1);
        }
        ExpressionKind::Projection { target, field } => {
            line_with(
                output,
                depth,
                "projection",
                expression.span,
                &field.normalized,
            );
            render_expression(output, target, depth + 1);
        }
        ExpressionKind::Name(name) => {
            render_name(output, name, depth, "name");
        }
        ExpressionKind::Literal(literal) => {
            line_with(
                output,
                depth,
                "literal",
                expression.span,
                &literal_text(literal),
            );
        }
        ExpressionKind::Unit => line(output, depth, "unit", expression.span),
        ExpressionKind::Tuple(elements) => {
            line(output, depth, "tuple", expression.span);
            for element in elements {
                render_expression(output, element, depth + 1);
            }
        }
        ExpressionKind::Record(fields) => {
            line(output, depth, "record", expression.span);
            render_record_fields(output, fields, depth + 1);
        }
        ExpressionKind::RecordUpdate { base, fields } => {
            line(output, depth, "record_update", expression.span);
            render_expression(output, base, depth + 1);
            render_record_fields(output, fields, depth + 1);
        }
        ExpressionKind::List(elements) => {
            line(output, depth, "list", expression.span);
            for element in elements {
                render_expression(output, element, depth + 1);
            }
        }
    }
}

fn render_match_case(output: &mut String, case: &MatchCase, depth: usize) {
    line(output, depth, "match_case", case.span);
    render_pattern(output, &case.pattern, depth + 1, "pattern");
    if let Some(guard) = &case.guard {
        plain_line(output, depth + 1, "guard");
        render_expression(output, guard, depth + 2);
    }
    render_expression(output, &case.body, depth + 1);
}

fn render_record_fields(output: &mut String, fields: &[RecordField], depth: usize) {
    for field in fields {
        line_with(output, depth, "field", field.span, &field.name.normalized);
        render_expression(output, &field.value, depth + 1);
    }
}

fn render_name(output: &mut String, name: &Name, depth: usize, label: &str) {
    line_with(output, depth, label, name.span, &name.normalized);
}

fn line(output: &mut String, depth: usize, label: &str, span: Span) {
    line_with(output, depth, label, span, "");
}

fn line_with(output: &mut String, depth: usize, label: &str, span: Span, value: &str) {
    let suffix = if value.is_empty() {
        String::new()
    } else {
        format!(" {value}")
    };
    writeln!(
        output,
        "{}{} @{}..{}{}",
        "  ".repeat(depth),
        label,
        span.start().get(),
        span.end().get(),
        suffix
    )
    .expect("writing to String cannot fail");
}

fn plain_line(output: &mut String, depth: usize, label: &str) {
    writeln!(output, "{}{label}", "  ".repeat(depth)).expect("writing to String cannot fail");
}

fn qualified_text(name: &QualifiedName) -> String {
    name.segments
        .iter()
        .map(|segment| segment.normalized.as_str())
        .collect::<Vec<_>>()
        .join(".")
}

fn literal_text(literal: &Literal) -> String {
    match literal {
        Literal::Integer { radix, digits } => format!("int({radix},{digits})"),
        Literal::Float(value) => format!("float({value})"),
        Literal::Text(value) => format!("text(\"{}\")", escape(value)),
        Literal::Boolean(value) => format!("bool({value})"),
    }
}

const fn binary_name(operator: BinaryOperator) -> &'static str {
    match operator {
        BinaryOperator::Equal => "equal",
        BinaryOperator::NotEqual => "not_equal",
        BinaryOperator::Less => "less",
        BinaryOperator::LessEqual => "less_equal",
        BinaryOperator::Greater => "greater",
        BinaryOperator::GreaterEqual => "greater_equal",
        BinaryOperator::Add => "add",
        BinaryOperator::Subtract => "subtract",
        BinaryOperator::Multiply => "multiply",
        BinaryOperator::Divide => "divide",
        BinaryOperator::Remainder => "remainder",
        BinaryOperator::BooleanAnd => "boolean_and",
        BinaryOperator::BooleanOr => "boolean_or",
    }
}

const fn unary_name(operator: UnaryOperator) -> &'static str {
    match operator {
        UnaryOperator::Positive => "positive",
        UnaryOperator::Negative => "negative",
    }
}

fn escape(value: &str) -> String {
    let mut escaped = String::new();
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_control() => escaped.extend(character.escape_default()),
            character => escaped.push(character),
        }
    }
    escaped
}
