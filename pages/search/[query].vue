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
              <UBadge :label="item.author" variant="soft"/>
            </div>
            <div class="space-x-2">
              <span class="text-gray-400">Year:</span>
              <UBadge :label="item.year" variant="soft"/>
            </div>
            <div class="space-x-2">
              <span class="text-gray-400">Language:</span>
              <UBadge :label="item.language" variant="soft"/>
            </div>
            <div class="space-x-2">
              <span class="text-gray-400">Filetype:</span>
              <UBadge :label="item.filetype" variant="soft"/>
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
  results: [
    {
      title:
        "Atomic Habits: An Easy & Proven Way to Build Good Habits & Break Bad Ones",
      thumbnail:
        "https://isbncoverdb.com/images/book-cover-atomic-habits-1770005.webp",
      year: 2022,
      language: "English",
      filetype: "pdf",
      author: "James Clear",
      publisher: "mrTartuf0",
    },
    {
      title:
        "Atomic Habits, I Will Teach You To Be Rich, Mindset, The One Thing 4 Books Collection Set",
      thumbnail:
        "https://isbncoverdb.com/images/book-cover-atomic-habits-i-will-teach-you-to-be-rich-mindset-the-one-thing-4-books-collection-set-803350.webp",
      year: 2020,
      language: "Swedish",
      filetype: "epub",
      author: "James Clear",
      publisher:
        "Random House Business/Yellow Kite/Robinson/John Murray Learning",
    },
    {
      title: "large image",
      thumbnail:
        "https://images.unsplash.com/photo-1566895291281-ea63efd4bdbc?q=80&w=1000&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MTF8fDklM0ExNnxlbnwwfHwwfHx8MA%3D%3D",
      year: 2020,
      language: "Swedish",
      filetype: "epub",
      author: "James Clear",
      publisher:
        "Random House Business/Yellow Kite/Robinson/John Murray Learning",
    },
  ],
});

async function formSubmit(q) {
  await navigateTo("/search/" + q);
}
</script>
