//! Canonical Audit Source rendering and compiler-CST-backed format IR.

mod author;
mod comments;
mod format_ir;

pub use author::{FormatDisposition, FormatResult, format_core, format_core_with_disposition};
pub use comments::{CommentAttachment, CommentKind, CommentPlacement};
pub use format_ir::{
    FORMAT_IR_SCHEMA, FormatDocument, FormatIrBuildError, FormatIrBuildErrorKind, FormatNode,
    FormatToken, build_format_ir,
};

use std::error::Error;
use std::fmt;

use ling_diagnostics::{Diagnostic, DiagnosticSpan, Severity, codes};
use ling_semantic::{
    AuditDefinition, AuditModel, AuditModule, AuditNode, AuditReference, SemanticImport,
    validate_audit_model,
};

pub const AUDIT_SCHEMA: &str = "ling.audit/0.1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditFormatError {
    pub kind: AuditFormatErrorKind,
    pub byte_offset: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuditFormatErrorKind {
    UnsupportedVersion { actual: String },
    UnexpectedToken { expected: String, actual: String },
    UnknownField { field: String },
    DuplicateField { field: String },
    MissingField { field: String },
    InvalidString { message: String },
    InvalidNumber,
    InvalidModel { message: String },
}

impl fmt::Display for AuditFormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            AuditFormatErrorKind::UnsupportedVersion { actual } => write!(
                formatter,
                "unsupported Audit schema `{actual}` at byte {}",
                self.byte_offset
            ),
            AuditFormatErrorKind::UnexpectedToken { expected, actual } => write!(
                formatter,
                "expected {expected}, found {actual} at byte {}",
                self.byte_offset
            ),
            AuditFormatErrorKind::UnknownField { field } => {
                write!(
                    formatter,
                    "unknown Audit field `{field}` at byte {}",
                    self.byte_offset
                )
            }
            AuditFormatErrorKind::DuplicateField { field } => write!(
                formatter,
                "duplicate Audit field `{field}` at byte {}",
                self.byte_offset
            ),
            AuditFormatErrorKind::MissingField { field } => {
                write!(formatter, "missing Audit field `{field}`")
            }
            AuditFormatErrorKind::InvalidString { message } => write!(
                formatter,
                "invalid Audit string at byte {}: {message}",
                self.byte_offset
            ),
            AuditFormatErrorKind::InvalidNumber => {
                write!(
                    formatter,
                    "invalid Audit number at byte {}",
                    self.byte_offset
                )
            }
            AuditFormatErrorKind::InvalidModel { message } => {
                write!(formatter, "invalid Audit model: {message}")
            }
        }
    }
}

impl Error for AuditFormatError {}

impl AuditFormatError {
    #[must_use]
    pub fn to_diagnostic(&self, source_name: &str) -> Diagnostic {
        let code = match &self.kind {
            AuditFormatErrorKind::UnexpectedToken { .. }
            | AuditFormatErrorKind::InvalidString { .. }
            | AuditFormatErrorKind::InvalidNumber => codes::AUDIT_SYNTAX,
            AuditFormatErrorKind::UnsupportedVersion { .. }
            | AuditFormatErrorKind::UnknownField { .. } => codes::AUDIT_VERSION,
            AuditFormatErrorKind::DuplicateField { .. }
            | AuditFormatErrorKind::MissingField { .. } => codes::AUDIT_STRUCTURE,
            AuditFormatErrorKind::InvalidModel { .. } => codes::AUDIT_MODEL,
        };
        let start = u32::try_from(self.byte_offset).unwrap_or(u32::MAX);
        Diagnostic::new(
            code,
            Severity::Error,
            format!("Audit Source 无效：{self}"),
            format!("invalid Audit Source: {self}"),
        )
        .with_primary_span(DiagnosticSpan::at(
            source_name,
            start,
            start.saturating_add(1),
        ))
        .with_fact("audit_schema", AUDIT_SCHEMA)
    }
}

/// Renders one canonical, BOM-free UTF-8 Audit Source document ending in one LF.
pub fn render_audit(model: &AuditModel) -> Result<String, AuditFormatError> {
    let mut model = model.clone();
    canonicalize(&mut model);
    validate_audit_model(&model).map_err(|error| AuditFormatError {
        kind: AuditFormatErrorKind::InvalidModel {
            message: error.to_string(),
        },
        byte_offset: 0,
    })?;

    let mut output = String::new();
    push_line(&mut output, 0, &format!("audit {AUDIT_SCHEMA} {{"));
    push_assignment(&mut output, 1, "language", &model.language_version)?;
    push_assignment(&mut output, 1, "semantic", &model.semantic_schema)?;
    push_assignment(&mut output, 1, "unicode", &model.unicode_version)?;
    push_assignment(&mut output, 1, "program", &model.program_id)?;
    push_assignment(&mut output, 1, "entry", &model.entry_module)?;

    for module in &model.modules {
        output.push('\n');
        push_line(
            &mut output,
            1,
            &format!("module {} {{", quote(&module.name)?),
        );
        push_line(&mut output, 2, &format!("explicit = {}", module.explicit));
        push_string_list(&mut output, 2, "capabilities", &module.capabilities)?;
        for import in &module.imports {
            push_line(
                &mut output,
                2,
                &format!(
                    "import {} = {}",
                    quote(&import.alias)?,
                    quote(&import.module)?
                ),
            );
        }
        for definition in &module.definitions {
            render_definition(&mut output, definition)?;
        }
        for node in &module.nodes {
            render_node(&mut output, node)?;
        }
        for reference in &module.references {
            render_reference(&mut output, reference)?;
        }
        push_line(&mut output, 1, "}");
    }
    push_line(&mut output, 0, "}");
    Ok(output)
}

