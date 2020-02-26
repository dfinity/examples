import calc from 'ic:canisters/calc';

calc.greet(window.prompt("Enter your name:")).then(greeting => {
  window.alert(greeting);
});
