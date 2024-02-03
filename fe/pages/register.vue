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
          @submit="onSubmit"
        >
          <UFormGroup label="Email" name="email">
            <UInput
              v-model="state.email"
              icon="i-heroicons-user-circle"
              trailing
              placeholder="Ye electronic parrot address, matey!"
            />
          </UFormGroup>

          <UFormGroup label="Password" name="password">
            <UInput
              v-model="state.password"
              type="password"
              icon="i-heroicons-lock-closed"
              trailing
              placeholder="Secret map coordinates, savvy?"
            />
          </UFormGroup>

        <UButton type="submit" class="mt-16 w-full justify-center" @click="handleFormSubmit"> Register now! </UButton>
        <UDivider label="OR" class="mb-16"/>
        <div class="flex justify-between">
          <p class="text-sm">Already aboard? Sail on, matey!</p> 
          <UButton @click="handleLoginRedirect">Log In</UButton>
        </div>
        </UForm>
      </div>
    </UContainer>
  </div>
</template>


<script setup>

import { useRouter } from 'vue-router';
const router = useRouter();

const state = reactive({
  email: undefined,
  password: undefined,
});

const validate = (state) => {
  const errors = [];
  if (!state.email) errors.push({ path: "email", message: "Required" });
  if (!state.password) errors.push({ path: "password", message: "Required" });
  return errors;
};

const handleFormSubmit = async () => {
  const response = await fetch('http://localhost:3001/create', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      email: state.email,
      password: state.password
    })
  });

  const data = await response.json();

  if (response.ok) {
    const token = data.token;
    localStorage.setItem('token', token);
    router.push('/');
  } else {
    console.error('Registration failed:', data.error);
  }
};

const handleLoginRedirect = () => {
  router.push('/login');
};
</script>