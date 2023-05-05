<template>
  <div class="container mx-auto bg-black">
    <h1 class="text-4xl mb-8">Game List</h1>
    <div class="grid grid-cols-2 gap-6">
      <div
        v-for="game in games"
        :key="game.id"
        class="p-6 rounded shadow bg-slate-500"
      >
        <img :src="game.img_url" :alt="game.name" class="w-full h-48 object-cover mb-4 rounded" />
        <h2 class="text-2xl mb-4">{{ game.name }}</h2>
        <p class="mb-4">{{ game.description }}</p>
        <input v-model="game.year" type="text" class="mb-4" />
        <input v-model="game.rating" type="text" class="mb-4" />
        <!-- add for every item in .json  -->
        <div class = " rounded-sm text-white text-slate-900 py-4">
          video_url:
          <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.video_url" type="text" />
        </div> 
        <input v-model="game.video_url" type="text" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import gamesData from './games.json';

const games = ref(gamesData);
</script>