<script lang="ts">

  import { ref } from "vue";
  import { invoke } from "@tauri-apps/api/tauri";

  import { defineComponent } from "vue";
  import { Swiper, SwiperSlide } from "swiper/vue";
  import "swiper/swiper.scss";
  import "swiper/components/pagination/pagination.min.css";
  import "swiper/components/navigation/navigation.min.css";
  import SwiperCore, { Autoplay, Pagination, Navigation } from "swiper";
  
  SwiperCore.use([Autoplay, Pagination, Navigation]);
  
  export default defineComponent({
    name: "HelloWorld",
    components: {
      Swiper,
      SwiperSlide,
    },
    methods: {
      onSwiper(swiper: any) {
        console.log(swiper);
      },
      onSlideChange() {
        console.log("slide change");
      },
    },
  });
</script>


  

<template>
  <swiper
    :spaceBetween="30"
    :centeredSlides="true"
    :autoplay="{
      delay: 2500,
      disableOnInteraction: false,
    }"
    :pagination="{
      clickable: true,
    }"
    :navigation="true"
  >
    <swiper-slide>Slide 1</swiper-slide>
    <swiper-slide>Slide 2</swiper-slide>
    <swiper-slide>Slide 3</swiper-slide>
    <swiper-slide>Slide 4</swiper-slide>
    <swiper-slide>Slide 5</swiper-slide>
    <swiper-slide>Slide 6</swiper-slide>
    <swiper-slide>Slide 7</swiper-slide>
    <swiper-slide>Slide 8</swiper-slide>
    <swiper-slide>Slide 9</swiper-slide>
  </swiper>
</template>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
  .swiper-container {
    width: 100%;
    height: 100%;
  }
  
  .swiper-slide {
    text-align: center;
    font-size: 18px;
    background: #fff;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  
  .swiper-slide img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
</style>