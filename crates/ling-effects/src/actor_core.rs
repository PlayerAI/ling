//! Checked-only Actor declaration projection authorized by DEC-0270.

use std::collections::BTreeMap;

use ling_concurrency::ActorTypeId;
use ling_hir as hir;
use ling_resolve::{BindingKey, DefinitionId, ExpressionKey};
use ling_source::Span;
use ling_types::{Type, TypeId, TypedProgram};

use crate::EffectRow;

pub const CHECKED_ACTOR_CORE_VERSION: &str = "ling.checked-actor-core/0";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckedActorIdContract;

impl CheckedActorIdContract {
    #[must_use]
    pub const fn is_runtime_scoped(self) -> bool {
        true
    }

    #[must_use]
    pub const fn requires_nonzero_unique_nonreusable_ids(self) -> bool {
        true
    }

    #[must_use]
    pub const fn allocates_instances(self) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckedActorSourceSpans {
    pub declaration: Span,
    pub actor_keyword: Span,
    pub message_type: Span,
    pub state_clause: Span,
    pub state_keyword: Span,
    pub state_type: Span,
    pub initializer: Span,
    pub receive_clause: Span,
    pub receive_keyword: Span,
    pub state_pattern: Span,
    pub message_pattern: Span,
    pub transition_body: Span,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckedActorRefType {
    actor_type: ActorTypeId,
    message: TypeId,
}

impl CheckedActorRefType {
    #[must_use]
    pub const fn actor_type(self) -> ActorTypeId {
        self.actor_type
    }

    #[must_use]
    pub const fn message(self) -> TypeId {
        self.message
    }

    #[must_use]
    pub const fn is_local_and_invariant(self) -> bool {
        true
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedActorCore {
    definition: DefinitionId,
    actor_type: ActorTypeId,
    message_type: TypeId,
    state_type: TypeId,
    reference_type: CheckedActorRefType,
    actor_id_contract: CheckedActorIdContract,
    initializer: ExpressionKey,
    transition_body: ExpressionKey,
    state_binding: BindingKey,
    message_binding: BindingKey,
    effects: EffectRow,
    source_spans: CheckedActorSourceSpans,
    canonical_bytes: Box<[u8]>,
}

impl CheckedActorCore {
    #[must_use]
    pub fn definition(&self) -> &DefinitionId {
        &self.definition
    }

    #[must_use]
    pub const fn actor_type(&self) -> ActorTypeId {
        self.actor_type
    }

    #[must_use]
    pub const fn message_type(&self) -> TypeId {
        self.message_type
    }

    #[must_use]
    pub const fn state_type(&self) -> TypeId {
        self.state_type
    }

    #[must_use]
    pub const fn reference_type(&self) -> CheckedActorRefType {
        self.reference_type
    }

    #[must_use]
    pub const fn actor_id_contract(&self) -> CheckedActorIdContract {
        self.actor_id_contract
    }

    #[must_use]
    pub const fn initializer(&self) -> ExpressionKey {
        self.initializer
    }

    #[must_use]
    pub const fn transition_body(&self) -> ExpressionKey {
        self.transition_body
    }

    #[must_use]
    pub const fn state_binding(&self) -> BindingKey {
        self.state_binding
    }

    #[must_use]
    pub const fn message_binding(&self) -> BindingKey {
        self.message_binding
    }

    #[must_use]
    pub const fn effects(&self) -> &EffectRow {
        &self.effects
    }

    #[must_use]
    pub const fn source_spans(&self) -> CheckedActorSourceSpans {
        self.source_spans
    }

    #[must_use]
    pub const fn source_span(&self) -> Span {
        self.source_spans.declaration
    }

    /// Deterministic internal evidence. This is not a public serialization schema.
    #[must_use]
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ActorCheckFailure {
    pub source_name: String,
    pub span: Span,
    pub reason: &'static str,
    pub actor: Option<String>,
}

pub(crate) fn build_checked_actor_cores(
    typed: &TypedProgram,
    definition_effects: &BTreeMap<DefinitionId, EffectRow>,
    expression_effects: &BTreeMap<ExpressionKey, EffectRow>,
) -> Result<BTreeMap<DefinitionId, CheckedActorCore>, Vec<ActorCheckFailure>> {
    let mut output = BTreeMap::new();
    let mut identities = BTreeMap::<ActorTypeId, DefinitionId>::new();
    let mut failures = Vec::new();

    for module in typed.resolved().modules() {
        for declaration in &module.hir.actors {
            let actor_name = Some(declaration.name.normalized.clone());
            let Some(definition) = typed
                .resolved()
                .definition_id(module.id, &declaration.name.normalized)
                .cloned()
            else {
                failures.push(failure(
                    module,
                    declaration.name.span,
                    "unresolved_actor_definition",
                    actor_name,
                ));
                continue;
            };
            let actor_type = stable_actor_type_id(&definition);
            if identities
                .insert(actor_type, definition.clone())
                .is_some_and(|existing| existing != definition)
            {
                failures.push(failure(
                    module,
                    declaration.name.span,
                    "actor_type_identity_collision",
                    actor_name,
                ));
                continue;
            }
            match build_one(
                typed,
                definition_effects,
                expression_effects,
                module,
                declaration,
                definition.clone(),
                actor_type,
            ) {
                Ok(core) => {
                    output.insert(definition, core);
                }
                Err(error) => failures.push(error),
            }
        }
    }

    if failures.is_empty() {
        Ok(output)
    } else {
        failures.sort_by_key(|failure| {
            (
                failure.source_name.clone(),
                failure.span.start().get(),
                failure.reason,
            )
        });
        Err(failures)
    }
}

fn build_one(
    typed: &TypedProgram,
    definition_effects: &BTreeMap<DefinitionId, EffectRow>,
    expression_effects: &BTreeMap<ExpressionKey, EffectRow>,
    module: &ling_resolve::ResolvedModule,
    declaration: &hir::ActorDeclaration,
    definition: DefinitionId,
    actor_type: ActorTypeId,
) -> Result<CheckedActorCore, ActorCheckFailure> {
    let Some(signature) = typed.definition_type(&definition) else {
        return Err(failure(
            module,
            declaration.name.span,
            "missing_actor_type",
            Some(declaration.name.normalized.clone()),
        ));
    };
    let Type::Actor {
        definition: type_definition,
        message,
        state,
    } = typed.arena().get(signature)
    else {
        return Err(failure(
            module,
            declaration.name.span,
            "invalid_actor_type",
            Some(declaration.name.normalized.clone()),
        ));
    };
    if type_definition != &definition {
        return Err(failure(
            module,
            declaration.name.span,
            "actor_type_definition_mismatch",
            Some(declaration.name.normalized.clone()),
        ));
    }

    let initializer = ExpressionKey::new(module.id, declaration.state.initializer.id);
    let transition_body = ExpressionKey::new(module.id, declaration.receive.body.id);
    let state_binding = binding(module, &declaration.receive.state_pattern).ok_or_else(|| {
        failure(
            module,
            declaration.receive.state_pattern.span,
            "state_pattern_must_bind_one_name",
            Some(declaration.name.normalized.clone()),
        )
    })?;
    let message_binding =
        binding(module, &declaration.receive.message_pattern).ok_or_else(|| {
            failure(
                module,
                declaration.receive.message_pattern.span,
                "message_pattern_must_bind_one_name",
                Some(declaration.name.normalized.clone()),
            )
        })?;

    for (actual, expected, span, reason) in [
        (
            typed.expression_type(initializer),
            Some(*state),
            declaration.state.initializer.span,
            "initializer_type_mismatch",
        ),
        (
            typed.expression_type(transition_body),
            Some(*state),
            declaration.receive.body.span,
            "transition_result_type_mismatch",
        ),
        (
            typed.binding_type(state_binding),
            Some(*state),
            declaration.receive.state_pattern.span,
            "state_binding_type_mismatch",
        ),
        (
            typed.binding_type(message_binding),
            Some(*message),
            declaration.receive.message_pattern.span,
            "message_binding_type_mismatch",
        ),
    ] {
        if actual != expected {
            return Err(failure(
                module,
                span,
                reason,
                Some(declaration.name.normalized.clone()),
            ));
        }
    }

    let pure_definition = definition_effects
        .get(&definition)
        .is_some_and(EffectRow::is_pure);
    let pure_initializer = expression_effects
        .get(&initializer)
        .is_some_and(EffectRow::is_pure);
    let pure_transition = expression_effects
        .get(&transition_body)
        .is_some_and(EffectRow::is_pure);
    if !(pure_definition && pure_initializer && pure_transition) {
        return Err(failure(
            module,
            declaration.span,
            "actor_transition_must_have_empty_residual_effect_row",
            Some(declaration.name.normalized.clone()),
        ));
    }

    let canonical_bytes = canonical_bytes(
        typed,
        &definition,
        actor_type,
        *message,
        *state,
        initializer,
        transition_body,
        state_binding,
        message_binding,
    );
    Ok(CheckedActorCore {
        definition,
        actor_type,
        message_type: *message,
        state_type: *state,
        reference_type: CheckedActorRefType {
            actor_type,
            message: *message,
        },
        actor_id_contract: CheckedActorIdContract,
        initializer,
        transition_body,
        state_binding,
        message_binding,
        effects: EffectRow::default(),
        source_spans: CheckedActorSourceSpans {
            declaration: declaration.span,
            actor_keyword: declaration.keyword_span,
            message_type: declaration.message_type.span,
            state_clause: declaration.state.span,
            state_keyword: declaration.state.keyword_span,
            state_type: declaration.state.state_type.span,
            initializer: declaration.state.initializer.span,
            receive_clause: declaration.receive.span,
            receive_keyword: declaration.receive.keyword_span,
            state_pattern: declaration.receive.state_pattern.span,
            message_pattern: declaration.receive.message_pattern.span,
            transition_body: declaration.receive.body.span,
        },
        canonical_bytes: canonical_bytes.into_boxed_slice(),
    })
}

fn binding(module: &ling_resolve::ResolvedModule, pattern: &hir::Pattern) -> Option<BindingKey> {
    let hir::PatternKind::Binding { id, .. } = pattern.kind else {
        return None;
    };
    Some(BindingKey::new(module.id, id))
}

fn failure(
    module: &ling_resolve::ResolvedModule,
    span: Span,
    reason: &'static str,
    actor: Option<String>,
) -> ActorCheckFailure {
    ActorCheckFailure {
        source_name: module.hir.source_name.clone(),
        span,
        reason,
        actor,
    }
}

fn stable_actor_type_id(definition: &DefinitionId) -> ActorTypeId {
    let mut hash = 0x811c_9dc5_u32;
    for byte in definition.as_str().as_bytes() {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    ActorTypeId::new(if hash == 0 { 1 } else { hash })
}

#[allow(clippy::too_many_arguments)]
fn canonical_bytes(
    typed: &TypedProgram,
    definition: &DefinitionId,
    actor_type: ActorTypeId,
    message: TypeId,
    state: TypeId,
    initializer: ExpressionKey,
    transition_body: ExpressionKey,
    state_binding: BindingKey,
    message_binding: BindingKey,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    push_text(&mut bytes, CHECKED_ACTOR_CORE_VERSION);
    push_text(&mut bytes, definition.as_str());
    push_text(
        &mut bytes,
        "actor-id:runtime-scoped/nonzero/unique/nonreusable/unallocated",
    );
    bytes.extend_from_slice(&actor_type.get().to_le_bytes());
    push_text(&mut bytes, &typed.arena().display(message));
    push_text(&mut bytes, &typed.arena().display(state));
    for value in [
        initializer.local().get(),
        transition_body.local().get(),
        state_binding.local().get(),
        message_binding.local().get(),
    ] {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes
}

fn push_text(output: &mut Vec<u8>, value: &str) {
    let length = u32::try_from(value.len()).unwrap_or(u32::MAX);
    output.extend_from_slice(&length.to_le_bytes());
    output.extend_from_slice(value.as_bytes());
}