fn render_node(output: &mut String, node: &AuditNode) -> Result<(), AuditFormatError> {
    push_line(output, 2, &format!("node {} {{", quote(&node.node_id)?));
    push_assignment(output, 3, "kind", &node.kind)?;
    if let Some(name) = &node.name {
        push_assignment(output, 3, "name", name)?;
    }
    push_assignment(output, 3, "owner", &node.owner)?;
    if let Some(type_name) = &node.type_name {
        push_assignment(output, 3, "type", type_name)?;
    }
    if let Some(mutable) = node.mutable {
        push_line(output, 3, &format!("mutable = {mutable}"));
    }
    if let Some(ordinal) = node.ordinal {
        push_line(output, 3, &format!("ordinal = {ordinal}"));
    }
    push_string_list(output, 3, "effects", &node.effects)?;
    push_string_list(output, 3, "capabilities", &node.capabilities)?;
    if let Some(source) = &node.identifier_source {
        push_assignment(output, 3, "unicode_source", source)?;
        push_assignment(
            output,
            3,
            "unicode_nfc",
            node.name.as_deref().unwrap_or(source),
        )?;
    }
    if let Some(skeleton) = &node.identifier_skeleton {
        push_assignment(output, 3, "unicode_skeleton", skeleton)?;
    }
    push_string_list(output, 3, "unicode_scripts", &node.identifier_scripts)?;
    push_line(
        output,
        3,
        &format!(
            "unicode_suspicious_mixed_script = {}",
            node.identifier_suspicious_mixed_script
        ),
    );
    push_assignment(output, 3, "implementation", &node.implementation)?;
    push_line(output, 2, "}");
    Ok(())
}

/// Parses Audit Source into an isolated model. No executable conversion exists.
pub fn parse_audit(input: &str) -> Result<AuditModel, AuditFormatError> {
    Parser::new(input)?.parse()
}

fn render_definition(
    output: &mut String,
    definition: &AuditDefinition,
) -> Result<(), AuditFormatError> {
    push_line(
        output,
        2,
        &format!("definition {} {{", quote(&definition.definition_id)?),
    );
    push_assignment(output, 3, "body", &definition.body_id)?;
    push_assignment(output, 3, "name", &definition.name)?;
    push_assignment(output, 3, "kind", &definition.kind)?;
    push_assignment(output, 3, "origin", &definition.origin)?;
    push_assignment(output, 3, "type", &definition.type_name)?;
    push_string_list(output, 3, "effects", &definition.effects)?;
    push_string_list(output, 3, "capabilities", &definition.capabilities)?;
    push_assignment(output, 3, "unicode_source", &definition.unicode_source)?;
    push_assignment(output, 3, "unicode_nfc", &definition.unicode_nfc)?;
    push_assignment(output, 3, "unicode_skeleton", &definition.unicode_skeleton)?;
    push_string_list(output, 3, "unicode_scripts", &definition.unicode_scripts)?;
    push_line(
        output,
        3,
        &format!(
            "unicode_suspicious_mixed_script = {}",
            definition.unicode_suspicious_mixed_script
        ),
    );
    push_assignment(output, 3, "implementation", &definition.implementation)?;
    push_line(output, 2, "}");
    Ok(())
}

fn render_reference(
    output: &mut String,
    reference: &AuditReference,
) -> Result<(), AuditFormatError> {
    push_line(
        output,
        2,
        &format!(
            "reference {} {} {{",
            reference.source_kind, reference.reference
        ),
    );
    if let Some(source_id) = &reference.source_id {
        push_assignment(output, 3, "source", source_id)?;
    }
    push_assignment(output, 3, "target_kind", &reference.target_kind)?;
    push_assignment(output, 3, "target", &reference.target)?;
    push_line(output, 2, "}");
    Ok(())
}

fn push_assignment(
    output: &mut String,
    indent: usize,
    name: &str,
    value: &str,
) -> Result<(), AuditFormatError> {
    push_line(output, indent, &format!("{name} = {}", quote(value)?));
    Ok(())
}

