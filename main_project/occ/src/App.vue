<script setup lang="ts">
  // This starter template is using Vue 3 <script setup> SFCs
  // Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
  // import Greet from "./components/Greet.vue";

  // 
  import { ref } from "vue";
  import { onMounted } from "vue";

  import SearchBar from "./components/SearchBar.vue";
  import TopBar from "./components/TopBar.vue";
  import Stats from "./components/Stats.vue";
  import GameCarousel from "./components/GameCarousel.vue"
  import Login from "./components/Login.vue";
  import StartScreen from "./components/StartScreen.vue";

  import movies from './movies.json';
  
  let isLoggedIn = ref(false);
  let isStarted = ref(false);

  function handleLogin() {
    // add logic to set the loggedIn state to true
    isLoggedIn.value = true;
  }

  function handleStart() {
    // add startup anims
    isStarted.value = true;
  }

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
            <div class="fixed w-full h-screen bg-purple-900">
          <div class="h-32 ">
            <div class="grid-container" >
              <div class="grid grid-cols-12 gap-0 grid-rows-8 scrollbar scrollbar-thumb-amber-300 scrollbar-track-slate-600"  style="height: calc(100% - 64px); overflow: scroll;">

                <div class="col-start-1 row-span-6 row-start-1 col-span-2 m-4">
                  <TopBar/>
                </div>

                <div class="items-center row-start-1 row-span-1 col-start-3 col-end-8 m-4">
                  <Stats/>
                </div>

                <div class="items-center row-start-1 col-start-8 col-end-13 m-4">
                  <SearchBar/>
                </div>  

                <div class="items-center row-start-2 row-span-1 col-start-3 col-end-13 m-4 text-amber-500">
                  <GameCarousel/>
                </div>

                <div class="items-center row-start-3 row-span-1 col-start-3 col-end-13 m-4 text-amber-500">
                  <GameCarousel/>
                </div>

                <div class="items-center row-start-4 row-span-1 col-start-3 col-end-13 m-4 text-amber-500">
                  <GameCarousel/>
                </div>
              </div>
              </div>
            </div>
          </div>
          </div>
      </transition>
    </div>
  </transition>
</template>
