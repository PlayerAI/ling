use super::*;

/// Checked-source lowering result for the version-1.2 aggregate boundary.
///
/// The model is kept separate from the 1.1 wrapper so callers cannot
/// accidentally encode a 1.2 artifact through a 1.1 entry point.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredProgramV1_2 {
    model: UnverifiedProgram,
}

impl LoweredProgramV1_2 {
    #[must_use]
    pub const fn model(&self) -> &UnverifiedProgram {
        &self.model
    }

    pub(crate) const fn new(model: UnverifiedProgram) -> Self {
        Self { model }
    }
}

/// Lowers the checked core to the version-1.2 model.
pub fn lower_v1_2(
    snapshot: &ProgramSnapshot,
    sources: &[LoweringSource<'_>],
) -> Result<LoweredProgramV1_2, LoweringError> {
    Ok(LoweredProgramV1_2::new(super::v1_1::lower_v1_2_model(
        snapshot, sources,
    )?))
}
