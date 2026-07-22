import Principal "mo:core/Principal";
import Time "mo:core/Time";
import Map "mo:core/Map";
import Text "mo:core/Text";
import Blob "mo:core/Blob";
import Array "mo:core/Array";
import List "mo:core/List";
import Nat64 "mo:core/Nat64";
import Nat "mo:core/Nat";
import Result "mo:core/Result";
import Int "mo:core/Int";

actor class (keyNameString : Text) {
  // Types
  type Message = {
    sender : Principal;
    encryptedMessage : Blob;
    timestamp : Nat64;
  };

  type Inbox = {
    messages : [Message];
  };

  type SendMessageRequest = {
    receiver : Principal;
    encryptedMessage : Blob;
  };

  type Result<T, E> = {
    #Ok : T;
    #Err : E;
  };

  // vetKD management canister interface. These names are the fixed system API
  // contract (snake_case), so they are kept as-is rather than camelCased.
  type VetKdKeyId = {
    curve : { #bls12_381_g2 };
    name : Text;
  };

  type VetKdPublicKeyArgs = {
    canister_id : ?Principal;
    context : Blob;
    key_id : VetKdKeyId;
  };

  type VetKdDeriveKeyArgs = {
    context : Blob;
    input : Blob;
    key_id : VetKdKeyId;
    transport_public_key : Blob;
  };

  type VetKdSystemApi = actor {
    vetkd_public_key : (VetKdPublicKeyArgs) -> async { public_key : Blob };
    vetkd_derive_key : (VetKdDeriveKeyArgs) -> async {
      encrypted_key : Blob;
    };
  };

  // Constants
  let MAX_MESSAGES_PER_INBOX : Nat = 1000;
  let DOMAIN_SEPARATOR : Text = "basic_ibe_example_dapp";

  // State
  var inboxes = Map.empty<Principal, Inbox>();

  // Management canister actor
  let vetKdSystemApi : VetKdSystemApi = actor ("aaaaa-aa");

  // Send a message to a receiver
  public shared ({ caller }) func sendMessage(request : SendMessageRequest) : async Result<(), Text> {
    let message : Message = {
      sender = caller;
      encryptedMessage = request.encryptedMessage;
      timestamp = Nat64.fromNat(Int.abs(Time.now()));
    };

    let receiver = request.receiver;
    let currentInbox = switch (Map.get(inboxes, Principal.compare, receiver)) {
      case (?inbox) { inbox };
      case null { { messages = [] } };
    };

    if (currentInbox.messages.size() >= MAX_MESSAGES_PER_INBOX) {
      return #Err("Inbox for " # Principal.toText(receiver) # " is full");
    };

    let newMessages = Array.concat(currentInbox.messages, [message]);
    let newInbox : Inbox = { messages = newMessages };
    ignore Map.insert(inboxes, Principal.compare, receiver, newInbox);

    #Ok();
  };

  // Get the IBE public key
  public shared func getIbePublicKey() : async Blob {
    let keyId : VetKdKeyId = {
      curve = #bls12_381_g2;
      name = keyNameString;
    };

    let context = Text.encodeUtf8(DOMAIN_SEPARATOR);
    let request : VetKdPublicKeyArgs = {
      canister_id = null;
      context = context;
      key_id = keyId;
    };

    let result = await vetKdSystemApi.vetkd_public_key(request);
    result.public_key;
  };

  // Get the caller's encrypted IBE key
  public shared ({ caller }) func getMyEncryptedIbeKey(transportKey : Blob) : async Blob {
    let keyId : VetKdKeyId = {
      curve = #bls12_381_g2;
      name = keyNameString;
    };

    let context = Text.encodeUtf8(DOMAIN_SEPARATOR);
    let input = Principal.toBlob(caller);
    let request : VetKdDeriveKeyArgs = {
      context = context;
      input = input;
      key_id = keyId;
      transport_public_key = transportKey;
    };

    let result = await (with cycles = 26_153_846_153) vetKdSystemApi.vetkd_derive_key(request);
    result.encrypted_key;
  };

  // Get the caller's messages
  public shared query ({ caller }) func getMyMessages() : async Inbox {
    switch (Map.get(inboxes, Principal.compare, caller)) {
      case (?inbox) { inbox };
      case null { { messages = [] } };
    };
  };

  // Remove a message by index
  public shared ({ caller }) func removeMyMessageByIndex(messageIndex : Nat64) : async Result<(), Text> {
    let currentInbox = switch (Map.get(inboxes, Principal.compare, caller)) {
      case (?inbox) { inbox };
      case null { { messages = [] } };
    };

    let index = Nat64.toNat(messageIndex);
    if (index >= currentInbox.messages.size()) {
      return #Err("Message index out of bounds");
    };

    // Create a new array without the specified index
    let messages = currentInbox.messages;
    let newMessagesList = List.empty<Message>();

    for (i in messages.keys()) {
      if (i != index) {
        List.add(newMessagesList, messages[i]);
      };
    };

    let newMessages = List.toArray(newMessagesList);
    let newInbox : Inbox = { messages = newMessages };
    ignore Map.insert(inboxes, Principal.compare, caller, newInbox);

    #Ok();
  };
};
