dfx canister create simple_to_do
dfx build
dfx canister install simple_to_do

# add todos
dfx canister call simple_to_do addTodo '("Create a project")'
dfx canister call simple_to_do addTodo '("Build the project")'
dfx canister call simple_to_do addTodo '("Deploy the project")'

# view todos
dfx canister call simple_to_do showTodos

# complete and observe change
dfx canister call simple_to_do completeTodo '(1)'
sleep 2
dfx canister call simple_to_do showTodos
