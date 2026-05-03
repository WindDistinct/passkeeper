<template>
    <div
        v-if="open"
        class="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
    >
        <div class="w-full max-w-md bg-zinc-900 border border-zinc-800 text-white rounded-2xl p-6">
            <h2 class="text-xl font-semibold mb-4">New Secret</h2>

            <div class="space-y-4">
                <input
                    v-model="title"
                    placeholder="Title"
                    class="w-full bg-zinc-950 border border-zinc-700 rounded-lg px-3 py-2"
                />

                <input
                    v-model="username"
                    placeholder="Username (optional)"
                    class="w-full bg-zinc-950 border border-zinc-700 rounded-lg px-3 py-2"
                />

                <select
                    v-model="type"
                    class="w-full bg-zinc-950 border border-zinc-700 rounded-lg px-3 py-2"
                >
                    <option value="password">Password</option>
                    <option value="api_key">API Key</option>
                    <option value="note">Note</option>
                    <option value="ssh_key">SSH Key</option>
                </select>

                <input
                    v-model="password"
                    placeholder="Password"
                    class="w-full bg-zinc-950 border border-zinc-700 rounded-lg px-3 py-2"
                    />
            </div>

            <div class="flex justify-end gap-3 mt-6">
                <button
                    @click="$emit('close')"
                    class="px-4 py-2 rounded-lg bg-zinc-800"
                    >
                    Cancel
                </button>

                <button
                    @click="submit"
                    class="px-4 py-2 rounded-lg bg-white text-black font-medium"
                    >
                    Save
                </button>
            </div>
        </div>
    </div>
</template>
  
<script setup lang="ts">
    import { ref } from 'vue'
    import { useSecretStore } from '../stores/secretStore'

    defineProps<{
        open: boolean
    }>()

    const password = ref('')

    const emit = defineEmits(['close'])

    const store = useSecretStore()

    const title = ref('')
    const username = ref('')
    const type = ref<'password' | 'api_key' | 'note' | 'ssh_key'>('password')

    function submit() {
        if (!title.value.trim()) return

        store.addSecret({
            title: title.value,
            username: username.value,
            type: type.value,
            password: password.value
        })

        title.value = ''
        username.value = ''
        type.value = 'password'
        password.value = ''

        emit('close')
    }
</script>
  