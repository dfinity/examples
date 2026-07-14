use candid::{CandidType, Principal};
use ic_cdk::update;
use serde::{Deserialize, Serialize};

// The backend calls the LLM canister's `v1_chat` endpoint directly. The request
// and response types below mirror that canister's Candid interface:
// https://dashboard.internetcomputer.org/canister/w36hm-eqaaa-aaaal-qr76a-cai#interface
//
// LLM canister interface:
//   v1_chat : (chat_request_v1) -> (chat_response_v1)
// where
//   chat_request_v1  = record { model : text; tools : opt vec tool; messages : vec chat_message_v1 };
//   chat_response_v1 = record { message : assistant_message };
//
// This example does not use tools, so the optional `tools` field is omitted
// from the request — Candid decodes an absent optional record field as `null`
// on the canister side, keeping the call wire-compatible.

// The model this example uses. The LLM canister identifies models by string;
// other available models include "qwen3:32b" and "llama4-scout".
const MODEL: &str = "llama3.1:8b";

// A message in a chat. Mirrors `chat_message_v1` in the LLM canister interface.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ChatMessage {
    #[serde(rename = "user")]
    User { content: String },
    #[serde(rename = "system")]
    System { content: String },
    #[serde(rename = "assistant")]
    Assistant(AssistantMessage),
    #[serde(rename = "tool")]
    Tool {
        content: String,
        tool_call_id: String,
    },
}

// The assistant's reply. Mirrors `assistant_message`.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AssistantMessage {
    pub content: Option<String>,
    pub tool_calls: Vec<ToolCall>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ToolCall {
    pub id: String,
    pub function: FunctionCall,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<ToolCallArgument>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ToolCallArgument {
    pub name: String,
    pub value: String,
}

/// Request sent to `v1_chat`. Mirrors `chat_request_v1` without the optional
/// `tools` field (see the note at the top of this file).
#[derive(CandidType, Serialize, Debug)]
struct Request {
    model: String,
    messages: Vec<ChatMessage>,
}

/// Response returned by `v1_chat`. Mirrors `chat_response_v1`.
#[derive(CandidType, Deserialize, Debug)]
struct Response {
    message: AssistantMessage,
}

// The LLM canister ID is injected as PUBLIC_CANISTER_ID:llm at deploy time:
//   local: auto-injected by icp-cli after deploying the pre-built llm canister
//   ic:    set explicitly in icp.yaml to the shared mainnet LLM canister
//
// See icp.yaml for the environment configuration.
fn llm_canister() -> Principal {
    let id = ic_cdk::api::env_var_value("PUBLIC_CANISTER_ID:llm");
    Principal::from_text(&id).expect("invalid PUBLIC_CANISTER_ID:llm")
}

/// Sends a chat request to the LLM canister and returns the assistant's reply.
async fn send_chat(messages: Vec<ChatMessage>) -> AssistantMessage {
    let response: Response = ic_cdk::call::Call::bounded_wait(llm_canister(), "v1_chat")
        .change_timeout(300)
        .with_arg(Request {
            model: MODEL.to_string(),
            messages,
        })
        .await
        .unwrap_or_else(|e| ic_cdk::trap(format!("LLM call failed: {e:?}")))
        .candid()
        .unwrap_or_else(|e| ic_cdk::trap(format!("failed to decode LLM response: {e:?}")));
    response.message
}

#[update]
async fn prompt(prompt_str: String) -> String {
    let message = send_chat(vec![ChatMessage::User { content: prompt_str }]).await;
    message.content.unwrap_or_default()
}

#[update]
async fn chat(messages: Vec<ChatMessage>) -> String {
    let message = send_chat(messages).await;
    message.content.unwrap_or_default()
}

// Export the canister's Candid interface.
ic_cdk::export_candid!();
