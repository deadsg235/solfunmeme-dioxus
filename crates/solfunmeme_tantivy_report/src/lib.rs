use anyhow::Result;
use solfunmeme_search_tantivy::SearchIndex;
use std::collections::HashMap;
use std::path::Path;

pub enum ReportType {
    Terms,
    Emojis,
    HexCodes,
}

pub fn get_top_entries(index_path: &Path, report_type: ReportType, limit: usize) -> Result<Vec<(String, usize)>> {
    let search_index = SearchIndex::new(index_path)?;
    
    let stats = match report_type {
        ReportType::Terms => search_index.get_stats_by_terms()?,
        ReportType::Emojis => search_index.get_stats_by_emojis()?,
        ReportType::HexCodes => search_index.get_stats_by_hex_codes()?,
    };

    let mut sorted_entries: Vec<(String, usize)> = stats.into_iter().collect();
    sorted_entries.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_entries.truncate(limit);

    Ok(sorted_entries)
}
