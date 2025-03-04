<script lang="ts" setup>
    /// <reference types="../../node_modules/.vue-global-types/vue_3.5_0_0_0.d.ts" />
    import type { PubAccountInfo } from '@/models/PubAccountInfo';

    const props = defineProps<{
        Players: PubAccountInfo[];
    }>();

    const selectedPlayer = defineModel('selectedPlayer');
</script>

<template>
    <div class="container">
        <div v-for="(player, index) in Players" 
            :key="index" 
            class="player" 
            :data-selected="selectedPlayer === index ? 'true' : 'false'"
            @click="selectPlayer(index)">
            {{ player }}
        </div>
    </div>
</template>

<style lang="scss">
    @import '@/assets/scss/styles.scss';

    .container {
        display: flex;
        flex-wrap: wrap-reverse; /* Makes the last row come up to the top */
        width: 100%;
        gap: 0px;
        padding: 0px !important;
        border: 0;
    }

    .player {
        width: calc(24.6%); /* Ensures 4 items per row */
        background: rgb(50, 50, 50);
        // background: linear-gradient(135deg, rgb(49, 46, 46), rgb(46, 49, 49));
        // padding: 20px;
        margin: 0.2%;
        text-align: center;
        border: 0px solid rgb(177, 177, 177);
        box-sizing: border-box;
        border-radius: 2px;
        height: 4rem;
        cursor: pointer;
        transition: 0.1s, box-shadow 0.3s;

        &:hover {
            background: rgb(66, 66, 66);
        }
    }
</style>
