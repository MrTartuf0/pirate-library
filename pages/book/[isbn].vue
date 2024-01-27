<template>
  <UContainer>
    <div class="flex items-center flex-col">
      <nuxt-link to="/" class="w-1/2 pb-8 pt-8">
        <img src="/search_logo.svg" />
      </nuxt-link>
      <UForm class="w-1/2 flex gap-4 pb-8" @submit="formSubmit(searchQuery)">
        <UInput
          v-model="searchQuery"
          class="w-full"
          icon="i-heroicons-magnifying-glass-20-solid"
          size="xl"
          color="white"
          trailing
          placeholder="Ahoy! search anythin' in this ocean o' freedom!"
        />
        <UButton type="submit"> Search </UButton>
      </UForm>
    </div>
    <UDivider class="mb-4" />
    <UCard
      v-for="(item, index) in mockData.results"
      :key="index"
      class="mb-6 dark:hover:ring-primary-500 dark:hover:ring-2"
    >
      <div class="flex gap-6">
        <img :src="item.thumbnail" class="w-32 cursor-pointer" />
        <div class="flex flex-col justify-between w-full">
          <div class="space-y-1">
            <h1 class="underline text-2xl cursor-pointer">
              {{ item.title }}
            </h1>
            <p
              class="text-gray-400 cursor-pointer hover:text-primary-200"
              @click="formSubmit(item.publisher.replaceAll('/', ' '))"
            >
              {{ item.publisher }}
            </p>
          </div>
          <div class="flex justify-end gap-4">
            <div class="space-x-2">
              <span class="text-gray-400">Author:</span>
              <UBadge :label="item.author" variant="soft" />
            </div>
            <div class="space-x-2">
              <span class="text-gray-400">Year:</span>
              <UBadge :label="item.year" variant="soft" />
            </div>
            <div class="space-x-2">
              <span class="text-gray-400">Language:</span>
              <UBadge :label="item.language" variant="soft" />
            </div>
            <div class="space-x-2">
              <span class="text-gray-400">Filetype:</span>
              <UBadge :label="item.filetype" variant="soft" />
            </div>
          </div>
        </div>
      </div>
    </UCard>
  </UContainer>
</template>

<script setup>
const route = useRoute();
const searchQuery = ref(route.params.query);

const mockData = ref({
  title:
    "Atomic Habits: An Easy & Proven Way to Build Good Habits & Break Bad Ones",
  thumbnail:
    "https://isbncoverdb.com/images/book-cover-atomic-habits-1770005.webp",
  year: 2022,
  language: English,
  isbn: 1804220493,
  pages: 69,
  filetype: "pdf",
  author: "James Clear",
  publisher: userName,
  categories: [
    "Self-help",
    "Personal development",
    "Habit Formation",
    "Behavioral Psychology",
  ],
  resource: "/api/file.pdf",
});

async function formSubmit(q) {
  await navigateTo("/search/" + q);
}
</script>
