//! Deterministic, bounded checked-source cases for DEC-0263.
//!
//! This module deliberately owns no compiler objects and performs no I/O. A
//! generated case must re-enter the normal source-to-checked pipeline in the
//! differential test before it has any evaluation or bytecode authority.

pub const FIXED_SEEDS: [u64; 8] = [
    0x0000_0000_0000_0001,
    0x0123_4567_89ab_cdef,
    0x243f_6a88_85a3_08d3,
    0x517c_c1b7_2722_0a95,
    0x9e37_79b9_7f4a_7c15,
    0xa409_3822_299f_31d0,
    0xd1b5_4a32_d192_ed03,
    0xffff_ffff_ffff_ffff,
];

pub const ORDINALS_PER_SEED: u32 = 12;
pub const MAX_DEFINITIONS: usize = 4;
pub const MAX_EXPRESSIONS: usize = 24;
pub const MAX_DEPTH: usize = 8;
pub const MAX_HANDLERS: usize = 2;
pub const MAX_CLAUSES: usize = 3;
pub const MAX_MUTABLE_BINDINGS: usize = 2;
pub const MAX_SOURCE_BYTES: usize = 4 * 1024;
pub const MAX_OUTPUT_BYTES: usize = 8 * 1024;
pub const MAX_SHRINK_ATTEMPTS: usize = 256;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Scenario {
    DirectConditional,
    Closure,
    Match,
    Mutation,
    ResumeOnce,
    Propagation,
    ResumeZero,
    ResumeFault,
    ClauseFault,
    SharedState,
    NestedHandlers,
    UnicodeSource,
}

