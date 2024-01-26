<template>
  <UContainer>
    <div class="flex items-center flex-col">
      <img src="/logo.svg" class="w-1/2">
      <p class="pb-8">
        🏴‍☠️ Ahoy, Matey! Welcome to the Literary Seas of Knowledge at our Pirate-themed Library! 🏴‍☠️
      </p>
      <UForm class="w-1/2 flex gap-4" @submit="formSubmit(searchQuery)">
        <UInput
          v-model="searchQuery"
          class="w-full"
          icon="i-heroicons-magnifying-glass-20-solid"
          size="xl"
          color="white"
          trailing
          placeholder="Ahoy! search anythin' in this ocean o' freedom!"
        />
        <UButton type="submit">
          Search
        </UButton>
      </UForm>
    </div>
  </UContainer>
</template>

<script setup>
  function formSubmit(q){
    console.log(q);
  }
</script>
