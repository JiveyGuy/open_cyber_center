<script setup lang="ts">
  import { ref } from "vue";
  import VideoBackground from 'vue-responsive-video-background-player'

  import start_video from '../assets/large_files/vids/start_video.mp4';
  import start_music from '../assets/large_files/music/start_music.mp3';
  let music_obj = new Audio(start_music);
  music_obj.loop = true;
  music_obj.play();

  const emits = defineEmits<{(id: 'start'): void}>();

  
  let handle_start = function() {
    // This callback will be triggered when the user selects or login to
    // his Google account from the popup

    music_obj.pause();
    
    emits("start");
  };

</script>

<template>
  
  <div class="h-fit bg-black flex justify-center items-center">
    <button @click="handle_start" type="submit" class="whitespace-nowrap overflow-hidden">
      <div class="w-screen h-screen relative z-0">
        <video-background 
          :src=start_video
          class="h-screen"
        />
      </div>
    </button>
  </div>
</template>