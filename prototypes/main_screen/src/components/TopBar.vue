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

  const emits = defineEmits<{
    (id: 'home'): void, 
    (id: 'live'): void, 
    (id: 'play'): void, 
    (id: 'fund'): void, 
    (id: 'tier'): void,
    (id: 'gg'): void, 
  }>();

  let handle_home = function() {
    emits("home");
  };
  let handle_live = function() {
    emits("live");
  };
  let handle_play = function() {
    emits("play");
  };
  let handle_fund = function() {
    emits("fund");
  };
  let handle_tier = function() {
    emits("tier");
  };
  let handle_gg = function() {
    emits("gg");
  };

</script>

<template>


    <div class="flex flex-col h-screen bg-gray-900 text-Foreground rounded-br-xl rounded-tr-2xl  text-lg mr-10">   
      <div>

      <div class="mx-auto my-auto h-[128px] w-[128px] ">
        <div>
          <img src='../assets/Square310x310Logo.png' />
        </div>
        
      </div>

        <!-- <div class=" mx-auto my-auto "> -->
          <div>
            <ProfileView/>
          </div>

          <button @click="handle_home" type="submit">
            <div class="flex mx-10 mt-20 flex-row items-center cursor-pointer">
              <HomeOutline fillColor="#f8f8f2" :size="20" />
              <div class="mx-1 my-3 items-center">
                HOME
              </div>
            </div> 
          </button>

          <button @click="handle_live" type="submit">
            <div class="mx-10 my-auto flex flex-row items-center text-Red cursor-pointer">
              <VideoBox fillColor="#ff5555" :size="20" />
              <div class="mx-1 my-3 items-center">
                VOD
              </div>
            </div> 
          </button> 

          <button @click="handle_play" type="submit">
            <div class="flex flex-row mx-10 my-auto items-center cursor-pointer">
              <AccountGroup fillColor="#f8f8f2" :size="20" />
              <div class="mx-1 my-3 items-center text-sm">
                PLAY2GETHER
              </div>
            </div>  
          </button>

          <button @click="handle_fund" type="submit">
            <div class="flex flex-row mx-10 my-auto items-center cursor-pointer">
              <AccountCash fillColor="#f8f8f2" :size="20" />
              <div class="mx-1 my-3 items-center">
                FUNDS
              </div>
            </div>  
          </button>

          <button @click="handle_tier" type="submit">
            <div class="flex flex-row mx-10 my-auto items-center cursor-pointer">
              <ListBox fillColor="#f8f8f2" :size="20"/>
              <div class="mx-1 my-3 items-center">
                TIER LIST
              </div>
            </div>  
          </button>

          <button @click="handle_gg" type="submit">
            <div class=" flex flex-row mx-10 my-auto items-center cursor-pointer text-Yellow hover:shadow-Yellow hover:drop-shadow-lg">
              <ExitToApp fillColor="#f1fa8c" :size="20" />
              <div class="mx-1 my-3 items-center">
                <div class="hover:shadow-Green ">
                  GG
                </div>
              </div>
            </div>
          </button>

        </div>
      </div>
    <!-- </div> -->
</template>
