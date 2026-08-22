use super::diagnostics::DiagnosticOrderKey;

/// Opaque, ordered diagnostic work item for the internal LSP batch boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DiagnosticItem {
    key: DiagnosticOrderKey,
    id: u64,
}

impl DiagnosticItem {
    #[must_use]
    pub(crate) fn new(key: DiagnosticOrderKey, id: u64) -> Self {
        Self { key, id }
    }

    #[must_use]
    pub(crate) fn key(&self) -> &DiagnosticOrderKey {
        &self.key
    }

    #[must_use]
    pub(crate) const fn id(&self) -> u64 {
        self.id
    }
}

/// Immutable-result boundary for internal diagnostic collection.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DiagnosticBatch {
    items: Vec<DiagnosticItem>,
}

impl DiagnosticBatch {
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self { items: Vec::new() }
    }

    #[must_use]
    pub(crate) fn len(&self) -> usize {
        self.items.len()
    }

    pub(crate) fn push(&mut self, item: DiagnosticItem) {
        self.items.push(item);
    }

    /// Consumes the mutable collection and returns canonical immutable output.
    pub(crate) fn finish(mut self) -> Box<[DiagnosticItem]> {
        self.items.sort_by(|left, right| left.key.cmp(&right.key));
        self.items.into_boxed_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(file: &str, start: u64, id: u64) -> DiagnosticItem {
        DiagnosticItem::new(
            DiagnosticOrderKey::new(file, start, start + 1, "L-LEX-0001", id),
            id,
        )
    }

    #[test]
    fn empty_batch_finishes_without_partial_state() {
        let batch = DiagnosticBatch::new();
        assert_eq!(batch.len(), 0);
        assert!(batch.finish().is_empty());
    }

    #[test]
    fn finish_orders_ids_and_preserves_equal_keys() {
        let mut batch = DiagnosticBatch::new();
        batch.push(item("z.ling", 0, 3));
        batch.push(item("a.ling", 4, 2));
        batch.push(item("a.ling", 1, 1));
        batch.push(item("a.ling", 1, 4));
        assert_eq!(batch.len(), 4);
        let items = batch.finish();
        assert_eq!(
            items.iter().map(DiagnosticItem::id).collect::<Vec<_>>(),
            vec![1, 4, 2, 3]
        );
        assert_eq!(items[0].key().start_byte(), 1);
        assert_eq!(items[1].key().tie_breaker(), 4);
    }

    #[test]
    fn repeated_batches_have_identical_immutable_output() {
        let mut left = DiagnosticBatch::new();
        let mut right = DiagnosticBatch::new();
        for (file, start, id) in [("凌.ling", 8, 1), ("main.ling", 5, 2)] {
            left.push(item(file, start, id));
            right.push(item(file, start, id));
        }
        assert_eq!(left.finish(), right.finish());
    }
}
