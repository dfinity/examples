import Map "mo:core/Map";
import Nat32 "mo:core/Nat32";

persistent actor Superheroes {

  public type SuperheroId = Nat32;

  public type Superhero = {
    name : Text;
    superpowers : [Text];
  };

  var next : SuperheroId = 0;

  let map = Map.empty<SuperheroId, Superhero>();

  public func create(superhero : Superhero) : async SuperheroId {
    let superheroId = next;
    next += 1;
    Map.add(map, Nat32.compare, superheroId, superhero);
    return superheroId
  };

  public query func read(superheroId : SuperheroId) : async ?Superhero {
    Map.get(map, Nat32.compare, superheroId)
  };

  public func update(superheroId : SuperheroId, superhero : Superhero) : async Bool {
    let exists = Map.get(map, Nat32.compare, superheroId) != null;
    if (exists) {
      Map.add(map, Nat32.compare, superheroId, superhero);
    };
    return exists
  };

  public func delete(superheroId : SuperheroId) : async Bool {
    Map.delete(map, Nat32.compare, superheroId)
  }
}
