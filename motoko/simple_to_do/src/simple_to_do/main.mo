import utils "utils";
import types "types";

type ToDo = types.ToDo;

// Define the actor
actor Assistant {

    var todos : [ToDo] = [];
    var nextId : Nat = 1;

    public query func getTodos () : async [ToDo] {
        todos
    };

    public func addTodo (description : Text) : async () {
        todos := utils.add(todos, description, nextId);
        nextId += 1;
    };

    public func completeTodo (id : Nat) : async () {
        todos := utils.complete(todos, id);
    };

    public query func showTodos () : async Text {
        utils.show(todos)
    };

    public func clearCompleted () : async () {
        todos := utils.clear(todos);
    };
};
