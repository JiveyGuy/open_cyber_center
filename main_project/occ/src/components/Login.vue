<!-- https://yobaji.github.io/vue3-google-login/#google-rendered-login-button-one-tap-prompts -->

<script setup lang="ts">
  import { ref } from "vue";
  import { decodeCredential } from "vue3-google-login"
  import type { CallbackTypes } from "vue3-google-login";
  const emits = defineEmits<{(id: 'login'): void}>();

  
  const callback: CallbackTypes.CredentialCallback = (response) => {
    // This callback will be triggered when the user selects or login to
    // his Google account from the popup
    console.log("Credential JWT string", response.credential);
    const userData = decodeCredential(response.credential);
    console.log("Handle the userData", userData);
    
    emits("login");
  };

</script>
<!-- <button @click="$emit('login')" type="submit" class=" bg-slate-500">login</button> -->

<template>
  <div class="bg-gray-900 h-screen flex flex-col justify-center items-center">
    <div class="w-80 rounded-lg shadow-lg p-6">
      <GoogleLogin :callback="callback"/>
    </div>
  </div>
</template>