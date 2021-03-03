import hello_cycles from 'ic:canisters/hello_cycles';

hello_cycles.greet(window.prompt("Enter your name:")).then(greeting => {
  window.alert(greeting);
});
