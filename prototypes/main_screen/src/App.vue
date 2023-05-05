<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import MainPage from "./components/MainPage.vue";

import { invoke } from "@tauri-apps/api/tauri";
import { ref } from 'vue';
import Loading from "./components/Loading.vue";

// default payload, may make gdrive version
const should_show = ref(false);
invoke("close_splash").then(() => {
  console.log("invoke(\"close_splash\").then(() ran")
  should_show.value = true;
});
</script>

<template>
  <div v-if="should_show">
    <MainPage/>
  </div>
  <div v-else>
    <Loading/>
  </div>
</template>
