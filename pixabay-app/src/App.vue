<template>
  <div id="app">
    <md-toolbar>
      <md-button class="md-icon-button" @click="showNavigation = true">
        <md-icon>menu</md-icon>
      </md-button>
      <h3 class="md-title">Pixabay App</h3>
    </md-toolbar>
    <md-drawer :md-active.sync="showNavigation" md-swipeable>
      <md-toolbar class="md-transparent" md-elevation="0">
        <span class="md-title">Pixabay App</span>
      </md-toolbar>
<md-list>
        <md-list-item>
          <router-link to="/">
            <span class="md-list-item-text">Home</span>
          </router-link>
        </md-list-item>
<md-list-item>
          <router-link to="/imagesearch">
            <span class="md-list-item-text">Image Search</span>
          </router-link>
        </md-list-item>
<md-list-item>
          <router-link to="/videosearch">
            <span class="md-list-item-text">Video Search</span>
          </router-link>
        </md-list-item>
      </md-list>
    </md-drawer>
<router-view />
  </div>
</template>
<script>
export default {
  name: "app",
  data: () => {
    return {
      showNavigation: false
    };
  }
};
</script>
<style>
.center {
  text-align: center;
}
form {
  width: 95vw;
  margin: 0 auto;
}
.md-toolbar.md-theme-default {
  background: #009688 !important;
  height: 60px;
}
.md-title,
.md-toolbar.md-theme-default .md-icon {
  color: #fff !important;
}
</style>