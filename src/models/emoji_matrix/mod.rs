use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmojiMatrix {
    pub entries: Vec<EmojiMatrixEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmojiMatrixEntry {
    pub path: String,
    pub emoji_counts: HashMap<String, u32>,
}

// This function will parse the summary_total.txt file and return an EmojiMatrix
pub fn parse_summary_total(file_content: &str) -> EmojiMatrix {
    let mut emoji_counts = HashMap::new();
    let lines: Vec<&str> = file_content.lines().collect();

    // Find the start of the table
    let mut table_start_index = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("Type") {
            table_start_index = i + 2; // Skip the header and the separator line
            break;
        }
    }

    for line in &lines[table_start_index..] {
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if parts.len() == 4 {
            let count: u32 = parts[1].parse().unwrap_or(0);
            let emoji = parts[3].to_string();
            *emoji_counts.entry(emoji).or_insert(0) += count;
        }
    }

    let entry = EmojiMatrixEntry {
        path: "total".to_string(),
        emoji_counts,
    };

    EmojiMatrix { entries: vec![entry] }
}

// This function will parse the summary_root.txt file and return an EmojiMatrix
pub fn parse_summary_root(file_content: &str) -> EmojiMatrix {
    let mut entries = Vec::new();
    let sections: Vec<&str> = file_content.split("=== Directory Emoji Summary:").collect();

    for section in sections.iter().skip(1) {
        let lines: Vec<&str> = section.lines().collect();
        let path = lines[0].trim().replace("===", "").trim().to_string();
        let mut emoji_counts = HashMap::new();

        if lines.len() > 1 {
            let emoji_line = lines[1];
            let emoji_parts: Vec<&str> = emoji_line.split('|').collect();
            if emoji_parts.len() > 1 {
                let emojis = emoji_parts[1].trim();
                for emoji_count in emojis.split_whitespace() {
                    let parts: Vec<&str> = emoji_count.split('×').collect();
                    if parts.len() == 2 {
                        let emoji = parts[0].to_string();
                        let count: u32 = parts[1].parse().unwrap_or(0);
                        *emoji_counts.entry(emoji).or_insert(0) += count;
                    }
                }
            }
        }

        entries.push(EmojiMatrixEntry {
            path,
            emoji_counts,
        });
    }

    EmojiMatrix { entries }
}

pub fn rollup_emoji_matrix(matrix: EmojiMatrix) -> EmojiMatrix {
    let mut rolled_up_entries: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for entry in matrix.entries {
        let path_parts: Vec<&str> = entry.path.split('/').collect();
        let mut current_path = String::new();

        for (i, part) in path_parts.iter().enumerate() {
            if i > 0 {
                current_path.push_str("/");
            }
            current_path.push_str(part);

            let entry_counts = rolled_up_entries.entry(current_path.clone()).or_default();
            for (emoji, count) in entry.emoji_counts.iter() {
                *entry_counts.entry(emoji.clone()).or_insert(0) += count;
            }
        }
    }

    let mut entries = Vec::new();
    for (path, emoji_counts) in rolled_up_entries {
        entries.push(EmojiMatrixEntry {
            path,
            emoji_counts,
        });
    }

    EmojiMatrix { entries }
}
