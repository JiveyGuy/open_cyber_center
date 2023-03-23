import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
document.addEventListener('contextmenu', event => event.preventDefault());
const my_app = createApp(App).mount("#app");
