<!-- src/components/PeopleList.vue -->

<template>
  <div class="p-4">
    <h1 class="text-2xl font-bold mb-4">People List</h1>
    <ul>
      <li v-for="(person, index) in people" :key="index">
        <input
          type="text"
          v-model="person.name"
          class="border border-gray-400 rounded-md p-2 mr-2"
        />
        <input
          type="text"
          v-model="person.address"
          class="border border-gray-400 rounded-md p-2"
        />
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

const people = [
  { name: "Alice", address: "123 Main St" },
  { name: "Bob", address: "456 Oak Ave" },
  { name: "Charlie", address: "789 Elm Rd" },
];

export default defineComponent({
  name: "PeopleList",
  setup() {
    return { people };
  },
});
</script>

<style>
li {
  margin-bottom: 0.5rem;
}
</style>
