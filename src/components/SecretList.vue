<template>
    <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
      <SecretCard
        v-for="secret in store.filteredSecrets"
        :key="secret.id"
        :item="secret"
        @edit="emit('edit', $event)"
      />
    </div>
</template>
  
<script setup lang="ts">
  import { useSecretStore } from '../stores/secretStore'
  import SecretCard from './SecretCard.vue'
  
  const store = useSecretStore()

  const emit = defineEmits<{
    edit: [id:string]
  }>()
</script>
  