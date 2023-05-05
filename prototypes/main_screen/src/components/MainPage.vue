<script setup lang="ts">
  import { ref } from "vue";
  import SearchBar from "./SearchBar.vue";
  import TopBar from "./TopBar.vue";
  import Stats from "./Stats.vue";
  import GameCarousel from "./GameCarousel.vue"
  import OtherPage from "./OtherPage.vue";


  let isLivePage = ref(false);
  let isTierPage = ref(false);
  let isPlayPage = ref(false);
  let isFundPage = ref(false);

  let isHomePage = ref(false);
  

  function showLivePage() {
    isLivePage.value = true;
    isFundPage.value = false;
    isHomePage.value = false;
  }

  function showFundPage() {
    isFundPage.value = true;
    isLivePage.value = false;
    isHomePage.value = false;
  }

  function showHomePage() {
    isHomePage.value = true;
    isLivePage.value = false;
    isFundPage.value = false;
  }


</script>

<template>
<div class="fixed w-full h-screen bg-purple-900">
  <div class="h-32 ">
    <div class="grid-container" >
      <div class="grid grid-cols-12 gap-0 grid-rows-8 scrollbar scrollbar-thumb-amber-300 scrollbar-track-slate-600"  style="height: calc(100% - 64px); overflow: scroll;">
          
        <div class="col-start-1 row-span-6 row-start-1 col-span-2 m-4">
          <TopBar @live="showLivePage" 
                  @fund="showFundPage" 
                  @home="showHomePage"/>
        </div>

          <div class="items-center row-start-1 row-span-1 col-start-3 col-end-8 m-4">
            <Stats/>
          </div>

          <div class="items-center row-start-1 col-start-8 col-end-13 m-4">
            <SearchBar/>
          </div>  

          <div class="items-center row-start-2 row-span-1 col-start-3 col-end-13 m-4 text-amber-500">
            <GameCarousel/>
          </div>

          <!-- div section for HOME page -->
          <div v-if="isHomePage && !isLivePage && !isFundPage">
            <MainPage />
          </div>

          <!-- div section for FUND page -->
          <div v-if="!isHomePage && !isLivePage && isFundPage">
            <OtherPage />
          </div>

          <!-- div section for LIVE page -->  
          <div v-if="!isHomePage && isLivePage && isFundPage">
            <OtherPage />
          </div>

          <!-- div section for other pages when conditions are not met-->
          <div v-else>
            <OtherPage />
          </div>

        </div>
      </div>
    </div>
  </div>
</template>
