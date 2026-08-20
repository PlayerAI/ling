use std::fs;
use std::path::Path;

use ling_ast::{BinaryOperator, Expression, ExpressionKind, Item, Literal, UnaryOperator, lower};
use ling_source::{SourceFile, SourceId};
use ling_syntax::parse;

const FIXTURE_PATH: &str = "../../editors/tree-sitter-ling/test/fixtures/expression-precedence.tsv";

#[test]
fn compiler_ast_matches_the_shared_precedence_corpus() {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
    let fixture = fs::read_to_string(&fixture_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", fixture_path.display()));

    for (index, line) in fixture.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let [case, expression, expected] = line
            .split('\t')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("invalid fixture row {}: {line}", index + 1));
        let text = format!("let result = {expression}\n");
        let source = SourceFile::from_bytes(
            SourceId::new(u32::try_from(index).expect("fixture row count fits u32")),
            format!("{case}.ling"),
            text.into_bytes(),
        )
        .expect("fixture is valid UTF-8");
        let parsed = parse(&source);
        assert!(
            parsed.is_valid(),
            "{case} failed to parse: lexical={:?}, parse={:?}",
            parsed.lexical_errors(),
            parsed.parse_errors()
        );
        let program = lower(&source, &parsed)
            .unwrap_or_else(|error| panic!("{case} failed to lower: {error}"));
        let Item::Let(declaration) = &program.items[0] else {
            panic!("{case} did not lower to a let declaration");
        };
        assert_eq!(render_expression(&declaration.value), expected, "{case}");
    }
}

#[test]
fn unparenthesized_assignment_chaining_is_rejected() {
    let source = SourceFile::from_bytes(
        SourceId::new(0),
        "assignment-chain.ling",
        b"let result = first <- second <- third\n".to_vec(),
    )
    .expect("source is valid UTF-8");

    let parsed = parse(&source);
    assert!(!parsed.is_valid(), "assignment chaining must be rejected");
    assert!(!parsed.parse_errors().is_empty());
}

#[test]
fn textual_operator_names_are_not_boolean_aliases() {
    let source = SourceFile::from_bytes(
        SourceId::new(0),
        "operator-alias.ling",
        b"let result = true or false\n".to_vec(),
    )
    .expect("source is valid UTF-8");

    let parsed = parse(&source);
    assert!(parsed.is_valid(), "or remains an ordinary identifier");
    let program = lower(&source, &parsed).expect("valid source lowers");
    let Item::Let(declaration) = &program.items[0] else {
        panic!("source did not lower to a let declaration");
    };
    assert_eq!(
        render_expression(&declaration.value),
        "apply(apply(b(true),n(or)),b(false))"
    );
}

fn render_expression(expression: &Expression) -> String {
    match &expression.kind {
        ExpressionKind::Assignment { place, value } => format!(
            "assign({},{})",
            render_expression(place),
            render_expression(value)
        ),
        ExpressionKind::Pipeline { input, target } => format!(
            "pipe({},{})",
            render_expression(input),
            render_expression(target)
        ),
        ExpressionKind::Binary {
            operator,
            left,
            right,
        } => format!(
            "{}({},{})",
            binary_name(*operator),
            render_expression(left),
            render_expression(right)
        ),
        ExpressionKind::Unary { operator, operand } => {
            format!("{}({})", unary_name(*operator), render_expression(operand))
        }
        ExpressionKind::Application { function, argument } => format!(
            "apply({},{})",
            render_expression(function),
            render_expression(argument)
        ),
        ExpressionKind::Projection { target, field } => {
            format!("proj({},{})", render_expression(target), field.normalized)
        }
        ExpressionKind::Name(name) => format!("n({})", name.normalized),
        ExpressionKind::Literal(Literal::Integer { radix: 10, digits }) => {
            format!("i({digits})")
        }
        ExpressionKind::Literal(Literal::Boolean(value)) => format!("b({value})"),
        other => panic!("unexpected expression in precedence fixture: {other:?}"),
    }
}

const fn binary_name(operator: BinaryOperator) -> &'static str {
    match operator {
        BinaryOperator::BooleanAnd => "and",
        BinaryOperator::BooleanOr => "or",
        BinaryOperator::Equal => "eq",
        BinaryOperator::NotEqual => "neq",
        BinaryOperator::Less => "lt",
        BinaryOperator::LessEqual => "lte",
        BinaryOperator::Greater => "gt",
        BinaryOperator::GreaterEqual => "gte",
        BinaryOperator::Add => "add",
        BinaryOperator::Subtract => "sub",
        BinaryOperator::Multiply => "mul",
        BinaryOperator::Divide => "div",
        BinaryOperator::Remainder => "rem",
    }
}

const fn unary_name(operator: UnaryOperator) -> &'static str {
    match operator {
        UnaryOperator::Positive => "pos",
        UnaryOperator::Negative => "neg",
    }
}
