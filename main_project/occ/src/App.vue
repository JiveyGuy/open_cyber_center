<script setup lang="ts">
  // This starter template is using Vue 3 <script setup> SFCs
  // Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
  // import Greet from "./components/Greet.vue";

  // 
  import { ref } from "vue";
  import { onMounted } from "vue";

  
  import Login from "./components/Login.vue";
  import StartScreen from "./components/StartScreen.vue";
  import MainPage from "./components/MainPage.vue";

  // import movies from './movies.json';
  
  let isLoggedIn = ref(false);
  let isStarted = ref(false);

</script>

<template>
  <transition name="fade" clas>
    <div v-if="!isStarted">
      <StartScreen @start="isStarted = true" />
    </div>
    <div v-else>
      <transition name="fade" clas>
          <div v-if="!isLoggedIn">         
            <Login @login="isLoggedIn = true" />
          </div> 
          <div v-else>
            <MainPage />
          </div>
      </transition>
    </div>
  </transition>
</template>
