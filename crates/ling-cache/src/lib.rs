//! Disposable, versioned query-cache envelopes.
//!
//! The cache stores only opaque bytes behind a canonical key. Callers must
//! validate and reconstruct any derived value before publication; this crate
//! never deserializes unchecked compiler IR or exposes cache paths as language
//! data.

use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

const MAGIC: &[u8] = b"LING-CACHE\0";
const ENVELOPE_VERSION: u16 = 1;
const MAX_KEY_BYTES: usize = 16 * 1024;
const MAX_PAYLOAD_BYTES: usize = 16 * 1024 * 1024;

/// A canonical, toolchain-scoped identity for one disposable cache entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CacheKey {
    compiler_version: String,
    language_version: [u16; 3],
    unicode_version: [u8; 3],
    schema_version: u16,
    profile: String,
    target: String,
    query: String,
    logical_name: String,
    source_digest: [u8; 32],
}

impl CacheKey {
    /// Creates a key whose source identity is the exact UTF-8 byte digest.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        compiler_version: impl Into<String>,
        language_version: [u16; 3],
        unicode_version: [u8; 3],
        schema_version: u16,
        profile: impl Into<String>,
        target: impl Into<String>,
        query: impl Into<String>,
        logical_name: impl Into<String>,
        source_bytes: &[u8],
    ) -> Self {
        Self {
            compiler_version: compiler_version.into(),
            language_version,
            unicode_version,
            schema_version,
            profile: profile.into(),
            target: target.into(),
            query: query.into(),
            logical_name: logical_name.into(),
            source_digest: *blake3::hash(source_bytes).as_bytes(),
        }
    }

    /// Returns the canonical bytes used to derive the on-disk entry name.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_text(&mut bytes, &self.compiler_version);
        for version in self.language_version {
            bytes.extend_from_slice(&version.to_le_bytes());
        }
        bytes.extend_from_slice(&self.unicode_version);
        bytes.extend_from_slice(&self.schema_version.to_le_bytes());
        push_text(&mut bytes, &self.profile);
        push_text(&mut bytes, &self.target);
        push_text(&mut bytes, &self.query);
        push_text(&mut bytes, &self.logical_name);
        bytes.extend_from_slice(&self.source_digest);
        bytes
    }

    /// Returns the lowercase BLAKE3 identity used for the cache filename.
    #[must_use]
    pub fn cache_id(&self) -> String {
        blake3::hash(&self.canonical_bytes()).to_hex().to_string()
    }
}

/// Returns a lowercase BLAKE3 digest for a workspace input dimension.
#[must_use]
pub fn bytes_digest(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

/// Errors while writing a cache entry. Reads deliberately fall back to a miss
/// for all malformed or unavailable entries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CacheStoreError {
    InvalidSize,
    Io {
        operation: &'static str,
        kind: io::ErrorKind,
    },
}

impl fmt::Display for CacheStoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSize => formatter.write_str("cache entry exceeds the bounded size"),
            Self::Io { operation, kind } => {
                write!(formatter, "cache {operation} failed ({kind:?})")
            }
        }
    }
}

impl std::error::Error for CacheStoreError {}

/// A disposable cache rooted at an explicit caller-owned directory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CacheStore {
    root: PathBuf,
}

impl CacheStore {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Loads and validates an entry. Any missing, corrupt, incompatible, or
    /// unreadable entry is a safe cache miss.
    #[must_use]
    pub fn load(&self, key: &CacheKey) -> Option<Vec<u8>> {
        let bytes = fs::read(self.path(key)).ok()?;
        decode(&bytes, key)
    }

    /// Atomically publishes one bounded opaque payload. An existing entry is
    /// retained because equal keys must be immutable and deterministic.
    pub fn store(&self, key: &CacheKey, payload: &[u8]) -> Result<(), CacheStoreError> {
        let key_bytes = key.canonical_bytes();
        if key_bytes.len() > MAX_KEY_BYTES || payload.len() > MAX_PAYLOAD_BYTES {
            return Err(CacheStoreError::InvalidSize);
        }
        let envelope = encode(&key_bytes, payload);
        fs::create_dir_all(&self.root).map_err(|error| io_error("directory creation", error))?;
        let target = self.path(key);
        let temporary = target.with_extension("tmp");
        let mut file = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
        {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => return Ok(()),
            Err(error) => return Err(io_error("temporary creation", error)),
        };
        if let Err(error) = file.write_all(&envelope).and_then(|()| file.sync_all()) {
            let _ = fs::remove_file(&temporary);
            return Err(io_error("temporary write", error));
        }
        drop(file);
        match fs::rename(&temporary, &target) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                let _ = fs::remove_file(&temporary);
                Ok(())
            }
            Err(error) => {
                let _ = fs::remove_file(&temporary);
                Err(io_error("publication", error))
            }
        }
    }

    fn path(&self, key: &CacheKey) -> PathBuf {
        self.root.join(format!("{}.lcache", key.cache_id()))
    }
}

fn push_text(output: &mut Vec<u8>, value: &str) {
    let length = u32::try_from(value.len()).expect("cache key text is bounded by the caller");
    output.extend_from_slice(&length.to_le_bytes());
    output.extend_from_slice(value.as_bytes());
}

