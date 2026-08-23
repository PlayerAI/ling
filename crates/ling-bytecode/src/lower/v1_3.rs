use super::*;

/// Checked-source lowering result for the DEC-0261 bytecode-1.3 Handler boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredProgramV1_3 {
    model: UnverifiedProgram,
}

impl LoweredProgramV1_3 {
    #[must_use]
    pub const fn model(&self) -> &UnverifiedProgram {
        &self.model
    }

    pub(crate) const fn new(model: UnverifiedProgram) -> Self {
        Self { model }
    }
}

/// Lowers checked source to the bytecode-1.3 model.
///
/// Handler-bearing input is accepted only through this revision; earlier
/// lowering entry points retain their exact atomic rejection behavior.
pub fn lower_v1_3(
    snapshot: &ProgramSnapshot,
    sources: &[LoweringSource<'_>],
) -> Result<LoweredProgramV1_3, LoweringError> {
    Ok(LoweredProgramV1_3::new(super::v1_1::lower_v1_3_model(
        snapshot, sources,
    )?))
}
