<template>
    <div class="fixed bottom-4 right-4 flex flex-col gap-2 z-50">
      <div
        v-for="toast in store.toasts"
        :key="toast.id"
        class="bg-zinc-900 text-white px-4 py-2 rounded-lg shadow-lg border border-zinc-700"
      >
        {{ toast.message }}
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { useToastStore } from '../stores/toastStore'
  
  const store = useToastStore()
  </script>