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
            <p class="text-xs mt-4 mb-4">
              Lost yer way in Davy Jones' locker?
              <span class="text-primary-300 underline cursor-pointer">Retrieve yer secret path</span>
              , landlubber!
            </p>
          </UFormGroup>

          <UButton type="submit" class="mt-16 w-full justify-center" @click="handleLogin">
            Log In
          </UButton>

          <UDivider label="OR" class="mb-16"/>
          <div class="flex justify-between">
            <p class="text-sm">Join the Pirate Crew: Chart yer course</p> 
            <UButton to="/register">Register now!</UButton>
          </div>
        </UForm>
      </div>
    </UContainer>
    <UNotifications />
  </div>
</template>


<script setup>
import { useRouter } from 'vue-router';
const router = useRouter();
const toast = useToast()

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

const handleLogin = async () => {
  const response = await fetch('http://localhost:3001/login', {
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
    toast.add({ title: 'Error' , description: data.error})
    console.error('Login failed:', data.error);
  }
};

</script>