fn push_string_list(
    output: &mut String,
    indent: usize,
    name: &str,
    values: &[String],
) -> Result<(), AuditFormatError> {
    let values = values
        .iter()
        .map(|value| quote(value))
        .collect::<Result<Vec<_>, _>>()?
        .join(", ");
    push_line(output, indent, &format!("{name} = [{values}]"));
    Ok(())
}

fn push_line(output: &mut String, indent: usize, line: &str) {
    for _ in 0..indent {
        output.push_str("  ");
    }
    output.push_str(line);
    output.push('\n');
}

fn quote(value: &str) -> Result<String, AuditFormatError> {
    serde_json::to_string(value).map_err(|error| AuditFormatError {
        kind: AuditFormatErrorKind::InvalidString {
            message: error.to_string(),
        },
        byte_offset: 0,
    })
}

fn canonicalize(model: &mut AuditModel) {
    model
        .modules
        .sort_by(|left, right| left.name.cmp(&right.name));
    for module in &mut model.modules {
        module.capabilities.sort();
        module.capabilities.dedup();
        module
            .imports
            .sort_by(|left, right| (&left.alias, &left.module).cmp(&(&right.alias, &right.module)));
        module
            .definitions
            .sort_by(|left, right| left.definition_id.cmp(&right.definition_id));
        for definition in &mut module.definitions {
            definition.effects.sort();
            definition.effects.dedup();
            definition.capabilities.sort();
            definition.capabilities.dedup();
            definition.unicode_scripts.sort();
            definition.unicode_scripts.dedup();
        }
        module
            .nodes
            .sort_by(|left, right| left.node_id.cmp(&right.node_id));
        for node in &mut module.nodes {
            node.effects.sort();
            node.effects.dedup();
            node.capabilities.sort();
            node.capabilities.dedup();
            node.identifier_scripts.sort();
            node.identifier_scripts.dedup();
        }
        module.references.sort_by(|left, right| {
            (
                &left.source_kind,
                left.reference,
                &left.source_id,
                &left.target_kind,
                &left.target,
            )
                .cmp(&(
                    &right.source_kind,
                    right.reference,
                    &right.source_id,
                    &right.target_kind,
                    &right.target,
                ))
        });
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum TokenKind {
    Word(String),
    String(String),
    Number(u32),
    Equals,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Eof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Token {
    kind: TokenKind,
    offset: usize,
}

struct Lexer<'input> {
    input: &'input str,
    position: usize,
}

impl<'input> Lexer<'input> {
    const fn new(input: &'input str) -> Self {
        Self { input, position: 0 }
    }

    fn next(&mut self) -> Result<Token, AuditFormatError> {
        while self
            .input
            .as_bytes()
            .get(self.position)
            .is_some_and(u8::is_ascii_whitespace)
        {
            self.position += 1;
        }
        let offset = self.position;
        let Some(byte) = self.input.as_bytes().get(self.position).copied() else {
            return Ok(Token {
                kind: TokenKind::Eof,
                offset,
            });
        };
        let punctuation = match byte {
            b'=' => Some(TokenKind::Equals),
            b'{' => Some(TokenKind::LeftBrace),
            b'}' => Some(TokenKind::RightBrace),
            b'[' => Some(TokenKind::LeftBracket),
            b']' => Some(TokenKind::RightBracket),
            b',' => Some(TokenKind::Comma),
            _ => None,
        };
        if let Some(kind) = punctuation {
            self.position += 1;
            return Ok(Token { kind, offset });
        }
        if byte == b'"' {
            return self.string();
        }
        if byte.is_ascii_digit() {
            return self.number();
        }
        if word_byte(byte) {
            let start = self.position;
            while self
                .input
                .as_bytes()
                .get(self.position)
                .is_some_and(|byte| word_byte(*byte))
            {
                self.position += 1;
            }
            return Ok(Token {
                kind: TokenKind::Word(self.input[start..self.position].to_owned()),
                offset,
            });
        }
        Err(unexpected(offset, "Audit token", &display_byte(byte)))
    }

    fn string(&mut self) -> Result<Token, AuditFormatError> {
        let offset = self.position;
        self.position += 1;
        let mut escaped = false;
        while let Some(byte) = self.input.as_bytes().get(self.position).copied() {
            self.position += 1;
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                let raw = &self.input[offset..self.position];
                let value = serde_json::from_str(raw).map_err(|error| AuditFormatError {
                    kind: AuditFormatErrorKind::InvalidString {
                        message: error.to_string(),
                    },
                    byte_offset: offset,
                })?;
                return Ok(Token {
                    kind: TokenKind::String(value),
                    offset,
                });
            }
        }
        Err(AuditFormatError {
            kind: AuditFormatErrorKind::InvalidString {
                message: "unterminated string".to_owned(),
            },
            byte_offset: offset,
        })
    }

    fn number(&mut self) -> Result<Token, AuditFormatError> {
        let offset = self.position;
        while self
            .input
            .as_bytes()
            .get(self.position)
            .is_some_and(u8::is_ascii_digit)
        {
            self.position += 1;
        }
        let value = self.input[offset..self.position]
            .parse()
            .map_err(|_| AuditFormatError {
                kind: AuditFormatErrorKind::InvalidNumber,
                byte_offset: offset,
            })?;
        Ok(Token {
            kind: TokenKind::Number(value),
            offset,
        })
    }
}

const fn word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'/' | b'<' | b'>')
}

