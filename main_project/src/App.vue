<script setup lang="ts">
  import {onMounted, ref} from 'vue';

  // Icons
  import Magnify from 'vue-material-design-icons/Magnify.vue';
  import HomeOutline from 'vue-material-design-icons/HomeOutline.vue';
  import VideoBox from 'vue-material-design-icons/VideoBox.vue';
  import AccountGroup from 'vue-material-design-icons/AccountGroup.vue';
  import AccountCash from 'vue-material-design-icons/AccountCash.vue';
  import AccountCog from 'vue-material-design-icons/AccountCog.vue';
  import ExitToApp from 'vue-material-design-icons/ExitToApp.vue';
  import ApplicationBrackets from 'vue-material-design-icons/ApplicationBrackets.vue';

  // Stores
  import {useGameStore} from './store/game';

  // Pinia instead of Vuex for states
  import {storeToRefs} from 'pinia'; 

  // States
  const useGame = useGameStore();

  // refs
  const {gamePopup} = useGameStore();



</script>

<template>
  <div class="text-red-500">
    HERE
  </div>
</template>
