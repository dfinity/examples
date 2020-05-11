import simple_to_do from 'ic:canisters/simple_to_do';

simple_to_do.addTodo(window.prompt("Enter your task")).then(Todos => {
  window.alert(Todos);
});
