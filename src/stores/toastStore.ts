import { defineStore } from 'pinia'
import { ref } from 'vue'

export type Toast = {
    id: number
    message: string
}

export const useToastStore = defineStore('toast', () => {
    const toasts = ref<Toast[]>([])

    function show(message: string, duration = 3000) {
        const id = Date.now()

        toasts.value.push({ id, message })

        setTimeout(() => {
            remove(id)
        }, duration)
    }

    function remove(id: number) {
        toasts.value = toasts.value.filter(t => t.id !== id)
    }

    return {
        toasts,
        show,
        remove
    }
})