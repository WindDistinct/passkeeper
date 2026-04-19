<template>
  <MainLayout>
    <div class="border border-dashed border-zinc-800 rounded-2xl h-full min-h-125 p-6">
      <h2 class="text-2xl font-semibold mb-4">Your Secrets</h2>
      <!-- <p class="text-zinc-400">Your secrets will appear here.</p> -->
      <SecretList />
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import MainLayout from './layouts/MainLayout.vue'
import SecretList from './components/SecretList.vue'
</script>
