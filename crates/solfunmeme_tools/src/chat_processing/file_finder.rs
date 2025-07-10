use std::path::PathBuf;
use walkdir::WalkDir;

pub fn find_chat_files(target_path: &PathBuf, limit: Option<usize>) -> Vec<PathBuf> {
    let mut chat_files: Vec<PathBuf> = WalkDir::new(target_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && 
            e.path().extension().map_or(false, |ext| ext == "md") &&
            e.path().file_name().map_or(false, |name| name.to_string_lossy().contains("grok-chat"))
        })
        .map(|e| e.path().to_owned())
        .collect();

    if let Some(limit) = limit {
        chat_files.truncate(limit);
    }

    chat_files
}