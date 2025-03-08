<template>
    <div class="container-fill justify-content-center">
        <div class="inner-container">
            <h1 class="titel">404 - Page Not Found</h1>
        </div>
    </div>
</template>

<script lang="ts" setup></script>

<style lang="scss" scoped>
.container-fill {
    min-height: 100vh;
}

.inner-container {
    overflow-x: hidden;
    justify-content: center;
    flex-direction: column;
    align-items: center;
    display: flex;
}

.titel {
    justify-content: center;
    text-align: center;
    margin: 3rem;
    font-style: bold;
    height: fit-content;
}
</style>
