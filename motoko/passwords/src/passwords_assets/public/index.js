import passwords from "ic:canisters/passwords";

// Add the form to the page
document.getElementById("app").innerHTML = `
  <p>
    <label for="username">Username</label>
    <input name="username" id="username" />
  </p>
  <p>
    <label for="password">Password</label>
    <input name="password" id="password" />
  </p>
  <p>
    <input type="button" id="submit" value="Who Am I?" />
  </p>
`;

// Listen for the submit button
document.getElementById("submit").addEventListener("click", async () => {
  const username = document.getElementById("username").value;
  const password = document.getElementById("password").value;

  try {
    // Try to authenticate using these credentials
    await passwords.login({ username, password });

    // If authentication succeeds, ask which account we're logged in as
    const login = await passwords.whoami();
    window.alert(`Logged in as: ${login}`);
  } catch (e) {
    // If authentication fails, tell the user
    window.alert("Wrong username or password");
    console.error(e);
  }
});
