use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ling_project::{
    LOCK_FILE_NAME, LockFile, MANIFEST_FILE_NAME, Manifest, ModuleGraph, discover_modules,
    parse_lock_file, parse_manifest, resolve_package_graph,
};

const MODULE_NAMES: [&str; 6] = ["A", "B", "C", "D", "E", "F"];
const GENERATED_GRAPH_CASES: usize = 128;
const LOCK_ROUND_TRIP_CASES: usize = 64;
const LOGICAL_PATH_CASES: usize = 128;
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn generated_module_graphs_match_an_independent_cycle_oracle() {
    let mut random = DeterministicRandom::new(0x6c69_6e67_2d63_7963);
    let mut cyclic_cases = 0;
    let mut acyclic_cases = 0;

    for case in 0..GENERATED_GRAPH_CASES {
        let spec = GraphSpec::generated(case, &mut random);
        let expected_cycle = contains_cycle(&spec.edges);
        assert_eq!(expected_cycle, case % 2 == 1, "generator case {case}");

        let project = TempProject::new("cycles", case);
        project.write_graph(&spec, &(0..spec.node_count()).collect::<Vec<_>>());

        match project.discover() {
            Ok(graph) => {
                acyclic_cases += 1;
                assert!(!expected_cycle, "case {case} published a cyclic graph");
                assert_eq!(graph.nodes().len(), spec.node_count(), "case {case}");
            }
            Err(failure) => {
                cyclic_cases += 1;
                assert!(expected_cycle, "case {case} rejected an acyclic graph");
                let diagnostics = failure
                    .diagnostics()
                    .expect("generated cycle failure must remain a public diagnostic");
                assert_eq!(diagnostics.len(), 1, "case {case}");
                assert_eq!(
                    diagnostics[0].code().as_str(),
                    "L-PROJECT-0013",
                    "case {case}"
                );
                let rendered: serde_json::Value = serde_json::from_str(
                    &diagnostics[0]
                        .render_json()
                        .expect("cycle diagnostic must satisfy the public JSON schema"),
                )
                .expect("rendered cycle diagnostic must be JSON");
                assert_eq!(rendered["facts"]["reason"], "cycle", "case {case}");
            }
        }
    }

    assert_eq!(acyclic_cases, GENERATED_GRAPH_CASES / 2);
    assert_eq!(cyclic_cases, GENERATED_GRAPH_CASES / 2);
}

#[test]
fn generated_projects_ignore_file_enumeration_order_and_locks_round_trip() {
    let mut random = DeterministicRandom::new(0x6c69_6e67_2d6c_6f63);
    let mut reordered_cases = 0;

    for case in 0..LOCK_ROUND_TRIP_CASES {
        let spec = GraphSpec::acyclic(&mut random);
        let ascending = (0..spec.node_count()).collect::<Vec<_>>();
        let mut descending = ascending.clone();
        descending.reverse();
        if ascending != descending {
            reordered_cases += 1;
        }

        let first = TempProject::new("order-first", case);
        let second = TempProject::new("order-second", case);
        first.write_graph(&spec, &ascending);
        second.write_graph(&spec, &descending);

        let first_graph = first.resolve();
        let second_graph = second.resolve();
        assert_eq!(first_graph, second_graph, "generated graph case {case}");
        assert_eq!(
            first_graph.id(),
            second_graph.id(),
            "generated graph identity case {case}"
        );

        let first_lock = LockFile::from_graph(&first_graph);
        let second_lock = LockFile::from_graph(&second_graph);
        let first_bytes = first_lock.to_canonical_bytes();
        let second_bytes = second_lock.to_canonical_bytes();
        assert_eq!(first_bytes, second_bytes, "canonical lock case {case}");

        let decoded = parse_lock_file(LOCK_FILE_NAME, &first_bytes)
            .expect("a generated canonical lock must decode");
        assert_eq!(decoded, first_lock, "lock model case {case}");
        assert_eq!(
            decoded.to_canonical_bytes(),
            first_bytes,
            "lock byte round-trip case {case}"
        );
        assert!(
            decoded.matches_graph(&first_graph),
            "lock graph case {case}"
        );
    }

    assert!(reordered_cases > 0);
}

#[test]
fn generated_logical_paths_accept_only_canonical_slash_spelling() {
    let mut random = DeterministicRandom::new(0x6c69_6e67_2d70_6174);

    for case in 0..LOGICAL_PATH_CASES {
        let component_count = 1 + random.index(4);
        let canonical = (0..component_count)
            .map(|component| format!("r{}x{component}", random.next() % 10_000))
            .collect::<Vec<_>>()
            .join("/");
        let manifest_bytes = manifest_with_root(&canonical);
        let manifest = parse_manifest("property/valid/ling.toml", manifest_bytes.as_bytes())
            .expect("generated canonical path must decode");
        assert_eq!(manifest.source().roots()[0].as_str(), canonical);

        for invalid in [
            format!("/{canonical}"),
            format!("{canonical}/"),
            format!("{canonical}//tail"),
            format!("{canonical}/./tail"),
            format!("{canonical}/../tail"),
            format!("{canonical}\\tail"),
            format!("C:/{canonical}"),
            format!("scheme:{canonical}"),
            "Cafe\u{301}".to_owned(),
        ] {
            let invalid_bytes = manifest_with_root(&invalid);
            let error = parse_manifest("property/invalid/ling.toml", invalid_bytes.as_bytes())
                .expect_err("noncanonical logical path must not be rewritten");
            assert_eq!(error.code().as_str(), "L-PROJECT-0005", "{invalid:?}");
            assert!(
                error.span().end as usize <= invalid_bytes.len(),
                "case {case}"
            );
        }
    }
}

