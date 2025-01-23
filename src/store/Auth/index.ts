import { defineStore } from 'pinia';

export const useAuthStore = defineStore('auth', {
    state: () => ({
        message: 'Welcome to Home Store!',
        count: 0,
    }),
    actions: {
        increment() {
            this.count++;
        },
        setMessage(newMessage: string) {
            this.message = newMessage;
        },
    },
});
