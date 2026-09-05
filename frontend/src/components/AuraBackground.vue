<script setup lang="ts">
import { onMounted, useTemplateRef } from 'vue';
import anime from 'animejs';

const containerRef = useTemplateRef<HTMLDivElement>('container');

onMounted(() => {
  const container = containerRef.value;
  if (!container) return;

  container.innerHTML = '';
  const particleCount = 28;
  const colors = [
    'rgba(168, 85, 247, 0.45)',
    'rgba(192, 132, 252, 0.35)',
    'rgba(6, 182, 212, 0.4)',
    'rgba(59, 130, 246, 0.35)',
    'rgba(236, 72, 153, 0.25)',
  ];

  for (let index = 0; index < particleCount; index++) {
    const element = document.createElement('div');
    const size = Math.floor(Math.random() * 22) + 6;
    const isPixel = Math.random() > 0.35;

    element.className = 'absolute pointer-events-none';
    element.style.width = `${size}px`;
    element.style.height = `${size}px`;
    element.style.left = `${Math.random() * 100}%`;
    element.style.top = `${Math.random() * 100}%`;
    element.style.backgroundColor = colors[Math.floor(Math.random() * colors.length)];
    element.style.borderRadius = isPixel ? '2px' : '4px';
    element.style.opacity = '0.15';
    container.appendChild(element);

    anime({
      targets: element,
      translateX: () => anime.random(-60, 60),
      translateY: () => anime.random(-40, 40),
      scale: () => [1, anime.random(12, 30) / 10],
      opacity: () => [0.12, 0.3],
      duration: () => anime.random(4000, 12000),
      direction: 'alternate',
      loop: true,
      easing: 'easeInOutSine',
    });
  }
});
</script>

<template>
  <div ref="container" class="absolute inset-0 overflow-hidden pointer-events-none z-0" />
</template>
