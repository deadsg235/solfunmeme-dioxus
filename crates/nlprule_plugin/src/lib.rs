use anyhow::Result;
use nlprule::tokenizer::Tokenizer;
use nlprule::rules::Rules;
use nlprule::types::Text;

pub struct NlpRulePlugin {
    tokenizer: Tokenizer,
    rules: Rules,
}

impl NlpRulePlugin {
    pub fn new() -> Result<Self> {
        let tokenizer = Tokenizer::new().unwrap(); // TODO: Handle errors properly
        let rules = Rules::new().unwrap(); // TODO: Handle errors properly
        Ok(NlpRulePlugin { tokenizer, rules })
    }

    pub fn check_text(&self, text: &str) -> Vec<Text> {
        let tokens = self.tokenizer.tokenize(text);
        self.rules.apply(&tokens)
    }

    // pub fn suggest_corrections(&self, text: &str) -> Vec<String> {
    //     // This would involve more complex logic with nlprule
    //     vec![]
    // }
}