fn display_byte(byte: u8) -> String {
    if byte.is_ascii_graphic() {
        format!("`{}`", char::from(byte))
    } else {
        format!("byte 0x{byte:02x}")
    }
}

struct Parser<'input> {
    lexer: Lexer<'input>,
    current: Token,
}

impl<'input> Parser<'input> {
    fn new(input: &'input str) -> Result<Self, AuditFormatError> {
        if input.starts_with('\u{feff}') {
            return Err(unexpected(0, "BOM-free UTF-8", "UTF-8 BOM"));
        }
        let mut lexer = Lexer::new(input);
        let current = lexer.next()?;
        Ok(Self { lexer, current })
    }

    fn parse(mut self) -> Result<AuditModel, AuditFormatError> {
        self.expect_word("audit")?;
        let (schema, offset) = self.take_word()?;
        if schema != AUDIT_SCHEMA {
            return Err(AuditFormatError {
                kind: AuditFormatErrorKind::UnsupportedVersion { actual: schema },
                byte_offset: offset,
            });
        }
        self.expect(TokenKind::LeftBrace, "`{`")?;
        let mut language_version = None;
        let mut semantic_schema = None;
        let mut unicode_version = None;
        let mut program_id = None;
        let mut entry_module = None;
        let mut modules = Vec::new();
        while self.current.kind != TokenKind::RightBrace {
            let (name, offset) = self.take_word()?;
            match name.as_str() {
                "language" => {
                    let value = self.assignment_string()?;
                    set_once(&mut language_version, value, "language", offset)?;
                }
                "semantic" => {
                    let value = self.assignment_string()?;
                    set_once(&mut semantic_schema, value, "semantic", offset)?;
                }
                "unicode" => {
                    let value = self.assignment_string()?;
                    set_once(&mut unicode_version, value, "unicode", offset)?;
                }
                "program" => {
                    let value = self.assignment_string()?;
                    set_once(&mut program_id, value, "program", offset)?;
                }
                "entry" => {
                    let value = self.assignment_string()?;
                    set_once(&mut entry_module, value, "entry", offset)?;
                }
                "module" => modules.push(self.module()?),
                extension if extension.starts_with("x-") => self.extension()?,
                field => return Err(unknown(field, offset)),
            }
        }
        self.advance()?;
        self.expect(TokenKind::Eof, "end of input")?;
        let mut model = AuditModel {
            language_version: required(language_version, "language")?,
            semantic_schema: required(semantic_schema, "semantic")?,
            unicode_version: required(unicode_version, "unicode")?,
            program_id: required(program_id, "program")?,
            entry_module: required(entry_module, "entry")?,
            modules,
        };
        canonicalize(&mut model);
        validate_audit_model(&model).map_err(|error| AuditFormatError {
            kind: AuditFormatErrorKind::InvalidModel {
                message: error.to_string(),
            },
            byte_offset: 0,
        })?;
        Ok(model)
    }

    fn module(&mut self) -> Result<AuditModule, AuditFormatError> {
        let name = self.take_string()?;
        self.expect(TokenKind::LeftBrace, "`{`")?;
        let mut explicit = None;
        let mut capabilities = None;
        let mut imports = Vec::new();
        let mut definitions = Vec::new();
        let mut nodes = Vec::new();
        let mut references = Vec::new();
        while self.current.kind != TokenKind::RightBrace {
            let (field, offset) = self.take_word()?;
            match field.as_str() {
                "explicit" => {
                    let value = self.assignment_bool()?;
                    set_once(&mut explicit, value, "explicit", offset)?;
                }
                "capabilities" => {
                    let value = self.assignment_string_list()?;
                    set_once(&mut capabilities, value, "capabilities", offset)?;
                }
                "import" => {
                    let alias = self.take_string()?;
                    self.expect(TokenKind::Equals, "`=`")?;
                    imports.push(SemanticImport {
                        alias,
                        module: self.take_string()?,
                        package: None,
                    });
                }
                "definition" => definitions.push(self.definition()?),
                "node" => nodes.push(self.node()?),
                "reference" => references.push(self.reference()?),
                extension if extension.starts_with("x-") => self.extension()?,
                field => return Err(unknown(field, offset)),
            }
        }
        self.advance()?;
        Ok(AuditModule {
            name,
            explicit: required(explicit, "explicit")?,
            capabilities: required(capabilities, "capabilities")?,
            imports,
            definitions,
            nodes,
            references,
        })
    }

