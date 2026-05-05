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
      {{ value ? 'Revealed (auto-hide)' : 'Show secret' }}
    </button>

    <p v-if="value" class="text-green-400 text-sm mt-2">
      {{ value }}
    </p>

    <button
      v-if="value"
      @click="handleCopy"
      class="text-xs text-blue-400 mt-2"
    >
      {{ copied ? 'Copied' : 'Copy' }}
    </button>

  </div>
</template>

<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import { decryptSecret } from '../services/secretService'
import type { SecretItem } from '../types/secret'

import { useClipboard } from '../composables/useClipboard'
import { useToastStore } from '../stores/toastStore'

const props = defineProps<{
  item: SecretItem
}>()

const value = ref('')

const { copy, copied } = useClipboard()
const toast = useToastStore()

let revealTimer: ReturnType<typeof setTimeout> | null = null

async function reveal() {
  if (!props.item.encrypted_payload) return

  if (value.value) {
    if (revealTimer) {
      clearTimeout(revealTimer)
    }
  } else {
    value.value = await decryptSecret(props.item.encrypted_payload)
  }

  revealTimer = setTimeout(() => {
    clearSecret()
  }, 8000)
}

async function handleCopy() {
  if (!value.value) return

  await copy(value.value)
  toast.show('Copied to clipboard (auto-clear in 10s)')
}

function clearSecret() {
  value.value = ''

  if (revealTimer) {
    clearTimeout(revealTimer)
    revealTimer = null
  }
}

onUnmounted(() => {
  clearSecret()
})

</script>