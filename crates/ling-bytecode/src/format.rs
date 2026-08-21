/// Public protocol label assigned by RFC-0014.
pub const BYTECODE_PROTOCOL: &str = BYTECODE_PROTOCOL_1_0;
/// Exact bytecode-1.0 protocol label.
pub const BYTECODE_PROTOCOL_1_0: &str = "ling.bytecode/1.0";
/// Exact bytecode-1.1 protocol label assigned by RFC-0015.
pub const BYTECODE_PROTOCOL_1_1: &str = "ling.bytecode/1.1";
/// Exact bytecode-1.2 protocol label assigned by RFC-0016.
pub const BYTECODE_PROTOCOL_1_2: &str = "ling.bytecode/1.2";
/// Exact eight-byte bytecode magic.
pub const BYTECODE_MAGIC: [u8; 8] = *b"LINGBC\0\0";
/// Exact encoded header width.
pub const HEADER_BYTES: u32 = 40;
/// Sentinel used only for explicitly optional encoded indexes.
pub const NO_INDEX: u32 = u32::MAX;

/// A two-component bytecode or language version.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormatVersion {
    major: u16,
    minor: u16,
}

impl FormatVersion {
    /// Creates an explicit two-component version.
    #[must_use]
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    /// Returns the major component.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor component.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }
}

/// Exact bytecode format accepted by RFC-0014.
pub const FORMAT_VERSION: FormatVersion = FORMAT_VERSION_1_0;
/// Exact version-1.0 format tuple.
pub const FORMAT_VERSION_1_0: FormatVersion = FormatVersion::new(1, 0);
/// Exact version-1.1 format tuple accepted by RFC-0015.
pub const FORMAT_VERSION_1_1: FormatVersion = FormatVersion::new(1, 1);
/// Exact version-1.2 format tuple accepted by RFC-0016.
pub const FORMAT_VERSION_1_2: FormatVersion = FormatVersion::new(1, 2);
/// Language compatibility version encoded by RFC-0014.
pub const LANGUAGE_VERSION: FormatVersion = FormatVersion::new(0, 1);

/// A three-component Unicode data version.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnicodeVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl UnicodeVersion {
    /// Creates an explicit Unicode version.
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major component.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor component.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch component.
    #[must_use]
    pub const fn patch(self) -> u16 {
        self.patch
    }
}

/// Unicode tables required by RFC-0014.
pub const UNICODE_VERSION: UnicodeVersion = UnicodeVersion::new(17, 0, 0);

/// Hard decoder/verifier maxima fixed by RFC-0014.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecodeLimits {
    artifact_bytes: u64,
    string_entries: u32,
    bytes_per_string_or_integer: u32,
    packages: u32,
    modules: u32,
    types: u32,
    constants: u32,
    sources: u32,
    functions: u32,
    registers_per_function: u32,
    blocks_per_function: u32,
    arguments_per_operation: u32,
    executable_locations: u32,
}

impl DecodeLimits {
    /// Returns the immutable RFC-0014 hard-limit set.
    #[must_use]
    pub const fn rfc_0014() -> Self {
        Self {
            artifact_bytes: 67_108_864,
            string_entries: 262_144,
            bytes_per_string_or_integer: 16_777_216,
            packages: 65_536,
            modules: 65_536,
            types: 4,
            constants: 1_048_576,
            sources: 65_536,
            functions: 262_144,
            registers_per_function: 65_536,
            blocks_per_function: 65_536,
            arguments_per_operation: 65_536,
            executable_locations: 4_194_304,
        }
    }

    /// Returns the RFC-0015 hard-limit set for bytecode 1.1.
    #[must_use]
    pub const fn rfc_0015() -> Self {
        Self {
            types: 262_144,
            ..Self::rfc_0014()
        }
    }

    /// Returns the RFC-0016 hard-limit set for bytecode 1.2.
    #[must_use]
    pub const fn rfc_0016() -> Self {
        Self::rfc_0015()
    }

    /// Returns the maximum artifact byte length.
    #[must_use]
    pub const fn artifact_bytes(self) -> u64 {
        self.artifact_bytes
    }

    /// Returns the maximum string count.
    #[must_use]
    pub const fn string_entries(self) -> u32 {
        self.string_entries
    }

    /// Returns the maximum byte length of one string or integer magnitude.
    #[must_use]
    pub const fn bytes_per_string_or_integer(self) -> u32 {
        self.bytes_per_string_or_integer
    }

    /// Returns the maximum package count.
    #[must_use]
    pub const fn packages(self) -> u32 {
        self.packages
    }

    /// Returns the maximum module count.
    #[must_use]
    pub const fn modules(self) -> u32 {
        self.modules
    }

    /// Returns the exact or maximum type count for the selected revision.
    #[must_use]
    pub const fn types(self) -> u32 {
        self.types
    }

    /// Returns the maximum constant count.
    #[must_use]
    pub const fn constants(self) -> u32 {
        self.constants
    }

    /// Returns the maximum source count.
    #[must_use]
    pub const fn sources(self) -> u32 {
        self.sources
    }

    /// Returns the maximum function count.
    #[must_use]
    pub const fn functions(self) -> u32 {
        self.functions
    }

    /// Returns the maximum register count in one function.
    #[must_use]
    pub const fn registers_per_function(self) -> u32 {
        self.registers_per_function
    }

    /// Returns the maximum block count in one function.
    #[must_use]
    pub const fn blocks_per_function(self) -> u32 {
        self.blocks_per_function
    }

    /// Returns the maximum call/branch argument count.
    #[must_use]
    pub const fn arguments_per_operation(self) -> u32 {
        self.arguments_per_operation
    }

    /// Returns the maximum combined instruction/terminator/source-map count.
    #[must_use]
    pub const fn executable_locations(self) -> u32 {
        self.executable_locations
    }
}

impl Default for DecodeLimits {
    fn default() -> Self {
        Self::rfc_0014()
    }
}
