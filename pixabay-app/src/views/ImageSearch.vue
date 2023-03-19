<template>
    <div class="imagesearch">
      <div class="center">
        <h1>Image Search</h1>
      </div>
      <form @submit="search" novalidate>
        <md-field :class="{ 'md-invalid': errors.has('q') }">
          <label for="q">Keyword</label>
          <md-input type="text" name="q" v-model="searchData.q" v-validate="'required'"></md-input>
          <span class="md-error" v-if="errors.has('q')">{{errors.first('q')}}</span>
        </md-field>
  <md-field :class="{ 'md-invalid': errors.has('minWidth') }">
          <label for="minWidth">Min Width</label>
          <md-input
            type="text"
            name="minWidth"
            v-model="searchData.min_width"
            v-validate="'numeric|min_value:0'"
          ></md-input>
          <span class="md-error" v-if="errors.has('minWidth')">{{errors.first('minWidth')}}</span>
        </md-field>
  <md-field :class="{ 'md-invalid': errors.has('minHeight') }">
          <label for="minHeight">Min Height</label>
          <md-input
            type="text"
            name="minHeight"
            v-model="searchData.min_height"
            v-validate="'numeric|min_value:0'"
          ></md-input>
          <span class="md-error" v-if="errors.has('minHeight')">{{errors.first('minHeight')}}</span>
        </md-field>
  <md-field>
          <label for="movie">Colors</label>
          <md-select v-model="searchData.colors" name="colors">
            <md-option :value="c" v-for="c in colorChoices" :key="c">{{c}}</md-option>
          </md-select>
        </md-field>
  <md-button class="md-raised" type="submit">Search</md-button>
      </form>
      <Results type='image' />
    </div>
  </template>
  <script>
  // @ is an alias to /src
  import Results from "@/components/Results.vue";
  import { photosMixin } from "@/components/mixins/photosMixin";
  export default {
    name: "home",
    components: {
      Results
    },
    data() {
      return {
        photos: [],
        searchData: {},
        colorChoices: [
          "grayscale",
          "transparent",
          "red",
          "orange",
          "yellow",
          "green",
          "turquoise",
          "blue",
          "lilac",
          "pink",
          "white",
          "gray",
          "black",
          "brown"
        ]
      };
    },
    mixins: [photosMixin],
    beforeMount() {
      this.$store.commit("setSearchResults", []);
    },
    computed: {
      isFormDirty() {
        return Object.keys(this.fields).some(key => this.fields[key].dirty);
      }
    },
    methods: {
      async search(evt) {
        evt.preventDefault();
        if (!this.isFormDirty || this.errors.items.length > 0) {
          return;
        }
        const response = await this.searchPhoto(this.searchData);
        this.photos = response.data.hits;
        this.$store.commit("setSearchResults", response.data.hits);
      }
    }
  };
  </script>