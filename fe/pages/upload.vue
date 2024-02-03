<template>
  <div class="flex">
    <img
      :src="`/pirates${Math.floor(Math.random() * 5)}.jpg`"
      class="h-screen"
    />
    <UContainer class="flex flex-col items-center justify-center">
      <div class="w-1/2">
        <img src="/search_logo.svg" class="mb-8" />
        <UForm
          :validate="validate"
          :state="state"
          class="space-y-4"
          @submit="uploadBook"
        >
          <UFormGroup label="ISBN" name="isbn">
            <UInput
              v-model="isbn"
              icon="i-heroicons-book-open"
              trailing
              placeholder="Enter the book's ISBN"
            />
          </UFormGroup>

          <UFormGroup label="Title" name="title">
            <UInput
              v-model="title"
              icon="i-heroicons-book"
              trailing
              placeholder="Enter the book's title"
            />
          </UFormGroup>

          <UFormGroup label="Plot" name="plot">
            <UInput
              v-model="plot"
              icon="i-heroicons-file-code"
              trailing
              placeholder="Enter a brief description of the book's plot"
            />
          </UFormGroup>

          <UFormGroup label="Year" name="year">
            <UInput
              v-model="year"
              icon="i-heroicons-calendar"
              trailing
              placeholder="Enter the book's publication year"
            />
          </UFormGroup>

          <UFormGroup label="Language" name="language">
            <UInput
              v-model="language"
              icon="i-heroicons-globe"
              trailing
              placeholder="Enter the book's language"
            />
          </UFormGroup>

          <UFormGroup label="Pages" name="pages">
            <UInput
              v-model="pages"
              icon="i-heroicons-file-lines"
              trailing
              placeholder="Enter the number of pages in the book"
            />
          </UFormGroup>

          <UFormGroup label="Author" name="author">
            <UInput
              v-model="author"
              icon="i-heroicons-user"
              trailing
              placeholder="Enter the author's name"
            />
          </UFormGroup>

          <UFormGroup label="Publisher" name="publisher">
            <UInput
              v-model="publisher"
              icon="i-heroicons-building"
              trailing
              placeholder="Enter the publisher's name"
            />
          </UFormGroup>

          <UFormGroup label="Categories" name="categories">
            <UInput
              v-model="categories"
              icon="i-heroicons-tag"
              trailing
              placeholder="Enter the book's categories"
            />
          </UFormGroup>
        
          <!-- Standard file input for uploading books -->
          <div class="flex flex-col space-y-2">
            <label for="bookFile" class="text-lg font-medium">Upload Book</label>
            <input type="file" id="bookFile" @change="onBookFileChange" accept=".pdf,.epub" required />
            <span v-if="bookFileError" class="text-red-500">{{ bookFileError }}</span>
          </div>

          <!-- Standard file input for uploading thumbnails -->
          <div class="flex flex-col space-y-2">
            <label for="thumbnailFile" class="text-lg font-medium">Upload Thumbnail</label>
            <input type="file" id="thumbnailFile" @change="onThumbnailFileChange" accept="image/*" required />
            <span v-if="thumbnailFileError" class="text-red-500">{{ thumbnailFileError }}</span>
          </div>

          <UButton type="submit" class="mt-16 w-full justify-center">
            Upload Book
          </UButton>


        </UForm>
      </div>
    </UContainer>
  </div>
</template>

<script setup>
import axios from 'axios';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const token = localStorage.getItem('token');

// Function to decode JWT token
function parseJwt(token) {
  const base64Url = token.split('.')[1];
  const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
  const jsonPayload = decodeURIComponent(atob(base64).split('').map(c => `%${(`00${c.charCodeAt(0).toString(16)}`).slice(-2)}`).join(''));
  return JSON.parse(jsonPayload);
}

// Get user ID from parsed JWT token
const userId = computed(() => {
  if (token) {
    const decodedToken = parseJwt(token);
    return decodedToken.userId;
  }
  return null;
});

// Refs for form fields
const isbn = ref('');
const title = ref('');
const plot = ref('');
const year = ref(0);
const language = ref('');
const pages = ref(0);
const author = ref('');
const publisher = ref('');
const categories = ref('');
const bookFile = ref(null);
const thumbnailFile = ref(null);
const bookFileError = ref('');
const thumbnailFileError = ref('');

// Function to handle book file change
function onBookFileChange(event) {
  const file = event.target.files[0];
  bookFile.value = file;
}

// Function to handle thumbnail file change
function onThumbnailFileChange(event) {
  const file = event.target.files[0];
  thumbnailFile.value = file;
}

// Function to validate file inputs
function validateFileInputs() {
  bookFileError.value = !bookFile.value ? 'Please upload a book file' : '';
  thumbnailFileError.value = !thumbnailFile.value ? 'Please upload a thumbnail image' : '';
  return !bookFileError.value && !thumbnailFileError.value;
}

// Function to upload book
async function uploadBook() {
  if (!validateFileInputs()) {
    return;
  }

  const formData = new FormData();
  formData.append('isbn', isbn.value);
  formData.append('title', title.value);
  formData.append('plot', plot.value);
  formData.append('year', year.value);
  formData.append('language', language.value);
  formData.append('pages', pages.value);
  formData.append('author', author.value);
  formData.append('publisher', publisher.value);
  formData.append('categories', categories.value);
  formData.append('book', bookFile.value);
  formData.append('thumbnail', thumbnailFile.value);

  try {
    const response = await axios.post('http://localhost:3001/upload-book', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
        'Authorization': `Bearer ${token}`,
      },
    });
    console.log(response.data);
    // Redirect to a success page or handle success as needed
    router.push('/success');
  } catch (error) {
    console.error(error.response.data);
    // Handle errors, display error messages, etc.
  }
}
</script>

<style scoped>
/* Styles */
</style>
