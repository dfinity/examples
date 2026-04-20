import Array "mo:core/Array";
import Blob "mo:core/Blob";
import HashMap "mo:map/Map";
import { phash; thash } "mo:map/Map";
import Iter "mo:core/Iter";
import Option "mo:core/Option";
import Principal "mo:core/Principal";
import Text "mo:core/Text";

persistent actor Filevault {

  // Define a data type for a file's chunks.
  type FileChunk = {
    chunk : Blob;
    index : Nat;
  };

  // Define a data type for a file's data.
  type File = {
    name : Text;
    chunks : [FileChunk];
    totalSize : Nat;
    fileType : Text;
  };

  // Define a data type for storing files associated with a user principal.
  type UserFiles = HashMap.Map<Text, File>;

  // HashMap to store the user data
  private let files = HashMap.new<Principal, UserFiles>();

  // Return files associated with a user's principal.
  private func getUserFiles(user : Principal) : UserFiles {
    switch (HashMap.get(files, phash, user)) {
      case null {
        let newFileMap = HashMap.new<Text, File>();
        let _ = HashMap.put(files, phash, user, newFileMap);
        newFileMap;
      };
      case (?existingFiles) existingFiles;
    };
  };

  // Check if a file name already exists for the user.
  public shared (msg) func checkFileExists(name : Text) : async Bool {
    HashMap.get(getUserFiles(msg.caller), thash, name).isSome();
  };

  // Upload a file in chunks.
  public shared (msg) func uploadFileChunk(name : Text, chunk : Blob, index : Nat, fileType : Text) : async () {
    let userFiles = getUserFiles(msg.caller);
    let fileChunk = { chunk = chunk; index = index };

    switch (HashMap.get(userFiles, thash, name)) {
      case null {
        let _ = HashMap.put(userFiles, thash, name, { name = name; chunks = [fileChunk]; totalSize = chunk.size(); fileType = fileType });
      };
      case (?existingFile) {
        let updatedChunks = existingFile.chunks.concat([fileChunk]);
        let _ = HashMap.put(
          userFiles,
          thash,
          name,
          {
            name = name;
            chunks = updatedChunks;
            totalSize = existingFile.totalSize + chunk.size();
            fileType = fileType;
          }
        );
      };
    };
  };

  // Return list of files for a user.
  public shared (msg) func getFiles() : async [{ name : Text; size : Nat; fileType : Text }] {
    HashMap.vals(getUserFiles(msg.caller)).map(
      func(file : File) : { name : Text; size : Nat; fileType : Text } {
        {
          name = file.name;
          size = file.totalSize;
          fileType = file.fileType;
        };
      }
    ).toArray();
  };

  // Return total chunks for a file
  public shared (msg) func getTotalChunks(name : Text) : async Nat {
    switch (HashMap.get(getUserFiles(msg.caller), thash, name)) {
      case null 0;
      case (?file) file.chunks.size();
    };
  };

  // Return specific chunk for a file.
  public shared (msg) func getFileChunk(name : Text, index : Nat) : async ?Blob {
    switch (HashMap.get(getUserFiles(msg.caller), thash, name)) {
      case null null;
      case (?file) {
        switch (file.chunks.find(func(chunk : FileChunk) : Bool { chunk.index == index })) {
          case null null;
          case (?foundChunk) ?foundChunk.chunk;
        };
      };
    };
  };

  // Get file's type.
  public shared (msg) func getFileType(name : Text) : async ?Text {
    switch (HashMap.get(getUserFiles(msg.caller), thash, name)) {
      case null null;
      case (?file) ?file.fileType;
    };
  };

  // Delete a file.
  public shared (msg) func deleteFile(name : Text) : async Bool {
    HashMap.remove(getUserFiles(msg.caller), thash, name).isSome();
  };
};
