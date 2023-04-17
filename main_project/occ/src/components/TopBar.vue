<script setup lang="ts">
  // import { ref } from "vue";
  // import { invoke } from "@tauri-apps/api/tauri";

  // Icons
  // import Magnify from 'vue-material-design-icons/Magnify.vue';

  import ProfileView from "./ProfileView.vue";

  import HomeOutline from 'vue-material-design-icons/HomeOutline.vue';
  import VideoBox from 'vue-material-design-icons/VideoBox.vue';
  import AccountGroup from 'vue-material-design-icons/AccountGroup.vue';
  import AccountCash from 'vue-material-design-icons/AccountCash.vue';
  import ListBox from 'vue-material-design-icons/ListBox.vue';
  import ExitToApp from 'vue-material-design-icons/ExitToApp.vue';


  // let name = ref("");

  const emits = defineEmits<{(id: 'live'): void}>();

  
  let handle_live = function() {
    // This callback will be triggered when the user selects or login to
    // his Google account from the popup    
    emits("live");
  };

</script>

<template>


    <div class="flex flex-col h-screen justify-start bg-gray-900 relative text-Foreground rounded-br-xl rounded-t-xl text-lg">   
      <div>

      <div class="mx-auto my-1 h-[128px] w-[128px] ">
        <div>
          <img src='../assets/Square310x310Logo.png' />
        </div>
        
      </div>

        <div class="flex flex-col items-center mx-auto my-auto ">
          <div>
            <ProfileView/>
          </div>
          
          <div class="mx-auto my-auto flex flex-row items-center cursor-pointer">
            <HomeOutline fillColor="#f8f8f2" :size="20" />
            <div class="mx-1 my-3 items-center">
              <button v-on:click="home_selected">HOME</button>
            </div>
          </div>  
          <button @click="handle_live" type="submit">
            <div class="mx-auto my-auto flex flex-row items-center text-Red cursor-pointer">
              <VideoBox fillColor="#ff5555" :size="20" />
              <div class="mx-1 my-3 items-center">
                <button @click="live_selected">LIVE</button>
              </div>
            </div> 
          </button> 

          <div class="flex flex-row mx-auto my-auto items-center cursor-pointer">
            <AccountGroup fillColor="#f8f8f2" :size="20" />
            <div class="mx-1 my-3 items-center text-sm">
              <button @click="play_selected">PLAY2GETHER</button>
            </div>
          </div>  

          <div class="flex flex-row mx-auto my-auto items-center cursor-pointer">
            <AccountCash fillColor="#f8f8f2" :size="20" />
            <div class="mx-1 my-3 items-center">
              <button @click="funds_selected">FUNDS</button>
            </div>
          </div>  

          <div class="flex flex-row mx-auto my-auto items-center cursor-pointer">
            <ListBox fillColor="#f8f8f2" :size="20"/>
            <div class="mx-1 my-3 items-center">
              <button @click="tier_selected">TIER LIST</button>
            </div>
          </div>  

          <div class=" flex flex-row mx-auto my-auto items-center cursor-pointer text-Yellow hover:shadow-Yellow hover:drop-shadow-lg">
              <ExitToApp fillColor="#f1fa8c" :size="20" />
              <div class="mx-1 my-3 items-center">
                <div class="hover:shadow-Green "><button @click="gg_selected">GG</button></div>
              </div>
          </div>
        </div>
      </div>
    </div>
</template>
