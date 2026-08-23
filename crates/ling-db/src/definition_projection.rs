use std::error::Error;
use std::fmt;

use ling_resolve::{DefinitionId, ResolvedProgram};
use ling_source::Span;

/// Resolver definition metadata reduced to public source presentation facts.
pub(crate) struct DefinitionProjection {
    pub(crate) name: String,
    pub(crate) source_name: Option<String>,
    pub(crate) name_span: Option<Span>,
}

/// Inconsistent resolver/HIR metadata cannot be projected publicly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DefinitionProjectionError {
    MissingDefinition,
    InvalidMember,
}

impl fmt::Display for DefinitionProjectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingDefinition => formatter.write_str("definition is absent"),
            Self::InvalidMember => formatter.write_str("member metadata is inconsistent"),
        }
    }
}

impl Error for DefinitionProjectionError {}

pub(crate) fn definition_projection(
    resolved: &ResolvedProgram,
    id: &DefinitionId,
) -> Result<DefinitionProjection, DefinitionProjectionError> {
    let definition = resolved
        .definition(id)
        .ok_or(DefinitionProjectionError::MissingDefinition)?;
    if let Some(member) = resolved.trait_member(id) {
        if member.definition != *id
            || definition.source_name.as_deref() != Some(member.source_name.as_str())
            || definition.span != Some(member.span)
        {
            return Err(DefinitionProjectionError::InvalidMember);
        }
        let declaration = resolved
            .module(member.module)
            .and_then(|module| {
                module
                    .hir
                    .traits
                    .iter()
                    .find(|declaration| declaration.name.normalized == member.trait_name)
            })
            .and_then(|declaration| declaration.members.get(member.ordinal))
            .filter(|declaration| declaration.name.normalized == member.member_name)
            .ok_or(DefinitionProjectionError::InvalidMember)?;
        return Ok(DefinitionProjection {
            name: format!("{}.{}", member.trait_name, member.member_name),
            source_name: Some(member.source_name.clone()),
            name_span: Some(declaration.name.span),
        });
    }
    if let Some(member) = resolved.impl_member(id) {
        if member.definition != *id
            || definition.source_name.as_deref() != Some(member.source_name.as_str())
            || definition.span != Some(member.span)
        {
            return Err(DefinitionProjectionError::InvalidMember);
        }
        let declaration = resolved
            .module(member.module)
            .and_then(|module| module.hir.impls.get(member.impl_ordinal))
            .and_then(|implementation| implementation.members.get(member.member_ordinal))
            .filter(|declaration| declaration.name.normalized == member.member_name)
            .ok_or(DefinitionProjectionError::InvalidMember)?;
        return Ok(DefinitionProjection {
            name: member.member_name.clone(),
            source_name: Some(member.source_name.clone()),
            name_span: Some(declaration.name.span),
        });
    }
    Ok(DefinitionProjection {
        name: definition.name.clone(),
        source_name: definition.source_name.clone(),
        name_span: definition.span,
    })
}
