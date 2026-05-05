import { ref, onMounted, onUnmounted } from 'vue'

const isVisible = ref(true)

export function useAppVisibility() {
    function handleBlur() {
        isVisible.value = false
    }

    function handleFocus() {
        isVisible.value = true
    }

    onMounted(() => {
        window.addEventListener('blur', handleBlur)
        window.addEventListener('focus', handleFocus)
    })

    onUnmounted(() => {
        window.removeEventListener('blur', handleBlur)
        window.removeEventListener('focus', handleFocus)
    })

    return {
        isVisible
    }
}