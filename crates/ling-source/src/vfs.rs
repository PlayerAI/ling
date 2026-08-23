//! Deterministic in-memory VFS and revision tracking for compiler services.
//!
//! The VFS stores exact caller-provided bytes. It does not read the host file
//! system, normalize paths, or publish a cache format. Parsing and source-span
//! projection remain owned by [`crate::SourceFile`].

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::SourceId;

/// A session-local immutable revision identifier.
///
/// Revisions are intentionally scoped to one [`VirtualFileSystem`]. They are
/// not Semantic IDs, serialized cache keys, or a promise about cross-process
/// numbering.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Revision(u64);

impl Revision {
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// The visible source layer for a file.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FileOrigin {
    Disk,
    Overlay,
}

/// A source file snapshot whose bytes cannot change after publication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileSnapshot {
    id: SourceId,
    revision: Revision,
    logical_name: String,
    bytes: Arc<[u8]>,
    origin: FileOrigin,
}

impl FileSnapshot {
    #[must_use]
    pub const fn id(&self) -> SourceId {
        self.id
    }

    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    #[must_use]
    pub fn logical_name(&self) -> &str {
        &self.logical_name
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub const fn origin(&self) -> FileOrigin {
        self.origin
    }
}

/// Non-source compiler input that participates in an internal query key.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorkspaceInput {
    PackageManifest,
    PackageLock,
    Config,
    Profile,
    Target,
}

/// An immutable workspace-input snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkspaceSnapshot {
    kind: WorkspaceInput,
    revision: Revision,
    bytes: Arc<[u8]>,
}

impl WorkspaceSnapshot {
    #[must_use]
    pub const fn kind(&self) -> WorkspaceInput {
        self.kind
    }

    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// An immutable, deterministic capture of the visible workspace state.
///
/// This is an in-process compiler boundary only. It contains the visible
/// source layer, non-source workspace inputs, and the session-local revision
/// high-water mark; it does not represent an LSP workspace notification,
/// filesystem watcher event, or serialized protocol message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkspaceStateSnapshot {
    revision: Revision,
    files: Box<[FileSnapshot]>,
    inputs: Box<[WorkspaceSnapshot]>,
}

impl WorkspaceStateSnapshot {
    /// Returns the session-local revision observed for this capture.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns visible source files in canonical logical-name order.
    #[must_use]
    pub fn files(&self) -> &[FileSnapshot] {
        &self.files
    }

    /// Returns present workspace inputs in their canonical enum order.
    #[must_use]
    pub fn inputs(&self) -> &[WorkspaceSnapshot] {
        &self.inputs
    }

    /// Finds a captured source file by its session-local identity.
    #[must_use]
    pub fn file(&self, id: SourceId) -> Option<&FileSnapshot> {
        self.files.iter().find(|snapshot| snapshot.id() == id)
    }

    /// Finds a captured workspace input by kind.
    #[must_use]
    pub fn input(&self, kind: WorkspaceInput) -> Option<&WorkspaceSnapshot> {
        self.inputs.iter().find(|snapshot| snapshot.kind() == kind)
    }
}

/// The visible consequence of a file update.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeEvent {
    Added {
        file: SourceId,
        revision: Revision,
        origin: FileOrigin,
    },
    Changed {
        file: SourceId,
        previous: Revision,
        current: Revision,
        origin: FileOrigin,
    },
    /// The requested layer changed, but the visible bytes did not. This is
    /// emitted for duplicate updates and disk updates hidden by an overlay.
    Unchanged {
        file: SourceId,
        revision: Revision,
        origin: FileOrigin,
    },
}

