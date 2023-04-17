// import essentials
import { createApp } from "vue";
import App from "./App.vue";

// import optionals
import "./styles.css";

// document.addEventListener('contextmenu', event => event.preventDefault());
const my_app = createApp(App);

//  for dev: 378618892902-lj68e95dn4k74nd4fdq6bq5rnucjf792.apps.googleusercontent.com
let gauthClientId = '378618892902-lj68e95dn4k74nd4fdq6bq5rnucjf792.apps.googleusercontent.com';

import GoogleSignInPlugin from "vue3-google-login"
my_app.use(GoogleSignInPlugin, {
  clientId: gauthClientId,
},);

my_app.mixin ({ 
  methods: {
    // define methods here
    home_selected() {
      window.location.href = 'https://google.com'
    },
    live_selected() {
      window.location.href = 'https://www.twitch.tv/nmstateesports'
    },
    play_selected() {
      window.location.href = 'https://youtube.com'
    },
    fund_selected() {
      window.location.href = 'https://facebook.com'
    },
    tier_selected() {
      window.location.href = 'https://monkeytype.com'
    },
    gg_selected() {
      window.location.href = 'https://apple.com'
    },
    myMethod() {
      // Navigate to another page
      this.$router.push('/other-page');
    },
  },

})

my_app.mount("#app");
