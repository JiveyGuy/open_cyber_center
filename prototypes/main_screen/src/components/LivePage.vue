<script setup lang="ts">

import { ref } from "vue";
// import VueTwitchPlayer from 'vue-twitch-player';

const clipUrl = ref("https://clips.twitch.tv/BashfulHelpfulSalamanderPrimeMe");

</script>

<template>
  <!-- LIVE-PAGE --> 

  <div>
    <iframe
      :src="clipUrl.valueOf"
      height="378"
      width="620"
      frameborder="0"
      scrolling="no"
      allowfullscreen="true">
    </iframe>
  </div>

</template>