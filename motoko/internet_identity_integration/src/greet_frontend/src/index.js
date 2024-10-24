import { createActor, greet_backend } from "../../declarations/greet_backend";
import { AuthClient } from "@dfinity/auth-client";
import { HttpAgent } from "@dfinity/agent";

// Global variables that we will set up
window.greetActor;
window.authClient;

const greetButton = document.getElementById("greet");
const loginButton = document.getElementById("login");

AuthClient.create().then((client) => {
  // Put this on the window so we can access it from the console. This is helpful for learning, but it's recommended to keep it in a closure in a real app.
  window.authClient = client;
  // We can use the authClient to check if the user is already authenticated
  setActor(client);

  // Now that the auth client is created, we can enable the login and greet buttons
  loginButton.removeAttribute("disabled");
  greetButton.removeAttribute("disabled");
});

loginButton.onclick = () => {
  // start the login process and wait for it to finish
  window.authClient.login({
    identityProvider: process.env.II_URL,
    onSuccess: () => {
      setActor(window.authClient);
    },
    onError: (err) => {
      console.error(err);
    },
  });
};

greetButton.onclick = async () => {
  // Disable the button to prevent multiple clicks
  greetButton.setAttribute("disabled", true);

  // Interact with backend actor, calling the greet method
  const greeting = await window.greetActor.greet();

  greetButton.removeAttribute("disabled");

  document.getElementById("greeting").innerText = greeting;
};

/**
 * Sets the actor to be used by the greet button
 * @param {AuthClient} an initialized AuthClient, which will have the identity of the user (logged in or not)
 */
function setActor(authClient) {
  // At this point we're authenticated, and we can get the identity from the auth client:
  const identity = authClient.getIdentity();
  // Using the identity obtained from the auth client, we can create an agent to interact with the IC.
  const agent = new HttpAgent({ identity });
  // Using the interface description of our webapp, we create an actor that we use to call the service methods.
  window.greetActor = createActor(process.env.GREET_BACKEND_CANISTER_ID, {
    agent,
  });
}