    fn definition(&mut self) -> Result<AuditDefinition, AuditFormatError> {
        let definition_id = self.take_string()?;
        self.expect(TokenKind::LeftBrace, "`{`")?;
        let mut body_id = None;
        let mut name = None;
        let mut kind = None;
        let mut origin = None;
        let mut type_name = None;
        let mut effects = None;
        let mut capabilities = None;
        let mut unicode_source = None;
        let mut unicode_nfc = None;
        let mut unicode_skeleton = None;
        let mut unicode_scripts = None;
        let mut unicode_suspicious_mixed_script = None;
        let mut implementation = None;
        while self.current.kind != TokenKind::RightBrace {
            let (field, offset) = self.take_word()?;
            match field.as_str() {
                "body" => set_once(&mut body_id, self.assignment_string()?, "body", offset)?,
                "name" => set_once(&mut name, self.assignment_string()?, "name", offset)?,
                "kind" => set_once(&mut kind, self.assignment_string()?, "kind", offset)?,
                "origin" => set_once(&mut origin, self.assignment_string()?, "origin", offset)?,
                "type" => set_once(&mut type_name, self.assignment_string()?, "type", offset)?,
                "effects" => set_once(
                    &mut effects,
                    self.assignment_string_list()?,
                    "effects",
                    offset,
                )?,
                "capabilities" => set_once(
                    &mut capabilities,
                    self.assignment_string_list()?,
                    "capabilities",
                    offset,
                )?,
                "unicode_source" => set_once(
                    &mut unicode_source,
                    self.assignment_string()?,
                    "unicode_source",
                    offset,
                )?,
                "unicode_nfc" => set_once(
                    &mut unicode_nfc,
                    self.assignment_string()?,
                    "unicode_nfc",
                    offset,
                )?,
                "unicode_skeleton" => set_once(
                    &mut unicode_skeleton,
                    self.assignment_string()?,
                    "unicode_skeleton",
                    offset,
                )?,
                "unicode_scripts" => set_once(
                    &mut unicode_scripts,
                    self.assignment_string_list()?,
                    "unicode_scripts",
                    offset,
                )?,
                "unicode_suspicious_mixed_script" => set_once(
                    &mut unicode_suspicious_mixed_script,
                    self.assignment_bool()?,
                    "unicode_suspicious_mixed_script",
                    offset,
                )?,
                "implementation" => set_once(
                    &mut implementation,
                    self.assignment_string()?,
                    "implementation",
                    offset,
                )?,
                extension if extension.starts_with("x-") => self.extension()?,
                field => return Err(unknown(field, offset)),
            }
        }
        self.advance()?;
        Ok(AuditDefinition {
            definition_id,
            body_id: required(body_id, "body")?,
            name: required(name, "name")?,
            kind: required(kind, "kind")?,
            origin: required(origin, "origin")?,
            type_name: required(type_name, "type")?,
            effects: required(effects, "effects")?,
            capabilities: required(capabilities, "capabilities")?,
            unicode_source: required(unicode_source, "unicode_source")?,
            unicode_nfc: required(unicode_nfc, "unicode_nfc")?,
            unicode_skeleton: required(unicode_skeleton, "unicode_skeleton")?,
            unicode_scripts: required(unicode_scripts, "unicode_scripts")?,
            unicode_suspicious_mixed_script: required(
                unicode_suspicious_mixed_script,
                "unicode_suspicious_mixed_script",
            )?,
            implementation: required(implementation, "implementation")?,
        })
    }

