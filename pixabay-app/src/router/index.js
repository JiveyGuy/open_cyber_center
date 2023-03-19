
import Vue from 'vue'
import Router from 'vue-router'
import Home from "E:/C S 448/Code/GITHUB-REPO/open_cyber_center/pixabay-app/src/views/Home.vue";
import ImageSearch from "E:/C S 448/Code/GITHUB-REPO/open_cyber_center/pixabay-app/src/views/ImageSearch.vue";
import VideoSearch from "E:/C S 448/Code/GITHUB-REPO/open_cyber_center/pixabay-app/src/views/VideoSearch.vue";
Vue.use(Router)
export default new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/ImageSearch',
      name: 'Imagesearch',
      component: ImageSearch
    },
    {
      path: '/VideoSearch',
      name: 'Videosearch',
      component: VideoSearch
    }
  ]
})