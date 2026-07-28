import LLM "canister:llm";

// The backend calls the LLM canister's `v1_chat` endpoint through the typed
// `canister:llm` import — no LLM actor type is declared here. The import is
// typed against the LLM canister's committed Candid interface (candid/llm.did),
// and the mops.toml `--actor-env-alias` flag binds it to the
// PUBLIC_CANISTER_ID:llm env var that icp-cli injects.
//
// The message types below are this backend's own public API (the frontend
// generates its bindings from them); they are structurally identical to the LLM
// canister's `chat_message_v1` / `assistant_message`, so values flow straight
// into `LLM.v1_chat` without conversion.
//
// This example does not use tools, so `tools` is passed as null.
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

  // The model this example uses. The LLM canister identifies models by string;
  // other available models include "qwen3:32b" and "llama4-scout".
  let model = "llama3.1:8b";

  func sendChat(messages : [ChatMessage]) : async AssistantMessage {
    let response = await LLM.v1_chat({ model; messages; tools = null });
    response.message;
  };

  public func prompt(prompt : Text) : async Text {
    let message = await sendChat([#user({ content = prompt })]);
    switch (message.content) {
      case (?text) text;
      case null "";
    };
  };

  public func chat(messages : [ChatMessage]) : async Text {
    let message = await sendChat(messages);
    switch (message.content) {
      case (?text) text;
      case null "";
    };
  };
};
