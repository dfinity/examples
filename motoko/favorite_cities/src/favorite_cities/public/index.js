import favorite_cities from 'ic:canisters/favorite_cities';

favorite_cities.greet(window.prompt("Enter your name:")).then(greeting => {
  window.alert(greeting);
});
