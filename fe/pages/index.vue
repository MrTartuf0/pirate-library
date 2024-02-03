<template>
  <UContainer>
    <div class="flex items-center flex-col">
      <div class="flex items-center justify-end w-full">
        <UButton @click="redirectTo('/login')" class="mr-4">Login</UButton>
        <UButton @click="redirectTo('/upload')">Upload</UButton>
      </div>
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
      <UDivider class="mt-8 mb-8">Recently added</UDivider>
      <div class="grid grid-cols-5 w-full gap-8 px-16 mb-16">
        <img v-for="book in books" :key="book._id" class="w-full" :src="book.thumbnail">
      </div>
    </div>
  </UContainer>
</template>

<script setup>
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const books = ref([])

async function formSubmit(q) {
  await navigateTo('/search/' + q)
}

function redirectTo(path) {
  router.push(path);
}

onMounted(async () => {
  const response = await fetch('http://localhost:3001/books')
  console.log(await response)
  const data = await response.json()
  books.value = data
})
</script>
