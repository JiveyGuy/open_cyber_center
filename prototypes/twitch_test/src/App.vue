<script setup lang="ts">
// import { TwitchEmbed, TwitchEmbedLayout } from 'twitch-player';
const embed = new TwitchEmbed('twitch-player', {
        width: 1280,
        height: 720,
        channel: '<channel ID here>',
        layout: TwitchEmbedLayout.VIDEO_WITH_CHAT
      });
</script>

<template>
 
</template>
