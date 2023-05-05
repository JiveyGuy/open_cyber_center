<script setup lang="ts">
import { Ref, ref } from 'vue'

const images = [
  { id: '1', src: '/src/assets/download.png' },
  { id: '2', src: '/src/assets/download.png' },
  { id: '3', src: '/src/assets/download.png' },
  { id: '4', src: '/src/assets/download.png' },
  { id: '5', src: '/src/assets/download.png' },
]

const sTierImages: any[] = []

function handleDragStart(event: DragEvent, id: string) {
  event.dataTransfer?.setData('text/plain', id)
}

function handleDrop(event: DragEvent) {
  event.preventDefault()
  const id = event.dataTransfer?.getData('text/plain')
  if (id) {
    const image = images.find(img => img.id === id)
    if (image) {
      sTierImages.push(image)
    }
  }
}

function handleDragOver(event: DragEvent) {
  event.preventDefault()
}

</script>

<template>
  <div class="w-screen h-screen bg-black">
    <div class="flex justify-center items-center h-full">
      <div class="grid grid-cols-5 gap-4">
        <div
          v-for="image in images"
          :key="image.id"
          class="bg-white rounded-lg h-28 w-28 flex justify-center items-center"
          draggable="true"
          @dragstart="handleDragStart($event, image.id)"
        >
          <img :src="image.src" class="h-full w-full object-contain" />
        </div>
      </div>
      <div class="bg-green-500 rounded-lg h-96 w-64 flex flex-col items-center justify-center ml-8"
           @drop="handleDrop"
           @dragover="handleDragOver"
      >
        <h2 class="text-2xl font-bold text-white mb-4">S Tier</h2>
        <div v-for="image in sTierImages" :key="image.id">
          <img :src="image.src" class="h-24 w-24 mb-2" />
        </div>
      </div>
    </div>
  </div>
</template>