    fn node(&mut self) -> Result<AuditNode, AuditFormatError> {
        let node_id = self.take_string()?;
        self.expect(TokenKind::LeftBrace, "`{`")?;
        let mut kind = None;
        let mut name = None;
        let mut owner = None;
        let mut type_name = None;
        let mut mutable = None;
        let mut ordinal = None;
        let mut effects = None;
        let mut capabilities = None;
        let mut unicode_source = None;
        let mut unicode_nfc = None;
        let mut unicode_skeleton = None;
        let mut unicode_scripts = None;
        let mut unicode_suspicious_mixed_script = None;
        let mut implementation = None;
        while self.current.kind != TokenKind::RightBrace {
            let (field, offset) = self.take_word()?;
            match field.as_str() {
                "kind" => set_once(&mut kind, self.assignment_string()?, "kind", offset)?,
                "name" => set_once(&mut name, self.assignment_string()?, "name", offset)?,
                "owner" => set_once(&mut owner, self.assignment_string()?, "owner", offset)?,
                "type" => set_once(&mut type_name, self.assignment_string()?, "type", offset)?,
                "mutable" => set_once(&mut mutable, self.assignment_bool()?, "mutable", offset)?,
                "ordinal" => set_once(&mut ordinal, self.assignment_number()?, "ordinal", offset)?,
                "effects" => set_once(
                    &mut effects,
                    self.assignment_string_list()?,
                    "effects",
                    offset,
                )?,
                "capabilities" => set_once(
                    &mut capabilities,
                    self.assignment_string_list()?,
                    "capabilities",
                    offset,
                )?,
                "unicode_source" => set_once(
                    &mut unicode_source,
                    self.assignment_string()?,
                    "unicode_source",
                    offset,
                )?,
                "unicode_nfc" => set_once(
                    &mut unicode_nfc,
                    self.assignment_string()?,
                    "unicode_nfc",
                    offset,
                )?,
                "unicode_skeleton" => set_once(
                    &mut unicode_skeleton,
                    self.assignment_string()?,
                    "unicode_skeleton",
                    offset,
                )?,
                "unicode_scripts" => set_once(
                    &mut unicode_scripts,
                    self.assignment_string_list()?,
                    "unicode_scripts",
                    offset,
                )?,
                "unicode_suspicious_mixed_script" => set_once(
                    &mut unicode_suspicious_mixed_script,
                    self.assignment_bool()?,
                    "unicode_suspicious_mixed_script",
                    offset,
                )?,
                "implementation" => set_once(
                    &mut implementation,
                    self.assignment_string()?,
                    "implementation",
                    offset,
                )?,
                extension if extension.starts_with("x-") => self.extension()?,
                field => return Err(unknown(field, offset)),
            }
        }
        self.advance()?;
        if unicode_nfc
            .as_ref()
            .is_some_and(|nfc| name.as_ref() != Some(nfc))
        {
            return Err(AuditFormatError {
                kind: AuditFormatErrorKind::InvalidModel {
                    message: "node unicode_nfc must equal its canonical name".to_owned(),
                },
                byte_offset: 0,
            });
        }
        Ok(AuditNode {
            node_id,
            kind: required(kind, "kind")?,
            name,
            owner: required(owner, "owner")?,
            type_name,
            mutable,
            ordinal,
            effects: required(effects, "effects")?,
            capabilities: required(capabilities, "capabilities")?,
            identifier_source: unicode_source,
            identifier_skeleton: unicode_skeleton,
            identifier_scripts: required(unicode_scripts, "unicode_scripts")?,
            identifier_suspicious_mixed_script: required(
                unicode_suspicious_mixed_script,
                "unicode_suspicious_mixed_script",
            )?,
            implementation: required(implementation, "implementation")?,
        })
    }

    fn reference(&mut self) -> Result<AuditReference, AuditFormatError> {
        let (source_kind, _) = self.take_word()?;
        let reference = self.take_number()?;
        self.expect(TokenKind::LeftBrace, "`{`")?;
        let mut source_id = None;
        let mut target_kind = None;
        let mut target = None;
        while self.current.kind != TokenKind::RightBrace {
            let (field, offset) = self.take_word()?;
            match field.as_str() {
                "source" => set_once(&mut source_id, self.assignment_string()?, "source", offset)?,
                "target_kind" => set_once(
                    &mut target_kind,
                    self.assignment_string()?,
                    "target_kind",
                    offset,
                )?,
                "target" => set_once(&mut target, self.assignment_string()?, "target", offset)?,
                extension if extension.starts_with("x-") => self.extension()?,
                field => return Err(unknown(field, offset)),
            }
        }
        self.advance()?;
        Ok(AuditReference {
            source_kind,
            reference,
            source_id,
            target_kind: required(target_kind, "target_kind")?,
            target: required(target, "target")?,
        })
    }

    fn assignment_string(&mut self) -> Result<String, AuditFormatError> {
        self.expect(TokenKind::Equals, "`=`")?;
        self.take_string()
    }

