<template>
  <div>
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
      <UCard v-if="error" class="">
        <div class="flex flex-col items-center gap-4">
          <h1 class="font-bold text-6xl">404 - NOT FOUND</h1>
          <p>
            🔍 Ahoy! No hidden treasures found on this quest. 🏴‍☠️ Keep sailin'
            and searching, matey!
          </p>
        </div>
      </UCard>
      <UCard v-else>
        <div class="flex justify-between gap-12">
          <div class="flex flex-col justify-between w-full">
            <div>
              <div class="flex justify-between">
                <h1 class="text-4xl font-bold pb-2">{{ book.title }}</h1>
                <div class="space-x-4" v-if="checkTokenExists()">
                  <UButton
                    icon="i-heroicons-trash"
                    size="sm"
                    color="primary"
                    variant="outline"
                    @click="deleteBook()"
                  />
                  <UButton
                    icon="i-heroicons-pencil-square"
                    size="sm"
                    color="primary"
                    variant="outline"
                  />
                </div>
              </div>
              <p class="underline cursor-pointer pb-1">{{ book.author }}</p>
              <p>{{ book.plot }}</p>
            </div>
            <div class="grid grid-cols-3 w-full gap-6">
              <div class="space-y-2">
                <p class="text-gray-400">
                  Categories:
                  <span
                    class="underline mr-1 text-white"
                    v-for="(cat, index) in book.categories"
                    :key="index"
                  >
                    {{ cat }}
                  </span>
                </p>
                <p class="text-gray-400">
                  Language:
                  <span class="text-white">{{ book.language }}</span>
                </p>
              </div>
              <div class="space-y-2">
                <p class="text-gray-400">
                  Year: <span class="text-white">{{ book.year }}</span>
                </p>
                <p class="text-gray-400">
                  Publisher:
                  <span class="text-white">{{ book.publisher }}</span>
                </p>
                <p class="text-gray-400">
                  Pages: <span class="text-white">{{ book.pages }}</span>
                </p>
              </div>
              <div class="space-y-2">
                <p class="text-gray-400">
                  ISBN:
                  <UBadge :label="book.isbn" variant="soft" />
                </p>
                <p class="font-semibold">
                  Resource:
                  <UButton
                    size="xs"
                    :to="book.url_link"
                    target="_blank"
                    external
                  >
                    {{
                      book.extension + " " + book.file_size.toFixed(2) + "Mb"
                    }}
                  </UButton>
                </p>
              </div>
            </div>
          </div>
          <nuxt-link :to="book.url_link" target="_blank" class="h-96 shrink-0">
            <img :src="book.thumbnail" class="h-96 shrink-0" />
          </nuxt-link>
        </div>
      </UCard>
    </UContainer>
    <UNotifications />
  </div>
</template>

<script setup>
const route = useRoute();
const toast = useToast();

const searchISBN = ref(route.params.isbn);

const { error, data: book } = await useFetch(
  "http://localhost:3001/search-by-isbn/" + searchISBN.value
);

function checkTokenExists() {
  const token = localStorage.getItem("token");
  return !!token;
}

async function formSubmit(q) {
  await navigateTo("/search/" + q);
}

function deleteBook() {
  const { data } = useFetch(
    "http://localhost:3001/delete-book/" + book.value._id,
    {
      method: "DELETE",
      headers: {
        Authorization: `Bearer ${localStorage.getItem("token")}`,
      },
      onResponse({ response }) {
        toast.add({ title: 'Success' , description: response.data.message})
        console.log(response);
      },
    }
  );
}

// const mockData = ref({
//   title: "Niente può fermarti. Can't Hurt Me",
//   thumbnail:
//     "https://static.1lib.sk/covers/books/1a/c7/52/1ac752391a26a02c3652e48ffa6e0b53.jpg",
//   plot: "For David Goggins, childhood was a nightmare — poverty, prejudice, and physical abuse colored his days and haunted his nights. But through self-discipline, mental toughness, and hard work, Goggins transformed himself from a depressed, overweight young man with no future into a U.S. Armed Forces icon and one of the world's top endurance athletes. The only man in history to complete elite training as a Navy SEAL, Army Ranger, and Air Force Tactical Air Controller, he went on to set records in numerous endurance events, inspiring Outside magazine to name him “The Fittest (Real) Man in America.”",
//   year: 2022,
//   language: "English",
//   isbn: 1804220493,
//   pages: 69,
//   author: "David Goggins",
//   publisher: "Vallardi",
//   categories: [
//     "Self-help",
//     "Personal development",
//     "Habit Formation",
//     "Behavioral Psychology",
//   ],
//   filetype: "pdf",
//   filesize: "7.0mb",
//   resource: "/api/file.pdf",
// });
</script>
