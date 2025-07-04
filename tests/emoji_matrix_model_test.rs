use solfunmeme_dioxus::models::emoji_matrix::{parse_summary_total, parse_summary_root, rollup_emoji_matrix, EmojiMatrix, EmojiMatrixEntry};
use std::collections::HashMap;

#[test]
fn test_parse_summary_total() {
    let content = r#"
=== Total Emoji Summary Table ===
Type       | Count    | Category           | Emoji
fn         | 10       | Rust Core          | 🦀⚙️
struct     | 5        | Rust Core          | 🏛️🧱
ident      | 20       | Rust Core          | 🆔
"#;

    let matrix = parse_summary_total(content);
    assert_eq!(matrix.entries.len(), 1);
    let entry = &matrix.entries[0];
    assert_eq!(entry.path, "total");
    assert_eq!(entry.emoji_counts.get("🦀⚙️"), Some(&10));
    assert_eq!(entry.emoji_counts.get("🏛️🧱"), Some(&5));
    assert_eq!(entry.emoji_counts.get("🆔"), Some(&20));
}

#[test]
fn test_parse_summary_root() {
    let content = r#"
=== Directory Emoji Summary: src/core ===
🦀⚙️(fn)×3 🏛️🧱(struct)×2 | 🦀⚙️🦀🏛️🧱
=== Directory Emoji Summary: src/utils ===
🆔(ident)×5 | 🆔🆔🆔🆔🆔
"#;

    let matrix = parse_summary_root(content);
    assert_eq!(matrix.entries.len(), 2);

    let entry1 = &matrix.entries[0];
    assert_eq!(entry1.path, "src/core");
    assert_eq!(entry1.emoji_counts.get("🦀⚙️"), Some(&3));
    assert_eq!(entry1.emoji_counts.get("🏛️🧱"), Some(&2));

    let entry2 = &matrix.entries[1];
    assert_eq!(entry2.path, "src/utils");
    assert_eq!(entry2.emoji_counts.get("🆔"), Some(&5));
}

#[test]
fn test_rollup_emoji_matrix() {
    let entries = vec![
        EmojiMatrixEntry {
            path: "src/core/file1.rs".to_string(),
            emoji_counts: HashMap::from([
                ("🦀⚙️".to_string(), 1),
                ("🆔".to_string(), 2),
            ]),
        },
        EmojiMatrixEntry {
            path: "src/core/file2.rs".to_string(),
            emoji_counts: HashMap::from([
                ("🦀⚙️".to_string(), 2),
                ("🏛️🧱".to_string(), 1),
            ]),
        },
        EmojiMatrixEntry {
            path: "src/utils/file3.rs".to_string(),
            emoji_counts: HashMap::from([
                ("🆔".to_string(), 3),
            ]),
        },
    ];

    let matrix = EmojiMatrix { entries };
    let rolled_up_matrix = rollup_emoji_matrix(matrix);

    assert_eq!(rolled_up_matrix.entries.len(), 5); // root, src, src/core, src/utils, src/core/file1.rs, src/core/file2.rs, src/utils/file3.rs

    let root_entry = rolled_up_matrix.entries.iter().find(|e| e.path == "").unwrap();
    assert_eq!(root_entry.emoji_counts.get("🦀⚙️"), Some(&3));
    assert_eq!(root_entry.emoji_counts.get("🆔"), Some(&5));
    assert_eq!(root_entry.emoji_counts.get("🏛️🧱"), Some(&1));

    let src_entry = rolled_up_matrix.entries.iter().find(|e| e.path == "src").unwrap();
    assert_eq!(src_entry.emoji_counts.get("🦀⚙️"), Some(&3));
    assert_eq!(src_entry.emoji_counts.get("🆔"), Some(&5));
    assert_eq!(src_entry.emoji_counts.get("🏛️🧱"), Some(&1));

    let src_core_entry = rolled_up_matrix.entries.iter().find(|e| e.path == "src/core").unwrap();
    assert_eq!(src_core_entry.emoji_counts.get("🦀⚙️"), Some(&3));
    assert_eq!(src_core_entry.emoji_counts.get("🆔"), Some(&2));
    assert_eq!(src_core_entry.emoji_counts.get("🏛️🧱"), Some(&1));

    let src_utils_entry = rolled_up_matrix.entries.iter().find(|e| e.path == "src/utils").unwrap();
    assert_eq!(src_utils_entry.emoji_counts.get("🆔"), Some(&3));
}
