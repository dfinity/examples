import List "mo:base/List";
import Option "mo:base/Option";
import Map "mo:base/OrderedMap";
import Nat32 "mo:base/Nat32";

actor Superheroes {

  /**
   * Types
   */

  // The type of a superhero identifier.
  public type SuperheroId = Nat32;

  // The type of a superhero.
  public type Superhero = {
    name : Text;
    superpowers : List.List<Text>
  };

  /**
   * Application State
   */

  // The next available superhero identifier.
  stable var next : SuperheroId = 0;

  // The superhero data store.
  let Ops = Map.Make<SuperheroId>(Nat32.compare);
  stable var map : Map.Map<SuperheroId, Superhero> = Ops.empty();

  /**
   * High-Level API
   */

  // Create a superhero.
  public func create(superhero : Superhero) : async SuperheroId {
    let superheroId = next;
    next += 1;
    map := Ops.put(map, superheroId, superhero);
    return superheroId
  };

  // Read a superhero.
  public query func read(superheroId : SuperheroId) : async ?Superhero {
    let result = Ops.get(map, superheroId);
    return result
  };

  // Update a superhero.
  public func update(superheroId : SuperheroId, superhero : Superhero) : async Bool {
    let (result, old_value) = Ops.replace(map, superheroId, superhero);
    let exists = Option.isSome(old_value);
    if (exists) {
      map := result
    };
    return exists
  };

  // Delete a superhero.
  public func delete(superheroId : SuperheroId) : async Bool {
    let (result, old_value) = Ops.remove(map, superheroId);
    let exists = Option.isSome(old_value);
    if (exists) {
      map := result
    };
    return exists
  }
}
