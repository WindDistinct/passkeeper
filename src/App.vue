<template>
  <!-- Loading inicial -->
  <div v-if="loading" class="h-screen bg-zinc-950" />

  <!-- Setup -->
  <SetupVaultPage v-else-if="!hasVault" />

  <!-- Unlock -->
  <UnlockPage v-else-if="!vault.unlocked" />

  <template v-else> 
    <MainLayout @new-secret="open = true">
      <div class="border border-dashed border-zinc-800 rounded-2xl h-full min-h-125 p-6">
        <h2 class="text-2xl font-semibold mb-4">Your Secrets</h2>
        <!-- <p class="text-zinc-400">Your secrets will appear here.</p> -->
        <SecretList />
      </div>
    </MainLayout>

    <CreateSecretModal :open="open" @close="open = false" />
  </template>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useVaultStore } from './stores/vaultStore'
import { useSecretStore } from './stores/secretStore'

import { vaultExists } from './services/secretService'

import SetupVaultPage from './pages/SetupVaultPage.vue'
import UnlockPage from './pages/UnlockPage.vue'
import MainLayout from './layouts/MainLayout.vue'
import SecretList from './components/SecretList.vue'
import CreateSecretModal from './components/CreateSecretModal.vue'

const vault = useVaultStore()
const store = useSecretStore()

const open = ref(false)
const hasVault = ref(false)
const loading = ref(true)

onMounted(async () => {
  hasVault.value = await vaultExists()
  loading.value = false

  if (hasVault.value) {
    await store.loadSecrets()
  }
})
</script>