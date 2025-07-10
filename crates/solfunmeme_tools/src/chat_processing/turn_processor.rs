pub fn process_turn(turn_content: &str) -> String {
    let (speaker, content) = extract_speaker_and_content(turn_content);
    let message_content = content.lines().skip(1).collect::<Vec<_>>().join("\n");
    
    format!("### {}\n\n{}\n", speaker, message_content)
}

fn extract_speaker_and_content(turn: &str) -> (&str, &str) {
    let trimmed_turn = turn.trim();
    if trimmed_turn.starts_with("User") {
        ("User", trimmed_turn)
    } else if trimmed_turn.starts_with("Grok AI") {
        ("Grok AI", trimmed_turn)
    } else {
        ("Unknown", trimmed_turn)
    }
}