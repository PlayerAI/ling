use std::error::Error;
use std::fmt;

/// Injected `Console.Write` host capability.
///
/// One successful call appends exactly one UTF-8 LF after `text`. An error
/// reports whether the current logical operation may already be observable.
pub trait ConsoleCapability {
    fn write_line(&mut self, text: &str) -> Result<(), HostError>;
}

/// Stable host failure categories admitted by RFC-0014.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostErrorCategory {
    BrokenPipe,
    PermissionDenied,
    Interrupted,
    Other,
}

impl HostErrorCategory {
    /// Returns the stable `L-RUNTIME-0001` category.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::BrokenPipe => "broken_pipe",
            Self::PermissionDenied => "permission_denied",
            Self::Interrupted => "interrupted",
            Self::Other => "other",
        }
    }
}

/// Failure returned by an injected host capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostError {
    category: HostErrorCategory,
    committed: bool,
}

impl HostError {
    /// Creates an error for an operation that did not become observable.
    #[must_use]
    pub const fn before_commit(category: HostErrorCategory) -> Self {
        Self {
            category,
            committed: false,
        }
    }

    /// Creates an error for an operation that may already be observable.
    #[must_use]
    pub const fn after_commit(category: HostErrorCategory) -> Self {
        Self {
            category,
            committed: true,
        }
    }

    #[must_use]
    pub const fn category(self) -> HostErrorCategory {
        self.category
    }

    #[must_use]
    pub const fn committed(self) -> bool {
        self.committed
    }
}

impl fmt::Display for HostError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.category.name())
    }
}

impl Error for HostError {}

/// Explicit set of host capabilities granted to one execution.
pub struct HostCapabilities<'host> {
    pub(crate) console: Option<&'host mut dyn ConsoleCapability>,
}

impl HostCapabilities<'_> {
    /// Grants no host capability.
    #[must_use]
    pub const fn none() -> Self {
        Self { console: None }
    }
}

impl<'host> HostCapabilities<'host> {
    /// Grants exactly `Console.Write` through the provided adapter.
    #[must_use]
    pub fn with_console(console: &'host mut dyn ConsoleCapability) -> Self {
        Self {
            console: Some(console),
        }
    }
}