    fn assignment_bool(&mut self) -> Result<bool, AuditFormatError> {
        self.expect(TokenKind::Equals, "`=`")?;
        let (value, offset) = self.take_word()?;
        match value.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(unexpected(offset, "`true` or `false`", &value)),
        }
    }

    fn assignment_number(&mut self) -> Result<u32, AuditFormatError> {
        self.expect(TokenKind::Equals, "`=`")?;
        self.take_number()
    }

    fn assignment_string_list(&mut self) -> Result<Vec<String>, AuditFormatError> {
        self.expect(TokenKind::Equals, "`=`")?;
        self.expect(TokenKind::LeftBracket, "`[`")?;
        let mut values = Vec::new();
        if self.current.kind != TokenKind::RightBracket {
            loop {
                values.push(self.take_string()?);
                if self.current.kind == TokenKind::Comma {
                    self.advance()?;
                } else {
                    break;
                }
            }
        }
        self.expect(TokenKind::RightBracket, "`]`")?;
        Ok(values)
    }

    fn extension(&mut self) -> Result<(), AuditFormatError> {
        self.expect(TokenKind::Equals, "`=`")?;
        self.skip_value()
    }

    fn skip_value(&mut self) -> Result<(), AuditFormatError> {
        match self.current.kind.clone() {
            TokenKind::String(_) | TokenKind::Number(_) | TokenKind::Word(_) => self.advance(),
            TokenKind::LeftBracket => {
                self.advance()?;
                while self.current.kind != TokenKind::RightBracket {
                    self.skip_value()?;
                    if self.current.kind == TokenKind::Comma {
                        self.advance()?;
                    }
                }
                self.advance()
            }
            actual => Err(unexpected(
                self.current.offset,
                "extension value",
                &token_name(&actual),
            )),
        }
    }

    fn take_word(&mut self) -> Result<(String, usize), AuditFormatError> {
        let offset = self.current.offset;
        let TokenKind::Word(value) = self.current.kind.clone() else {
            return Err(unexpected(
                offset,
                "identifier",
                &token_name(&self.current.kind),
            ));
        };
        self.advance()?;
        Ok((value, offset))
    }

    fn take_string(&mut self) -> Result<String, AuditFormatError> {
        let offset = self.current.offset;
        let TokenKind::String(value) = self.current.kind.clone() else {
            return Err(unexpected(
                offset,
                "string",
                &token_name(&self.current.kind),
            ));
        };
        self.advance()?;
        Ok(value)
    }

    fn take_number(&mut self) -> Result<u32, AuditFormatError> {
        let offset = self.current.offset;
        let TokenKind::Number(value) = self.current.kind else {
            return Err(unexpected(
                offset,
                "number",
                &token_name(&self.current.kind),
            ));
        };
        self.advance()?;
        Ok(value)
    }

    fn expect_word(&mut self, expected: &str) -> Result<(), AuditFormatError> {
        let (actual, offset) = self.take_word()?;
        if actual == expected {
            Ok(())
        } else {
            Err(unexpected(offset, &format!("`{expected}`"), &actual))
        }
    }

    fn expect(&mut self, expected: TokenKind, name: &str) -> Result<(), AuditFormatError> {
        if self.current.kind == expected {
            self.advance()
        } else {
            Err(unexpected(
                self.current.offset,
                name,
                &token_name(&self.current.kind),
            ))
        }
    }

    fn advance(&mut self) -> Result<(), AuditFormatError> {
        self.current = self.lexer.next()?;
        Ok(())
    }
}

fn set_once<T>(
    slot: &mut Option<T>,
    value: T,
    field: &str,
    offset: usize,
) -> Result<(), AuditFormatError> {
    if slot.replace(value).is_some() {
        Err(AuditFormatError {
            kind: AuditFormatErrorKind::DuplicateField {
                field: field.to_owned(),
            },
            byte_offset: offset,
        })
    } else {
        Ok(())
    }
}

fn required<T>(value: Option<T>, field: &str) -> Result<T, AuditFormatError> {
    value.ok_or_else(|| AuditFormatError {
        kind: AuditFormatErrorKind::MissingField {
            field: field.to_owned(),
        },
        byte_offset: 0,
    })
}

fn unknown(field: &str, offset: usize) -> AuditFormatError {
    AuditFormatError {
        kind: AuditFormatErrorKind::UnknownField {
            field: field.to_owned(),
        },
        byte_offset: offset,
    }
}

fn unexpected(offset: usize, expected: &str, actual: &str) -> AuditFormatError {
    AuditFormatError {
        kind: AuditFormatErrorKind::UnexpectedToken {
            expected: expected.to_owned(),
            actual: actual.to_owned(),
        },
        byte_offset: offset,
    }
}

