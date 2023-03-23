<script setup lang="ts">

  
  import { ref } from "vue";
  import { invoke } from "@tauri-apps/api/tauri";

  let img = ("https://media.istockphoto.com/id/1307387607/vector/anonymous-vector-icon-incognito-sign-privacy-concept-human-head-with-glitch-face-personal.jpg?s=612x612&w=0&k=20&c=gABTCg7-xeCyeWDF-JnNzPbpnCeTX8wH-TxIYdkC0yQ=");
  let name = ("player");
  let level = ("lvl 10");
  let timeLeft = ("10:00");

</script>

<template>
  <div class="p-4">
  <div class="bg-gray-900 rounded-lg overflow-hidden border-4 border-Green">
    <div class="relative">
      <img class="w-full h-43 object-cover" :src="img" alt="User's profile picture">
      <div class="absolute bottom-0 left-0 right-0 px-6 py-4 bg-gradient-to-t from-gray-900 to-transparent">
        <h2 class="text-2xl shadow-lg font-bold text-white">{{ name }}</h2>
        <div class="flex items-center text-gray-400 mt-2">
          <div class="flex items-center mr-4">
            <svg class="h-5 w-5 fill-current text-green-500 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M20 10a10 10 0 11-20 0 10 10 0 0120 0zm-8.59 3.29a1 1 0 01-1.41 0L8 11.41V15a1 1 0 01-2 0V9a1 1 0 01.5-.87L10 6.58l3.5 3.55a1 1 0 010 1.41z"/></svg>
            <span class="text-sm">{{ level }}</span>
          </div>
          <div class="flex items-center">
            <svg class="h-5 w-5 fill-current text-gray-400 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M10 0a10 10 0 0110 10c0 5.52-4.48 10-10 10a10 10 0 01-10-10c0-5.52 4.48-10 10-10zm0 17.5a7.5 7.5 0 110-15 7.5 7.5 0 010 15zM9 9a1 1 0 112 0v4a1 1 0 11-2 0V9zm1-4a1 1 0 00-1 1v2a1 1 0 102 0V6a1 1 0 00-1-1z"/></svg>
            <span class="text-sm">{{ timeLeft }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
</template>