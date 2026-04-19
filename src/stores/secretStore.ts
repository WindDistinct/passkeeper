import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { SecretItem } from '../types/secret'
import { fetchSecrets, createSecret } from '../services/secretService'

export const useSecretStore = defineStore('secrets', () => {
    const currentFilter = ref('all')

    const secrets = ref<SecretItem[]>([
        {
            id: '1',
            title: 'GitHub Personal',
            username: 'nickdev',
            type: 'password',
            favorite: true
        },
        {
            id: '2',
            title: 'OpenAI API',
            type: 'api_key',
            favorite: false
        },
        {
            id: '3',
            title: 'Production Server',
            type: 'ssh_key',
            favorite: false
        }
    ])

    const filteredSecrets = computed(() => {
        if (currentFilter.value === 'all') return secrets.value
        if (currentFilter.value === 'favorites') {
            return secrets.value.filter(s => s.favorite)
        }

        return secrets.value.filter(
            s => s.type === currentFilter.value
        )
    })

    function setFilter(value: string) {
        currentFilter.value = value
    }

    async function loadSecrets() {
        secrets.value = await fetchSecrets() as any
    }

    async function addSecret(payload: {
        title: string
        username?: string
        type: 'password' | 'api_key' | 'note' | 'ssh_key'
    }) {
        await createSecret({
            id: crypto.randomUUID(),
            title: payload.title,
            username: payload.username,
            secret_type: payload.type
        })

        await loadSecrets()
    }

    return {
        secrets,
        filteredSecrets,
        currentFilter,
        setFilter,
        addSecret,
        loadSecrets
    }
})
