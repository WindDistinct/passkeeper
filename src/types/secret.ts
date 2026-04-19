export type SecretType =
    | 'password'
    | 'api_key'
    | 'note'
    | 'ssh_key'

export interface SecretItem {
    id: string
    title: string
    username?: string
    type: SecretType
    folder?: string
    favorite: boolean
}
