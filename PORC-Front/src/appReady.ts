import { ref } from 'vue';
export const appReady = ref(false);

export async function waitForAppReady() {
    while (!appReady.value) {
        await new Promise(resolve => setTimeout(resolve, 50)); // waits for 100ms
    }
}