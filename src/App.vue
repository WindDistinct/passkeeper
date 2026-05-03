<template>
  <UnlockPage v-if="!vault.unlocked" />

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
import { ref } from 'vue'
import { onMounted } from 'vue'
import { useVaultStore } from './stores/vaultStore'
import { useSecretStore } from './stores/secretStore'

import UnlockPage from './pages/UnlockPage.vue'
import MainLayout from './layouts/MainLayout.vue'
import SecretList from './components/SecretList.vue'
import CreateSecretModal from './components/CreateSecretModal.vue'

const open = ref(false)

const vault = useVaultStore()
const secretStore = useSecretStore()

onMounted(async () => {
  await secretStore.loadSecrets()
})

</script>
