<script setup lang="ts">
  // This starter template is using Vue 3 <script setup> SFCs
  // Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
  import MainPage from "./components/MainPage.vue";

  import { invoke } from "@tauri-apps/api/tauri";
  import { ref } from 'vue';
  import Loading from "./components/Loading.vue";
  import StartScreen from "./components/StartScreen.vue";
  import Login from "./components/Login.vue";

  // default payload, may make gdrive version
  const should_show = ref(false);
  invoke("close_splash").then(() => {
    console.log("invoke(\"close_splash\").then(() ran")
    should_show.value = true;
  });

  function handleGG() {
    isLoggedIn.value = false;
    isStarted.value = false;
  }

  let isLoggedIn = ref(false);
  let isStarted = ref(false);
</script>

<template>
  <div v-if="should_show">
    <div v-if="!isStarted">
      <StartScreen @start="isStarted = true" />
    </div>
    <div v-else>
      <transition name="fade">
  
        <div v-if="!isLoggedIn">         
          <Login @login="isLoggedIn = true" />
        </div> 

        <div v-else>
          <MainPage @gg="handleGG"/>
        </div>

      </transition>
    </div>
  </div>
  <div v-else>
    <Loading/>
  </div>
</template>
