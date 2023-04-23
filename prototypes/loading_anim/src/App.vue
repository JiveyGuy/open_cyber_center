<template>
    <div class="w-screen h-screen bg-black">
      <div class="flex justify-center items-center h-full">
        <div class="relative">
          <div class="absolute top-0 left-0 w-full h-full flex justify-center items-center">
            <div class="flex flex-col">
              <div class="text-white text-lg mb-2">S Tier</div>
              <div ref="sTier" class="flex flex-wrap">
                <div v-for="(image, index) in sTierImages" :key="index" class="w-24 h-24 mr-2 mb-2">
                  <img :src="image" class="w-full h-full object-cover rounded-lg shadow-md">
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import interact from 'vue-interactjs';
  
  const sTierImages = [
    'https://picsum.photos/id/237/200/300',
    'https://picsum.photos/id/238/200/300',
    'https://picsum.photos/id/239/200/300',
    'https://picsum.photos/id/240/200/300',
  ];
  
  interact('.draggable').draggable({
    modifiers: [
      interact.modifiers.restrictRect({
        endOnly: true,
        elementRect: { top: 0, left: 0, bottom: 1, right: 1 },
      }),
    ],
    listeners: {
      move(event) {
        const target = event.target;
        const x = (parseFloat(target.getAttribute('data-x')) || 0) + event.dx;
        const y = (parseFloat(target.getAttribute('data-y')) || 0) + event.dy;
        target.style.webkitTransform = target.style.transform = `translate(${x}px, ${y}px)`;
        target.setAttribute('data-x', x);
        target.setAttribute('data-y', y);
      },
    },
  });
  
  </script>
  