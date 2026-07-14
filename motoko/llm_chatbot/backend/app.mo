import Runtime "mo:core/Runtime";

// The backend calls the LLM canister's `v1_chat` endpoint directly. The request
// and response types below mirror that canister's Candid interface:
// https://dashboard.internetcomputer.org/canister/w36hm-eqaaa-aaaal-qr76a-cai#interface
//
// LLM canister interface:
//   v1_chat : (chat_request_v1) -> (chat_response_v1)
//
// This example does not use tools, so the optional `tools` field is omitted from
// the request — Candid decodes an absent optional record field as `null` on the
// canister side, keeping the call wire-compatible.
actor {

  // A message in a chat. Mirrors `chat_message_v1` in the LLM canister interface.
  public type ChatMessage = {
    #user : { content : Text };
    #system_ : { content : Text };
    #assistant : AssistantMessage;
    #tool : { content : Text; tool_call_id : Text };
  };

  // The assistant's reply. Mirrors `assistant_message`.
  public type AssistantMessage = {
    content : ?Text;
    tool_calls : [ToolCall];
  };

  public type ToolCall = {
    id : Text;
    function : FunctionCall;
  };

  public type FunctionCall = {
    name : Text;
    arguments : [ToolCallArgument];
  };

  public type ToolCallArgument = {
    name : Text;
    value : Text;
  };

  // Request/response for `v1_chat`. The request omits the optional `tools` field
  // (see the note above).
  type Request = {
    model : Text;
    messages : [ChatMessage];
  };
  type Response = { message : AssistantMessage };
  type LlmActor = actor { v1_chat : (Request) -> async Response };

  // The model this example uses. The LLM canister identifies models by string;
  // other available models include "qwen3:32b" and "llama4-scout".
  let model = "llama3.1:8b";

  // The LLM canister ID is injected as PUBLIC_CANISTER_ID:llm at deploy time:
  //   local: auto-injected by icp-cli after deploying the pre-built llm canister
  //   ic:    set explicitly in icp.yaml to the shared mainnet LLM canister
  //
  // See icp.yaml for the environment configuration.
  func llmCanister<system>() : LlmActor {
    let ?id = Runtime.envVar<system>("PUBLIC_CANISTER_ID:llm") else Runtime.trap("PUBLIC_CANISTER_ID:llm not set — run icp deploy");
    actor (id) : LlmActor;
  };

  func sendChat<system>(messages : [ChatMessage]) : async AssistantMessage {
    let response = await llmCanister<system>().v1_chat({ model; messages });
    response.message;
  };

  public func prompt(prompt : Text) : async Text {
    let message = await sendChat<system>([#user({ content = prompt })]);
    switch (message.content) {
      case (?text) text;
      case null "";
    };
  };

  public func chat(messages : [ChatMessage]) : async Text {
    let message = await sendChat<system>(messages);
    switch (message.content) {
      case (?text) text;
      case null "";
    };
  };
};
