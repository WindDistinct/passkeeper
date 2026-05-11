export type SecretType =
    | 'password'
    | 'api_key'
    | 'note'
    | 'ssh_key'

export interface SecretPreview {
    id: string
    title: string
    username?: string
    type: SecretType
    favorite: boolean
}