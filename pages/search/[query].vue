<template>
  <UContainer>
    <div class="flex items-center flex-col">
      <img src="/search_logo.svg" class="w-1/2 pb-8 pt-8">
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
  const route = useRoute()
  const searchQuery = ref(route.params.query)
  function formSubmit(q){
    console.log(q);
  }
</script>  