impl Scenario {
    const ALL: [Self; ORDINALS_PER_SEED as usize] = [
        Self::DirectConditional,
        Self::Closure,
        Self::Match,
        Self::Mutation,
        Self::ResumeOnce,
        Self::Propagation,
        Self::ResumeZero,
        Self::ResumeFault,
        Self::ClauseFault,
        Self::SharedState,
        Self::NestedHandlers,
        Self::UnicodeSource,
    ];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Shape {
    pub definitions: usize,
    pub expressions: usize,
    pub depth: usize,
    pub handlers: usize,
    pub clauses: usize,
    pub mutable_bindings: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedCase {
    pub seed: u64,
    pub ordinal: u32,
    pub logical_name: String,
    pub source: String,
    pub scenario: Scenario,
    pub shape: Shape,
}

impl GeneratedCase {
    pub fn validate_bounds(&self) -> Result<(), &'static str> {
        if self.shape.definitions > MAX_DEFINITIONS {
            return Err("definition bound");
        }
        if self.shape.expressions > MAX_EXPRESSIONS {
            return Err("expression bound");
        }
        if self.shape.depth > MAX_DEPTH {
            return Err("nesting-depth bound");
        }
        if self.shape.handlers > MAX_HANDLERS {
            return Err("handler bound");
        }
        if self.shape.clauses > MAX_CLAUSES {
            return Err("operation-clause bound");
        }
        if self.shape.mutable_bindings > MAX_MUTABLE_BINDINGS {
            return Err("mutable-binding bound");
        }
        if self.source.len() > MAX_SOURCE_BYTES {
            return Err("source-byte bound");
        }
        Ok(())
    }
}

pub fn generate(seed: u64, ordinal: u32) -> GeneratedCase {
    let scenario = Scenario::ALL[(ordinal as usize) % Scenario::ALL.len()];
    let mut random = SplitMix64::new(seed ^ u64::from(ordinal).rotate_left(23));
    let number = i64::try_from(random.next() % 17 + 1).expect("bounded value fits i64");
    let word = ["alpha", "beta", "gamma", "delta"][(random.next() % 4) as usize];
    let (source, shape) = render(scenario, number, word);
    let source = match ((seed ^ u64::from(ordinal)) & 3) as u32 {
        1 => source.replace('\n', "\r\n"),
        2 if scenario == Scenario::UnicodeSource => format!("\u{feff}{source}"),
        _ => source,
    };
    GeneratedCase {
        seed,
        ordinal,
        logical_name: format!("generated/eff-2105/{seed:016x}-{ordinal:02}.ling"),
        source,
        scenario,
        shape,
    }
}

/// Returns deterministic structural simplifications in DEC-0263 order.
///
/// Rechecking and failure preservation belong to the caller because this
/// module is intentionally unable to fabricate or retain checked compiler
/// state.
pub fn shrink_candidates(case: &GeneratedCase) -> Vec<String> {
    let mut candidates = Vec::new();
    let normalized = case.source.replace("\r\n", "\n");
    let without_bom = normalized.strip_prefix('\u{feff}').unwrap_or(&normalized);

    // Removable declaration and sequence-element candidates.
    if let Some(value) = remove_first_helper_definition(without_bom) {
        candidates.push(value);
    }
    candidates.push(without_bom.replace("    Console.write \"after\"\n", ""));

    // Handler nesting and clause/body simplification.
    candidates.push(without_bom.replace("            resume ()\n", "            ()\n"));
    candidates.push(without_bom.replace("                resume ()\n", "                ()\n"));

    // Literal magnitude/text length, then lexical-name simplification.
    for literal in 2..=17 {
        candidates.push(without_bom.replace(&literal.to_string(), "1"));
    }
    for text in ["alpha", "beta", "gamma", "delta", "你好🙂", "e\u{301}"] {
        candidates.push(without_bom.replace(text, "x"));
    }
    candidates.push(without_bom.replace("结果", "value"));

    candidates.retain(|candidate| candidate != &case.source && candidate.len() <= MAX_SOURCE_BYTES);
    candidates.sort_by(|left, right| {
        left.len()
            .cmp(&right.len())
            .then_with(|| left.as_bytes().cmp(right.as_bytes()))
    });
    candidates.dedup();
    candidates.truncate(MAX_SHRINK_ATTEMPTS);
    candidates
}

pub fn minimize_failure<F>(case: &GeneratedCase, mut preserves_failure: F) -> (String, usize)
where
    F: FnMut(&str) -> bool,
{
    let mut best = case.source.clone();
    let mut attempts = 0;
    for candidate in shrink_candidates(case) {
        if attempts == MAX_SHRINK_ATTEMPTS {
            break;
        }
        attempts += 1;
        if preserves_failure(&candidate)
            && (candidate.len(), candidate.as_bytes()) < (best.len(), best.as_bytes())
        {
            best = candidate;
        }
    }
    (best, attempts)
}

fn remove_first_helper_definition(source: &str) -> Option<String> {
    let start = source.find("\nlet ")? + 1;
    let main = source.find("\nlet main ")? + 1;
    if start == main {
        return None;
    }
    let next = source[start..].find("\n\nlet ")? + start + 2;
    Some(format!("{}{}", &source[..start], &source[next..]))
}

fn render(scenario: Scenario, number: i64, word: &str) -> (String, Shape) {
    let console = "module Main\n    requires Console.Write\n\n";
    match scenario {
        Scenario::DirectConditional => (
            format!(
                "module Main\n\nlet choose condition =\n    if condition then {number} else 0\n\nlet main () =\n    let ignored = choose true\n    ()\n"
            ),
            shape(2, 8, 3, 0, 0, 0),
        ),
        Scenario::Closure => (
            format!(
                "{console}let factory prefix =\n    let local value = Console.write prefix\n    local\n\nlet main () =\n    let callback = factory \"{word}\"\n    callback \"x\"\n"
            ),
            shape(2, 10, 3, 0, 0, 0),
        ),
        Scenario::Match => (
            format!(
                "{console}type Choice =\n    | Empty\n    | Value of Int\n\nlet choose item =\n    match item with\n    | Value value -> value\n    | Empty -> 0\n\nlet main () =\n    Console.write (Text.format \"{{}}\" (choose (Value {number})))\n"
            ),
            shape(3, 10, 4, 0, 0, 0),
        ),
        Scenario::Mutation => (
            format!(
                "{console}let main () =\n    let mutable cell = 0\n    cell <- {number}\n    Console.write (Text.format \"{{}}\" cell)\n"
            ),
            shape(1, 8, 3, 0, 0, 1),
        ),
        Scenario::ResumeOnce => (
            format!(
                "{console}let main () =\n    handle Console.write \"{word}\" with\n        operation Console.Write.write(message, resume) ->\n            resume ()\n            Console.write \"after\"\n"
            ),
            shape(1, 7, 4, 1, 1, 0),
        ),
        Scenario::Propagation => (
            format!(
                "{console}let emitBoth () =\n    Console.write \"first\"\n    Console.write \"{word}\"\n\nlet main () =\n    handle emitBoth () with\n        operation Console.Write.write(message, resume) ->\n            if message == \"first\" then\n                resume ()\n            else\n                ()\n"
            ),
            shape(2, 12, 5, 1, 1, 0),
        ),
        Scenario::ResumeZero => (
            format!(
                "{console}let main () =\n    let ignored =\n        handle Console.write \"{word}\" with\n            operation Console.Write.write(message, resume) -> ()\n    Console.write \"after\"\n"
            ),
            shape(1, 8, 4, 1, 1, 0),
        ),
        Scenario::ResumeFault => (
            format!(
                "{console}let invokeTwice callback =\n    let ignored = callback ()\n    callback ()\n\nlet main () =\n    handle Console.write \"{word}\" with\n        operation Console.Write.write(message, resume) -> invokeTwice resume\n"
            ),
            shape(2, 10, 4, 1, 1, 0),
        ),
        Scenario::ClauseFault => (
            format!(
                "{console}let main () =\n    handle Console.write \"{word}\" with\n        operation Console.Write.write(message, resume) ->\n            Console.write \"committed\"\n            let ignored = {number} / 0\n            ()\n"
            ),
            shape(1, 10, 4, 1, 1, 0),
        ),
        Scenario::SharedState => (
            format!(
                "{console}let main () =\n    let mutable cell = 0\n    let ignored =\n        handle Console.write \"{word}\" with\n            operation Console.Write.write(message, resume) ->\n                resume ()\n                cell <- {number}\n    Console.write (Text.format \"{{}}\" cell)\n"
            ),
            shape(1, 12, 5, 1, 1, 1),
        ),
        Scenario::NestedHandlers => (
            format!(
                "{console}let inner () =\n    handle Console.write \"{word}\" with\n        operation Console.Write.write(message, resume) ->\n            Console.write \"inner\"\n\nlet main () =\n    handle inner () with\n        operation Console.Write.write(message, resume) -> ()\n"
            ),
            shape(2, 10, 4, 2, 2, 0),
        ),
        Scenario::UnicodeSource => (
            format!(
                "{console}let 结果 value = Console.write value\n\nlet main () =\n    let ignored = 结果 \"你好🙂\"\n    Console.write \"e\u{301}-{number}\"\n"
            ),
            shape(2, 7, 2, 0, 0, 0),
        ),
    }
}

const fn shape(
    definitions: usize,
    expressions: usize,
    depth: usize,
    handlers: usize,
    clauses: usize,
    mutable_bindings: usize,
) -> Shape {
    Shape {
        definitions,
        expressions,
        depth,
        handlers,
        clauses,
        mutable_bindings,
    }
}

struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    const fn new(state: u64) -> Self {
        Self { state }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }
}
