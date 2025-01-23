<script setup lang="ts">
import { ref, watch } from 'vue';
import { useAuthStore } from '@/store';
import { useHello, useGetList } from '@/utils/client';

const authStore = useAuthStore();
const hello = useHello();
const getList = useGetList();

const msg = ref('');

console.log('hello', hello);
console.log('getList', getList);

watch(msg, () => {
    authStore.increment();
    console.log('useAuthStore', authStore.count);
});
</script>

<template>
    <main>
        <h1>Playground</h1>
        <InputText v-model="msg" />
        <p>{{ msg }}</p>
    </main>
</template>

<style scoped></style>
