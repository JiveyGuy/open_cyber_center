<template>
  <div class="container mx-auto bg-black">
  <div class="flex text-white justify-center">
    OPEN CYBER CENTER ADMIN EDITOR
  </div>
    
    <h1 class="text-4xl mb-8">Game List</h1>
    <div class="grid grid-cols-1 gap-6">
      <div
        v-for="game in games"
        :key="game.id"
        class="p-6 rounded shadow bg-slate-500"
      >
        <img :src="game.img_url" :alt="game.name" class="w-full h-48 object-cover mb-4 rounded" />
        <h2 class="text-2xl mb-4">{{ game.name }}</h2>
        <p class="mb-4">{{ game.description }}</p>
        <!-- <input v-model="game.year" type="text" class="mb-4" /> -->
        <!-- input v-model="game.rating" type="text" class="mb-4" /> -->
        
        <!-- add for every item in .json  -->

        <div class = " rounded-sm text-white text-slate-900 py-4">
          name:  
          <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.name" type="text" />
        </div>

        <div class = " rounded-sm text-white text-slate-900 py-4">
          description:  
          <input class = "py-3 rounded-sm text-white bg-neutral-700" v-model="game.description" type="text" />
          
        </div>

        <div class = " rounded-sm text-white text-slate-900 py-4">
          year:  
          <input class = "py-3 rounded-sm text-white bg-neutral-700" v-model="game.year" type="text" />
        </div>

        <div class = " rounded-sm text-white text-slate-900 py-4">
          rating:  
          <input class = "py-3 rounded-sm text-white bg-neutral-700" v-model="game.rating" type="text" />
        </div>

        <div class = " rounded-sm text-white text-slate-900 py-4">
          video_url:  
          <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.video_url" type="text" />
        </div>

        <div class = " rounded-sm text-white text-slate-900 py-4">
          img_url:  
          <input class = "py-3 rounded-sm text-white bg-neutral-700" v-model="game.img_url" type="text" />
        </div>

        <div class = " rounded-sm text-white text-slate-900 py-4">
          exe_url:  
          <input class = "py-3 rounded-sm text-white bg-neutral-700" v-model="game.exe_url" type="text" />
        </div>
        
        <!-- Fix this connection to main.rs so that when update_entry is called it prints the info. -->
        <button class=" rounded-lg h-fit w-fit bg-green-800 px-1 py-1 " @click="update_entry(game.id, game.name, game.description, game.year, game.rating, game.video_url, game.img_url, game.exe_url)" type="submit">
          <div class="text-white" >UPDATE</div>
        </button>
        

      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import gamesData from './games.json';
import { invoke } from "@tauri-apps/api/tauri";

const games = ref(gamesData);

function hello_world(){
  console.log("hello world!")
}
// @click="update_entry(game.id, game.name, game.description, game.year, game.rating, game.video_url, game.img_url, game.exe_url)""
async function update_entry(id: number, name: string, description: string, year: string, rating: string, video_url: string, img_url: string, exe_url: string) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("update_entry", { id: id, name: name, description: description, year: year, rating: rating, video_url: video_url, img_url: img_url, exe_url: exe_url });
}
</script>