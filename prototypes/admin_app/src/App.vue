<template>
  <div class="w-full h-full mx-auto bg-black border-gray-800 p-4 ">
  <div class="flex text-white justify-center">
    OPEN CYBER CENTER ADMIN EDITOR
  </div>
    
    <h1 class="text-4xl mb-8">Game List</h1>

    <div class="grid grid-cols-1 md:grid-cols-2 lg: grid-cols-3 gap-6">
      <div
        v-for="game in games"
        :key="game._id.$oid"
        class="rounded shadow bg-slate-500 "
      >
      
        <h2 class="text-2xl mb-4">{{ game.name }}</h2>
        <p class="mb-4">{{ game.description }}</p>
        <!-- <input v-model="game.year" type="text" class="mb-4" /> -->
        <!-- input v-model="game.rating" type="text" class="mb-4" /> -->
        
        <!-- add for every item in .json  -->
        <div class=" grid grid-cols-2"> 
        <div class = " row-span-6">
          <div class = "text-white text-slate-900">Name:
            <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.name" type="text" />
          </div>

          <div class = "text-white text-slate-900">Description:
            <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.description" type="text" />
          </div>

          <div class = "text-white text-slate-900">Year:
            <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.year" type="text" />
          </div>

          <div class = "text-white text-slate-900">Rating:
            <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.rating" type="text" />
          </div>

          <div class = "text-white text-slate-900">Video Link:
            <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.video_url" type="text" />
          </div>

          <div class = "text-white text-slate-900">Image Link:
            <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.img_url" type="text" />
          </div>

          <div class = "text-white text-slate-900">Exe Link:
            <input class = "py-2 rounded-sm text-white bg-neutral-700" v-model="game.exe_url" type="text" />
          </div>

          <div class=" rounded-lg h-fit w-fit justify-end bg-green-800 px-1 py-1 transition-all duration-200 transform hover:scale-105 " type="button" @click="update_entry(game._id.$oid, game.name, game.description, game.year, game.rating, game.video_url, game.img_url, game.exe_url)">
            <button class="text-white" >UPDATE</button>
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