import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useVaultStore = defineStore('vault', () => {
    const unlocked = ref(false)

    function unlock() {
        unlocked.value = true
    }

    function lock() {
        unlocked.value = false
    }

    return {
        unlocked,
        unlock,
        lock
    }
})
