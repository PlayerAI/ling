use std::fmt;

/// The currently implemented Ling command set from DEC-0003 and its
/// subsequent accepted Preview slices. This is an internal parser catalog;
/// it does not advertise any execution-plan command that is not implemented.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Command {
    Run,
    Check,
    Repl,
    Semantic,
    Audit,
    Format,
    ProjectCheck,
    Lsp,
}

impl Command {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "run" => Some(Self::Run),
            "check" => Some(Self::Check),
            "repl" => Some(Self::Repl),
            "semantic" => Some(Self::Semantic),
            "audit" => Some(Self::Audit),
            "fmt" => Some(Self::Format),
            "lsp" => Some(Self::Lsp),
            _ => None,
        }
    }

    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Check => "check",
            Self::Repl => "repl",
            Self::Semantic => "semantic",
            Self::Audit => "audit",
            Self::Format => "fmt",
            Self::ProjectCheck => "project check",
            Self::Lsp => "lsp",
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_parses_only_implemented_root_commands() {
        for (name, command) in [
            ("run", Command::Run),
            ("check", Command::Check),
            ("repl", Command::Repl),
            ("semantic", Command::Semantic),
            ("audit", Command::Audit),
            ("fmt", Command::Format),
            ("lsp", Command::Lsp),
        ] {
            assert_eq!(Command::parse(name), Some(command));
            assert_eq!(command.name(), name);
        }
        assert_eq!(Command::parse("build"), None);
        assert_eq!(Command::parse("test"), None);
        assert_eq!(Command::ProjectCheck.name(), "project check");
    }
}
