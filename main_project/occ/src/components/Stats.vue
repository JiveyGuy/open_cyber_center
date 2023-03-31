<script setup lang="ts">
  import { ref } from "vue";
  import { invoke } from "@tauri-apps/api/tauri";


  
  let name = ref("");
</script>

<template>
  <div class="flex justify-between bg-gray-900 p-4 rounded-lg shadow-lg">
    <div class="flex flex-row items-center">
      <span class="text-green-500 text-1xl font-bold">{{ "41" }}</span>
      <span class="text-gray-500 text-sm uppercase mx-1">Online</span>
    </div>
    <div class="flex flex-row items-center">
      <span class="text-blue-500 text-1xl font-bold">{{ "3.2k" }}</span>
      <span class="text-gray-500 text-sm uppercase mx-1">Total</span>
    </div>
    <div class="flex flex-row items-center">
      <span class="text-pink-500 text-1xl font-bold">{{ "4.7k"  }}</span>
      <span class="text-gray-500 text-[12px] uppercase mx-1">Discord</span>
    </div>
  </div>
</template>