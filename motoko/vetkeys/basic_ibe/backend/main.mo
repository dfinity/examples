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
import Runtime "mo:core/Runtime";

actor {
  // The vetKD key name this canister derives from. Set the `VETKD_KEY_NAME`
  // canister environment variable (see `icp.yaml`) to pick a different key; it
  // defaults to `test_key_1` so a deploy can never leave the canister
  // half-initialized. Do not change it once the canister holds data: the key
  // feeds vetKD derivation, so a different key cannot decrypt what the old one
  // encrypted.
  transient let keyNameString = Runtime.envVar<system>("VETKD_KEY_NAME") ?? "test_key_1";

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
  let inboxes = Map.empty<Principal, Inbox>();

  // Management canister actor
  let vetKdSystemApi : VetKdSystemApi = actor ("aaaaa-aa");

  // Send a message to a receiver
  public shared ({ caller }) func sendMessage(request : SendMessageRequest) : async Result<(), Text> {
    let message : Message = {
      sender = caller;
      encryptedMessage = request.encryptedMessage;
      timestamp = Int.abs(Time.now()).toNat64();
    };

    let receiver = request.receiver;
    let currentInbox = switch (inboxes.get(receiver)) {
      case (?inbox) { inbox };
      case null { { messages = [] } };
    };

    if (currentInbox.messages.size() >= MAX_MESSAGES_PER_INBOX) {
      return #Err("Inbox for " # receiver.toText() # " is full");
    };

    let newMessages = currentInbox.messages.concat([message]);
    let newInbox : Inbox = { messages = newMessages };
    ignore inboxes.insert(receiver, newInbox);

    #Ok();
  };

  // Get the IBE public key
  public shared func getIbePublicKey() : async Blob {
    let keyId : VetKdKeyId = {
      curve = #bls12_381_g2;
      name = keyNameString;
    };

    let context = DOMAIN_SEPARATOR.encodeUtf8();
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

    let context = DOMAIN_SEPARATOR.encodeUtf8();
    let input = caller.toBlob();
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
    switch (inboxes.get(caller)) {
      case (?inbox) { inbox };
      case null { { messages = [] } };
    };
  };

  // Remove a message by index
  public shared ({ caller }) func removeMyMessageByIndex(messageIndex : Nat64) : async Result<(), Text> {
    let currentInbox = switch (inboxes.get(caller)) {
      case (?inbox) { inbox };
      case null { { messages = [] } };
    };

    let index = messageIndex.toNat();
    if (index >= currentInbox.messages.size()) {
      return #Err("Message index out of bounds");
    };

    // Create a new array without the specified index
    let messages = currentInbox.messages;
    let newMessagesList = List.empty<Message>();

    for (i in messages.keys()) {
      if (i != index) {
        newMessagesList.add(messages[i]);
      };
    };

    let newMessages = newMessagesList.toArray();
    let newInbox : Inbox = { messages = newMessages };
    ignore inboxes.insert(caller, newInbox);

    #Ok();
  };
};
