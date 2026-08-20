//! Span-preserving, unresolved high-level IR for Ling Seed.

use std::error::Error;
use std::fmt;

use ling_ast as ast;
use ling_source::Span;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExpressionId(u32);

impl ExpressionId {
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReferenceId(u32);

impl ReferenceId {
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BindingId(u32);

impl BindingId {
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PatternId(u32);

impl PatternId {
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub source_name: String,
    pub span: Span,
    pub module: Module,
    pub imports: Vec<Import>,
    pub definitions: Vec<Definition>,
    pub types: Vec<TypeDeclaration>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Module {
    pub span: Span,
    pub name: QualifiedName,
    pub explicit: bool,
    pub requires: Vec<QualifiedName>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Import {
    pub span: Span,
    pub module: QualifiedName,
    pub alias: Name,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedName {
    pub span: Span,
    pub segments: Vec<Name>,
}

impl QualifiedName {
    #[must_use]
    pub fn normalized(&self) -> String {
        self.segments
            .iter()
            .map(|segment| segment.normalized.as_str())
            .collect::<Vec<_>>()
            .join(".")
    }
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
pub struct Definition {
    pub span: Span,
    pub name: Name,
    pub recursive: bool,
    pub mutable: bool,
    pub parameters: Vec<Pattern>,
    pub annotation: Option<TypeSyntax>,
    pub value: Expression,
    /// REPL-only generation used to isolate session identity from file identity.
    pub session_generation: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalBinding {
    pub span: Span,
    pub id: BindingId,
    pub name: Name,
    pub recursive: bool,
    pub mutable: bool,
    pub parameters: Vec<Pattern>,
    pub annotation: Option<TypeSyntax>,
    pub value: Expression,
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
    Alias(TypeSyntax),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeField {
    pub span: Span,
    pub name: Name,
    pub mutable: bool,
    pub field_type: TypeSyntax,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariantCase {
    pub span: Span,
    pub name: Name,
    pub payload: Option<TypeSyntax>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeSyntax {
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
    pub id: PatternId,
    pub span: Span,
    pub kind: PatternKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordPatternField {
    pub span: Span,
    pub name: Name,
    pub pattern: Pattern,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PatternKind {
    Binding {
        id: BindingId,
        name: Name,
    },
    Wildcard,
    Unit,
    Literal(Literal),
    Tuple(Vec<Pattern>),
    Record(Vec<RecordPatternField>),
    Constructor {
        qualifier: Option<Name>,
        name: Name,
        arguments: Vec<Pattern>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expression {
    pub id: ExpressionId,
    pub span: Span,
    pub kind: ExpressionKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpressionKind {
    Sequence(Vec<SequenceElement>),
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
        place: Place,
        value: Box<Expression>,
    },
    Application {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Projection {
        reference: ReferenceId,
        target: Box<Expression>,
        field: Name,
    },
    Name {
        reference: ReferenceId,
        name: Name,
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
    Let(LocalBinding),
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
pub struct Place {
    pub span: Span,
    pub root_reference: ReferenceId,
    pub root: Name,
    pub fields: Vec<Name>,
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
    pub kind: LowerErrorKind,
    pub span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LowerErrorKind {
    DuplicateModule,
    ModuleMustBeFirst,
    ImportAfterDeclaration,
    InvalidTopLevelBinding,
    InvalidPattern,
    InvalidAssignmentPlace,
}

impl fmt::Display for LowerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self.kind {
            LowerErrorKind::DuplicateModule => "a file may declare at most one module",
            LowerErrorKind::ModuleMustBeFirst => "the module declaration must be first",
            LowerErrorKind::ImportAfterDeclaration => "imports must precede ordinary declarations",
            LowerErrorKind::InvalidTopLevelBinding => {
                "a top-level declaration must bind exactly one name"
            }
            LowerErrorKind::InvalidPattern => "the pattern is not supported by Ling Seed",
            LowerErrorKind::InvalidAssignmentPlace => {
                "the left side of assignment is not a syntactic place"
            }
        };
        formatter.write_str(message)
    }
}

impl Error for LowerError {}

/// Lowers an AST into unresolved HIR and applies syntax-directed normalizations.
pub fn lower(
    source_name: impl Into<String>,
    program: &ast::Program,
) -> Result<Program, LowerError> {
    Lowerer::new(source_name.into(), IdCounters::default()).program(program)
}

/// Next local HIR IDs for merging independently parsed REPL submissions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IdCounters {
    pub expression: u32,
    pub reference: u32,
    pub binding: u32,
    pub pattern: u32,
}

/// Lowers one source while allocating IDs after `counters`.
pub fn lower_with_counters(
    source_name: impl Into<String>,
    program: &ast::Program,
    counters: IdCounters,
) -> Result<(Program, IdCounters), LowerError> {
    let mut lowerer = Lowerer::new(source_name.into(), counters);
    let program = lowerer.program(program)?;
    Ok((program, lowerer.counters()))
}

struct Lowerer {
    source_name: String,
    next_expression: u32,
    next_reference: u32,
    next_binding: u32,
    next_pattern: u32,
}

impl Lowerer {
    const fn new(source_name: String, counters: IdCounters) -> Self {
        Self {
            source_name,
            next_expression: counters.expression,
            next_reference: counters.reference,
            next_binding: counters.binding,
            next_pattern: counters.pattern,
        }
    }

    const fn counters(&self) -> IdCounters {
        IdCounters {
            expression: self.next_expression,
            reference: self.next_reference,
            binding: self.next_binding,
            pattern: self.next_pattern,
        }
    }

    fn program(&mut self, program: &ast::Program) -> Result<Program, LowerError> {
        let mut module = None;
        let mut imports = Vec::new();
        let mut definitions = Vec::new();
        let mut types = Vec::new();
        let mut saw_declaration = false;

        for (index, item) in program.items.iter().enumerate() {
            match item {
                ast::Item::Module(declaration) => {
                    if module.is_some() {
                        return Err(self.error(LowerErrorKind::DuplicateModule, declaration.span));
                    }
                    if index != 0 {
                        return Err(self.error(LowerErrorKind::ModuleMustBeFirst, declaration.span));
                    }
                    module = Some(Module {
                        span: declaration.span,
                        name: qualified_name(&declaration.name),
                        explicit: true,
                        requires: declaration.requires.iter().map(qualified_name).collect(),
                    });
                }
                ast::Item::Import(declaration) => {
                    if saw_declaration {
                        return Err(
                            self.error(LowerErrorKind::ImportAfterDeclaration, declaration.span)
                        );
                    }
                    let module_name = qualified_name(&declaration.module);
                    let alias = declaration.alias.as_ref().map(name).unwrap_or_else(|| {
                        module_name
                            .segments
                            .last()
                            .expect("AST qualified names are non-empty")
                            .clone()
                    });
                    imports.push(Import {
                        span: declaration.span,
                        module: module_name,
                        alias,
                    });
                }
                ast::Item::Let(declaration) => {
                    saw_declaration = true;
                    definitions.push(self.definition(declaration)?);
                }
                ast::Item::Type(declaration) => {
                    saw_declaration = true;
                    types.push(self.type_declaration(declaration));
                }
            }
        }

        let module = module.unwrap_or_else(|| Module {
            span: program.span,
            name: QualifiedName {
                span: program.span,
                segments: vec![Name {
                    span: program.span,
                    source: "Main".to_owned(),
                    normalized: "Main".to_owned(),
                    skeleton: "Main".to_owned(),
                    scripts: vec!["Latin".to_owned()],
                    suspicious_mixed_script: false,
                }],
            },
            explicit: false,
            requires: Vec::new(),
        });

        Ok(Program {
            source_name: self.source_name.clone(),
            span: program.span,
            module,
            imports,
            definitions,
            types,
        })
    }

    fn definition(&mut self, declaration: &ast::LetDeclaration) -> Result<Definition, LowerError> {
        let [ast::PatternAtom::Name(binding)] = declaration.binding.atoms.as_slice() else {
            return Err(self.error(
                LowerErrorKind::InvalidTopLevelBinding,
                declaration.binding.span,
            ));
        };
        Ok(Definition {
            span: declaration.span,
            name: name(binding),
            recursive: declaration.recursive,
            mutable: declaration.mutable,
            parameters: declaration
                .parameters
                .iter()
                .map(|pattern| self.pattern(pattern))
                .collect::<Result<Vec<_>, _>>()?,
            annotation: declaration.annotation.as_ref().map(type_syntax),
            value: self.expression(&declaration.value)?,
            session_generation: None,
        })
    }

    fn local_binding(
        &mut self,
        declaration: &ast::LetDeclaration,
    ) -> Result<LocalBinding, LowerError> {
        let [ast::PatternAtom::Name(binding)] = declaration.binding.atoms.as_slice() else {
            return Err(self.error(LowerErrorKind::InvalidPattern, declaration.binding.span));
        };
        let id = self.binding_id();
        Ok(LocalBinding {
            span: declaration.span,
            id,
            name: name(binding),
            recursive: declaration.recursive,
            mutable: declaration.mutable,
            parameters: declaration
                .parameters
                .iter()
                .map(|pattern| self.pattern(pattern))
                .collect::<Result<Vec<_>, _>>()?,
            annotation: declaration.annotation.as_ref().map(type_syntax),
            value: self.expression(&declaration.value)?,
        })
    }

    fn type_declaration(&self, declaration: &ast::TypeDeclaration) -> TypeDeclaration {
        let definition = match &declaration.definition {
            ast::TypeDefinition::Record(fields) => TypeDefinition::Record(
                fields
                    .iter()
                    .map(|field| TypeField {
                        span: field.span,
                        name: name(&field.name),
                        mutable: field.mutable,
                        field_type: type_syntax(&field.field_type),
                    })
                    .collect(),
            ),
            ast::TypeDefinition::Variant(cases) => TypeDefinition::Variant(
                cases
                    .iter()
                    .map(|case| VariantCase {
                        span: case.span,
                        name: name(&case.name),
                        payload: case.payload.as_ref().map(type_syntax),
                    })
                    .collect(),
            ),
            ast::TypeDefinition::Alias(alias) => TypeDefinition::Alias(type_syntax(alias)),
        };
        TypeDeclaration {
            span: declaration.span,
            name: name(&declaration.name),
            parameters: declaration.parameters.iter().map(name).collect(),
            definition,
        }
    }

    fn pattern(&mut self, pattern: &ast::Pattern) -> Result<Pattern, LowerError> {
        let mut position = 0;
        let lowered = self.pattern_sequence(&pattern.atoms, &mut position, pattern.span)?;
        if position != pattern.atoms.len() {
            return Err(self.error(LowerErrorKind::InvalidPattern, pattern.span));
        }
        Ok(lowered)
    }

    fn pattern_sequence(
        &mut self,
        atoms: &[ast::PatternAtom],
        position: &mut usize,
        span: Span,
    ) -> Result<Pattern, LowerError> {
        if let Some(ast::PatternAtom::Name(first)) = atoms.get(*position) {
            *position += 1;
            let (qualifier, constructor) = if atoms
                .get(*position)
                .is_some_and(|atom| matches!(atom, ast::PatternAtom::Dot))
            {
                *position += 1;
                let Some(ast::PatternAtom::Name(constructor)) = atoms.get(*position) else {
                    return Err(self.error(LowerErrorKind::InvalidPattern, span));
                };
                *position += 1;
                (Some(name(first)), constructor)
            } else {
                (None, first)
            };
            let mut arguments = Vec::new();
            while atoms
                .get(*position)
                .is_some_and(pattern_atom_starts_primary)
            {
                arguments.push(self.pattern_primary(atoms, position, span)?);
            }
            if qualifier.is_some() || !arguments.is_empty() {
                return Ok(Pattern {
                    id: self.pattern_id(),
                    span,
                    kind: PatternKind::Constructor {
                        qualifier,
                        name: name(constructor),
                        arguments,
                    },
                });
            }
            let kind = if constructor.normalized == "_" {
                PatternKind::Wildcard
            } else {
                PatternKind::Binding {
                    id: self.binding_id(),
                    name: name(constructor),
                }
            };
            return Ok(Pattern {
                id: self.pattern_id(),
                span: constructor.span,
                kind,
            });
        }
        self.pattern_primary(atoms, position, span)
    }

    fn pattern_primary(
        &mut self,
        atoms: &[ast::PatternAtom],
        position: &mut usize,
        span: Span,
    ) -> Result<Pattern, LowerError> {
        match atoms.get(*position) {
            Some(ast::PatternAtom::Name(binding)) => {
                *position += 1;
                let kind = if binding.normalized == "_" {
                    PatternKind::Wildcard
                } else {
                    PatternKind::Binding {
                        id: self.binding_id(),
                        name: name(binding),
                    }
                };
                Ok(Pattern {
                    id: self.pattern_id(),
                    span: binding.span,
                    kind,
                })
            }
            Some(ast::PatternAtom::Literal(literal)) => {
                *position += 1;
                Ok(Pattern {
                    id: self.pattern_id(),
                    span,
                    kind: PatternKind::Literal(literal_value(literal)),
                })
            }
            Some(ast::PatternAtom::LeftParen) => {
                *position += 1;
                if atoms
                    .get(*position)
                    .is_some_and(|atom| matches!(atom, ast::PatternAtom::RightParen))
                {
                    *position += 1;
                    return Ok(Pattern {
                        id: self.pattern_id(),
                        span,
                        kind: PatternKind::Unit,
                    });
                }
                let first = self.pattern_sequence(atoms, position, span)?;
                let mut elements = vec![first];
                let mut tuple = false;
                while atoms
                    .get(*position)
                    .is_some_and(|atom| matches!(atom, ast::PatternAtom::Comma))
                {
                    tuple = true;
                    *position += 1;
                    if atoms
                        .get(*position)
                        .is_some_and(|atom| matches!(atom, ast::PatternAtom::RightParen))
                    {
                        break;
                    }
                    elements.push(self.pattern_sequence(atoms, position, span)?);
                }
                if !atoms
                    .get(*position)
                    .is_some_and(|atom| matches!(atom, ast::PatternAtom::RightParen))
                {
                    return Err(self.error(LowerErrorKind::InvalidPattern, span));
                }
                *position += 1;
                if tuple {
                    Ok(Pattern {
                        id: self.pattern_id(),
                        span,
                        kind: PatternKind::Tuple(elements),
                    })
                } else {
                    Ok(elements.pop().expect("grouped pattern has one element"))
                }
            }
            Some(ast::PatternAtom::LeftBrace) => {
                *position += 1;
                let mut fields = Vec::new();
                while !atoms
                    .get(*position)
                    .is_some_and(|atom| matches!(atom, ast::PatternAtom::RightBrace))
                {
                    let Some(ast::PatternAtom::Name(field)) = atoms.get(*position) else {
                        return Err(self.error(LowerErrorKind::InvalidPattern, span));
                    };
                    *position += 1;
                    if !atoms
                        .get(*position)
                        .is_some_and(|atom| matches!(atom, ast::PatternAtom::Equals))
                    {
                        return Err(self.error(LowerErrorKind::InvalidPattern, span));
                    }
                    *position += 1;
                    let value = self.pattern_sequence(atoms, position, span)?;
                    fields.push(RecordPatternField {
                        span: field.span,
                        name: name(field),
                        pattern: value,
                    });
                    if atoms
                        .get(*position)
                        .is_some_and(|atom| matches!(atom, ast::PatternAtom::Semicolon))
                    {
                        *position += 1;
                    } else if !atoms
                        .get(*position)
                        .is_some_and(|atom| matches!(atom, ast::PatternAtom::RightBrace))
                    {
                        return Err(self.error(LowerErrorKind::InvalidPattern, span));
                    }
                }
                *position += 1;
                Ok(Pattern {
                    id: self.pattern_id(),
                    span,
                    kind: PatternKind::Record(fields),
                })
            }
            _ => Err(self.error(LowerErrorKind::InvalidPattern, span)),
        }
    }

    fn expression(&mut self, expression: &ast::Expression) -> Result<Expression, LowerError> {
        let id = self.expression_id();
        let kind = match &expression.kind {
            ast::ExpressionKind::Sequence(elements) => {
                let mut lowered = elements
                    .iter()
                    .map(|element| match element {
                        ast::SequenceElement::Let(declaration) => {
                            self.local_binding(declaration).map(SequenceElement::Let)
                        }
                        ast::SequenceElement::Expression(expression) => {
                            self.expression(expression).map(SequenceElement::Expression)
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if lowered.len() == 1 {
                    match lowered.pop().expect("length checked above") {
                        SequenceElement::Expression(expression) => expression.kind,
                        element => ExpressionKind::Sequence(vec![element]),
                    }
                } else {
                    ExpressionKind::Sequence(lowered)
                }
            }
            ast::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            } => ExpressionKind::If {
                condition: Box::new(self.expression(condition)?),
                then_branch: Box::new(self.expression(then_branch)?),
                else_branch: Box::new(self.expression(else_branch)?),
            },
            ast::ExpressionKind::Match { scrutinee, cases } => ExpressionKind::Match {
                scrutinee: Box::new(self.expression(scrutinee)?),
                cases: cases
                    .iter()
                    .map(|case| {
                        Ok(MatchCase {
                            span: case.span,
                            pattern: self.pattern(&case.pattern)?,
                            guard: case
                                .guard
                                .as_ref()
                                .map(|guard| self.expression(guard))
                                .transpose()?,
                            body: self.expression(&case.body)?,
                        })
                    })
                    .collect::<Result<Vec<_>, LowerError>>()?,
            },
            ast::ExpressionKind::Assignment { place, value } => ExpressionKind::Assignment {
                place: self.place(place)?,
                value: Box::new(self.expression(value)?),
            },
            ast::ExpressionKind::Pipeline { input, target } => {
                let lowered_target = self.expression(target)?;
                let lowered_input = self.expression(input)?;
                match lowered_target.kind {
                    ExpressionKind::Application {
                        function,
                        mut arguments,
                    } => {
                        arguments.push(lowered_input);
                        ExpressionKind::Application {
                            function,
                            arguments,
                        }
                    }
                    kind => ExpressionKind::Application {
                        function: Box::new(Expression {
                            id: lowered_target.id,
                            span: lowered_target.span,
                            kind,
                        }),
                        arguments: vec![lowered_input],
                    },
                }
            }
            ast::ExpressionKind::Application { .. } => {
                let mut arguments = Vec::new();
                let function = collect_application(expression, &mut arguments);
                ExpressionKind::Application {
                    function: Box::new(self.expression(function)?),
                    arguments: arguments
                        .into_iter()
                        .map(|argument| self.expression(argument))
                        .collect::<Result<Vec<_>, _>>()?,
                }
            }
            ast::ExpressionKind::Projection { target, field } => ExpressionKind::Projection {
                reference: self.reference_id(),
                target: Box::new(self.expression(target)?),
                field: name(field),
            },
            ast::ExpressionKind::Name(value) => ExpressionKind::Name {
                reference: self.reference_id(),
                name: name(value),
            },
            ast::ExpressionKind::Binary {
                operator,
                left,
                right,
            } => ExpressionKind::Binary {
                operator: binary_operator(*operator),
                left: Box::new(self.expression(left)?),
                right: Box::new(self.expression(right)?),
            },
            ast::ExpressionKind::Unary { operator, operand } => ExpressionKind::Unary {
                operator: unary_operator(*operator),
                operand: Box::new(self.expression(operand)?),
            },
            ast::ExpressionKind::Literal(literal) => {
                ExpressionKind::Literal(literal_value(literal))
            }
            ast::ExpressionKind::Unit => ExpressionKind::Unit,
            ast::ExpressionKind::Tuple(elements) => ExpressionKind::Tuple(
                elements
                    .iter()
                    .map(|element| self.expression(element))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            ast::ExpressionKind::Record(fields) => {
                ExpressionKind::Record(self.record_fields(fields)?)
            }
            ast::ExpressionKind::RecordUpdate { base, fields } => ExpressionKind::RecordUpdate {
                base: Box::new(self.expression(base)?),
                fields: self.record_fields(fields)?,
            },
            ast::ExpressionKind::List(elements) => ExpressionKind::List(
                elements
                    .iter()
                    .map(|element| self.expression(element))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        };
        Ok(Expression {
            id,
            span: expression.span,
            kind,
        })
    }

    fn record_fields(
        &mut self,
        fields: &[ast::RecordField],
    ) -> Result<Vec<RecordField>, LowerError> {
        fields
            .iter()
            .map(|field| {
                Ok(RecordField {
                    span: field.span,
                    name: name(&field.name),
                    value: self.expression(&field.value)?,
                })
            })
            .collect()
    }

    fn place(&mut self, expression: &ast::Expression) -> Result<Place, LowerError> {
        let mut fields = Vec::new();
        let mut current = expression;
        while let ast::ExpressionKind::Projection { target, field } = &current.kind {
            fields.push(name(field));
            current = target;
        }
        fields.reverse();
        let ast::ExpressionKind::Name(root) = &current.kind else {
            return Err(self.error(LowerErrorKind::InvalidAssignmentPlace, expression.span));
        };
        Ok(Place {
            span: expression.span,
            root_reference: self.reference_id(),
            root: name(root),
            fields,
        })
    }

    const fn error(&self, kind: LowerErrorKind, span: Span) -> LowerError {
        LowerError { kind, span }
    }

    fn expression_id(&mut self) -> ExpressionId {
        let id = ExpressionId(self.next_expression);
        self.next_expression = self.next_expression.saturating_add(1);
        id
    }

    fn reference_id(&mut self) -> ReferenceId {
        let id = ReferenceId(self.next_reference);
        self.next_reference = self.next_reference.saturating_add(1);
        id
    }

    fn binding_id(&mut self) -> BindingId {
        let id = BindingId(self.next_binding);
        self.next_binding = self.next_binding.saturating_add(1);
        id
    }

    fn pattern_id(&mut self) -> PatternId {
        let id = PatternId(self.next_pattern);
        self.next_pattern = self.next_pattern.saturating_add(1);
        id
    }
}

fn pattern_atom_starts_primary(atom: &ast::PatternAtom) -> bool {
    matches!(
        atom,
        ast::PatternAtom::Name(_)
            | ast::PatternAtom::Literal(_)
            | ast::PatternAtom::LeftParen
            | ast::PatternAtom::LeftBrace
    )
}

fn collect_application<'expression>(
    expression: &'expression ast::Expression,
    arguments: &mut Vec<&'expression ast::Expression>,
) -> &'expression ast::Expression {
    if let ast::ExpressionKind::Application { function, argument } = &expression.kind {
        let root = collect_application(function, arguments);
        arguments.push(argument);
        root
    } else {
        expression
    }
}

fn qualified_name(value: &ast::QualifiedName) -> QualifiedName {
    QualifiedName {
        span: value.span,
        segments: value.segments.iter().map(name).collect(),
    }
}

fn name(value: &ast::Name) -> Name {
    Name {
        span: value.span,
        source: value.source.clone(),
        normalized: value.normalized.clone(),
        skeleton: value.skeleton.clone(),
        scripts: value.scripts.clone(),
        suspicious_mixed_script: value.suspicious_mixed_script,
    }
}

fn type_syntax(value: &ast::TypeExpression) -> TypeSyntax {
    TypeSyntax {
        span: value.span,
        atoms: value
            .atoms
            .iter()
            .map(|atom| match atom {
                ast::TypeAtom::Name(value) => TypeAtom::Name(name(value)),
                ast::TypeAtom::Variable(value) => TypeAtom::Variable(name(value)),
                ast::TypeAtom::Arrow => TypeAtom::Arrow,
                ast::TypeAtom::Product => TypeAtom::Product,
                ast::TypeAtom::LeftParen => TypeAtom::LeftParen,
                ast::TypeAtom::RightParen => TypeAtom::RightParen,
                ast::TypeAtom::LeftAngle => TypeAtom::LeftAngle,
                ast::TypeAtom::RightAngle => TypeAtom::RightAngle,
                ast::TypeAtom::Comma => TypeAtom::Comma,
                ast::TypeAtom::Dot => TypeAtom::Dot,
            })
            .collect(),
    }
}

fn literal_value(value: &ast::Literal) -> Literal {
    match value {
        ast::Literal::Integer { radix, digits } => Literal::Integer {
            radix: *radix,
            digits: digits.clone(),
        },
        ast::Literal::Float(value) => Literal::Float(value.clone()),
        ast::Literal::Text(value) => Literal::Text(value.clone()),
        ast::Literal::Boolean(value) => Literal::Boolean(*value),
    }
}

const fn binary_operator(value: ast::BinaryOperator) -> BinaryOperator {
    match value {
        ast::BinaryOperator::Equal => BinaryOperator::Equal,
        ast::BinaryOperator::NotEqual => BinaryOperator::NotEqual,
        ast::BinaryOperator::Less => BinaryOperator::Less,
        ast::BinaryOperator::LessEqual => BinaryOperator::LessEqual,
        ast::BinaryOperator::Greater => BinaryOperator::Greater,
        ast::BinaryOperator::GreaterEqual => BinaryOperator::GreaterEqual,
        ast::BinaryOperator::Add => BinaryOperator::Add,
        ast::BinaryOperator::Subtract => BinaryOperator::Subtract,
        ast::BinaryOperator::Multiply => BinaryOperator::Multiply,
        ast::BinaryOperator::Divide => BinaryOperator::Divide,
        ast::BinaryOperator::Remainder => BinaryOperator::Remainder,
        ast::BinaryOperator::BooleanAnd => BinaryOperator::BooleanAnd,
        ast::BinaryOperator::BooleanOr => BinaryOperator::BooleanOr,
    }
}

const fn unary_operator(value: ast::UnaryOperator) -> UnaryOperator {
    match value {
        ast::UnaryOperator::Positive => UnaryOperator::Positive,
        ast::UnaryOperator::Negative => UnaryOperator::Negative,
    }
}

#[cfg(test)]
mod tests {
    use ling_ast::lower as lower_ast;
    use ling_source::{SourceFile, SourceId};
    use ling_syntax::parse;

    use super::*;

    fn lower_text(text: &str) -> Program {
        let source =
            SourceFile::from_bytes(SourceId::new(0), "test.ling", text.as_bytes().to_vec())
                .expect("valid source");
        let parsed = parse(&source);
        assert!(parsed.is_valid(), "{:?}", parsed.parse_errors());
        let ast = lower_ast(&source, &parsed).expect("valid AST");
        lower(source.name(), &ast).expect("valid HIR")
    }

    #[test]
    fn pipeline_becomes_a_final_argument_application() {
        let program = lower_text("let result input = input |> f a\n");
        let ExpressionKind::Application {
            function,
            arguments,
        } = &program.definitions[0].value.kind
        else {
            panic!("expected application");
        };
        assert!(matches!(function.kind, ExpressionKind::Name { .. }));
        assert_eq!(arguments.len(), 2);
        let ExpressionKind::Name { name, .. } = &arguments[1].kind else {
            panic!("expected pipeline input as final argument");
        };
        assert_eq!(name.normalized, "input");
    }

    #[test]
    fn classifies_projection_assignment_places() {
        let program =
            lower_text("let update () =\n    let mutable person = value\n    person.health <- 1\n");
        let ExpressionKind::Sequence(elements) = &program.definitions[0].value.kind else {
            panic!("expected sequence");
        };
        let SequenceElement::Expression(assignment) = &elements[1] else {
            panic!("expected assignment");
        };
        let ExpressionKind::Assignment { place, .. } = &assignment.kind else {
            panic!("expected assignment");
        };
        assert_eq!(place.root.normalized, "person");
        assert_eq!(place.fields[0].normalized, "health");
    }
}
