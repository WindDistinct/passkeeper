import { onUnmounted } from 'vue'

type Options = {
    timeout: number
    onTimeout: () => void
}

export function useInactivity({ timeout, onTimeout }: Options) {
    let timer: ReturnType<typeof setTimeout> | null = null

    function resetTimer() {
        if (timer) clearTimeout(timer)

        timer = setTimeout(() => {
            onTimeout()
        }, timeout)
    }

    function start() {
        window.addEventListener('mousemove', resetTimer)
        window.addEventListener('keydown', resetTimer)
        window.addEventListener('click', resetTimer)

        resetTimer()
    }

    function stop() {
        window.removeEventListener('mousemove', resetTimer)
        window.removeEventListener('keydown', resetTimer)
        window.removeEventListener('click', resetTimer)

        if (timer) {
            clearTimeout(timer)
            timer = null
        }
    }

    onUnmounted(() => {
        stop()
    })

    return {
        start,
        stop
    }
}