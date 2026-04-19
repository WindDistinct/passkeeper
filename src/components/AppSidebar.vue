<template>
    <aside class="w-64 bg-zinc-900 border-r border-zinc-800 p-4 flex flex-col">
        <h1 class="text-xl font-semibold mb-6">Passkeeper</h1>

        <nav class="space-y-2 text-sm">
            <button class="w-full text-left px-3 py-2 rounded-lg hover:bg-zinc-800" @click="store.setFilter('all')">All Secrets</button>
            <button class="w-full text-left px-3 py-2 rounded-lg hover:bg-zinc-800" @click="store.setFilter('password')">Passwords</button>
            <button class="w-full text-left px-3 py-2 rounded-lg hover:bg-zinc-800" @click="store.setFilter('api_key')">API Keys</button>
            <button class="w-full text-left px-3 py-2 rounded-lg hover:bg-zinc-800" @click="store.setFilter('note')">Notes</button>
            <button class="w-full text-left px-3 py-2 rounded-lg hover:bg-zinc-800" @click="store.setFilter('favorites')">Favorites</button>
            <button class="w-full text-left px-3 py-2 rounded-lg hover:bg-zinc-800" @click="store.setFilter('ssh_key')">SSH Keys</button>
            <button class="w-full text-left px-3 py-2 rounded-lg hover:bg-zinc-800">Trash</button>
        </nav>

        <div class="mt-auto text-xs text-zinc-500">
            v0.1 MVP
        </div>
    </aside>
</template>

<script setup lang="ts">
    import { useSecretStore } from '../stores/secretStore'
    const store = useSecretStore()
</script>
