<template>
    <header class="h-16 border-b border-zinc-800 px-6 flex items-center justify-between">
        <input type="text" placeholder="Search secrets..."
            class="w-80 bg-zinc-900 border border-zinc-700 rounded-lg px-3 py-2 text-sm" />

        <button @click="$emit('new-secret')" class="bg-white text-black px-4 py-2 rounded-lg text-sm font-medium">
            New Secret
        </button>

        <button @click="lock">Lock</button>
    </header>
</template>

<script setup lang="ts">
import { useVaultStore } from '../stores/vaultStore'
import { lockVault } from '../services/secretService'

defineEmits(['new-secret'])

const vault = useVaultStore()

async function lock() {
    await lockVault()
    vault.lock()
}

</script>