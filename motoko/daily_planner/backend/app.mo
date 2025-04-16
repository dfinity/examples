import Array "mo:base/Array";
import Blob "mo:base/Blob";
import Bool "mo:base/Bool";
import Cycles "mo:base/ExperimentalCycles";
import Int "mo:base/Int";
import Iter "mo:base/Iter";
import Text "mo:base/Text";
import Nat "mo:base/Nat";
import Result "mo:base/Result";
import JSON "mo:json";
import Option "mo:base/Option";
import HashMap "mo:map/Map";
import { thash } "mo:map/Map";
import IC "ic:aaaaa-aa";

persistent actor DailyPlanner {
  // General types used by the planner
  public type Note = {
    id : Nat;
    content : Text;
    isCompleted : Bool;
  };

  public type OnThisDay = {
    title : Text;
    year : Text;
    wikiLink : Text;
  };

  public type DayData = {
    notes : [Note];
    onThisDay : ?OnThisDay;
  };

  public type AddNoteResult = Result.Result<Text, Text>;

  // HashMap to store the data for each day.
  var dayData = HashMap.new<Text, DayData>();

  // Query function to get data for a specific date.
  // Returns null if the date does not contain any data.
  public query func getDayData(date : Text) : async ?DayData {
    HashMap.get(dayData, thash, date);
  };

  // Query function to get data for an entire month.
  // Returns a
  public query func getMonthData(year : Nat, month : Nat) : async [(Text, DayData)] {
    let monthPrefix = Text.concat(Int.toText(year), "-" # Int.toText(month) # "-");
    Iter.toArray(
      Iter.filter(
        HashMap.entries(dayData),
        func((k, _) : (Text, DayData)) : Bool {
          Text.startsWith(k, #text monthPrefix);
        }
      )
    );
  };

  // Update function to add a new note
  public func addNote(date : Text, content : Text) : async AddNoteResult {
    let currentData = switch (HashMap.get(dayData, thash, date)) {
      case null { { notes = []; onThisDay = null } };
      case (?data) { data };
    };

    let newNote : Note = {
      id = Array.size(currentData.notes);
      content = content;
      isCompleted = false;
    };

    let updatedNotes = Array.append(currentData.notes, [newNote]);
    let updatedData : DayData = {
      notes = updatedNotes;
      onThisDay = currentData.onThisDay;
    };
    let _ = HashMap.put(dayData, thash, date, updatedData);
    #ok("Added note for date: " # date);
    // Currently there is no error case in the result.
    // Code could be extended to disallow adding notes for past days.
  };

  // Update function to mark a note as completed.
  // Does nothing if the specified note is not found.
  public func completeNote(date : Text, noteId : Nat) : async () {
    switch (HashMap.get(dayData, thash, date)) {
      case null { /* Do nothing if no data for this date */ };
      case (?data) {
        let updatedNotes = Array.map<Note, Note>(
          data.notes,
          func(note) {
            if (note.id == noteId) {
              return {
                id = note.id;
                content = note.content;
                isCompleted = true;
              };
            } else {
              return note;
            };
          }
        );
        let updatedData : DayData = {
          notes = updatedNotes;
          onThisDay = data.onThisDay;
        };
        let _ = HashMap.put(dayData, thash, date, updatedData);
      };
    };
  };

  // Update function to fetch and store "On This Day" data via HTTPS outcall.
  public func fetchAndStoreOnThisDay(date : Text) : async Result.Result<Text, Text> {
    let currentData : DayData = switch (HashMap.get(dayData, thash, date)) {
      case null { { notes = []; onThisDay = null } };
      case (?data) { data };
    };

    // Perform HTTPS outcall only if needed.
    if (currentData.onThisDay == null) {
      let parts = Iter.toArray(Text.split(date, #char '-'));
      let month = Option.get(Nat.fromText(parts[1]), 1);
      let day = Option.get(Nat.fromText(parts[2]), 1);

      // Prepare the https request.
      // "transform" is used to specify how the HTTP response is processed before consensus tries to agree on a response.
      // This is useful to e.g. filter out timestamps out of headers that will be different across the responses the different replicas receive.
      // You can read more about it here: https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/https-outcalls/https-outcalls-how-to-use
      let http_request : IC.http_request_args = {
        // API must support IPv6
        url = "https://byabbe.se/on-this-day/" # Nat.toText(month) # "/" # Nat.toText(day) # "/events.json";
        max_response_bytes = null; //optional for request
        headers = [];
        body = null; //optional for request
        method = #get;
        transform = ?{
          function = transform;
          context = Blob.fromArray([]);
        };
      };

      // Perform HTTPS outcall using roughly 100B cycles.
      // See https outcall cost calculator: https://7joko-hiaaa-aaaal-ajz7a-cai.icp0.io.
      // Unused cycles are returned.
      Cycles.add<system>(100_000_000_000);

      // Execute the https outcall
      let http_response : IC.http_request_result = await IC.http_request(http_request);

      // Parse response into JSON
      let decoded_text : Text = switch (Text.decodeUtf8(http_response.body)) {
        case (null) { "No value returned" };
        case (?y) { y };
      };
      let json = switch (JSON.parse(decoded_text)) {
        case (#ok(parsed)) { parsed };
        case (#err(_)) { return #err("Error parsing JSON") };
      };

      // Get the event, year and link from the JSON
      let event = switch (JSON.getAsText(json, "events[0].description")) {
        case (#ok(value)) { value };
        case (#err(_)) { return #err("Error getting Event text") };
      };

      let year = switch (JSON.getAsText(json, "events[0].year")) {
        case (#ok(value)) { value };
        case (#err(_)) { return #err("Error getting Year text") };
      };
      let link = switch (JSON.getAsText(json, "events[0].wikipedia[0].wikipedia")) {
        case (#ok(value)) { value };
        case (#err(_)) { return #err("Error getting Wikipedia link") };
      };

      let otd : OnThisDay = {
        title = event;
        year = year;
        wikiLink = link;
      };

      let updatedData : DayData = {
        notes = currentData.notes;
        onThisDay = ?otd;
      };
      let _ = HashMap.put(dayData, thash, date, updatedData);
      #ok("data successfully obtained and stored for date: " # date);

    } else {
      #err("data already stored for date: " # date);
    };
  };

  // Transforms the raw HTTPS call response to an HttpResponsePayload on which the nodes can run consensus on.
  public query func transform({
    context : Blob;
    response : IC.http_request_result;
  }) : async IC.http_request_result {
    {
      response with headers = []; // not intersted in the headers
    };
  };
};
