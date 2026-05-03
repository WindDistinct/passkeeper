<template>
    <div class="h-screen flex items-center justify-center bg-zinc-950 text-white">
      <div class="w-full max-w-sm bg-zinc-900 border border-zinc-800 rounded-2xl p-6">
        <h2 class="text-xl font-semibold mb-4">Unlock Vault</h2>
  
        <input
          v-model="password"
          type="password"
          placeholder="Master password"
          class="w-full bg-zinc-950 border border-zinc-700 rounded-lg px-3 py-2"
        />
  
        <button
          @click="submit"
          class="w-full mt-4 bg-white text-black py-2 rounded-lg font-medium"
        >
          Unlock
        </button>
      </div>
    </div>
</template>
  
<script setup lang="ts">
    import { ref } from 'vue'
    import { useVaultStore } from '../stores/vaultStore'
    import { unlockVault } from '../services/secretService'
    
    const password = ref('')
    const vault = useVaultStore()

    async function submit() {
      if (!password.value) return

      const ok = await unlockVault(password.value)

      if (ok) {
        vault.unlock()
      } else {
        alert('Invalid password')
      }
    }

</script>
  