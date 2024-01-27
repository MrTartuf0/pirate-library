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
      <UDivider class="mt-8 mb-8">Recently added</UDivider>
      <div class="grid grid-cols-5 w-full gap-8 px-16 mb-16">
        <img class="w-full" src="https://zlibrary-global.com/uploads/covers/2018/lcAYuF1wc6OmJxxnAdEj.webp">
        <img class="w-full" src="https://copertine.hoepli.it/archivio/978/8836/9788836003365.jpg">
        <img class="w-full" src="https://zlibrary-global.com/uploads/books/2021/the-mountain-is-you/GGeOaK7Qkm3twjeAGzIpCgu6P1hdkWNN.webp">
        <img class="w-full" src="https://zlibrary-global.com/uploads/books/2023/covers/ckGdVJyznsigubhmj5OC.webp">
        <img class="w-full" src="https://m.media-amazon.com/images/I/81iDt6AKy5L._AC_UF1000,1000_QL80_.jpg">
        <img class="w-full" src="https://m.media-amazon.com/images/I/61mIq2iJUXL._AC_UF1000,1000_QL80_.jpg">
        <img class="w-full" src="https://m.media-amazon.com/images/I/61sEsmKcwrL._AC_UF1000,1000_QL80_.jpg">
      </div>
    </div>
  </UContainer>
</template>

<script setup>
  async function formSubmit(q){
    await navigateTo('/search/' + q)
  }
</script>
