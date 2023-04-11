<script setup lang="ts">
  import { ref } from "vue";
  import VueTwitchPlayer from 'vue-twitch-player';
</script>

<template>
  <twitch-player
      :channel="'nmstateesports'"
  ></twitch-player>
</template>
<!-- broken -->