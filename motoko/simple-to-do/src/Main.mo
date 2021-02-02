import Types "Types";
import Utils "Utils";

// Define the actor
actor Assistant {

  type ToDo = Types.ToDo;

  var todos : [ToDo] = [];
  var nextId : Nat = 1;

  public query func getTodos() : async [ToDo] {
    return todos;
  };

  public func addTodo(description : Text) : async () {
    todos := Utils.add(todos, description, nextId);
    nextId += 1;
  };

  public func completeTodo(id : Nat) : async () {
    todos := Utils.complete(todos, id);
  };

  public query func showTodos() : async Text {
    return Utils.show(todos);
  };

  public func clearCompleted() : async () {
    todos := Utils.clear(todos);
  };
};
