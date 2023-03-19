<template>
    <div class="center">
      <div class="results">
        <md-card v-for="r in searchResults" :key="r.id">
          <md-card-media v-if="type == 'image'">
            <img :src="r.previewURL" class="image" />
          </md-card-media>
          <md-card-media v-if="type == 'video'">
            <video class="image">
              <source :src="r.videos.tiny.url" type="video/mp4" />
            </video>
          </md-card-media>
        </md-card>
      </div>
    </div>
  </template>
  <script>
  export default {
    name: "results",
    props: {
      type: String
    },
    computed: {
      searchResults() {
        return this.$store.state.searchResults;
      }
    },
    data() {
      return {};
    }
  };
  </script>
  <!-- Add "scoped" attribute to limit CSS to this component only -->
  <style scoped lang="scss">
  .md-card {
    width: 30vw;
    margin: 4px;
    display: inline-block;
    vertical-align: top;
  }
  </style>