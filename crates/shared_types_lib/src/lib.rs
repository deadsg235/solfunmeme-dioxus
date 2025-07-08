pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// Shared types used across multiple crates

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum MenuOption {
    Embedding,
    PerformanceCharts,
    BertTest,
    RustParser,
    #[allow(dead_code)]
    MemeManagement,
    #[allow(dead_code)]
    ExpressionParsing,
    #[allow(dead_code)]
    Encryption,
    #[allow(dead_code)]
    MetaMemeOperations,
    #[allow(dead_code)]
    StylingAndEmojis,
    #[allow(dead_code)]
    CryptoFrontend,
    #[allow(dead_code)]
    Memes,
    #[allow(dead_code)]
    Lean,
    #[allow(dead_code)]
    ConnectionManagement,
    #[allow(dead_code)]
    ConnectionList,
    #[allow(dead_code)]
    SendSol,
    #[allow(dead_code)]
    ReceiveSol,
    #[allow(dead_code)]
    QueryAccounts,
    #[allow(dead_code)]
    Dashboard,
    #[allow(dead_code)]
    Connections,
    #[allow(dead_code)]
    ClustersManagement,
    #[allow(dead_code)]
    Clusters,
    #[allow(dead_code)]
    Airdrop,
    #[allow(dead_code)]
    Accounts,
    #[allow(dead_code)]
    ComponentMemes,
    #[allow(dead_code)]
    MonsterMetaMeme,
    #[allow(dead_code)]
    SolFunMeme,
    #[allow(dead_code)]
    Extractor,
    SourceBrowser,
    EmojiMatrix,
    Mcp
}
