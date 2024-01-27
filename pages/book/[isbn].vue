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
    <UCard>
      <div class="flex justify-between gap-12">
        <div class="flex flex-col justify-between">
          <div>
            <h1 class="text-4xl font-bold pb-2">{{ mockData.title }}</h1>
            <p class="underline cursor-pointer pb-1">{{ mockData.author }}</p>
            <p>{{ mockData.plot }}</p>
          </div>
          <div class="grid grid-cols-3 w-full gap-6">
            <div class="space-y-2">
              <p class="text-gray-400">
                Categories:
                <span
                  class="underline mr-1 text-white"
                  v-for="(cat, index) in mockData.categories"
                  :key="index"
                >
                  {{ cat }}
                </span>
              </p>
              <p class="text-gray-400">
                Language:
                <span class="text-white">{{ mockData.language }}</span>
              </p>
            </div>
            <div class="space-y-2">
              <p class="text-gray-400">
                Year: <span class="text-white">{{ mockData.year }}</span>
              </p>
              <p class="text-gray-400">
                Publisher:
                <span class="text-white">{{ mockData.publisher }}</span>
              </p>
              <p class="text-gray-400">
                Pages: <span class="text-white">{{ mockData.pages }}</span>
              </p>
            </div>
            <div class="space-y-2">
              <p class="text-gray-400">
                ISBN:
                <UBadge :label="mockData.isbn" variant="soft" />
              </p>
              <p class="font-semibold ">
                Resource: <UButton size="xs">{{ mockData.filetype + ' ' + mockData.filesize }}</UButton>
              </p>
            </div>
          </div>
        </div>
        <img :src="mockData.thumbnail" class="h-96" />
      </div>
    </UCard>
  </UContainer>
</template>

<script setup>
const route = useRoute();
const searchQuery = ref(route.params.query);

const mockData = ref({
  title: "Niente può fermarti. Can't Hurt Me",
  thumbnail:
    "https://static.1lib.sk/covers/books/1a/c7/52/1ac752391a26a02c3652e48ffa6e0b53.jpg",
  plot: "For David Goggins, childhood was a nightmare — poverty, prejudice, and physical abuse colored his days and haunted his nights. But through self-discipline, mental toughness, and hard work, Goggins transformed himself from a depressed, overweight young man with no future into a U.S. Armed Forces icon and one of the world's top endurance athletes. The only man in history to complete elite training as a Navy SEAL, Army Ranger, and Air Force Tactical Air Controller, he went on to set records in numerous endurance events, inspiring Outside magazine to name him “The Fittest (Real) Man in America.”",
  year: 2022,
  language: "English",
  isbn: 1804220493,
  pages: 69,
  author: "David Goggins",
  publisher: "Vallardi",
  categories: [
    "Self-help",
    "Personal development",
    "Habit Formation",
    "Behavioral Psychology",
  ],
  filetype: "pdf",
  filesize: "7.0mb",
  resource: "/api/file.pdf",
});

async function formSubmit(q) {
  await navigateTo("/search/" + q);
}
</script>
