use super::*;

/// Checked-source lowering result for the DEC-0262 bytecode-1.4 boundary.
///
/// This versioned wrapper prevents a 1.4 model from being encoded by an older
/// writer. Cell selection and shared mutable Handler capture lowering are
/// added by the later DEC-0262 lowering phase; until then, unsupported checked
/// inputs fail atomically through the shared lowerer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredProgramV1_4 {
    model: UnverifiedProgram,
}

impl LoweredProgramV1_4 {
    #[must_use]
    pub const fn model(&self) -> &UnverifiedProgram {
        &self.model
    }

    pub(crate) const fn new(model: UnverifiedProgram) -> Self {
        Self { model }
    }
}

/// Lowers the currently representable checked subset to a bytecode-1.4 model.
///
/// Mutable Handler captures remain failure-atomic until the explicit binding
/// storage phase emits the DEC-0262 Cell instructions.
pub fn lower_v1_4(
    snapshot: &ProgramSnapshot,
    sources: &[LoweringSource<'_>],
) -> Result<LoweredProgramV1_4, LoweringError> {
    Ok(LoweredProgramV1_4::new(super::v1_1::lower_v1_4_model(
        snapshot, sources,
    )?))
}
