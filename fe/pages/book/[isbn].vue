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
                    @click="showConfirmDeletionModal = true"
                  />
                  <UButton
                    icon="i-heroicons-pencil-square"
                    size="sm"
                    color="primary"
                    variant="outline"
                    @click="showEdit = true"
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
    <UModal v-model="showConfirmDeletionModal">
      <UCard
        :ui="{
          ring: '',
          divide: 'divide-y divide-gray-100 dark:divide-gray-800',
        }"
      >
        <template #header>
          <h1 class="font-bold text-xl">Confirm deletion?</h1>
        </template>
        <p>
          Are you sure that you want to delete:
          <strong>
            {{ book.title }}
          </strong>
          ?
        </p>

        <template #footer>
          <div class="flex w-full justify-end gap-4">
            <UButton color="primary" variant="solid" @click="deleteBook()"
              >Delete</UButton
            >
            <UButton
              color="primary"
              variant="soft"
              @click="showConfirmDeletionModal = false"
              >Cancel</UButton
            >
          </div>
        </template>
      </UCard>
    </UModal>
    <UModal v-model="showEdit">
      <UCard
        :ui="{
          ring: '',
        }"
      >
        <UForm
          :validate="validate"
          :state="state"
          class="space-y-4"
          @submit="editBook"
        >
          <UFormGroup label="ISBN" name="isbn">
            <UInput
              v-model="editFormData.isbn"
              type="number"
              icon="i-heroicons-book-open"
              trailing
              placeholder="Enter the book's ISBN"
            />
          </UFormGroup>

          <UFormGroup label="Title" name="title">
            <UInput
              v-model="editFormData.title"
              icon="i-heroicons-book"
              trailing
              placeholder="Enter the book's title"
            />
          </UFormGroup>

          <UFormGroup label="Plot" name="plot">
            <UInput
              v-model="editFormData.plot"
              icon="i-heroicons-file-code"
              trailing
              placeholder="Enter a brief description of the book's plot"
            />
          </UFormGroup>

          <div class="flex gap-4">
            <UFormGroup label="Year" name="year" class="w-full">
              <UInput
                v-model="editFormData.year"
                type="number"
                icon="i-heroicons-calendar"
                trailing
                placeholder="Enter the book's publication year"
              />
            </UFormGroup>

            <UFormGroup label="Pages" name="pages" class="w-full">
              <UInput
                v-model="editFormData.pages"
                icon="i-heroicons-file-lines"
                type="number"
                trailing
                placeholder="Enter the number of pages in the book"
              />
            </UFormGroup>
          </div>

          <UFormGroup label="Language" name="language">
            <UInput
              v-model="editFormData.language"
              icon="i-heroicons-globe"
              trailing
              placeholder="Enter the book's language"
            />
          </UFormGroup>

          <UFormGroup label="Author" name="author">
            <UInput
              v-model="editFormData.author"
              icon="i-heroicons-user"
              trailing
              placeholder="Enter the author's name"
            />
          </UFormGroup>

          <UFormGroup label="Publisher" name="publisher">
            <UInput
              v-model="editFormData.publisher"
              icon="i-heroicons-building"
              trailing
              placeholder="Enter the publisher's name"
            />
          </UFormGroup>

          <UFormGroup label="Categories" name="categories">
            <UInput
              v-model="editFormData.categories"
              icon="i-heroicons-tag"
              trailing
              placeholder="Enter the book's categories"
            />
          </UFormGroup>

          <div class="flex w-full justify-end gap-4">
            <UButton type="submit" color="primary" variant="solid">
              Edit
            </UButton>
            <UButton color="primary" variant="soft" @click="showEdit = false">
              Cancel
            </UButton>
          </div>
        </UForm>
      </UCard>
    </UModal>
    <UNotifications />
  </div>
</template>

<script setup>
const route = useRoute();
const toast = useToast();

const searchISBN = ref(route.params.isbn);

const showConfirmDeletionModal = ref(false);
const showEdit = ref(false);

const { error, data: book } = await useFetch(
  "http://localhost:3001/search-by-isbn/" + searchISBN.value
);

const editFormData = ref({ ...book.value });

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
        toast.add({ title: "Success", description: response.data.message });
        console.log(response);
      },
    }
  ).then(() => {
    return navigateTo("/", { redirectCode: 301 });
  });
}

async function editBook() {
  const formData = new FormData();
  formData.append("isbn", editFormData.value.isbn);
  formData.append("title", editFormData.value.title);
  formData.append("plot", editFormData.value.plot);
  formData.append("year", editFormData.value.year);
  formData.append("language", editFormData.value.language);
  formData.append("pages", editFormData.value.pages);
  formData.append("author", editFormData.value.author);
  formData.append("publisher", editFormData.value.publisher);
  formData.append("categories", editFormData.value.categories);
  // formData.append("book", bookFile.value);
  // formData.append("thumbnail", thumbnailFile.value);

  console.log(formData);

  try {
    const response = await axios.put(
      "http://localhost:3001/edit-book/"+book.value._id,
      formData,
      {
        headers: {
          "Content-Type": "multipart/form-data",
          Authorization: `Bearer ${token}`,
        },
      }
    );
    console.log(response.data);
  } catch (error) {
    console.error(error.response);
  }
}
</script>
