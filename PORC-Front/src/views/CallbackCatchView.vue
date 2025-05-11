<script lang="ts" setup>
    import { useRouter } from 'vue-router'
    import { onMounted } from 'vue'

    const router = useRouter()

    onMounted(() => {
    // optional: do any login status check or exchange here

    // redirect to the main view (e.g., "/")
    router.push('/')
    })
</script>

<template>
</template>