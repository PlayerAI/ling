use std::io::IsTerminal as _;

use ling_diagnostics::{Diagnostic, MessageOrder, Severity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OutputFormat {
    Human,
    Json,
}

impl OutputFormat {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "human" => Some(Self::Human),
            "json" => Some(Self::Json),
            _ => None,
        }
    }

    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Json => "json",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HumanLanguage {
    Bilingual,
    Chinese,
    English,
}

impl HumanLanguage {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "bilingual" => Some(Self::Bilingual),
            "zh-CN" => Some(Self::Chinese),
            "en" => Some(Self::English),
            _ => None,
        }
    }

    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Bilingual => "bilingual",
            Self::Chinese => "zh-CN",
            Self::English => "en",
        }
    }

    const fn message_order(self) -> MessageOrder {
        match self {
            Self::Bilingual => MessageOrder::Bilingual,
            Self::Chinese => MessageOrder::ChineseFirst,
            Self::English => MessageOrder::EnglishFirst,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ColorChoice {
    Auto,
    Always,
    Never,
}

impl ColorChoice {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "auto" => Some(Self::Auto),
            "always" => Some(Self::Always),
            "never" => Some(Self::Never),
            _ => None,
        }
    }

    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Always => "always",
            Self::Never => "never",
        }
    }

    const fn enabled(self, stderr_is_terminal: bool) -> bool {
        match self {
            Self::Auto => stderr_is_terminal,
            Self::Always => true,
            Self::Never => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Verbosity {
    Normal,
    Quiet,
    Verbose,
}

impl Verbosity {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Quiet => "quiet",
            Self::Verbose => "verbose",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct OutputPolicy {
    format: OutputFormat,
    language: HumanLanguage,
    color: ColorChoice,
    verbosity: Verbosity,
}

impl OutputPolicy {
    pub(crate) const fn new(
        format: OutputFormat,
        language: HumanLanguage,
        color: ColorChoice,
        verbosity: Verbosity,
    ) -> Self {
        Self {
            format,
            language,
            color,
            verbosity,
        }
    }

    pub(crate) const fn human() -> Self {
        Self::new(
            OutputFormat::Human,
            HumanLanguage::Bilingual,
            ColorChoice::Never,
            Verbosity::Normal,
        )
    }

    pub(crate) const fn json() -> Self {
        Self::new(
            OutputFormat::Json,
            HumanLanguage::Bilingual,
            ColorChoice::Never,
            Verbosity::Normal,
        )
    }

    pub(crate) const fn format(self) -> OutputFormat {
        self.format
    }

    pub(crate) const fn language(self) -> HumanLanguage {
        self.language
    }

    pub(crate) const fn color(self) -> ColorChoice {
        self.color
    }

    pub(crate) const fn verbosity(self) -> Verbosity {
        self.verbosity
    }

    pub(crate) const fn is_quiet(self) -> bool {
        matches!(self.verbosity, Verbosity::Quiet)
    }

    pub(crate) const fn is_verbose(self) -> bool {
        matches!(self.verbosity, Verbosity::Verbose)
    }

    pub(crate) fn human_text(self, chinese: &str, english: &str) -> String {
        match self.language {
            HumanLanguage::Bilingual => format!("{chinese} / {english}"),
            HumanLanguage::Chinese => format!("{chinese}\nEnglish: {english}"),
            HumanLanguage::English => format!("{english}\n中文: {chinese}"),
        }
    }

    pub(crate) fn human_summary(self, chinese: &str, english: &str, facts: &str) -> String {
        match self.language {
            HumanLanguage::Bilingual => format!("{chinese} / {english}: {facts}"),
            HumanLanguage::Chinese => format!("{chinese}：{facts}\nEnglish: {english}: {facts}"),
            HumanLanguage::English => format!("{english}: {facts}\n中文: {chinese}：{facts}"),
        }
    }

    pub(crate) fn render_diagnostic(self, diagnostic: &Diagnostic) -> String {
        self.render_diagnostic_for_terminal(diagnostic, std::io::stderr().is_terminal())
    }

    fn render_diagnostic_for_terminal(
        self,
        diagnostic: &Diagnostic,
        stderr_is_terminal: bool,
    ) -> String {
        let rendered = diagnostic.render_human_bilingual(self.language.message_order());
        if !self.color.enabled(stderr_is_terminal) {
            return rendered;
        }
        let code = match diagnostic.severity() {
            Severity::Error => 31,
            Severity::Warning => 33,
            Severity::Note => 36,
        };
        format!("\u{1b}[{code}m{rendered}\u{1b}[0m")
    }

    pub(crate) fn verbose_event(self, command: &str) -> String {
        self.human_text(
            &format!(
                "详细：command={command} format={} language={} color={} verbosity={}",
                self.format().as_str(),
                self.language().as_str(),
                self.color().as_str(),
                self.verbosity().as_str()
            ),
            &format!(
                "verbose: command={command} format={} language={} color={} verbosity={}",
                self.format().as_str(),
                self.language().as_str(),
                self.color().as_str(),
                self.verbosity().as_str()
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ling_diagnostics::{DiagnosticCode, Severity};

    fn diagnostic() -> Diagnostic {
        Diagnostic::new(
            DiagnosticCode::new("L-TEST-0001"),
            Severity::Error,
            "测试失败",
            "test failed",
        )
    }

    #[test]
    fn language_order_retains_both_diagnostic_languages() {
        for (language, expected) in [
            (
                HumanLanguage::Bilingual,
                "error[L-TEST-0001]: 测试失败 / test failed",
            ),
            (
                HumanLanguage::Chinese,
                "error[L-TEST-0001]: 测试失败\n = English: test failed",
            ),
            (
                HumanLanguage::English,
                "error[L-TEST-0001]: test failed\n = 中文: 测试失败",
            ),
        ] {
            let policy = OutputPolicy::new(
                OutputFormat::Human,
                language,
                ColorChoice::Never,
                Verbosity::Normal,
            );
            assert_eq!(
                policy.render_diagnostic_for_terminal(&diagnostic(), false),
                expected
            );
        }
    }

    #[test]
    fn color_is_explicit_or_terminal_dependent_and_never_changes_text() {
        let always = OutputPolicy::new(
            OutputFormat::Human,
            HumanLanguage::Bilingual,
            ColorChoice::Always,
            Verbosity::Normal,
        );
        assert_eq!(
            always.render_diagnostic_for_terminal(&diagnostic(), false),
            "\u{1b}[31merror[L-TEST-0001]: 测试失败 / test failed\u{1b}[0m"
        );

        let auto = OutputPolicy::new(
            OutputFormat::Human,
            HumanLanguage::Bilingual,
            ColorChoice::Auto,
            Verbosity::Normal,
        );
        assert!(
            !auto
                .render_diagnostic_for_terminal(&diagnostic(), false)
                .contains('\u{1b}')
        );
        assert!(
            auto.render_diagnostic_for_terminal(&diagnostic(), true)
                .contains("\u{1b}[31m")
        );
    }

    #[test]
    fn verbose_event_is_path_free_and_deterministic() {
        let policy = OutputPolicy::new(
            OutputFormat::Human,
            HumanLanguage::English,
            ColorChoice::Never,
            Verbosity::Verbose,
        );
        let first = policy.verbose_event("check");
        assert_eq!(first, policy.verbose_event("check"));
        assert!(first.starts_with("verbose: command=check"));
        assert!(first.contains("中文: 详细：command=check"));
        assert!(!first.contains('\\'));
        assert!(!first.contains("D:"));
    }
}
