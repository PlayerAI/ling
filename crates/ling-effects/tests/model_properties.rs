use ling_effects::{
    EffectConstraint, EffectConstraintOrigin, EffectConstraintSolver, EffectGraphProjection,
    EffectId, EffectLabel, EffectOperation, EffectRowModel, EffectRowTail, EffectTypeRef,
    HandlerClause, HandlerContract, HandlerCore, HandlerCoreClause, HandlerCoreNodeId, ResumeMode,
    ResumeUse, RowVariableId,
};

fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }

    let mut result = Vec::new();
    for index in 0..items.len() {
        let mut rest = Vec::with_capacity(items.len() - 1);
        rest.extend_from_slice(&items[..index]);
        rest.extend_from_slice(&items[index + 1..]);
        for mut suffix in permutations(&rest) {
            let mut permutation = vec![items[index].clone()];
            permutation.append(&mut suffix);
            result.push(permutation);
        }
    }
    result
}

fn type_ref(name: &str) -> EffectTypeRef {
    EffectTypeRef::new(name).expect("test type reference is canonical")
}

fn operation(owner: &str, name: &str, mode: ResumeMode) -> EffectOperation {
    EffectOperation::new(
        EffectId::new(owner).expect("test effect identity is canonical"),
        name,
        [],
        type_ref("Unit"),
        mode,
    )
    .expect("test operation is canonical")
}

#[test]
fn bounded_row_permutations_have_one_canonical_projection() {
    let labels = vec![
        EffectLabel::clock(),
        EffectLabel::random(),
        EffectLabel::console_write(),
        EffectLabel::state(type_ref("Int")),
    ];
    let baseline = EffectRowModel::closed(labels.clone());
    let open_baseline = EffectRowModel::open(labels.clone(), RowVariableId::new(3));

    for permutation in permutations(&labels) {
        let mut with_duplicate = permutation.clone();
        with_duplicate.push(permutation[0].clone());
        let row = EffectRowModel::closed(with_duplicate.clone());
        let open_row = EffectRowModel::open(with_duplicate, RowVariableId::new(3));
        assert_eq!(row.canonical_name(), baseline.canonical_name());
        assert_eq!(row.canonical_bytes(), baseline.canonical_bytes());
        assert_eq!(open_row.canonical_bytes(), open_baseline.canonical_bytes());
        assert_eq!(
            open_row.tail(),
            EffectRowTail::Variable(RowVariableId::new(3))
        );
    }
}

#[test]
fn bounded_solver_permutations_have_one_substitution() {
    let variable = RowVariableId::new(7);
    let constraints = vec![
        EffectConstraint::Equal {
            left: EffectRowModel::open([EffectLabel::clock()], variable),
            right: EffectRowModel::closed([EffectLabel::clock(), EffectLabel::random()]),
            origin: EffectConstraintOrigin::new(1),
        },
        EffectConstraint::Requires {
            row: EffectRowModel::open([], RowVariableId::new(8)),
            label: EffectLabel::console_write(),
            origin: EffectConstraintOrigin::new(2),
        },
    ];
    let expected = EffectConstraintSolver::from_constraints(constraints.clone())
        .solve()
        .expect("bounded constraints solve")
        .substitution()
        .canonical_bytes();

    for permutation in permutations(&constraints) {
        let actual = EffectConstraintSolver::from_constraints(permutation)
            .solve()
            .expect("permuted constraints solve")
            .substitution()
            .canonical_bytes();
        assert_eq!(actual, expected);
    }
}

#[test]
fn bounded_handler_and_graph_permutations_have_one_projection() {
    let clock = EffectLabel::clock();
    let random = EffectLabel::random();
    let input = EffectRowModel::closed([clock.clone(), random.clone()]);
    let clock_operation = operation("Clock", "now", ResumeMode::Once);
    let random_operation = operation("Random", "next", ResumeMode::Many);
    let clauses = vec![
        HandlerCoreClause::new(
            HandlerClause::new(clock.clone(), clock_operation.clone()).expect("clock clause"),
            HandlerCoreNodeId::new(2),
            ResumeUse::Once,
        ),
        HandlerCoreClause::new(
            HandlerClause::new(random.clone(), random_operation.clone()).expect("random clause"),
            HandlerCoreNodeId::new(3),
            ResumeUse::Many,
        ),
    ];
    let expected_core = HandlerCore::new(
        input.clone(),
        HandlerCoreNodeId::new(1),
        type_ref("Unit"),
        clauses.clone(),
        Some(ling_effects::EffectSourceSpan::new("first.ling", 2, 8)),
    )
    .expect("handler core is checked")
    .canonical_bytes();
    let handler = HandlerContract::for_input(
        &input,
        [
            HandlerClause::new(clock, clock_operation.clone()).expect("clock clause"),
            HandlerClause::new(random, random_operation.clone()).expect("random clause"),
        ],
    )
    .expect("handler contract is checked");
    let expected_graph = EffectGraphProjection::new(
        [input.clone(), EffectRowModel::pure()],
        [clock_operation.clone(), random_operation.clone()],
        [handler.clone()],
    )
    .canonical_bytes();

    for permutation in permutations(&clauses) {
        let core = HandlerCore::new(
            input.clone(),
            HandlerCoreNodeId::new(1),
            type_ref("Unit"),
            permutation,
            Some(ling_effects::EffectSourceSpan::new("second.ling", 40, 49)),
        )
        .expect("permuted handler core is checked");
        assert_eq!(core.canonical_bytes(), expected_core);
    }
    let reversed_graph = EffectGraphProjection::new(
        [EffectRowModel::pure(), input],
        [random_operation, clock_operation],
        [handler],
    );
    assert_eq!(reversed_graph.canonical_bytes(), expected_graph);
}
