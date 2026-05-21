# Passkeeper

Passkeeper is an open-source, local-first desktop application for securely storing and managing passwords, API keys, credentials and sensitive notes.

Built with security and privacy as core principles:

- Local-first architecture
- No cloud dependency
- No plaintext persistence
- Modern desktop experience
- Secure cryptographic practices

Passkeeper aims to provide users with complete ownership and control over their sensitive data.

---

## Features

### Implemented

#### Security

- Vault protected by a master password
- Key derivation using Argon2id
- AES-256-GCM encryption
- In-memory key management
- Memory zeroization on lock
- Secure clipboard copy
- Clipboard auto-clear
- Auto-lock after inactivity
- Blur protection
- Hold-to-reveal sensitive values
- Decrypt-on-demand approach
- Safe error handling

#### Secret Management

- Create secrets
- View secrets
- Edit secrets
- Delete secrets
- Favorite secrets
- Filter by type and favorites

#### User Experience

- Vault onboarding flow
- Unlock screen
- Modern desktop layout
- Toast notification system
- Reusable modal system

---

### Roadmap

#### Near-term

- Search with debounce
- Sorting options
- Better confirmation dialogs
- Session indicators
- Security feedback improvements

#### Mid-term

- Tags
- Folder organization
- Import / Export
- Backups
- Database migrations

#### Long-term

- Multi-vault support
- Key rotation
- Optional end-to-end sync
- Experimental screenshot protection

---

## Screenshots

Screenshots will be added as development progresses.

```md
![Setup Vault](docs/images/setup.png)

![Secret List](docs/images/secrets.png)

![Secret Modal](docs/images/modal.png)
```

---

## Tech Stack

### Frontend

- Vue 3 (Composition API)
- Pinia
- Tailwind CSS v4
- Vite

### Backend

- Rust
- Tauri v2

### Database

- SQLite

### Cryptography

- Argon2id
- AES-256-GCM
- Base64

---

## Architecture

```text
Frontend (Vue)
      ↓ invoke()
Tauri Bridge
      ↓
Rust Backend
      ↓
SQLite
```

---

## Security Model

### Encryption flow

```text
Master Password
      ↓
Argon2id
      ↓
32-byte derived key
      ↓
AES-256-GCM
      ↓
Encrypted Payload
```

### Security principles

- Master password is never stored
- Keys exist only in memory
- Secrets remain encrypted in the database
- Decryption occurs only when necessary
- Sensitive memory is zeroized when possible
- Clipboard contents are automatically cleared

---

## Installation

### Requirements

- Node.js (v20+ recommended)
- Rust
- Cargo
- Tauri prerequisites

### Clone repository

```bash
git clone https://github.com/WindDistinct/passkeeper.git

cd passkeeper
```

### Install dependencies

```bash
npm install
```

### Run in development mode

```bash
npm run tauri dev
```

### Build application

```bash
npm run tauri build
```

---

## Usage

### First launch

1. Open Passkeeper
2. Create a master password
3. Unlock the vault
4. Start adding secrets

### Managing secrets

- Create a new secret
- Hold to reveal sensitive values
- Securely copy values
- Edit existing entries
- Delete secrets
- Mark favorites

---

## Project Structure

```text
src/
├── components/
├── composables/
├── layouts/
├── pages/
├── services/
├── stores/
├── types/

src-tauri/
├── crypto.rs
├── db.rs
├── lib.rs
```

---

## Project Status

Current stage:

```text
MVP with security hardening in progress
```

Current focus:

- Search implementation
- UX improvements
- Security hardening
- Organization features

---

## Contributing

Contributions, ideas, bug reports and feature suggestions are welcome.

Steps:

1. Fork repository
2. Create feature branch

```bash
git checkout -b feature/my-feature
```

3. Commit changes

```bash
git commit -m "Add feature"
```

4. Push branch

```bash
git push origin feature/my-feature
```

5. Open a Pull Request

---

## License

MIT License