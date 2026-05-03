import { invoke } from '@tauri-apps/api/core'

export async function fetchSecrets() {
    return await invoke('get_secrets')
}

export async function createSecret(data: any) {
    return await invoke('create_secret', {
        payload: data
    })
}

export async function decryptSecret(payload: string): Promise<string> {
    return await invoke<string>('get_secret_value', {
        encryptedPayload: payload
    })
}

