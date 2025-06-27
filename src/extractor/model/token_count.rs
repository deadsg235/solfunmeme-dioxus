/// Estimates token count for a given text (rough approximation).  
pub fn estimate_token_count(text: &str) -> usize {  
    let word_count = text.split_whitespace().count();  
    let char_count = text.chars().count();  
    (word_count + char_count / 4) / 2  
}  
  


// /// Estimates token count for a given text (rough approximation).
// pub fn estimate_token_count_old(text: &str) -> usize {
//     // Simple tokenization: split on whitespace and punctuation
//     let word_count = text.split_whitespace().count();
//     let char_count = text.chars().count();
    
//     // Rough approximation: average of word count and char_count/4
//     (word_count + char_count / 4) / 2
// }

// pub fn estimate_token_count(text: &str) -> usize {
//     let word_count = text.split_whitespace().count();
//     let char_count = text.chars().count();
//     (word_count + char_count / 4) / 2
// }
