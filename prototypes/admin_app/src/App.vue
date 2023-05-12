<template>
<div class="w-full h-full mx-auto bg-gradient-to-r from-purple-500 via-pink-500 to-red-500 border-gray-800 p-4">
  <div class="flex text-white justify-center">
    <h1 class="text-white text-4xl font-bold" style="text-shadow: 2px 2px 0 #6B46C1, -2px -2px 0 #6B46C1;">
      OPEN CYBER CENTER ADMIN EDITOR
    </h1>
  </div>
  
  <h2 class="text-white text-3xl font-semibold underline mt-7 mr-4 mb-5">Game List</h2>
  
  <div class="grid grid-cols-1 md:grid-cols-2 lg: grid-cols-3 gap-6">
    <div
    v-for="game in games"
    :key="game._id.$oid"
    class="rounded shadow bg-violet-400 backdrop-filter backdrop-blur-sm"
    >
    
    <h2 class="text-2xl mb-5 text-gray-800 font-bold">{{ game.name }}</h2>
    <p class="mb-5 ">{{ game.description }}</p>
    
    <!-- add for every item in .json  -->
    <div class=" grid grid-cols-3"> 
      <div class = " row-span-6">

        <div class = "text-white text-slate-900 mb-4">Name:
          <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.name" type="text" />
        </div>
        
        <div class = "text-white text-slate-900 mb-4">Description:
          <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.description" type="text" />
        </div>
        
        <div class = "text-white text-slate-900 mb-4">Year:
          <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.year" type="text" />
        </div>
        
        <div class = "text-white text-slate-900 mb-4">Rating:
          <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.rating" type="text" />
        </div>
        
        <div class = "text-white text-slate-900 mb-4">Video Link:
          <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.video_url" type="text" />
        </div>
        
        <div class = "text-white text-slate-900 mb-4">Image Link:
          <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.img_url" type="text" />
        </div>
        
        <div class = "text-white text-slate-900 mb-4">Exe Link:
          <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.exe_url" type="text" />
        </div>
        
        <div class="button-container">
          <button class="rounded-lg bg-gray-800 px-1 py-1 transition-all duration-200 transform hover:scale-105 text-white" type="button" @click="update_entry(game._id.$oid, game.name, game.description, game.year, game.rating, game.video_url, game.img_url, game.exe_url)">UPDATE</button>
          <button class="rounded-lg bg-gray-800 px-1 mb-4 mt-4 py-1 transition-all duration-200 transform hover:scale-105 text-white" type="button" @click="add_entry()">PLUS</button>
        </div>

        <div class = "col-start-3 h-fit row-span-4">
          <img :src="game.img_url" :alt="game.name" class="w-full h-full object-cover mb-4 rounded" />
        </div>
      </div>
    </div>
  </div>

  </div>
</div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import gamesData from './games.json';
import { invoke } from "@tauri-apps/api/tauri";

const games = ref(gamesData);

async function update_entry(_id: string, name: string, description: string, year: string, rating: string, video_url: string, img_url: string, exe_url: string) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("update_entry", { id: _id, name: name, description: description, year: year, rating: rating, videoUrl: video_url, imgUrl: img_url, exeUrl: exe_url });
}
</script>