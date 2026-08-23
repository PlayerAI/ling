use ling_db::CompilerDb;

fn codes(database: &mut CompilerDb) -> Vec<String> {
    database
        .workspace_diagnostics()
        .expect("fixture query has no internal failure")
        .iter()
        .map(|diagnostic| diagnostic.code().as_str().to_owned())
        .collect()
}

fn database(source: &str) -> CompilerDb {
    let mut database = CompilerDb::new();
    database
        .set_disk_snapshot("src/Main.ling", source.as_bytes().to_vec())
        .expect("fixture source installs");
    database
}

#[test]
fn clean_workspace_has_no_diagnostics() {
    let mut database = database("module Main\n\nlet value = 1\n");
    assert!(codes(&mut database).is_empty());
}

#[test]
fn reports_registered_frontend_and_checked_failures() {
    let cases = [
        ("module Main\n\nlet value = @\n", "L-LEX-0004"),
        ("module Main\n\nlet value =\n", "L-SYNTAX-0010"),
        ("module Main\n\nlet (left, right) = (1, 2)\n", "L-NAME-0003"),
        ("module Main\n\nlet value = missing\n", "L-NAME-0001"),
        ("module Main\n\nlet value: Int = \"text\"\n", "L-TYPE-0001"),
        (
            "module Main\n\nlet main () = Console.write \"x\"\n",
            "L-CAP-0001",
        ),
    ];

    for (source, expected) in cases {
        assert_eq!(codes(&mut database(source)), vec![expected], "{source:?}");
    }
}

#[test]
fn syntax_failures_prevent_workspace_semantic_cascades() {
    let mut database = CompilerDb::new();
    database
        .set_disk_snapshot(
            "src/Main.ling",
            b"module Main\n\nlet value: Int = \"text\"\n".to_vec(),
        )
        .expect("main installs");
    database
        .set_disk_snapshot(
            "src/Other.ling",
            b"module Other\n\nlet broken = @\n".to_vec(),
        )
        .expect("other installs");

    assert_eq!(codes(&mut database), vec!["L-LEX-0004"]);
}

#[test]
fn repeated_diagnostic_queries_are_identical() {
    let mut database = database("module Main\n\nlet value = missing\n");
    let first = database.workspace_diagnostics().expect("first query");
    let second = database.workspace_diagnostics().expect("cached query");
    assert_eq!(first, second);
}
