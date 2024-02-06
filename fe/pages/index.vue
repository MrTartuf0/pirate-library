<template>
  <UContainer>
    <div class="flex items-center flex-col">
      <div class="flex items-center justify-end w-full gap-4 pt-4">
        <UButton
          v-if="checkTokenExists()"
          @click="logout()"
          icon="i-heroicons-user"
        >
          Logout
        </UButton>
        <UButton v-else to="/login" icon="i-heroicons-user"> Login </UButton>
        <UButton
          v-if="checkTokenExists()"
          to="/upload"
          icon="i-heroicons-arrow-up-tray"
        >
          Upload
        </UButton>
      </div>
      <img src="/logo.svg" class="w-1/2" />
      <p class="pb-8">
        🏴‍☠️ Ahoy, Matey! Welcome to the Literary Seas of Knowledge at our
        Pirate-themed Library! 🏴‍☠️
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
        <UButton type="submit"> Search </UButton>
      </UForm>
      <UDivider class="mt-8 mb-8">Recently added</UDivider>
      <div class="grid grid-cols-5 w-full gap-8 px-16 mb-16">
        <NuxtLink
          v-for="(book, index) in books"
          :key="index"
          :to="`/book/${book.isbn}`"
          external
        >
          <img class="w-full cursor-pointer" :src="book.thumbnail" />
        </NuxtLink>
      </div>
    </div>
  </UContainer>
</template>

<script setup>
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const books = ref([]);

function checkTokenExists() {
  const token = localStorage.getItem("token");
  return !!token;
}

function logout(){
  localStorage.removeItem('token');
  location.reload();
}

async function formSubmit(q) {
  await navigateTo("/search/" + q);
}


onMounted(async () => {
  const response = await fetch("http://localhost:3001/books");
  console.log(await response);
  const data = await response.json();
  books.value = data;
});
</script>
