import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
// document.addEventListener('contextmenu', event => event.preventDefault());
const my_app = createApp(App);

//  for dev: 378618892902-lj68e95dn4k74nd4fdq6bq5rnucjf792.apps.googleusercontent.com
let gauthClientId = '378618892902-lj68e95dn4k74nd4fdq6bq5rnucjf792.apps.googleusercontent.com';

import GoogleSignInPlugin from "vue3-google-login"

my_app.use(GoogleSignInPlugin, {
  clientId: gauthClientId,
});

my_app.mount("#app");