/// The visible consequence of a workspace-input update.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputChange {
    Added {
        kind: WorkspaceInput,
        revision: Revision,
    },
    Changed {
        kind: WorkspaceInput,
        previous: Revision,
        current: Revision,
    },
    Unchanged {
        kind: WorkspaceInput,
        revision: Revision,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Layer {
    revision: Revision,
    bytes: Arc<[u8]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FileEntry {
    logical_name: String,
    disk: Layer,
    overlay: Option<Layer>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InputEntry {
    revision: Revision,
    bytes: Arc<[u8]>,
}

/// Errors raised by the session-local VFS.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VfsError {
    EmptyLogicalName,
    NonCanonicalLogicalName { name: String },
    UnknownFile { file: SourceId },
    NoOpenOverlay { file: SourceId },
    OpenOverlay { file: SourceId },
    FileIdExhausted,
    RevisionExhausted,
}

impl fmt::Display for VfsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLogicalName => formatter.write_str("logical source name is empty"),
            Self::NonCanonicalLogicalName { name } => {
                write!(formatter, "logical source name is not canonical: {name:?}")
            }
            Self::UnknownFile { file } => write!(formatter, "unknown source file {}", file.get()),
            Self::NoOpenOverlay { file } => {
                write!(formatter, "source file {} has no open overlay", file.get())
            }
            Self::OpenOverlay { file } => {
                write!(formatter, "source file {} has an open overlay", file.get())
            }
            Self::FileIdExhausted => formatter.write_str("source file ID space is exhausted"),
            Self::RevisionExhausted => formatter.write_str("revision space is exhausted"),
        }
    }
}

impl Error for VfsError {}

/// A host-independent, session-local virtual file system.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VirtualFileSystem {
    next_file_id: u32,
    next_revision: u64,
    files: BTreeMap<SourceId, FileEntry>,
    names: BTreeMap<String, SourceId>,
    inputs: BTreeMap<WorkspaceInput, InputEntry>,
}