fn token_name(token: &TokenKind) -> String {
    match token {
        TokenKind::Word(value) => format!("`{value}`"),
        TokenKind::String(_) => "string".to_owned(),
        TokenKind::Number(value) => value.to_string(),
        TokenKind::Equals => "`=`".to_owned(),
        TokenKind::LeftBrace => "`{`".to_owned(),
        TokenKind::RightBrace => "`}`".to_owned(),
        TokenKind::LeftBracket => "`[`".to_owned(),
        TokenKind::RightBracket => "`]`".to_owned(),
        TokenKind::Comma => "`,`".to_owned(),
        TokenKind::Eof => "end of input".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(character: char) -> String {
        format!("experimental:blake3:{}", character.to_string().repeat(64))
    }

    fn model() -> AuditModel {
        AuditModel {
            language_version: "0.0.1-dev".to_owned(),
            semantic_schema: "ling.semantic/0.1".to_owned(),
            unicode_version: "17.0.0".to_owned(),
            program_id: id('a'),
            entry_module: "Main".to_owned(),
            modules: vec![AuditModule {
                name: "Main".to_owned(),
                explicit: true,
                capabilities: vec!["Console.Write".to_owned()],
                imports: Vec::new(),
                definitions: vec![AuditDefinition {
                    definition_id: id('b'),
                    body_id: id('c'),
                    name: "main".to_owned(),
                    kind: "value".to_owned(),
                    origin: "user".to_owned(),
                    type_name: "Unit -> Unit".to_owned(),
                    effects: vec!["Console.Write".to_owned()],
                    capabilities: vec!["Console.Write".to_owned()],
                    unicode_source: "main".to_owned(),
                    unicode_nfc: "main".to_owned(),
                    unicode_skeleton: "rnain".to_owned(),
                    unicode_scripts: vec!["Latn".to_owned()],
                    unicode_suspicious_mixed_script: false,
                    implementation: "implemented".to_owned(),
                }],
                nodes: vec![
                    AuditNode {
                        node_id: id('d'),
                        kind: "function".to_owned(),
                        name: Some("main".to_owned()),
                        owner: id('b'),
                        type_name: Some("Unit -> Unit".to_owned()),
                        mutable: None,
                        ordinal: None,
                        effects: vec!["Console.Write".to_owned()],
                        capabilities: vec!["Console.Write".to_owned()],
                        identifier_source: Some("main".to_owned()),
                        identifier_skeleton: Some("rnain".to_owned()),
                        identifier_scripts: vec!["Latn".to_owned()],
                        identifier_suspicious_mixed_script: false,
                        implementation: "implemented".to_owned(),
                    },
                    AuditNode {
                        node_id: id('e'),
                        kind: "expression".to_owned(),
                        name: Some("call".to_owned()),
                        owner: id('b'),
                        type_name: Some("Unit".to_owned()),
                        mutable: None,
                        ordinal: Some(0),
                        effects: vec!["Console.Write".to_owned()],
                        capabilities: vec!["Console.Write".to_owned()],
                        identifier_source: None,
                        identifier_skeleton: None,
                        identifier_scripts: Vec::new(),
                        identifier_suspicious_mixed_script: false,
                        implementation: "implemented".to_owned(),
                    },
                ],
                references: vec![AuditReference {
                    source_kind: "expression".to_owned(),
                    reference: 0,
                    source_id: Some(id('e')),
                    target_kind: "definition".to_owned(),
                    target: id('b'),
                }],
            }],
        }
    }

    #[test]
    fn canonical_audit_round_trips() {
        let model = model();
        let rendered = render_audit(&model).expect("model renders");

        assert_eq!(parse_audit(&rendered).expect("Audit parses"), model);
        assert!(!rendered.starts_with('\u{feff}'));
        assert!(rendered.ends_with('\n'));
        assert!(!rendered.ends_with("\n\n"));
    }

    #[test]
    fn parser_accepts_field_reordering_and_extensions() {
        let rendered = render_audit(&model()).expect("model renders");
        let extended = rendered.replacen(
            "  language = \"0.0.1-dev\"\n",
            "  x-vendor = [\"ignored\", 1]\n  language = \"0.0.1-dev\"\n",
            1,
        );
        let parsed = parse_audit(&extended).expect("extension parses");

        assert_eq!(
            render_audit(&parsed).expect("parsed model renders"),
            rendered
        );
    }

    #[test]
    fn parser_rejects_bad_headers_duplicates_unknown_fields_and_dangling_ids() {
        let rendered = render_audit(&model()).expect("model renders");
        let header = parse_audit(&rendered.replacen(AUDIT_SCHEMA, "ling.audit/9.9", 1))
            .expect_err("bad header fails");
        assert_eq!(
            header.to_diagnostic("model.audit").code(),
            codes::AUDIT_VERSION
        );
        let duplicate = parse_audit(&rendered.replacen(
            "  semantic =",
            "  language = \"duplicate\"\n  semantic =",
            1,
        ))
        .expect_err("duplicate field fails");
        assert_eq!(
            duplicate.to_diagnostic("model.audit").code(),
            codes::AUDIT_STRUCTURE
        );
        let unknown =
            parse_audit(&rendered.replacen("  semantic =", "  mystery = \"bad\"\n  semantic =", 1))
                .expect_err("unknown core field fails");
        assert_eq!(
            unknown.to_diagnostic("model.audit").code(),
            codes::AUDIT_VERSION
        );
        let invalid_escape = parse_audit(&rendered.replacen("\"0.0.1-dev\"", "\"\\q\"", 1))
            .expect_err("invalid JSON string escape fails");
        assert_eq!(
            invalid_escape.to_diagnostic("model.audit").code(),
            codes::AUDIT_SYNTAX
        );
        let invalid_id = parse_audit(&rendered.replacen(&id('a'), "not-an-id", 1))
            .expect_err("invalid program ID fails");
        assert_eq!(
            invalid_id.to_diagnostic("model.audit").code(),
            codes::AUDIT_MODEL
        );

        let dangling = rendered.replacen(
            "    definition",
            &format!(
                "    reference expression 1 {{\n      target_kind = \"definition\"\n      target = {}\n    }}\n    definition",
                quote(&id('f')).expect("ID quotes")
            ),
            1,
        );
        let dangling = parse_audit(&dangling).expect_err("dangling target fails");
        assert_eq!(
            dangling.to_diagnostic("model.audit").code(),
            codes::AUDIT_MODEL
        );
    }
}