#[derive(Clone, Debug)]
struct GraphSpec {
    edges: Vec<Vec<bool>>,
}

impl GraphSpec {
    fn generated(case: usize, random: &mut DeterministicRandom) -> Self {
        let mut spec = Self::acyclic(random);
        if case % 2 == 1 {
            let mut cycle = (0..spec.node_count()).collect::<Vec<_>>();
            random.shuffle(&mut cycle);
            cycle.truncate(1 + random.index(spec.node_count()));
            for index in 0..cycle.len() {
                let from = cycle[index];
                let to = cycle[(index + 1) % cycle.len()];
                spec.edges[from][to] = true;
            }
        }
        spec
    }

    fn acyclic(random: &mut DeterministicRandom) -> Self {
        let node_count = 1 + random.index(MODULE_NAMES.len());
        let mut edges = vec![vec![false; node_count]; node_count];
        for (from, row) in edges.iter_mut().enumerate() {
            for edge in row.iter_mut().skip(from + 1) {
                *edge = random.one_in(3);
            }
        }
        Self { edges }
    }

    fn node_count(&self) -> usize {
        self.edges.len()
    }

    fn source(&self, module: usize) -> String {
        let mut source = format!("module {}\n", MODULE_NAMES[module]);
        for (target, imported) in self.edges[module].iter().copied().enumerate() {
            if imported {
                source.push_str(&format!("import {}\n", MODULE_NAMES[target]));
            }
        }
        source.push_str("\nlet value = 0\n");
        source
    }
}

fn contains_cycle(edges: &[Vec<bool>]) -> bool {
    let mut indegree = vec![0_usize; edges.len()];
    for row in edges {
        for (target, edge) in row.iter().copied().enumerate() {
            if edge {
                indegree[target] += 1;
            }
        }
    }

    let mut ready = indegree
        .iter()
        .enumerate()
        .filter_map(|(node, degree)| (*degree == 0).then_some(node))
        .collect::<Vec<_>>();
    let mut visited = 0;
    while let Some(node) = ready.pop() {
        visited += 1;
        for (target, edge) in edges[node].iter().copied().enumerate() {
            if edge {
                indegree[target] -= 1;
                if indegree[target] == 0 {
                    ready.push(target);
                }
            }
        }
    }
    visited != edges.len()
}

fn manifest_with_root(root: &str) -> String {
    let root = serde_json::to_string(root).expect("logical path is representable as a TOML string");
    format!(
        "manifest-version = 1\n\n[package]\nname = \"property\"\nversion = \"0.1.0\"\nlanguage = \"0.1\"\n\n[source]\nroots = [{root}]\nentry = \"A\"\n"
    )
}

struct TempProject {
    root: PathBuf,
}

impl TempProject {
    fn new(label: &str, case: usize) -> Self {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ling-prj-1108-{label}-{}-{case}-{sequence}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("stale generated project is removable");
        }
        fs::create_dir_all(&root).expect("generated project root is creatable");
        Self { root }
    }

    fn write_graph(&self, spec: &GraphSpec, order: &[usize]) {
        self.write(MANIFEST_FILE_NAME, &manifest_with_root("src"));
        for module in order {
            self.write(
                &format!("src/{}.ling", MODULE_NAMES[*module]),
                &spec.source(*module),
            );
        }
    }

    fn manifest(&self) -> Manifest {
        let path = self.root.join(MANIFEST_FILE_NAME);
        let bytes = fs::read(&path).expect("generated manifest is readable");
        parse_manifest(&path.to_string_lossy(), &bytes).expect("generated manifest is valid")
    }

    fn discover(&self) -> Result<ModuleGraph, ling_project::DiscoveryFailure> {
        discover_modules(&self.root, &self.manifest())
    }

    fn resolve(&self) -> ling_project::PackageGraph {
        resolve_package_graph(&self.root, &self.manifest())
            .expect("generated acyclic project must resolve")
    }

    fn write(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("generated source parent is creatable");
        }
        fs::write(path, contents).expect("generated project input is writable");
    }
}

impl Drop for TempProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

struct DeterministicRandom(u64);

impl DeterministicRandom {
    const fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    fn index(&mut self, upper: usize) -> usize {
        let upper = u64::try_from(upper).expect("generated bound fits in u64");
        usize::try_from(self.next() % upper).expect("generated index fits in usize")
    }

    fn one_in(&mut self, denominator: u64) -> bool {
        self.next().rem_euclid(denominator) == 0
    }

    fn shuffle<T>(&mut self, values: &mut [T]) {
        for index in (1..values.len()).rev() {
            let other = self.index(index + 1);
            values.swap(index, other);
        }
    }
}
