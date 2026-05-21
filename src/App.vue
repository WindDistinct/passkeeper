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

    <SecretModal :open="open" @close="open = false" />
  </template>

  <ToastContainer />
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useVaultStore } from './stores/vaultStore'
import { useSecretStore } from './stores/secretStore'
import { useInactivity } from './composables/useInactivity'
import { useToastStore } from './stores/toastStore'

import { vaultExists } from './services/secretService'

import SetupVaultPage from './pages/SetupVaultPage.vue'
import UnlockPage from './pages/UnlockPage.vue'
import MainLayout from './layouts/MainLayout.vue'
import SecretList from './components/SecretList.vue'
import SecretModal from './components/SecretModal.vue'
import ToastContainer from './components/ToastContainer.vue'

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

const toast = useToastStore()

const { start, stop } = useInactivity({
  timeout: 5 * 60 * 1000,
  onTimeout: () => {
    vault.lock()
    toast.show('Vault locked due to inactivity')
  }
})

watch(
  () => vault.unlocked,
  (isUnlocked) => {
    if (isUnlocked) {
      start()
    } else {
      stop()
    }
  },
  { immediate: true }
)
</script>