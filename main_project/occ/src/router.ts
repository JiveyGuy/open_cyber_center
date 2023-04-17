import Vue from 'vue';
import VueRouter from 'vue-router';
import MainPage from './components/MainPage.vue';
import OtherPage from './components/OtherPage.vue';

Vue.use(VueRouter);

const routes = [
  { path: '/', component: MainPage },
  { path: '/other-page', component: OtherPage },
];

const router = new VueRouter({
  mode: 'history',
  routes,
});

export default router;
