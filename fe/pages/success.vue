<template>
  <div class="flex items-center justify-center h-full">
    <div>
      <h2 class="text-3xl font-bold mb-8">Upload Successful!</h2>
      <canvas ref="confettiCanvas" class="w-64 h-64"></canvas>
    </div>
  </div>
</template>

<script setup>
import ConfettiGenerator from 'confetti-js';
import { useRouter } from 'vue-router';

const router = useRouter();

// Create confetti animation
const confettiSettings = { target: 'confettiCanvas' };
const confettiCanvas = document.querySelector("[ref='confettiCanvas']");
if (confettiCanvas instanceof HTMLCanvasElement) {
  const confetti = new ConfettiGenerator(confettiSettings);
  confetti.render();

  // Redirect to home page after 3 seconds
  setTimeout(() => {
    router.push('/');
  }, 3000);
} else {
  console.error("Target element 'confettiCanvas' not found or not a canvas element.");
}
</script>

<style scoped>
/* Add any custom styling here */
</style>
