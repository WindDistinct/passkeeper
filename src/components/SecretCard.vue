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

    <button
      @mousedown="startReveal"
      @mouseup="stopReveal"
      @mouseleave="stopReveal"
      @touchstart.prevent="startReveal"
      @touchend="stopReveal"
      class="text-xs text-zinc-400 mt-2"
    >
      Hold to reveal
    </button>

    <p v-if="value" class="text-green-400 text-sm mt-2">
      {{ value }}
    </p>

    <button
      @click="handleSecureCopy"
      class="text-xs text-emerald-400 mt-2"
    >
      Secure Copy
    </button>

  </div>
</template>

<script setup lang="ts">
import { ref, onUnmounted, watch } from 'vue'
import { decryptSecret, copySecretToClipboard } from '../services/secretService'
import type { SecretPreview } from '../types/secret'

import { useToastStore } from '../stores/toastStore'
import { useAppVisibility } from '../composables/useAppVisibility'

const props = defineProps<{
  item: SecretPreview
}>()

const value = ref('')

const toast = useToastStore()
const { isVisible } = useAppVisibility()

let isHolding = false
console.log(isHolding)

watch(isVisible, (visible) => {
  if (!visible) {
    clearSecret()
  }
})

async function startReveal() {

  isHolding = true

  // evitar múltiples decrypts
  if (!value.value) {
    value.value = await decryptSecret(props.item.id)  
  }
}

function stopReveal() {
  isHolding = false
  clearSecret()
}

function clearSecret() {
  value.value = ''
}

onUnmounted(() => {
  clearSecret()
})

async function handleSecureCopy() {

  await copySecretToClipboard(props.item.id)
  toast.show('Copied securely (auto-clear in 10s)')
}

</script>