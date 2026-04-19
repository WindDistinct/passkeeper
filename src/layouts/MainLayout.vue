<template>
    <div class="h-screen bg-zinc-950 text-white flex">
      <AppSidebar />
  
      <div class="flex-1 flex flex-col">
        <AppTopbar />
  
        <main class="p-6">
          <slot />
        </main>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import AppSidebar from '../components/AppSidebar.vue'
  import AppTopbar from '../components/AppTopbar.vue'
  </script>
  