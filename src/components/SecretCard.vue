<template>
    <div class="bg-zinc-900 border border-zinc-800 rounded-xl p-4">
      <div class="flex items-center justify-between">
        <h3 class="font-medium">{{ item.title }}</h3>
        <span class="text-xs text-zinc-400 uppercase">
          {{ item.type }}
        </span>
      </div>
  
      <p class="text-sm text-zinc-500 mt-2">
        {{ item.username || 'No username' }}
      </p>
    </div>
  </template>
  
  <script setup lang="ts">
  import type { SecretItem } from '../types/secret'
  
  defineProps<{
    item: SecretItem
  }>()
  </script>
  