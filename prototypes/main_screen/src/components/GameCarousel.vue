<script lang="ts">
  import { ref } from 'vue';
  import { defineComponent } from 'vue'
  import games from '../games.json';
  import { invoke } from "@tauri-apps/api/tauri";

  interface Game {
    id: number;
    name: string;
    description: string;
    year: string;
    rating: string;
    video_url: string;
    img_url: string;
    exe_url: string;
  }


  export default defineComponent({
    name: 'GameList',
    data() {
      return {
        games: [] as Game[]
      };
    },
    mounted() {
      this.games = games;
    },

    setup() {
      const imageTrack = ref<HTMLElement | null>(null);
      const imageElem = ref<HTMLElement | null>(null);
      console.log("images");

      let mouseDownAt = ref("0");
      let prevPercentage= ref("0%");
      let g_percentage = ref("0%");

      async function handle_click(name: string)
      {
        await invoke("run_game", { name: name});
      };

      function handleOnDown (e: MouseEvent)
      {
        mouseDownAt.value = e.clientX.toString();
      }

      function handleOnDown2 (payload: TouchEvent)
      {
        mouseDownAt.value = payload.touches[0].clientX.toString();
      }

      function handleOnUp()
      {
        mouseDownAt.value = "0";  
        prevPercentage.value = g_percentage.value;
      }

      function handleOnMove(e: MouseEvent)
      {
        if(mouseDownAt.value === "0") return;
        
        const mouseDelta = parseFloat(mouseDownAt.value) - e.clientX,
              maxDelta = window.innerWidth / 2;
        
        const percentage = (mouseDelta / maxDelta) * -10,
              nextPercentageUnconstrained = parseFloat(prevPercentage.value) + percentage,
              nextPercentage = Math.max(Math.min(nextPercentageUnconstrained, 0), -100);
        
        g_percentage.value = nextPercentage.toString();
        
        imageTrack.value!.animate({
          transform: `translate(${nextPercentage}%, -50%)`
        }, { duration: 1200, fill: "forwards" });
        
        for(const image of imageTrack.value!.getElementsByClassName("image")) {
          image!.animate({
            objectPosition: `${100 + nextPercentage}% center`
          }, { duration: 1200, fill: "forwards" });
        }
      }

      function handleOnMove2(payload: TouchEvent)
      {
        let e = payload.touches[0];
        if(mouseDownAt.value === "0") return;
        
        const mouseDelta = parseFloat(mouseDownAt.value) - e.clientX,
              maxDelta = window.innerWidth / 2;
        
        const percentage = (mouseDelta / maxDelta) * -10,
              nextPercentageUnconstrained = parseFloat(prevPercentage.value) + percentage,
              nextPercentage = Math.max(Math.min(nextPercentageUnconstrained, 0), -100);
        
        g_percentage.value = nextPercentage.toString();
        
        imageTrack.value!.animate({
          transform: `translate(${nextPercentage}%, -50%)`
        }, { duration: 1200, fill: "forwards" });
        
        for(const image of imageTrack.value!.getElementsByClassName("image")) {
          image!.animate({
            objectPosition: `${100 + nextPercentage}% center`
          }, { duration: 1200, fill: "forwards" });
        }
      }
    
      return {
        imageTrack,
        imageElem,
        handleOnDown,
        handleOnDown2,
        handleOnMove,
        handleOnMove2,
        handleOnUp,
        handle_click
      };
    },
  });
</script>


<template>
  <div @mouseleave="handleOnUp">
    <div id="image_track" data-mouse-down-at="0" data-prev-percentage="0" ref="imageTrack" class="my_style"
          @mousedown="handleOnDown"
          @touchstart="handleOnDown2"
          @touchend="handleOnUp"
          @mousemove="handleOnMove"
          @mouseup="handleOnUp"
          @touchmove="handleOnMove2"
          >
      <div v-for="game in games" :key="game.id">
        <button @dblclick="handle_click(game.exe_url)" type="submit" class="relative">
          <img class="image" :src="game.img_url" draggable="false"/>
          <div class="absolute bottom-0 left-0 w-full z-10">
            <div class="bg-gradient-to-t from-black to-transparent p-10">
              <div class="font-bold text-white text-left">{{ game.name }}</div>
              <div class="text-center">
                <div class="font-ocr-a text-gray-500 text-base">{{ game.rating }}</div>
              </div>
            </div>
          </div>
        </button>
      </div>
    </div>
  </div>
</template>






<style scoped>
.my_style {
  margin: 0rem;
  width: max-content;
}

#image_track {
  display: flex;
  gap: 4vmin;
  transform: translate(0%, -50%);
  user-select: none;
}

#image_track > button {
  display: flex;
  justify-content: center;
  align-items: center;
}

#image_track > div > button > .image {
  width: 40vmin;
  height: 56vmin;
  object-fit: cover;
  object-position: 100% center;
}
</style>
