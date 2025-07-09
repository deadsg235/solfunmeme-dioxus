use emoji_workflow_macro::emoji_workflow;
use workflow_manager::WorkflowManager;
use emoji_lang_plugin::EmojiWorkflow;

#[emoji_workflow("📜🔗")]
fn my_ttl_workflow() {
    println!("This is my TTL workflow function!");
}

fn main() {
    my_ttl_workflow();

    // You can also execute the registered workflow manually
    let manager = emoji_lang_plugin::GLOBAL_WORKFLOW_MANAGER.lock().unwrap();
    if let Ok(_) = manager.execute_workflow("my_ttl_workflow") {
        println!("Successfully executed workflow via manager.");
    } else {
        println!("Failed to execute workflow via manager.");
    }
}