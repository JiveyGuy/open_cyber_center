<script setup lang="ts">
  // This starter template is using Vue 3 <script setup> SFCs
  // Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
  // import Greet from "./components/Greet.vue";

  // 
  import SearchBar from "./components/SearchBar.vue";
  import TopBar from "./components/TopBar.vue";
  import Stats from "./components/Stats.vue";

  

</script>

<template>
  <div class="fixed w-full h-screen bg-Background">
    <div class="grid grid-cols-12 gap-0">
      <div class="">
        <TopBar/>
      </div>
      <div class="items-center col-start-3 col-end-7 my-4">
        <Stats/>
      </div>

      <div class="items-center col-start-10 col-end-12 my-4">
        <SearchBar/>
      </div>  
    </div>
  </div>
</template>
