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
  import AccountCog from 'vue-material-design-icons/AccountCog.vue';
  import ExitToApp from 'vue-material-design-icons/ExitToApp.vue';

  // let name = ref("");

</script>

<template>


    <div class="flex flex-col h-screen w-64 items-center justify-center bg-gray-900 relative text-Foreground rounded-br-xl text-lg">   
      <div>

      <div class="mx-auto items-center h-[155px] w-[155px] ">
        <div>
          <img src='../assets/Square310x310Logo.png' />
        </div>
        
      </div>

        <div class="flex flex-col items-center">
          <div>
            <ProfileView/>
          </div>

          <div class="flex flex-row my-6 items-center cursor-pointer">
            <HomeOutline fillColor="#f8f8f2" :size="20" />
            <div class="mx-6 items-center">
              HOME
            </div>
          </div>  
          
          <div class="flex flex-row my-6 items-center text-Red cursor-pointer">
            <VideoBox fillColor="#ff5555" :size="20" />
            <div class="mx-6 items-center">
              LIVE
            </div>
          </div>  

          <div class="flex flex-row my-6 items-center cursor-pointer">
            <AccountGroup fillColor="#f8f8f2" :size="20" />
            <div class="mx-6 items-center text-sm">
              PLAY2GETHER
            </div>
          </div>  

          <div class="flex flex-row my-6 items-center cursor-pointer">
            <AccountCash fillColor="#f8f8f2" :size="20" />
            <div class="mx-6 items-center">
              FUNDS
            </div>
          </div>  

          <div class="flex flex-row my-6 items-center cursor-pointer">
            <AccountCog fillColor="#f8f8f2" :size="20"/>
            <div class="mx-6 items-center">
              ACCOUNT
            </div>
          </div>  

          <div class=" flex flex-row my-6 items-center text-Yellow cursor-pointer">
            <ExitToApp fillColor="#f1fa8c" :size="20" />
            <div class="mx-6 items-center">
              EXIT
            </div>
          </div> 

        </div>
      </div>
    </div>
</template>
