<script setup lang="ts">
  import { ref } from "vue";
  import { invoke } from "@tauri-apps/api/tauri";



  // Icons
  import Magnify from 'vue-material-design-icons/Magnify.vue';

  let searchTerm = ref("");
  let search = ref("");
</script>

<template>
  <div class="flex justify-between bg-gray-900  rounded-lg shadow-lg">
    <div class="flex items-center justify-center p-2">
      <input
        v-model="searchTerm"
        placeholder="Search"
        class="px-3 py-2 text-Foreground bg-transparent rounded-md focus:outline-none focus:shadow-blue-500 transition-colors duration-300 ease-in-out"
      />
    </div>
  </div>
</template>
