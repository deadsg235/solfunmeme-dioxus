use rrust_kontekst::mcp_component;

#[mcp_component(
    menu = "ai_tools",
    label = "Test Component", 
    emoji = "🧪",
    description = "A test component for demonstration",
    order = 1
)]
pub async fn test_component() -> Result<String, Box<dyn std::error::Error>> {
    Ok("Test component executed successfully".to_string())
}
