<script>
  import { auth } from "../stores/auth";
</script>

<div class="container">
  <h1>Internet Identity Client</h1>
  <h2>You are not authenticated</h2>
  <p>To log in, click this button!</p>
  <button type="button" id="loginButton" on:click={$auth.login}>Log in</button>
</div>
