import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { SecretPreview } from '../types/secret'
import { fetchSecrets, createSecret, updateSecret, deleteSecret } from '../services/secretService'

export const useSecretStore = defineStore('secrets', () => {
    const currentFilter = ref('all')

    const secrets = ref<SecretPreview[]>([])

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
        const data = await fetchSecrets() as any[]

        secrets.value = data.map(secret => ({
            id: secret.id,
            title: secret.title,
            username: secret.username,
            type: secret.secret_type,
            favorite: Boolean(secret.favorite)
        }))
    }

    async function addSecret(payload: {
        value: any
        title: string
        username?: string
        type: 'password' | 'api_key' | 'note' | 'ssh_key'
    }) {
        await createSecret({
            id: crypto.randomUUID(),
            title: payload.title,
            username: payload.username,
            secret_type: payload.type,
            value: payload.value
        })


        await loadSecrets()
    }

    async function removeSecret(secretId: string) {
        await deleteSecret(secretId)

        secrets.value = secrets.value.filter(
            s => s.id !== secretId
        )
    }

    async function editSecret(payload: {
        id: string
        title: string
        username?: string
        type: 'password' | 'api_key' | 'note' | 'ssh_key'
        value: string
    }) {

        await updateSecret({
            id: payload.id,
            title: payload.title,
            username: payload.username,
            secret_type: payload.type,
            value: payload.value
        })

        await loadSecrets()
    }

    return {
        secrets,
        filteredSecrets,
        currentFilter,
        setFilter,
        addSecret,
        removeSecret,
        editSecret,
        loadSecrets
    }
})
