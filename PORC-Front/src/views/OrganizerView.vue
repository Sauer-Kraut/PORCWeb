<script lang="ts" setup>
    import IconSidebar from '@/components/IconSidebar.vue';
    import SeasonControll from '@/components/OrganizerPanels/SeasonControll.vue';
    import SeasonOverview from '@/components/OrganizerPanels/SeasonOverview.vue';
    import config from '@/config';
    import { computed, reactive, ref, watch } from 'vue';

    const signUpURL = `${config.getBackendUrl()}`;
    const discordServerURL = 'https://discord.gg/TUQd26DTzg';
    const discordTimeStamps = 'https://www.geeksforgeeks.org/how-to-make-timestamps-on-discord/#what-is-a-discord-timestamp';

    const selectedPannel = ref<string | null>('⌂');
    const viewMode = ref<'wide' | 'centered'>('centered');

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

    export interface ContentArea {
        preview: boolean;
        forceFullHeight: boolean;
        content: string;
    }


    const draggedPlayer = ref<ContentArea | null>(null);
    const HoverDivision = ref<ContentArea | null>(null);
    const LatestHoverDivision = ref<DivisionBlueprint | null>(null);

    const position = reactive({ x: 0, y: 0 })
    let offset = { x: 0, y: 0 }

    const style = computed(() => ({
        position: draggedPlayer.value != null ? ('fixed' as const) : ('relative' as const),
        left: draggedPlayer.value != null ? `${position.x - offset.x}px` : 'auto',
        top: draggedPlayer.value != null ? `${position.y - offset.y}px` : 'auto',
        cursor: draggedPlayer.value != null ? ('grabbing' as const) : ('grab' as const),
        zIndex: draggedPlayer.value != null ? 999 : 'auto',
    }))

    function onMove(e: PointerEvent) {
        position.x = e.clientX
        position.y = e.clientY
        determineHoveredDivision();
    }

    function onDragStart(e: DragEvent, player: PlayerBlueprint) {
        e.preventDefault();

        draggedPlayer.value = player;

        offset.x = e.offsetX
        offset.y = e.offsetY

        window.addEventListener('pointermove', onMove)
        window.addEventListener('pointerup', onDropPlayer)
    }

    function onDropPlayer() {
        if (draggedPlayer.value && HoverDivision.value && HoverDivision.value.name != "unlisted") {
            // Remove from unsorted or previous division
            const index = Blueprint.value.players_to_sort.findIndex(p => p.id === draggedPlayer.value!.id);
            if (index !== -1) {
                Blueprint.value.players_to_sort.splice(index, 1);
            } 

            for (const [i, div] of Blueprint.value.divisions.entries()) {
                const divIndex = div.players.findIndex(p => p.id === draggedPlayer.value!.id);
                if (divIndex !== -1) {
                    Blueprint.value.divisions[i].players.splice(divIndex, 1);
                    break;
                }
            }

            // Add to division (store a non-reactive copy so new additions aren't Vue proxies)
            HoverDivision.value.players.push(markRaw(toRaw(draggedPlayer.value)));
            HoverDivision.value.players = HoverDivision.value.players.sort((a: PlayerBlueprint, b: PlayerBlueprint) => DetermineMovement(b, Blueprint.value).movement - DetermineMovement(a, Blueprint.value).movement);

            // TODO: Update backend with new blueprint

            // Reset refs
            draggedPlayer.value = null;
            HoverDivision.value = null;

            
            window.removeEventListener('pointermove', onMove)
            window.removeEventListener('pointerup', onDropPlayer)
        } 
        else if (draggedPlayer.value) {
            // Remove from unsorted or previous division
            const index = Blueprint.value.players_to_sort.findIndex(p => p.id === draggedPlayer.value!.id);
            if (index !== -1) {
                Blueprint.value.players_to_sort.splice(index, 1);
            } 

            for (const [i, div] of Blueprint.value.divisions.entries()) {
                const divIndex = div.players.findIndex(p => p.id === draggedPlayer.value!.id);
                if (divIndex !== -1) {
                    Blueprint.value.divisions[i].players.splice(divIndex, 1);
                    break;
                }
            }

            // Dropped outside any division, return to unsorted if needed
                if (!Blueprint.value.players_to_sort.find(p => p.id === draggedPlayer.value!.id)) {
                Blueprint.value.players_to_sort.push(markRaw(toRaw(draggedPlayer.value)));
                console.log("didnt find em");
            }

            // Reset ref
            draggedPlayer.value = null;

            
            window.removeEventListener('pointermove', onMove)
            window.removeEventListener('pointerup', onDropPlayer)
        }

        console.log("Current Blueprint:", Blueprint.value);
    }

    function determineHoveredDivision() {
        for (let div of Blueprint.value.divisions) {
            const dropZone = document.getElementById(`division-drop-${div.name}`);
            if (dropZone) {
                const rect = dropZone.getBoundingClientRect();
                if (position.x >= rect.left && position.x <= rect.right &&
                    position.y >= rect.top && position.y <= rect.bottom) {
                    LatestHoverDivision.value = div;
                    HoverDivision.value = div;
                    return;
                }
            }
        }

        // This part is completely uneccecary right now but might come in handy later
        const unsortedZone = document.querySelector('.tierlist.unsorted .drop-zone');
        if (unsortedZone) {
            const rect = unsortedZone.getBoundingClientRect();
            if (position.x >= rect.left && position.x <= rect.right &&
                position.y >= rect.top && position.y <= rect.bottom) {
                LatestHoverDivision.value = {name: "unlisted", order: -1, players:[]} as DivisionBlueprint;
                HoverDivision.value = {name: "unlisted", order: -1, players:[]} as DivisionBlueprint;
                return;
            }
        }
        HoverDivision.value = null;
    }

</script>

<template>

    <div class="d-flex justify-content-center mt-5 mb-5 pb-5">

        <!-- <header class="mb-5 mt-3 text-center pb-4">
            <h1 class="decor-title primary">Organizer Page</h1>
            <p class="content-subtitle">Tool for all organizers to access tournament info and control seasons</p>
        </header> -->


        <div class="d-flex flex-row dashboard-container" :class="{ [`${viewMode}-view`]: true }">

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
    $bg: rgb(24, 24, 24);

    display: flex;
    flex-direction: row;

    width: fit-content;

    gap: 10px;

    border: 1px solid $border-color;
    border-radius: $border-radius;

    //background: linear-gradient(to bottom, $bg, color-mix(in srgb, $bg 50%, var(--primary) 2%)) !important;

    background: $darker-bg;

    overflow: hidden;
    transition: all 0.2s ease-out;
    transition: box-shadow 0.1s;

    resize: horizontal;

    @media (min-width: 1200px) {
        min-width: 1100px;
    }


    &.centered-view {
        width: 80rem;
    }

    &.wide-view {
        align-items: flex-start;
        width: 100%;
    }

    // &:hover {
    //     box-shadow: 2px 2px 0px var(--primary);
    // }
}

.sidebar {
    background-color: rgba(27, 27, 27, 0);
    flex-grow: 1;
    height: fit-content;

    // border: 1px solid rgb(166, 166, 166);

    padding: 1.5rem !important;
    width: 4rem !important;
    margin: 0.5rem;
    border-radius: $border-radius;
}

.sidebar-container {
    border-right: 1px solid $border-color;
    margin-right: -0.5rem;
}
</style>
