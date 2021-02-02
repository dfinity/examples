// Import standard libraries
import Array "mo:base/Array";
import Nat "mo:base/Nat";

// Import the 'ToDo' type definition
import Types "Types";

module Utils {

  type ToDo = Types.ToDo;

  // Add to-do item utility
  public func add(todos : [ToDo], desc : Text, nextId : Nat) : [ToDo] {
    let todo : ToDo = {
      id = nextId;
      description = desc;
      completed = false;
    };
    Array.append<ToDo>([todo], todos)
  };

  // Complete to-do item utility
  public func complete(todos : [ToDo], id : Nat) : [ToDo] {
    Array.map<ToDo,ToDo>(todos, func (todo : ToDo) : ToDo {
      if (todo.id == id) {
        return {
          id = todo.id;
          description = todo.description;
          completed = true;
        };
      };
      todo
    })
  };

  // Show to-do item utility
  public func show(todos : [ToDo]) : Text {
    var output : Text = "\n___TO-DOs___";
    for (todo : ToDo in todos.vals()) {
      output #= "\n(" # Nat.toText(todo.id) # ") " # todo.description;
      if (todo.completed) { output #= " ✔"; };
    };
    output
  };

  // Clear to-do item utility
  public func clear(todos: [ToDo]) : [ToDo] {
    var updated : [ToDo] = [];
    for (todo : ToDo in todos.vals()) {
      if (not todo.completed) {
        updated := Array.append<ToDo>(updated, [todo]);
      };
    };
    updated
  };
};
