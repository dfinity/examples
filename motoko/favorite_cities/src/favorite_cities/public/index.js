import hello_location_2_13 from 'ic:canisters/hello_location_2_13';

hello_location_2_13.greet(window.prompt("Enter your name:")).then(greeting => {
  window.alert(greeting);
});
