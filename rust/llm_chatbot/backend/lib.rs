use ic_cdk::update;
use ic_llm::{ChatMessage, Model};

#[update]
async fn prompt(prompt_str: String) -> String {
    ic_llm::prompt(Model::Llama3_1_8B, prompt_str).await
}

#[update]
async fn chat(messages: Vec<ChatMessage>) -> String {
    let response = ic_llm::chat(Model::Llama3_1_8B)
        .with_messages(messages)
        .send()
        .await;
    
    response.message.content.unwrap_or_default()
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
