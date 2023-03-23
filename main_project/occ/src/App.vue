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

  import movies from './movies.json';

</script>

<template>
  <div class="fixed w-full h-screen bg-Background">
    <div class="grid grid-cols-12 gap-0">

      <div class="col-start-1 col-span-2">
        <TopBar/>
      </div>

      <div class="items-center col-start-3 col-end-7 my-2 mx-4">
        <Stats/>
      </div>

      <div class="items-center col-start-11 col-end-12 my-2">
        <SearchBar/>
      </div>  

      <!-- <div class="items-center row-start col-start-3 my-2">
        <GameCarousel/>
      </div> -->


    </div>
  </div>
</template>