fn encode(key: &[u8], payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(MAGIC.len() + 2 + 4 + 8 + key.len() + payload.len() + 32);
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&ENVELOPE_VERSION.to_le_bytes());
    bytes.extend_from_slice(&(key.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    bytes.extend_from_slice(key);
    bytes.extend_from_slice(payload);
    let checksum = blake3::hash(&bytes);
    bytes.extend_from_slice(checksum.as_bytes());
    bytes
}

fn decode(bytes: &[u8], key: &CacheKey) -> Option<Vec<u8>> {
    let header_len = MAGIC.len() + 2 + 4 + 8;
    if bytes.len() < header_len + 32 || !bytes.starts_with(MAGIC) {
        return None;
    }
    let mut cursor = MAGIC.len();
    let version = read_u16(bytes, &mut cursor)?;
    let key_len = usize::try_from(read_u32(bytes, &mut cursor)?).ok()?;
    let payload_len = usize::try_from(read_u64(bytes, &mut cursor)?).ok()?;
    if version != ENVELOPE_VERSION
        || key_len > MAX_KEY_BYTES
        || payload_len > MAX_PAYLOAD_BYTES
        || bytes.len() != header_len + key_len + payload_len + 32
    {
        return None;
    }
    let key_end = cursor.checked_add(key_len)?;
    let payload_end = key_end.checked_add(payload_len)?;
    if bytes.get(cursor..key_end)? != key.canonical_bytes()
        || blake3::hash(&bytes[..payload_end]).as_bytes() != bytes.get(payload_end..)?
    {
        return None;
    }
    Some(bytes[key_end..payload_end].to_vec())
}

fn read_u16(bytes: &[u8], cursor: &mut usize) -> Option<u16> {
    let end = cursor.checked_add(2)?;
    let value = u16::from_le_bytes(bytes.get(*cursor..end)?.try_into().ok()?);
    *cursor = end;
    Some(value)
}

fn read_u32(bytes: &[u8], cursor: &mut usize) -> Option<u32> {
    let end = cursor.checked_add(4)?;
    let value = u32::from_le_bytes(bytes.get(*cursor..end)?.try_into().ok()?);
    *cursor = end;
    Some(value)
}

fn read_u64(bytes: &[u8], cursor: &mut usize) -> Option<u64> {
    let end = cursor.checked_add(8)?;
    let value = u64::from_le_bytes(bytes.get(*cursor..end)?.try_into().ok()?);
    *cursor = end;
    Some(value)
}

fn io_error(operation: &'static str, error: io::Error) -> CacheStoreError {
    CacheStoreError::Io {
        operation,
        kind: error.kind(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);

    fn root() -> PathBuf {
        let id = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("ling-cache-test-{}-{id}", std::process::id()))
    }

    fn key(source: &[u8]) -> CacheKey {
        CacheKey::new(
            "0.0.1-dev",
            [0, 1, 0],
            [17, 0, 0],
            1,
            "default",
            "host",
            "line_index",
            "src/Main.ling",
            source,
        )
    }

    #[test]
    fn round_trips_and_rejects_foreign_keys() {
        let root = root();
        let store = CacheStore::new(&root);
        let first = key(b"let main () = ()\n");
        let second = key(b"let main () = 1\n");
        store.store(&first, b"payload").unwrap();
        assert_eq!(store.load(&first), Some(b"payload".to_vec()));
        assert_eq!(store.load(&second), None);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn corruption_and_version_changes_are_safe_misses() {
        let root = root();
        let store = CacheStore::new(&root);
        let cache_key = key(b"source");
        store.store(&cache_key, b"payload").unwrap();
        let path = fs::read_dir(&root).unwrap().next().unwrap().unwrap().path();
        let mut bytes = fs::read(&path).unwrap();
        let index = bytes.len() - 1;
        bytes[index] ^= 0xFF;
        fs::write(&path, bytes).unwrap();
        assert_eq!(store.load(&cache_key), None);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn hostile_envelopes_are_bounded_safe_misses() {
        let cache_key = key(b"source");
        let key_bytes = cache_key.canonical_bytes();
        let valid = encode(&key_bytes, b"payload");

        let mut corrupt_checksum = valid.clone();
        let last = corrupt_checksum.len() - 1;
        corrupt_checksum[last] ^= 0xff;

        let mut incompatible_version = valid.clone();
        incompatible_version[MAGIC.len()] ^= 0xff;

        let header_len = MAGIC.len() + 2 + 4 + 8;
        let mut excessive_key = valid.clone();
        excessive_key[MAGIC.len() + 2..MAGIC.len() + 6].copy_from_slice(&u32::MAX.to_le_bytes());

        let payload_offset = MAGIC.len() + 2 + 4;
        let mut excessive_payload = valid.clone();
        excessive_payload[payload_offset..header_len].copy_from_slice(&u64::MAX.to_le_bytes());

        for hostile in [
            Vec::new(),
            b"not-a-cache".to_vec(),
            valid[..header_len].to_vec(),
            corrupt_checksum,
            incompatible_version,
            excessive_key,
            excessive_payload,
        ] {
            assert_eq!(decode(&hostile, &cache_key), None);
        }

        let foreign_key = key(b"different source");
        assert_eq!(decode(&valid, &foreign_key), None);
    }

    #[test]
    fn keys_include_all_version_and_profile_dimensions() {
        let base = key(b"source");
        let mut language = base.clone();
        language.language_version = [0, 2, 0];
        let mut unicode = base.clone();
        unicode.unicode_version = [16, 0, 0];
        let mut profile = base.clone();
        profile.profile = "safe".to_owned();
        let mut target = base.clone();
        target.target = "wasm".to_owned();
        assert_ne!(base.cache_id(), language.cache_id());
        assert_ne!(base.cache_id(), unicode.cache_id());
        assert_ne!(base.cache_id(), profile.cache_id());
        assert_ne!(base.cache_id(), target.cache_id());
    }
}
