<!-- https://yobaji.github.io/vue3-google-login/#google-rendered-login-button-one-tap-prompts -->

<script setup lang="ts">
  import { ref } from "vue";
  import VideoBackground from 'vue-responsive-video-background-player';
  import { decodeCredential } from "vue3-google-login"
  import type { CallbackTypes } from "vue3-google-login";
  import login_video from '../assets/login_video.mp4';
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

<div class="h-screen w-screen bg-black">
    <div class="w-screen h-screen absolute z-0">
      <video-background 
        :src=login_video
        class="h-screen"
      />
        <div class="absolute inset-0 h-screen w-screen">
          <GoogleLogin :callback="callback" class="content-center p-96"/>
        </div>
    </div>
  </div>
</template>