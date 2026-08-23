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
    Init,
    Test,
    ProjectCheck,
    Lsp,
}

impl Command {
    #[cfg(test)]
    pub(crate) const fn all() -> &'static [Self] {
        &[
            Self::Run,
            Self::Check,
            Self::Repl,
            Self::Semantic,
            Self::Audit,
            Self::Format,
            Self::Init,
            Self::Test,
            Self::ProjectCheck,
            Self::Lsp,
        ]
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "run" => Some(Self::Run),
            "check" => Some(Self::Check),
            "repl" => Some(Self::Repl),
            "semantic" => Some(Self::Semantic),
            "audit" => Some(Self::Audit),
            "fmt" => Some(Self::Format),
            "init" => Some(Self::Init),
            "test" => Some(Self::Test),
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
            Self::Init => "init",
            Self::Test => "test",
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
        let roots = [
            ("run", Command::Run),
            ("check", Command::Check),
            ("repl", Command::Repl),
            ("semantic", Command::Semantic),
            ("audit", Command::Audit),
            ("fmt", Command::Format),
            ("init", Command::Init),
            ("test", Command::Test),
            ("lsp", Command::Lsp),
        ];
        for (name, command) in roots {
            assert_eq!(Command::parse(name), Some(command));
            assert_eq!(command.name(), name);
        }
        assert_eq!(
            roots.map(|(name, _)| name),
            [
                "run", "check", "repl", "semantic", "audit", "fmt", "init", "test", "lsp",
            ]
        );
        for planned_only in [
            "project", "build", "query", "patch", "replay", "explain", "evidence", "version",
            "support", "migrate",
        ] {
            assert_eq!(
                Command::parse(planned_only),
                None,
                "plan-only root command must stay rejected: {planned_only}"
            );
        }
        assert_eq!(Command::ProjectCheck.name(), "project check");
    }

    #[test]
    fn catalog_contains_each_implemented_command_once() {
        let commands = Command::all();
        assert_eq!(commands.len(), 10);
        assert_eq!(
            commands
                .iter()
                .map(|command| command.name())
                .collect::<Vec<_>>(),
            vec![
                "run",
                "check",
                "repl",
                "semantic",
                "audit",
                "fmt",
                "init",
                "test",
                "project check",
                "lsp",
            ]
        );
        for (index, command) in commands.iter().enumerate() {
            assert!(!commands[..index].contains(command));
        }
    }
}
