import { invoke } from '@tauri-apps/api/core'

export async function fetchSecrets() {
    return await invoke('get_secrets')
}

export async function createSecret(data: any) {
    return await invoke('create_secret', {
        payload: data
    })
}

export async function decryptSecret(secretId: string): Promise<string> {
    return await invoke<string>('get_secret_value', {
        secretId: secretId
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

export async function copySecretToClipboard(secretId: string): Promise<void> {
    await invoke('copy_secret_to_clipboard', {
        secretId
    })
}

export async function deleteSecret(secretId: string) {
    return await invoke('delete_secret', {
        secretId
    })
}