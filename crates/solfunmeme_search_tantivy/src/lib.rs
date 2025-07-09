use tantivy::schema::TantivyDocument;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, IndexWriter};

use shared_analysis_types::CodeChunk;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub path: String,
    pub content: String,
    pub emoji: String,
    pub line_start: u32,
    pub line_end: u32,
    pub score: f32,
}

pub struct SearchIndex {
    index: Index,
    schema: Schema,
    writer: IndexWriter,
}

impl SearchIndex {
    pub fn new(index_path: &Path) -> Result<Self> {
        // Create schema
        let mut schema_builder = Schema::builder();
        
        // Fields for indexing
        let path_field = schema_builder.add_text_field("path", TEXT | STORED);
        let content_field = schema_builder.add_text_field("content", TEXT | STORED);
        let emoji_field = schema_builder.add_text_field("emoji", TEXT | STORED);
        let line_start_field = schema_builder.add_u64_field("line_start", STORED);
        let line_end_field = schema_builder.add_u64_field("line_end", STORED);
        let chunk_type_field = schema_builder.add_text_field("chunk_type", TEXT | STORED);
        
        let schema = schema_builder.build();
        
        // Create or open index
        let index = if index_path.exists() {
            Index::open_in_dir(index_path)?
        } else {
            std::fs::create_dir_all(index_path)?;
            Index::create_in_dir(index_path, schema.clone())?
        };
        
        let writer = index.writer(50_000_000)?; // 50MB buffer
        
        Ok(Self {
            index,
            schema,
            writer,
        })
    }
    
    pub fn add_chunk(&mut self, chunk: &CodeChunk) -> Result<()> {
        let path_field = self.schema.get_field("path")?;
        let content_field = self.schema.get_field("content")?;
        let emoji_field = self.schema.get_field("emoji")?;
        let line_start_field = self.schema.get_field("line_start")?;
        let line_end_field = self.schema.get_field("line_end")?;
        let chunk_type_field = self.schema.get_field("chunk_type")?;
        
        let doc = doc!(
            path_field => chunk.path.clone(),
            content_field => chunk.content.clone(),
            emoji_field => chunk.emoji.clone(),
            line_start_field => chunk.line_start as u64,
            line_end_field => chunk.line_end as u64,
            chunk_type_field => chunk.chunk_type.clone(),
        );
        
        self.writer.add_document(doc)?;
        Ok(())
    }
    
    pub fn commit(&mut self) -> Result<()> {
        self.writer.commit()?;
        Ok(())
    }
    
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        
        let query_parser = QueryParser::for_index(&self.index, vec![
            self.schema.get_field("content")?,
            self.schema.get_field("path")?,
            self.schema.get_field("chunk_type")?,
        ]);
        
        let query = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;
        
        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            
            let path = doc
                .get_first(self.schema.get_field("path")?)
                .and_then(|v| v.as_value().as_str())
                .unwrap_or("")
                .to_string();
                
            let content = doc
                .get_first(self.schema.get_field("content")?)
                .and_then(|v| v.as_value().as_str())
                .unwrap_or("")
                .to_string();
                
            let emoji = doc
                .get_first(self.schema.get_field("emoji")?)
                .and_then(|v| v.as_value().as_str())
                .unwrap_or("")
                .to_string();
                
            let line_start = doc
                .get_first(self.schema.get_field("line_start")?)
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32;
                
            let line_end = doc
                .get_first(self.schema.get_field("line_end")?)
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32;
            
            results.push(SearchResult {
                path,
                content,
                emoji,
                line_start,
                line_end,
                score,
            });
        }
        
        Ok(results)
    }
    
    pub fn search_by_emoji(&self, emoji: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query = format!("emoji:{}", emoji);
        self.search(&query, limit)
    }
    
    pub fn search_by_path(&self, path_pattern: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query = format!("path:*{}*", path_pattern);
        self.search(&query, limit)
    }
    
    pub fn get_stats(&self) -> Result<HashMap<String, usize>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        
        let mut stats = HashMap::new();
        
        let content_field = self.schema.get_field("content")?;
        
        for segment_reader in reader.segments() {
            let term_dict = segment_reader.fast_fields().text(content_field).expect("Failed to get term dict");
            for (term, _doc_freq) in term_dict.terms() {
                let term_str = term.text();
                *stats.entry(term_str.to_string()).or_insert(0) += 1;
            }
        }
        
        Ok(stats)
    }
}

// Bag of words functionality
pub struct BagOfWords {
    word_counts: HashMap<String, usize>,
    total_words: usize,
}

impl BagOfWords {
    pub fn new() -> Self {
        Self {
            word_counts: HashMap::new(),
            total_words: 0,
        }
    }
    
    pub fn add_text(&mut self, text: &str) {
        // Simple tokenization - split on whitespace and punctuation
        let words: Vec<&str> = text
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|word| !word.is_empty())
            .collect();
            
        for word in words {
            let word = word.to_lowercase();
            *self.word_counts.entry(word).or_insert(0) += 1;
            self.total_words += 1;
        }
    }
    
    pub fn get_word_frequency(&self, word: &str) -> f64 {
        let word = word.to_lowercase();
        let count = self.word_counts.get(&word).unwrap_or(&0);
        if self.total_words > 0 {
            *count as f64 / self.total_words as f64
        } else {
            0.0
        }
    }
    
    pub fn get_top_words(&self, limit: usize) -> Vec<(String, usize)> {
        let mut words: Vec<(String, usize)> = self.word_counts
            .iter()
            .map(|(word, count)| (word.clone(), *count))
            .collect();
            
        words.sort_by(|a, b| b.1.cmp(&a.1));
        words.truncate(limit);
        words
    }
    
    pub fn similarity(&self, other: &BagOfWords) -> f64 {
        let mut similarity = 0.0;
        let mut total_weight = 0.0;
        
        for (word, count) in &self.word_counts {
            let other_count = other.word_counts.get(word).unwrap_or(&0);
            let weight = (*count + *other_count) as f64;
            similarity += weight * (1.0 - (*count as f64 - *other_count as f64).abs() / weight);
            total_weight += weight;
        }
        
        if total_weight > 0.0 {
            similarity / total_weight
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_bag_of_words() {
        let mut bow = BagOfWords::new();
        bow.add_text("hello world hello rust");
        
        assert_eq!(bow.get_word_frequency("hello"), 0.5);
        assert_eq!(bow.get_word_frequency("world"), 0.25);
        assert_eq!(bow.get_word_frequency("rust"), 0.25);
        
        let top_words = bow.get_top_words(2);
        assert_eq!(top_words[0], ("hello".to_string(), 2));
    }
    
    #[test]
    fn test_search_index() -> Result<()> {
        let temp_dir = tempdir()?;
        let mut index = SearchIndex::new(temp_dir.path())?;
        
        // Add a test chunk
        let chunk = CodeChunk {
            path: "test.rs".to_string(),
            content: "fn hello() { println!(\"world\"); }".to_string(),
            emoji: "🦀⚙️".to_string(),
            line_start: 1,
            line_end: 1,
            chunk_type: "function".to_string(),
        };
        
        index.add_chunk(&chunk)?;
        index.commit()?;
        
        // Search
        let results = index.search("hello", 10)?;
        assert!(!results.is_empty());
        assert_eq!(results[0].content, chunk.content);
        
        Ok(())
    }
}