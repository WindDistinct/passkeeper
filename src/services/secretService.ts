import { invoke } from '@tauri-apps/api/core'

export async function fetchSecrets() {
    return await invoke('get_secrets')
}

export async function createSecret(data: any) {
    return await invoke('create_secret', {
        payload: data
    })
}
