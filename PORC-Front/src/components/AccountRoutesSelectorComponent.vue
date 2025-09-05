<script lang="ts" setup>
    import { ref, onMounted, onBeforeUnmount } from 'vue'

</script>





The fuck does this component do and where did it come from?

<template>
    <div class="menu-container">
        <l class="menu-list">
            <li class="menu-item">
                <router-link to="/matchplanner" class="router-link" @click="closeMenu">Match planner</router-link>
            </li>
            <li class="menu-item">
                <router-link to="/faq" class="router-link" @click="closeMenu">FAQ</router-link>
            </li>
        </l>
    </div>
</template>








<style scoped>
    .menu-container {
        position: absolute;
        top: 1rem;
        right: 1rem;
        width: 200px;
        background-color: #333;
        color: white;
        z-index: 1000;
    }
</style>
