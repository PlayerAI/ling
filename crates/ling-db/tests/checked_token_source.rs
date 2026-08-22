use std::sync::Arc;

use ling_db::CompilerDb;
use ling_source::{ChangeEvent, SourceId};

fn source_file(event: ChangeEvent) -> SourceId {
    match event {
        ChangeEvent::Added { file, .. }
        | ChangeEvent::Changed { file, .. }
        | ChangeEvent::Unchanged { file, .. } => file,
    }
}

#[test]
fn unicode_bom_crlf_source_facts_keep_original_order_and_bytes() {
    let mut db = CompilerDb::new();
    let bytes = "\u{feff}module Main\r\n\r\nlet 人物 = \"😀\"\r\n\r\nlet main () = 人物\r\n"
        .as_bytes()
        .to_vec();
    let file = source_file(
        db.set_disk_snapshot("fixtures/Unicode.ling", bytes.clone())
            .expect("fixture source is accepted"),
    );

    let index = db
        .checked_token_source_index(file)
        .expect("checked source fixture builds");
    assert_eq!(index.source(), file);
    assert_eq!(index.revision().get(), 1);
    assert_eq!(index.source_name(), "fixtures/Unicode.ling");
    assert!(
        index
            .entries()
            .windows(2)
            .all(|pair| pair[0].token().span().start() <= pair[1].token().span().start())
    );

    let snapshot = db.source_bytes(file).expect("source snapshot exists");
    let person = index
        .entries()
        .iter()
        .find(|entry| entry.token().text() == "人物")
        .expect("Chinese identifier is retained");
    let start = usize::try_from(person.token().span().start().get()).expect("span start fits");
    let end = usize::try_from(person.token().span().end().get()).expect("span end fits");
    assert_eq!(&snapshot.bytes()[start..end], "人物".as_bytes());
    let emoji = index
        .entries()
        .iter()
        .find(|entry| entry.token().text().contains('😀'))
        .expect("emoji literal is retained");
    let start = usize::try_from(emoji.token().span().start().get()).expect("span start fits");
    let end = usize::try_from(emoji.token().span().end().get()).expect("span end fits");
    assert_eq!(&snapshot.bytes()[start..end], "\"😀\"".as_bytes());
}

#[test]
fn checked_source_fixture_reuses_then_invalidates_by_vfs_revision() {
    let mut db = CompilerDb::new();
    let file = source_file(
        db.set_disk_snapshot(
            "fixtures/Revision.ling",
            b"module Main\n\nlet helper = 1\n\nlet main () = helper\n".to_vec(),
        )
        .expect("initial fixture source is accepted"),
    );
    let first = db
        .checked_token_source_index(file)
        .expect("initial checked source builds");
    let repeated = db
        .checked_token_source_index(file)
        .expect("repeated checked source builds");
    assert!(Arc::ptr_eq(&first, &repeated));

    let changed = source_file(
        db.set_disk_snapshot(
            "fixtures/Revision.ling",
            b"module Main\n\nlet helper = 2\n\nlet main () = helper\n".to_vec(),
        )
        .expect("edited fixture source is accepted"),
    );
    assert_eq!(changed, file);
    let refreshed = db
        .checked_token_source_index(changed)
        .expect("edited checked source builds");
    assert!(!Arc::ptr_eq(&first, &refreshed));
    assert!(refreshed.revision() > first.revision());
    assert_eq!(first.revision().get(), 1);
    assert_eq!(first.source(), refreshed.source());
}
