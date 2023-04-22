<script setup lang="ts">
  // import { listen } from "@tauri-apps/api/event"
  import { invoke } from "@tauri-apps/api/tauri";
  import { ref } from 'vue';
  import Loading from "./components/Loading.vue";
  import Main_Window from "./components/Main_Window.vue";

  // default payload, may make gdrive version
  const should_show = ref(false);
  invoke("close_splash").then(() => {
    console.log("invoke(\"close_splash\").then(() ran")
    should_show.value = true;
  });

</script>

<template>
  <div v-if="should_show">
    <Main_Window/>
  </div>
  <div v-else>
    <Loading/>
  </div>    
</template>
