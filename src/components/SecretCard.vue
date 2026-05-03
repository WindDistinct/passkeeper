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

    <button @click="reveal" class="text-xs text-zinc-400 mt-2">
      Show secret
    </button>

    <p v-if="value" class="text-green-400 text-sm mt-2">
      {{ value }}
    </p>

  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { decryptSecret } from '../services/secretService'
import type { SecretItem } from '../types/secret'

const props = defineProps<{
  item: SecretItem
}>()

const value = ref('')

async function reveal() {
  if (!props.item.encrypted_payload) return

  value.value = await decryptSecret(props.item.encrypted_payload)
}
</script>