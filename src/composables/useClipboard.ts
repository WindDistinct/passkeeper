import { ref } from 'vue'

export function useClipboard() {
    const copied = ref(false)

    async function copy(text: string, timeout = 10000) {
        try {
            await navigator.clipboard.writeText(text)
            copied.value = true

            setTimeout(async () => {
                try {
                    const current = await navigator.clipboard.readText()
                    if (current === text) {
                        await navigator.clipboard.writeText('')
                    }
                } catch {
                    // ignore (permisos o fallback)
                }

                copied.value = false
            }, timeout)

        } catch (err) {
            console.error('Clipboard error:', err)
        }
    }

    return {
        copy,
        copied
    }
}