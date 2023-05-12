<script setup lang="ts">
  import { ref } from "vue";
  import SearchBar from "./SearchBar.vue";
  import TopBar from "./TopBar.vue";
  import Stats from "./Stats.vue";
  import GameCarousel from "./GameCarousel.vue"
  import TierPage from "./TierPage.vue";
  import LivePage from "./LivePage.vue";


  const emits = defineEmits<{
    (id: 'gg'): void, 
  }>();

  let isLivePage = ref(false);
  let isTierPage = ref(false);
  let isPlayPage = ref(false);
  let isFundPage = ref(false);
  let isHomePage = ref(true);


  // function for LIVE button
  function showLivePage() {
    isLivePage.value = true;
    isTierPage.value = false;
    isPlayPage.value = false;
    isFundPage.value = false;
    isHomePage.value = false;
  }

  // function for TIER button 
  function showTierPage() {
    isLivePage.value = false;
    isTierPage.value = true;
    isPlayPage.value = false;
    isFundPage.value = false;
    isHomePage.value = false;
  }

  // function for PLAY button 
  function showPlayPage() {
    isLivePage.value = false;
    isTierPage.value = false;
    isPlayPage.value = true;
    isFundPage.value = false;
    isHomePage.value = false;
  }

  // function for FUND button 
  function showFundPage() {
    isLivePage.value = false;
    isTierPage.value = false;
    isPlayPage.value = false;
    isFundPage.value = true;
    isHomePage.value = false;
  }

  // function for HOME button
  function showHomePage() {
    isLivePage.value = false;
    isTierPage.value = false;
    isPlayPage.value = false;
    isFundPage.value = false;
    isHomePage.value = true;
  }

  // gg will log person out
  //  then go to login screen
  function handleGG() {
    emits("gg");
  }

</script>

<template>
<div class="w-full h-full bg-purple-900">
  <div class="">
    <div class="grid-container " >
      <div class="grid grid-cols-12 gap-0 grid-rows-8 " >
          
        <div class="col-start-1 row-span-6 row-start-1 col-span-2">
          <TopBar @live="showLivePage"
                  @tier="showTierPage"
                  @play="showPlayPage"
                  @fund="showFundPage" 
                  @home="showHomePage"
                  @gg="handleGG"
                  />
        </div>

          <div class="items-center row-start-1 row-span-1 col-start-3 col-end-8 m-4">
            <Stats/>
          </div>

          <div class="items-center row-start-1 col-start-8 col-end-13 m-4">
            <SearchBar/>
          </div>  

          
          
          <!-- div section for LIVE page -->  
          <div v-if="isLivePage && !isTierPage && !isPlayPage && !isFundPage && !isHomePage">
            <LivePage/>
          </div>
          
          <!-- div section for TIER page -->  
          <div v-if="!isLivePage && isTierPage && !isPlayPage && !isFundPage && !isHomePage">
            <TierPage/>
          </div>

          <!-- div section for PLAY page -->  
          <div v-if="!isLivePage && !isTierPage && isPlayPage && !isFundPage && !isHomePage">
            PLAY page
          </div>

          <!-- div section for FUND page -->
          <div v-if="!isLivePage && !isTierPage && !isPlayPage && isFundPage && !isHomePage">
            FUND page
          </div>

          <!-- div section for HOME page -->
          <div v-if="!isLivePage && !isTierPage && !isPlayPage && !isFundPage && isHomePage" class="items-center row-start-2 row-span-1 col-start-3 col-end-13 m-4">
            <div class="text-amber-500">
              Recommended
              <GameCarousel/>
          
            </div>
          </div>
         

        </div>
      </div>
    </div>
  </div>
</template>
