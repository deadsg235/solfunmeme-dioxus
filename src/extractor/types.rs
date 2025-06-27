use serde::Serialize;
use serde::Deserialize;

//mod types {
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub language: String,
    pub content: String,
    pub content_hash: String,  
    pub line_start: usize,
    pub line_end: usize,
    pub token_count: usize,  
    pub line_count: usize,  
    pub char_count: usize,  
    pub test_result: Option<TestResult>,  
}


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CodeSnippet2 {  
    pub content: String,  
    pub content_hash: String,  
    pub language: String,  
    pub token_count: usize,  
    pub line_count: usize,  
    pub char_count: usize,  
    pub test_result: Option<TestResult>,  
}  


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Multivector { scalar: f32, vector: [f32; 3] }

    
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AnnotatedWord {
    pub word: String,
    pub primary_emoji: String,
    pub secondary_emoji: String,
    pub wikidata: Option<String>,
    pub embedding: Vec<f32>,
    pub multivector: Multivector,
}


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExtractedFile {
    pub name: String,
    pub snippets: Vec<CodeSnippet>,
    pub total_lines: usize,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessingFile {
    pub name: String,
    pub progress: usize,
    pub total_lines: usize,
    pub current_content: String,
    pub summary: Option<DocumentSummary>,
}


//use crate::playground::doc_cleaner::DocumentSummary;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessingFile2 {
    pub name: String,
    pub contents: String,
//    annotations: Vec<AnnotatedWord>,
    pub progress: usize,
    pub total_lines: usize,
    pub summary: Option<DocumentSummary>,
//    duplicate_report: Option<DuplicateReport>,
//    code_annotations: Vec<AnnotatedWord>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]

pub struct UploadedFile {
    pub name: String,
    pub contents: String,
//    annotations: Vec<AnnotatedWord>,
    pub generated_program: String,
    pub summary: Option<DocumentSummary>,
//    duplicate_report: Option<DuplicateReport>,
    pub zip_url: Option<String>,
//    code_annotations: Vec<AnnotatedWord>,
}

// #[derive(Clone, Debug, PartialEq)]
// pub struct CodeSnippet {
//     pub language: String,
//     pub content: String,
//     pub line_start: usize,
//     pub line_end: usize,
// }

// #[derive(Clone, Debug, PartialEq)]
// pub struct ExtractedFile {
//     pub name: String,
//     pub snippets: Vec<CodeSnippet>,
//     pub total_lines: usize,
// }

// // pub struct FileData {  
// //     pub name: String,  
// //     pub snippets: Vec<CodeSnippet>,  
// //     pub total_lines: usize,  
// // }  

// #[derive(Clone, Debug, PartialEq, Default)]
// pub struct ProcessingFile {
//     pub name: String,
//     pub progress: usize,
//     pub total_lines: usize,
//     pub current_content: String,
// }


  
// Keep your existing structs unchanged  
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationTurn {  
    pub author: String,  
    pub content: String,  
    pub code_snippets: Vec<CodeSnippet>,  
}  
  

  
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TestResult {  
    pub success: bool,  
    pub error_message: Option<String>,  
    pub execution_time_ms: u64,  
}  
  
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentSummary {  
    pub total_turns: usize,  
    pub total_code_snippets: usize,  
    pub total_tokens: usize,  
    pub languages_found: Vec<String>,  
    pub content_hashes: Vec<String>,  
}  

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationTurn2 {
    pub author: String,
    pub content: String,
    pub code_snippets: Vec<CodeSnippet>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CodeSnippet3 {
    pub content: String,
    pub content_hash: String,
    pub language: String,
    pub token_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub test_result: Option<TestResult>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TestResult2 {
    pub success: bool,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentSummary2 {
    pub total_turns: usize,
    pub total_code_snippets: usize,
    pub total_tokens: usize,
    pub languages_found: Vec<String>,
    pub content_hashes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationTurnOld {
    pub author: String,
    pub content: String,
    pub code_snippets: Vec<CodeSnippet>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CodeSnippetOld {
    pub content: String,
    pub content_hash: String,
    pub language: String,
    pub token_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub test_result: Option<TestResult>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TestResultOld {
    pub success: bool,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentSummaryOld {
    pub total_turns: usize,
    pub total_code_snippets: usize,
    pub total_tokens: usize,
    pub languages_found: Vec<String>,
    pub content_hashes: Vec<String>,
}