impl VirtualFileSystem {
    /// Creates an empty VFS with deterministic ID and revision allocation.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            next_file_id: 0,
            next_revision: 0,
            files: BTreeMap::new(),
            names: BTreeMap::new(),
            inputs: BTreeMap::new(),
        }
    }

    /// Publishes or replaces the disk snapshot for a logical source name.
    pub fn set_disk_snapshot(
        &mut self,
        logical_name: impl Into<String>,
        bytes: Vec<u8>,
    ) -> Result<ChangeEvent, VfsError> {
        let logical_name = logical_name.into();
        validate_logical_name(&logical_name)?;
        if let Some(file) = self.names.get(&logical_name).copied() {
            return self.update_disk(file, bytes);
        }

        let next_file_id = self
            .next_file_id
            .checked_add(1)
            .ok_or(VfsError::FileIdExhausted)?;
        let revision = self.next_revision()?;
        let file = SourceId::new(self.next_file_id);
        self.next_file_id = next_file_id;
        let entry = FileEntry {
            logical_name: logical_name.clone(),
            disk: Layer {
                revision,
                bytes: bytes.into(),
            },
            overlay: None,
        };
        self.files.insert(file, entry);
        self.names.insert(logical_name, file);
        Ok(ChangeEvent::Added {
            file,
            revision,
            origin: FileOrigin::Disk,
        })
    }

    /// Opens or replaces an in-memory editor overlay for a file.
    pub fn open_overlay(
        &mut self,
        file: SourceId,
        bytes: Vec<u8>,
    ) -> Result<ChangeEvent, VfsError> {
        let (previous, current_bytes) = {
            let entry = self
                .files
                .get(&file)
                .ok_or(VfsError::UnknownFile { file })?;
            let current = visible_layer(entry);
            (current.revision, current.bytes.clone())
        };
        if current_bytes.as_ref() == bytes.as_slice() {
            let entry = self
                .files
                .get_mut(&file)
                .ok_or(VfsError::UnknownFile { file })?;
            entry.overlay = Some(Layer {
                revision: previous,
                bytes: current_bytes,
            });
            return Ok(ChangeEvent::Unchanged {
                file,
                revision: previous,
                origin: FileOrigin::Overlay,
            });
        }

        let revision = self.next_revision()?;
        let entry = self
            .files
            .get_mut(&file)
            .ok_or(VfsError::UnknownFile { file })?;
        entry.overlay = Some(Layer {
            revision,
            bytes: bytes.into(),
        });
        Ok(ChangeEvent::Changed {
            file,
            previous,
            current: revision,
            origin: FileOrigin::Overlay,
        })
    }

    /// Closes an editor overlay and reveals the latest disk snapshot.
    pub fn close_overlay(&mut self, file: SourceId) -> Result<ChangeEvent, VfsError> {
        let entry = self
            .files
            .get_mut(&file)
            .ok_or(VfsError::UnknownFile { file })?;
        let overlay = entry
            .overlay
            .take()
            .ok_or(VfsError::NoOpenOverlay { file })?;
        if overlay.bytes == entry.disk.bytes {
            return Ok(ChangeEvent::Unchanged {
                file,
                revision: entry.disk.revision,
                origin: FileOrigin::Disk,
            });
        }
        Ok(ChangeEvent::Changed {
            file,
            previous: overlay.revision,
            current: entry.disk.revision,
            origin: FileOrigin::Disk,
        })
    }

    /// Removes a session-local file and its disk/overlay layers.
    ///
    /// Source IDs are never reused. This is intended for temporary editor
    /// documents whose `didClose` lifecycle has no disk layer to reveal.
    pub fn remove_file(&mut self, file: SourceId) -> Result<(), VfsError> {
        let entry = self
            .files
            .remove(&file)
            .ok_or(VfsError::UnknownFile { file })?;
        self.names.remove(&entry.logical_name);
        Ok(())
    }

    /// Removes one closed disk source and advances the workspace revision.
    ///
    /// Open overlays are rejected so a host reload cannot discard unsaved
    /// editor bytes. Source IDs and revisions are never reused.
    pub fn remove_disk_snapshot(&mut self, file: SourceId) -> Result<Revision, VfsError> {
        let entry = self
            .files
            .get(&file)
            .ok_or(VfsError::UnknownFile { file })?;
        if entry.overlay.is_some() {
            return Err(VfsError::OpenOverlay { file });
        }
        let logical_name = entry.logical_name.clone();
        let revision = self.next_revision()?;
        self.files
            .remove(&file)
            .ok_or(VfsError::UnknownFile { file })?;
        self.names.remove(&logical_name);
        Ok(revision)
    }

    /// Returns the currently visible immutable snapshot.
    #[must_use]
    pub fn snapshot(&self, file: SourceId) -> Option<FileSnapshot> {
        let entry = self.files.get(&file)?;
        let (layer, origin) = visible_layer_with_origin(entry);
        Some(FileSnapshot {
            id: file,
            revision: layer.revision,
            logical_name: entry.logical_name.clone(),
            bytes: layer.bytes.clone(),
            origin,
        })
    }

    /// Returns the immutable disk layer even when an overlay is open.
    #[must_use]
    pub fn disk_snapshot(&self, file: SourceId) -> Option<FileSnapshot> {
        let entry = self.files.get(&file)?;
        Some(FileSnapshot {
            id: file,
            revision: entry.disk.revision,
            logical_name: entry.logical_name.clone(),
            bytes: entry.disk.bytes.clone(),
            origin: FileOrigin::Disk,
        })
    }

    /// Looks up a file ID by its canonical logical name.
    #[must_use]
    pub fn file_id(&self, logical_name: &str) -> Option<SourceId> {
        self.names.get(logical_name).copied()
    }

    /// Returns all visible snapshots in canonical logical-name order.
    #[must_use]
    pub fn snapshots(&self) -> Vec<FileSnapshot> {
        let mut snapshots = self
            .files
            .keys()
            .filter_map(|file| self.snapshot(*file))
            .collect::<Vec<_>>();
        snapshots.sort_by(|left, right| {
            left.logical_name()
                .cmp(right.logical_name())
                .then_with(|| left.id().cmp(&right.id()))
        });
        snapshots
    }

    /// Captures the visible files and workspace inputs atomically from this
    /// immutable VFS view.
    ///
    /// The returned collections are canonical and owned, so later VFS
    /// mutations cannot change a previously captured workspace state. This is
    /// deliberately an internal snapshot boundary; it does not publish a
    /// reload notification or prescribe watcher behavior.
    #[must_use]
    pub fn workspace_snapshot(&self) -> WorkspaceStateSnapshot {
        let files = self.snapshots().into_boxed_slice();
        let inputs = self
            .inputs
            .iter()
            .map(|(&kind, entry)| WorkspaceSnapshot {
                kind,
                revision: entry.revision,
                bytes: entry.bytes.clone(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        WorkspaceStateSnapshot {
            revision: self.revision(),
            files,
            inputs,
        }
    }

    /// Returns the latest session-local revision allocated by this VFS.
    ///
    /// The value is an internal observation boundary for immutable request
    /// capture. It is not a client document version, Semantic ID, serialized
    /// cache key, or cross-process identity.
    #[must_use]
    pub const fn revision(&self) -> Revision {
        Revision(self.next_revision)
    }

    /// Publishes a package manifest, config, profile, or target revision.
    pub fn set_workspace_input(
        &mut self,
        kind: WorkspaceInput,
        bytes: Vec<u8>,
    ) -> Result<InputChange, VfsError> {
        if let Some(previous) = self
            .inputs
            .get(&kind)
            .and_then(|entry| (entry.bytes.as_ref() == bytes.as_slice()).then_some(entry.revision))
        {
            return Ok(InputChange::Unchanged {
                kind,
                revision: previous,
            });
        }
        if let Some(previous) = self.inputs.get(&kind).map(|entry| entry.revision) {
            let current = self.next_revision()?;
            let entry = self
                .inputs
                .get_mut(&kind)
                .ok_or(VfsError::RevisionExhausted)?;
            entry.revision = current;
            entry.bytes = bytes.into();
            return Ok(InputChange::Changed {
                kind,
                previous,
                current,
            });
        }

        let revision = self.next_revision()?;
        self.inputs.insert(
            kind,
            InputEntry {
                revision,
                bytes: bytes.into(),
            },
        );
        Ok(InputChange::Added { kind, revision })
    }

    /// Removes one workspace input and advances the revision when it existed.
    pub fn remove_workspace_input(
        &mut self,
        kind: WorkspaceInput,
    ) -> Result<Option<Revision>, VfsError> {
        if !self.inputs.contains_key(&kind) {
            return Ok(None);
        }
        let revision = self.next_revision()?;
        self.inputs.remove(&kind);
        Ok(Some(revision))
    }

    /// Returns an immutable workspace-input snapshot.
    #[must_use]
    pub fn workspace_input(&self, kind: WorkspaceInput) -> Option<WorkspaceSnapshot> {
        let entry = self.inputs.get(&kind)?;
        Some(WorkspaceSnapshot {
            kind,
            revision: entry.revision,
            bytes: entry.bytes.clone(),
        })
    }

    fn update_disk(&mut self, file: SourceId, bytes: Vec<u8>) -> Result<ChangeEvent, VfsError> {
        let (disk_revision, disk_bytes, visible_revision, visible_origin) = {
            let entry = self
                .files
                .get(&file)
                .ok_or(VfsError::UnknownFile { file })?;
            let (layer, origin) = visible_layer_with_origin(entry);
            (
                entry.disk.revision,
                entry.disk.bytes.clone(),
                layer.revision,
                origin,
            )
        };
        if disk_bytes.as_ref() == bytes.as_slice() {
            return Ok(ChangeEvent::Unchanged {
                file,
                revision: visible_revision,
                origin: visible_origin,
            });
        }

        let revision = self.next_revision()?;
        let entry = self
            .files
            .get_mut(&file)
            .ok_or(VfsError::UnknownFile { file })?;
        entry.disk = Layer {
            revision,
            bytes: bytes.into(),
        };
        if visible_origin == FileOrigin::Overlay {
            return Ok(ChangeEvent::Unchanged {
                file,
                revision: visible_revision,
                origin: visible_origin,
            });
        }
        Ok(ChangeEvent::Changed {
            file,
            previous: disk_revision,
            current: revision,
            origin: FileOrigin::Disk,
        })
    }

    fn next_revision(&mut self) -> Result<Revision, VfsError> {
        self.next_revision = self
            .next_revision
            .checked_add(1)
            .ok_or(VfsError::RevisionExhausted)?;
        Ok(Revision(self.next_revision))
    }
}

fn visible_layer(entry: &FileEntry) -> &Layer {
    entry.overlay.as_ref().unwrap_or(&entry.disk)
}

fn visible_layer_with_origin(entry: &FileEntry) -> (&Layer, FileOrigin) {
    entry
        .overlay
        .as_ref()
        .map_or((&entry.disk, FileOrigin::Disk), |layer| {
            (layer, FileOrigin::Overlay)
        })
}

/// Validates a path-free logical source name used by the session VFS.
pub fn validate_logical_name(name: &str) -> Result<(), VfsError> {
    if name.is_empty() {
        return Err(VfsError::EmptyLogicalName);
    }
    let invalid = name.starts_with('/')
        || name.ends_with('/')
        || name.contains('\\')
        || name.contains(':')
        || name.contains('\0')
        || name
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..");
    if invalid {
        return Err(VfsError::NonCanonicalLogicalName {
            name: name.to_owned(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disk_snapshots_are_immutable_and_duplicate_updates_are_deduplicated() {
        let mut vfs = VirtualFileSystem::new();
        let added = vfs
            .set_disk_snapshot("src/Main.ling", b"let main () = ()".to_vec())
            .unwrap();
        let file = match added {
            ChangeEvent::Added { file, .. } => file,
            other => panic!("expected Added, got {other:?}"),
        };
        let first = vfs.snapshot(file).unwrap();
        let unchanged = vfs
            .set_disk_snapshot("src/Main.ling", b"let main () = ()".to_vec())
            .unwrap();
        assert!(matches!(
            unchanged,
            ChangeEvent::Unchanged {
                file: same,
                revision,
                origin: FileOrigin::Disk
            } if same == file && revision == first.revision()
        ));
        assert_eq!(vfs.snapshot(file).unwrap(), first);
    }

    #[test]
    fn overlay_hides_disk_changes_until_close_and_reveals_latest_disk_bytes() {
        let mut vfs = VirtualFileSystem::new();
        let file = match vfs
            .set_disk_snapshot("src/Main.ling", b"disk-1".to_vec())
            .unwrap()
        {
            ChangeEvent::Added { file, .. } => file,
            other => panic!("expected Added, got {other:?}"),
        };
        let overlay = vfs.open_overlay(file, b"editor".to_vec()).unwrap();
        assert!(matches!(
            overlay,
            ChangeEvent::Changed {
                origin: FileOrigin::Overlay,
                ..
            }
        ));
        let hidden = vfs
            .set_disk_snapshot("src/Main.ling", b"disk-2".to_vec())
            .unwrap();
        assert!(matches!(
            hidden,
            ChangeEvent::Unchanged {
                origin: FileOrigin::Overlay,
                ..
            }
        ));
        assert_eq!(vfs.snapshot(file).unwrap().bytes(), b"editor");
        assert_eq!(vfs.disk_snapshot(file).unwrap().bytes(), b"disk-2");
        let closed = vfs.close_overlay(file).unwrap();
        assert!(matches!(
            closed,
            ChangeEvent::Changed {
                origin: FileOrigin::Disk,
                ..
            }
        ));
        assert_eq!(vfs.snapshot(file).unwrap().bytes(), b"disk-2");
    }

    #[test]
    fn identical_workspace_inputs_reuse_their_revision() {
        let mut vfs = VirtualFileSystem::new();
        let first = vfs
            .set_workspace_input(WorkspaceInput::PackageManifest, b"manifest".to_vec())
            .unwrap();
        let revision = match first {
            InputChange::Added { revision, .. } => revision,
            other => panic!("expected Added, got {other:?}"),
        };
        let second = vfs
            .set_workspace_input(WorkspaceInput::PackageManifest, b"manifest".to_vec())
            .unwrap();
        assert!(matches!(
            second,
            InputChange::Unchanged {
                kind: WorkspaceInput::PackageManifest,
                revision: same
            } if same == revision
        ));
        assert_eq!(
            vfs.workspace_input(WorkspaceInput::PackageManifest)
                .unwrap()
                .bytes(),
            b"manifest"
        );
    }

    #[test]
    fn snapshots_and_names_are_deterministic_and_names_are_canonical() {
        let mut vfs = VirtualFileSystem::new();
        vfs.set_disk_snapshot("z/Main.ling", b"z".to_vec()).unwrap();
        vfs.set_disk_snapshot("a/Main.ling", b"a".to_vec()).unwrap();
        let names = vfs
            .snapshots()
            .into_iter()
            .map(|snapshot| snapshot.logical_name().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(names, ["a/Main.ling", "z/Main.ling"]);
        for invalid in [
            "",
            "/Main.ling",
            "src\\Main.ling",
            "src//Main.ling",
            "src/../Main.ling",
            "C:/Main.ling",
            "C:Main.ling",
        ] {
            assert!(matches!(
                vfs.set_disk_snapshot(invalid, Vec::new()),
                Err(VfsError::EmptyLogicalName) | Err(VfsError::NonCanonicalLogicalName { .. })
            ));
        }
    }

    #[test]
    fn temporary_file_removal_does_not_reuse_source_ids() {
        let mut vfs = VirtualFileSystem::new();
        let first = match vfs.set_disk_snapshot("untitled/Buffer.ling", b"one".to_vec()) {
            Ok(ChangeEvent::Added { file, .. }) => file,
            other => panic!("expected Added, got {other:?}"),
        };
        vfs.remove_file(first).expect("temporary file exists");
        assert!(vfs.snapshot(first).is_none());
        let second = match vfs.set_disk_snapshot("untitled/Buffer.ling", b"two".to_vec()) {
            Ok(ChangeEvent::Added { file, .. }) => file,
            other => panic!("expected Added, got {other:?}"),
        };
        assert!(second > first);
    }

    #[test]
    fn reload_removals_are_revisioned_and_preserve_open_overlays() {
        let mut vfs = VirtualFileSystem::new();
        let closed = match vfs
            .set_disk_snapshot("src/Closed.ling", b"closed".to_vec())
            .unwrap()
        {
            ChangeEvent::Added { file, .. } => file,
            other => panic!("expected Added, got {other:?}"),
        };
        let open = match vfs
            .set_disk_snapshot("src/Open.ling", b"disk".to_vec())
            .unwrap()
        {
            ChangeEvent::Added { file, .. } => file,
            other => panic!("expected Added, got {other:?}"),
        };
        vfs.open_overlay(open, b"editor".to_vec()).unwrap();
        vfs.set_workspace_input(WorkspaceInput::PackageLock, b"lock".to_vec())
            .unwrap();

        let before_closed_removal = vfs.revision();
        let removed_at = vfs
            .remove_disk_snapshot(closed)
            .expect("closed source can be removed");
        assert!(removed_at > before_closed_removal);
        assert!(vfs.snapshot(closed).is_none());
        assert!(matches!(
            vfs.remove_disk_snapshot(open),
            Err(VfsError::OpenOverlay { file }) if file == open
        ));
        assert_eq!(vfs.snapshot(open).unwrap().bytes(), b"editor");

        let removed_input_at = vfs
            .remove_workspace_input(WorkspaceInput::PackageLock)
            .expect("input removal succeeds")
            .expect("input existed");
        assert!(removed_input_at > removed_at);
        let revision = vfs.revision();
        assert_eq!(
            vfs.remove_workspace_input(WorkspaceInput::PackageLock)
                .expect("absent input removal is a no-op"),
            None
        );
        assert_eq!(vfs.revision(), revision);
    }

    #[test]
    fn workspace_state_snapshot_is_canonical_and_immutable() {
        let mut vfs = VirtualFileSystem::new();
        let z_file = match vfs
            .set_disk_snapshot("z/Main.ling", b"disk".to_vec())
            .unwrap()
        {
            ChangeEvent::Added { file, .. } => file,
            other => panic!("expected Added, got {other:?}"),
        };
        let a_file = match vfs.set_disk_snapshot("a/Main.ling", b"a".to_vec()).unwrap() {
            ChangeEvent::Added { file, .. } => file,
            other => panic!("expected Added, got {other:?}"),
        };
        vfs.open_overlay(z_file, b"editor".to_vec()).unwrap();
        vfs.set_workspace_input(WorkspaceInput::Config, b"config".to_vec())
            .unwrap();
        vfs.set_workspace_input(WorkspaceInput::PackageManifest, b"manifest".to_vec())
            .unwrap();

        let captured = vfs.workspace_snapshot();
        assert_eq!(captured.revision(), vfs.revision());
        assert_eq!(
            captured
                .files()
                .iter()
                .map(FileSnapshot::logical_name)
                .collect::<Vec<_>>(),
            ["a/Main.ling", "z/Main.ling"]
        );
        assert_eq!(captured.file(z_file).unwrap().bytes(), b"editor");
        assert_eq!(captured.file(a_file).unwrap().bytes(), b"a");
        assert_eq!(
            captured
                .inputs()
                .iter()
                .map(WorkspaceSnapshot::kind)
                .collect::<Vec<_>>(),
            [WorkspaceInput::PackageManifest, WorkspaceInput::Config]
        );
        assert_eq!(
            captured
                .input(WorkspaceInput::PackageManifest)
                .unwrap()
                .bytes(),
            b"manifest"
        );

        let expected = captured.clone();
        vfs.set_disk_snapshot("b/Main.ling", b"b".to_vec()).unwrap();
        vfs.set_workspace_input(WorkspaceInput::Profile, b"profile".to_vec())
            .unwrap();
        assert_eq!(captured, expected);
    }
}
