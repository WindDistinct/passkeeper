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

export async function setupVault(password: string) {
    return await invoke('setup_vault', { password })
}

export async function unlockVault(password: string): Promise<boolean> {
    return await invoke<boolean>('unlock_vault', { password })
}

export async function vaultExists(): Promise<boolean> {
    return await invoke<boolean>('vault_exists')
}

export async function lockVault() {
    return await invoke('lock_vault')
}
