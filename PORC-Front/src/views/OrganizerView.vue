<script lang="ts" setup>
    import IconSidebar from '@/components/IconSidebar.vue';
    import SeasonControll from '@/components/OrganizerPanels/SeasonControll.vue';
    import SeasonOverview from '@/components/OrganizerPanels/SeasonOverview.vue';
    import config from '@/config';
    import { ref, watch } from 'vue';

    const signUpURL = `${config.getBackendUrl()}`;
    const discordServerURL = 'https://discord.gg/TUQd26DTzg';
    const discordTimeStamps = 'https://www.geeksforgeeks.org/how-to-make-timestamps-on-discord/#what-is-a-discord-timestamp';

    const selectedPannel = ref<string | null>('⌂');

    watch(selectedPannel, (newVal) => {
        console.log("Selected Pannel changed to: ", newVal);
    });
    
    const Pannels: (string | null)[] = [
        "⌂" ,
        "🔍",
        "📥",
        null,
        "🔔",
        null,
        "🕘",
        "📅",
        "👤"
    ]



</script>

<template>
    <div class="d-flex justify-content-center mt-5">

        <!-- <header class="mb-5 mt-3 text-center pb-4">
            <h1 class="decor-title primary">Organizer Page</h1>
            <p class="content-subtitle">Tool for all organizers to access tournament info and control seasons</p>
        </header> -->
        
        <div class="d-flex flex-row dashboard-container">

            <div class="d-flex flex-column sidebar-container">
                <IconSidebar class="sidebar" :items="Pannels" v-model="selectedPannel"/>
            </div>

            <div class="d-flex flex-grow-1 p-4">
                <SeasonOverview v-if="selectedPannel == '⌂'" />
                <SeasonControll v-else/>
            </div>

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

    width: fit-content;

    gap: 10px;

    border: 1px solid $border-color;
    border-radius: $border-radius;

    background-color: $darker-bg;

    overflow: hidden;
    transition: all 0.3s ease-in-out;

    @media (min-width: 1200px) {
        min-width: 1100px;
    }
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
    margin-right: -0.5rem;
}
</style>
