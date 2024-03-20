<template>
  <div class="container">
    <h1>Internet Identity Client</h1>
    <h2>You are not authenticated</h2>
    <p>To log in, click this button!</p>
    <button type="button" id="loginButton" @click="authStore.login()">
      Log in
    </button>
  </div>
</template>

<script setup>
import { useAuthStore } from "../store/auth";

const authStore = useAuthStore();
</script>
