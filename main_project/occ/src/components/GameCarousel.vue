<script lang="ts">
  import { ref } from 'vue';
  import { defineComponent } from 'vue'
  import games from '../games.json';

  interface Game {
    id: number;
    name: string;
    description: string;
    year: string;
    rating: string;
    video_url: string;
    img_url: string;
  }

  export default defineComponent({
    name: 'GameList',
    data() {
      return {
        games: [] as Game[],
      };
    },
    mounted() {
      this.games = games;
    },

    setup() {
      const carouselTrack = ref<HTMLElement | null>(null);
      let startX: number | null = null;
      let currentTranslate = 0;
    
      function handleMouseDown(event: MouseEvent) {
        startX = event.clientX;
        carouselTrack.value?.classList.add('grabbing');
      }
    
      function handleMouseMove(event: MouseEvent) {
        if (startX === null) return;
        const distance = event.clientX - startX;
        currentTranslate = distance;
        carouselTrack.value!.style.transform = `translateX(${distance}px)`;
      }
    
      function handleMouseUp() {
        startX = null;
        carouselTrack.value?.classList.remove('grabbing');
        carouselTrack.value!.style.transform = '';
      }
    
      return {
        carouselTrack,
        handleMouseDown,
        handleMouseMove,
        handleMouseUp,
      };
    },
  });
</script>


<template>
  <div class="flex p-5 bg-gray-900 rounded-lg shadow-lg overflow-hidden">
    <div class = "items-center my-0 mx-4" style="position: relative; height: 200px;  ;">
      <div
        class="carousel-track absolute top-0 left-0 w-full h-full flex"
        ref="carouselTrack"
        @mousedown="handleMouseDown"
        @mousemove="handleMouseMove"
        @mouseup="handleMouseUp"
      >
      <div v-for="game in games" :key="game.id">

        <div class="carousel-item flex-none w-40 h-40 px-2 shadow-sm">
          <img :src = "game.img_url" :alt="game.name" class="rounded-3xl" />
        </div>

       </div>
       </div>

    </div>
  </div>
</template>