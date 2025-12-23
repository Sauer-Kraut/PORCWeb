<script lang="ts" setup>
    import IconSidebar from '@/components/IconSidebar.vue';
import SeasonOverview from '@/components/OrganizerPanels/SeasonOverview.vue';
import config from '@/config';
    import { ref } from 'vue';

    const signUpURL = `${config.getBackendUrl()}`;
    const discordServerURL = 'https://discord.gg/TUQd26DTzg';
    const discordTimeStamps = 'https://www.geeksforgeeks.org/how-to-make-timestamps-on-discord/#what-is-a-discord-timestamp';
    



</script>

<template>
    <div class="col-12 col-lg-11 col-xl-9 col-xxl-6 mt-5">

        <header class="mb-5 mt-3 text-center pb-4">
            <h1 class="decor-title primary">Organizer Page</h1>
            <p class="content-subtitle">Tool for all organizers to access tournament info and control seasons</p>
        </header>
        
        <div class="d-flex flex-row dashboard-container">

            <div class="d-flex flex-column sidebar-container">
                <IconSidebar class="sidebar" />
            </div>

            <SeasonOverview />

        </div>

        <br/>
        <br/>
    </div>
</template>

<style scoped lang="scss">
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

.mt-6 {
    margin-top: 5rem !important;

    @media (max-width: 768px) {
        margin-top: 2rem !important;
    }
}

.dashboard-container {
    display: flex;
    flex-direction: row;

    gap: 10px;

    border: 1px solid $border-color;
    border-radius: $border-radius;

    background-color: $darker-bg;

    overflow: hidden;
}

.sidebar {
    background-color: rgba(27, 27, 27, 0);
    flex-grow: 1;
    height: fit-content;

    // border: 1px solid rgb(166, 166, 166);

    width: 4.5rem;
    padding: 1.5rem !important;
    width: 5rem !important;
    margin: 0.5rem;
    border-radius: $border-radius;
}

.sidebar-container {
    border-right: 1px solid $border-color;
}
</style>
