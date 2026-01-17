<script setup lang="ts">
    import { ref, computed, reactive, toRaw, markRaw, onMounted } from "vue"
    import type { PlanBlueprint } from "@/models/matchplan_blueprint/MatchplanBlueprintModel.ts"
    import type { PlayerBlueprint } from "@/models/matchplan_blueprint/PlayerBlueprintModel.ts";
    import { showErrorModal } from "@/services/ErrorModalService";
    import type { DivisionModel } from "@/models/matchplan/DivisionModel";
    import type { DivisionBlueprint } from "@/models/matchplan_blueprint/DivisionBlueprintModel.ts";
    import DatePicker from '@vuepic/vue-datepicker';
    import { getDivisionImage } from "@/util/ImageHelper";
    import InfoCard from "./InfoCard.vue";
    import type { Matchplan } from "@/models/matchplan/Matchplan";
    import type { PlayerModel } from "@/models/matchplan/PlayerModel";
    import { filter_str } from "@/util/stringFilter";
    import DiscordAvatarComponent from "../DiscordAvatarComponent.vue";
    import type { PubAccountInfo } from "@/models/pub_account_info/PubAccountInfo";
    import { accountsStore } from "@/storage/st_accounts";
    import { signupStore } from "@/storage/st_signups";
    import { matchplanStore } from "@/storage/st_matchplan";
import type { SignUpInfo } from "@/models/SignUpInfo";

    const totalPlayers = computed(() => {
        const fromDivs = Blueprint.value.divisions.reduce((acc, d) => acc + (d.players?.length ?? 0), 0);
        const extras = Blueprint.value.players_to_sort?.length ?? 0;
        return fromDivs + extras;
    });

    const daysUntilEnd = computed(() => {
        if (!Blueprint.value.end_timestamp) return '—';
        const diff = Math.max(0, Blueprint.value.end_timestamp - Date.now());
        const days = Math.ceil(diff / (1000 * 60 * 60 * 24));
        return `${days} day${days === 1 ? '' : 's'}`;
    });

    const infoCards = computed(() => ([
        { title: 'Season', value: Blueprint.value.season, sub: 'season id' },
        { title: 'Total Players', value: totalPlayers.value, sub: 'across divisions' },
        { title: 'Open Registrations', value: 'Yes', sub: 'public signup' },
        { title: 'Days Left', value: daysUntilEnd.value, sub: 'until season end' },
        { title: 'Pending Signups', value: Blueprint.value.players_to_sort.length, sub: 'to be sorted' },
        { title: 'Preview Mode', value: 'Read-only', sub: 'no competitor changes' }
    ]));

    const Blueprint = ref<PlanBlueprint>({
        "divisions": [],
        "end_timestamp": null,
        "pause_end_timestamp": null,
        "players_to_sort": [],
        "season": 0
    });

    let LiveBlueprint = reactive<PlanBlueprint>({
    "divisions": [
        {
        "name": "Meteorite",
        "order": 0,
        "players": [
            {
            "id": "306467062530965514",
            "tag": "sauerkarut"
            },
            {
            "id": "529716120433721344",
            "tag": ".milong"
            },
            {
            "id": "618546688516161546",
            "tag": "error_real_sir"
            },
            {
            "id": "176842075591933952",
            "tag": "savitarian"
            },
            {
            "id": "445016080868442122",
            "tag": "peppastone"
            },
            {
            "id": "219590386644025342",
            "tag": "UwUwarrior"
            }
        ]
        },
        {
        "name": "Diamond",
        "order": 1,
        "players": [
            {
            "id": "178905571682942976",
            "tag": "2guib"
            },
            {
            "id": "142689578967498762",
            "tag": "omelette.du.fromage."
            },
            {
            "id": "281221611271487489",
            "tag": "delta35"
            },
            {
            "id": "164901822761402368",
            "tag": "yourneighbornat"
            },
            {
            "id": "836935862842294274",
            "tag": "hyper3231"
            },
            {
            "id": "491784389341216768",
            "tag": "abbathorsdagger"
            }
        ]
        },
        {
        "name": "Mithril",
        "order": 2,
        "players": [
            {
            "id": "244833436450291722",
            "tag": "alonshpo"
            },
            {
            "id": "1278614011226751043",
            "tag": "rumblenocerous"
            },
            {
            "id": "400798999025680394",
            "tag": "odwamne"
            },
            {
            "id": "126754956429623298",
            "tag": ".grand"
            },
            {
            "id": "279991236029317121",
            "tag": "crwnd"
            },
            {
            "id": "300974296283480065",
            "tag": "mr_lux43"
            },
            {
            "id": "1329333748151615488",
            "tag": "thormac48"
            }
        ]
        },
        {
        "name": "Adamantium",
        "order": 3,
        "players": [
            {
            "id": "695132277792964618",
            "tag": "donk69420"
            },
            {
            "id": "189354759050887168",
            "tag": "thetrogdor"
            },
            {
            "id": "691615336943845407",
            "tag": "orangenaln"
            },
            {
            "id": "661914123503927307",
            "tag": "._thom_."
            },
            {
            "id": "525674055169212426",
            "tag": "l3mmi05"
            },
            {
            "id": "693603723234115635",
            "tag": "zukamiforever"
            }
        ]
        },
        {
        "name": "Gold",
        "order": 4,
        "players": [
            {
            "id": "620712303100428288",
            "tag": "bobomonster315"
            },
            {
            "id": "553364927797264392",
            "tag": "roaby."
            },
            {
            "id": "764531742450384896",
            "tag": "_ulul_"
            }
        ]
        },
        {
        "name": "Iron",
        "order": 5,
        "players": [
            {
            "id": "537310675656245258",
            "tag": "pentali"
            },
            {
            "id": "848641896817623100",
            "tag": "samdamusican"
            },
            {
            "id": "545487068873228308",
            "tag": "lojay"
            }
        ]
        }
    ],
    "end_timestamp": null,
    "pause_end_timestamp": null,
    "players_to_sort": [],
    "season": 0
    });

    const PlanStorage = matchplanStore();
    const accountStorage = accountsStore();
    const signupStorage = signupStore();

    // TODO: Synchronize this with the back end via a web hook
    function UpdateBlueprint(newBlueprint: PlanBlueprint) {
        Blueprint.value = newBlueprint;
        console.log("Blueprint updated:", Blueprint.value);

        if (false) {
            // revert to old blueprint
        }
    }

    async function GetBlueprint() {
        for (let div of LiveBlueprint.divisions) {
            div.players = div.players.sort((a: PlayerBlueprint, b: PlayerBlueprint) => DetermineMovement(b, LiveBlueprint).movement - DetermineMovement(a, LiveBlueprint).movement);
        }
        console.log(LiveBlueprint)
        Blueprint.value = LiveBlueprint;
    }

    interface RankingMovement {
        new: boolean;
        movement: number; // number of divisions moved, plus is up, minus is down
        age: number; // number of seasons since last signup
    }

    const matchplanCache = ref<(string | Matchplan)[]>([])
    const accountInfos = ref<Record<string, PubAccountInfo>>({} as Record<string, PubAccountInfo>)
    const signupInfos = ref<Record<string, SignUpInfo>>({} as Record<string, SignUpInfo>)


    async function setCache() {
        const seasons = (await (PlanStorage.get_all_season_infos())).sort((a, b) => b.start_timestamp - a.start_timestamp).slice(0, 5);

        let matchplanlist = [];
        for (const [idx, season] of seasons.entries()) {
            let matchplan = await PlanStorage.get_matchplan(season.name);
            matchplanlist.push(matchplan);
        }
        
        matchplanCache.value = matchplanlist;

        let players: string[] = [];
        for (let plan of matchplanlist) {
            if (typeof plan != "string") {
                for (let div of plan.divisions) {
                    for (let player of div.players) {
                        if (!players.find(p => player.id == p)) {
                            players.push(player.id);
                        }
                    }
                }
            }
        }

        let playerInfosfut = accountStorage.get_competitors_min(players);
        let signupsfut = signupStorage.get_signups(null);
        let [playerInfos, signups] = await Promise.all([playerInfosfut, signupsfut]);
        if (typeof playerInfos != "string") {
            accountInfos.value = Object.fromEntries(playerInfos.map(p => [p.id, p]));
        }
        signupInfos.value = Object.fromEntries((signups ?? []).map(s => [s.discord_id, s]));
    }


    function DetermineMovement(player: PlayerBlueprint, planBlueprint: PlanBlueprint, division?: DivisionBlueprint | null): RankingMovement {
        for (const [idx, matchplan] of matchplanCache.value.entries()) {

            if (typeof matchplan === "string" || matchplan === null) {
                showErrorModal("An Error occured while retrieving the matchplan: " + matchplan);
                continue;
            }
            else {
                let lastDivision: DivisionModel | null = null;

                for (const division of matchplan.divisions) {
                    const foundPlayer = division.players.find(p => p.id === player.id);
                    if (foundPlayer) {
                        lastDivision = division;
                        break;
                    }
                }

                if (lastDivision === null) {
                    continue;
                }

                let equivelantDivision: DivisionBlueprint | null = null;
                let currentDivision: DivisionBlueprint | null = null;

                for (const division of planBlueprint.divisions) {
                    if (division.players.find(p => p.id === player.id)) {
                        currentDivision = division;
                    }
                    if (lastDivision && division.name === lastDivision.name) {
                        equivelantDivision = division;
                    }
                }

                if (division && division != null) {
                    currentDivision = division;
                }

                if (lastDivision && equivelantDivision && currentDivision) {
                    let divisionDifference = equivelantDivision.order - currentDivision.order;

                    if (division?.name == "unlisted") {
                        divisionDifference = 0;
                    }

                    return {
                        new: false,
                        movement: divisionDifference,
                        age: idx
                    } as RankingMovement;
                } else {
                    return {
                        new: true,
                        movement: 0,
                        age: 0
                    } as RankingMovement;
                }
            }
        }

        return {
            new: true,
            movement: 0,
            age: 0
        } as RankingMovement;
    }

    function translateMovement(movement: RankingMovement): string {
        const movements = ['up-l', 'up-s', 'mov-0', 'down-s', 'down-l'];

        let mov = -movement.movement + 2;

        if (mov < 0) {
            mov = 0;
        } 
        else if (mov > 4) {
            mov = 4;
        }
        if (movement.new) {
            mov = 2;
        }    

        return movements[mov];
    }

    
    // Player drag and drop logic

    const draggedPlayer = ref<PlayerBlueprint | null>(null);
    const HoverDivision = ref<DivisionBlueprint | null>(null);
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


    onMounted(async () => {
        await setCache();
        await GetBlueprint();
    });
</script>
                    

<template>
    <div class="dashboard p-0 d-flex">

        <!-- Season Header -->
        <div class="season-bar">
            <div class="season-left">
                <h2 class="season-title">Season Controll</h2>
                <p class="season-sub">Access seeding, placements and controls</p>
            </div>

            <div class="ms-auto">
                <div class="season-select-wrap">
                    <div class="season-meta">
                        <span class="badge-active">Active</span>
                        <span class="muted">Ends in 12d 4h</span>
                    </div>
                </div>
            </div>

            <div class="season-right">
                <button class="btn btn-small" @click.prevent>Export CSV</button>
            </div>
        </div>

        <div class="panel-card flex-row">

            <!-- Season Info -->
            <div class="season-info d-flex flex-column">

                <!-- Left: season info cards -->
                <div class="info-card mt-0">
                    <h4 class="preview-title">Season Info</h4>
                    <div class="config-grid">
                        <InfoCard titel="Season" :value='Blueprint.season.toString()' subtitle="season id"/>

                        <InfoCard titel="Total Players" :value='totalPlayers.toString()' subtitle="across divisions"/>

                        <InfoCard titel="Open Registrations" value="Yes" subtitle="public signup"/>

                        <InfoCard titel="Days Left" :value='daysUntilEnd' subtitle="until season end"/>

                        <InfoCard titel="Pending Signups" :value='Blueprint.players_to_sort.length.toString()' subtitle="to be sorted"/>

                        <InfoCard titel="Preview Mode" value="Read-only" subtitle="no competitor changes"/>
                    </div>
                </div>

                <!-- Middle: Name and date pickers -->
                <div class="d-flex flex-column h-100">

                    <!-- Season Name -->
                    <div class="info-card mb-3">
                        <div class="date-row">
                            <label class="form-label">Season Name</label>
                            <input class="form-input m-0" v-model="time" placeholder="Season Name" />
                        </div>
                    </div>

                    <!-- Season Date -->
                    <div class="info-card">
                        <div class="date-row">
                            <label class="form-label">Season End Date</label>
                            <DatePicker class="range_selector season-date" v-model="time" :range="false" placeholder="Select End" />
                        </div>

                        <div class="date-row">
                            <label class="form-label">Pause End Date</label>
                            <DatePicker class="range_selector season-date" v-model="time" :range="false" placeholder="Select Pause End" />
                        </div>

                        <div class="hint muted">Dates are local. Use these to preview the season timeline.</div>
                    </div>
                </div>
                

                <!-- Right: preview and actions -->
                <aside class="actions">
                    <div class="preview">
                        <h4 class="preview-title mb-2 pb-1">Divisions Preview</h4>
                        <ul class="preview-list">
                            <li v-for="div in Blueprint.divisions" :key="div.order" class="preview-item">
                                <span class="dot" :style="{ background: `var(--${div.name.toLowerCase()}, #6c6c6c)` }"></span>
                                <span class="pname">{{ div.name }}</span>
                                <span class="pcount muted">{{ div.players.length }}</span>
                            </li>
                        </ul>
                    </div>

                    <div class="d-flex flex-row justify-content-end mt-2 align-items-center">
                        <p class="hint muted m-0 me-auto h-50ms-1">52 players unsorted</p>
                        <button class="btn btn-small" @click.prevent>Reset</button>
                    </div>
                </aside>

            </div>

            <!-- <div class="d-flex flex-row mt-4">
                <div class="seperator-h me-4"></div>
                <i class="icon icon-chevron-up" :style="{transform: true ? 'rotate(180deg) !important' : 'rotate(0deg) !important'}"></i>
                <div class="seperator-h ms-4"></div>
            </div> -->

            <!-- Tier List (logo-based, keep names & counts) -->
            <div class="d-flex flex-row flex-grow-1">
                <!-- Tierlist Sorted-->
                <div class="tierlist d-flex flex-column justify-content-center flex-grow-1">
                    <div class="tier d-flex flex-row align-items-center" v-for="div in Blueprint.divisions" :key="div.order">
                        <img :src="getDivisionImage(div.name)" class="division-icon" />
                        <div class="tier-info">
                            <div class="tier-name">{{ div.name }}</div>
                            <div class="tier-count muted">{{ div.players.length }} players</div>
                        </div>
                        <div class="drop-zone d-flex flex-row w-100 h-100"
                            :id='`division-drop-${div.name}`'
                            @dragover.prevent="console.log('hi')"
                            @drop="onDropPlayer"
                            :class="{ 'hovered': HoverDivision === div }">
                            <div v-for="player in div.players" :key="player.id" 
                                draggable="true" 
                                @dragstart="(e) => onDragStart(e, player)" 
                                :style="draggedPlayer?.id === player.id ? style : {}"
                                class="player-item m-1">

                                <div class="icon-box d-flex flex-column position-relative movement-box" :class="[draggedPlayer?.id === player.id ? `${translateMovement(DetermineMovement(player, Blueprint, LatestHoverDivision))}`: `${translateMovement(DetermineMovement(player, Blueprint))}`]">
                                    <div class="icon icon-chevron-up me-1"></div>
                                    <div class="icon icon-chevron-up support-chevron me-1"></div>
                                </div>
                                <DiscordAvatarComponent :account="accountInfos[player.id]" class="me-2 avatar"/>
                                {{ filter_str(player.tag, 10) }}

                                <div class="bp ms-2 ps-2" v-if="signupInfos[player.id] && signupInfos[player.id].bp != 0">
                                    {{ signupInfos[player.id].bp }}
                                </div>
                            </div>

                        </div>
                    </div>
                </div>

                <!-- Unsorted Players -->
                <div class="tierlist unsorted d-flex flex-column">
                    <div class="unsorted-header">
                        <h4 class="preview-title align-text-center mb-2 pb-1">Unsorted Players</h4>
                    </div>
                    <div class="drop-zone d-flex flex-column flex-grow-1 justify-content-start align-items-center"
                        @dragover.prevent
                        @drop="onDropPlayer"
                        :class="{ 'hovered': HoverDivision === null }">
                        <div v-for="player in Blueprint.players_to_sort" :key="player.id" 
                            draggable="true" 
                            @dragstart="(e) => onDragStart(e, player)" 
                                :style="draggedPlayer?.id === player.id ? style : {}"
                            class="player-item">
                            <div class="icon-box d-flex flex-column position-relative pe-4 movement-box" v-if="DetermineMovement(player, Blueprint, LatestHoverDivision).movement != 0">
                                <div class="icon icon-chevron-up me-1" :class="[draggedPlayer?.id === player.id ? `${translateMovement(DetermineMovement(player, Blueprint, LatestHoverDivision))}`: `${translateMovement(DetermineMovement(player, Blueprint))}`]"></div>
                                <div class="icon icon-chevron-up support-chevron me-1" :class="[draggedPlayer?.id === player.id ? `${translateMovement(DetermineMovement(player, Blueprint, LatestHoverDivision))}`: `${translateMovement(DetermineMovement(player, Blueprint))}`]"></div>
                            </div>
                            <DiscordAvatarComponent :account="accountInfos[player.id]"/>
                            {{ filter_str(player.tag, 5) }}
                        </div>
                    </div>
                </div>

            </div>
            
        </div>

    </div>
</template>

<style scoped lang="scss">
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    $card-background: #1c1c1c00;

    /* === Faithful theme === */

    .dashboard {
        // max-width: 1400px;
        margin: auto;
        display: flex;
        flex-direction: column;
        flex-grow: 1;
        gap: 24px;

        max-width: 1600px;

        .season-info {
            min-width: 21rem;
        }
    }

    .muted {
        color: #a0a0a0;
        font-size: 13px;
        line-height: 1.5rem;
    }

    /* Season Bar */

    .season-bar {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 16px;
        background: $darker-bg;
        border: 1px solid $border-color;
        border-radius: 14px;
        padding: 14px 18px;
        backdrop-filter: blur(6px);
    }

    .season-left {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .season-title {
        margin: 0;
        font-size: 18px;
        font-weight: 700;
        color: #eaeaea;
    }

    .season-sub {
        margin: 0;
        color: #a0a0a0;
        font-size: 13px;
    }

    .season-center {
        display: flex;
        align-items: center;
        gap: 12px;
        flex: 1 1 auto;
        justify-content: center;
    }

    .season-select-wrap {
        display: flex;
        align-items: center;
        gap: 12px;
    }

    .season-right {
        display: flex;
        gap: 8px;
        align-items: center;
    }

    .season-select {
        background: #151515;
        border: 1px solid $border-color;
        color: #eaeaea;
        padding: 10px 16px;
        border-radius: 999px;
    }

    .season-meta {
        display: flex;
        gap: 12px;
        align-items: center;
    }

    .badge-active {
        background: color-mix(in srgb, var(--primary) 16%, transparent);
        color: var(--primary);
        padding: 6px 12px;
        border-radius: 999px;
        font-size: 13px;
    }

    .graph-card {
        background: $darker-bg;
        border: 1px solid $border-color;
        border-radius: 18px;
        padding: 20px;
    }

    .form-grid {
        display: grid;
        grid-template-columns: 1fr 1fr 0.8fr;
        gap: 18px;
        align-items: start;
    }

    .actions {
        // background: linear-gradient(180deg, rgba(255,255,255,0.01), transparent);
        border: 1px solid $secondary-border-color;
        border-radius: 12px;
        padding: 14px;
    }

    .date-row {
        display: flex;
        flex-direction: column;
        gap: 8px;
        margin-bottom: 12px;
    }

    .hint { font-size: 12px; }

    .preview-title { margin: 0 0 8px 0; font-size: 14px; }

    .preview-list { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 8px; }

    .preview-item { display: flex; align-items: center; gap: 10px; }

    .dot { 
        width: 10px; 
        height: 10px; 
        border-radius: 999px; 
        display:inline-block;

        &::after {
            content: '';
            display: block;
            position: relative;
            width: 10px;
            height: 10px;
            border-radius: 999px;

            background: #0000002a;
        }
    }

    .pname { font-weight: 600; }

    .pcount { margin-left: auto; font-size: 12px; }

    /* Season info small cards */
    .config-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px; margin-top: 8px; }

    .config-card { 
        background: linear-gradient(180deg, rgba(255,255,255,0.01), transparent); 
        border: 1px solid rgba(255,255,255,0.02); 
        padding: 10px; 
        border-radius: 8px; 
    }

    .mini-title { font-size: 12px; color: #a0a0a0; font-weight: 600; }
    .mini-value { font-size: 18px; font-weight: 700; margin-top: 6px; color: #eaeaea; }
    .mini-sub { font-size: 12px; color: #9aa0a6; margin-top: 6px; }

    @media (max-width: 920px) {
        .form-grid { grid-template-columns: 1fr; }
        .action-buttons { justify-content: space-between }
    }

    

    h3 {
        font-weight: 600;
    }

    .form-check-input {
        border-color: $border-color !important;
    }

    .page-arrows {
        i {
            cursor: pointer;
            &:hover {
                color: rgb(255, 255, 255, 0.8);
            }
        }
    }

    .page-counter {
        margin-bottom: 0.1rem;
    }


    .form-input {
        background-color: transparent !important;
        margin-bottom: 0 !important;
    }

    .dp__theme_light {
        --dp-background-color: #0f0f0f;
    }
















    .tierlist {
        min-height: 150px;
        border: 1px solid $secondary-border-color;
        border-radius: 12px;

        .tier {
            width: 100%;
            border-bottom: 1px solid $secondary-border-color;
            padding: 8px;
            display: flex;
            align-items: center;
            gap: 12px;

            .division-icon {
                width: 4rem;
                height: 4rem;
                object-fit: contain;
                margin: -8px;
                margin-left: 4px;
                margin-right: 8px;
                margin-bottom: 0px;
            }

            .tier-info {
                display: flex;
                flex-direction: column;
                border-right: solid 1px $secondary-border-color;
                min-width: 7.5rem;
                padding-right: 4px;
            }

            .tier-name { font-weight: 700; color: #eaeaea; }
            .tier-count { color: #a0a0a0; font-size: 12px; }

            .drop-zone {
                flex-grow: 1;
                height: 100%;
                padding: 6px;
                padding-left: 12px;
                border-radius: 8px;
                transition: background-color 0.15s ease;
                transition: margin 0.2s;

                flex-wrap: wrap;

                align-items: center;

                gap: 8px;

                &.hovered {
                    background-color: color-mix(in srgb, var(--primary) 15%, transparent);
                    border: 2px dashed var(--primary);
                    padding: 4px;
                    padding-right: 6px;
                    padding-left: 12px;

                    margin-inline: -1px;
                }

                @include media-breakpoint-up(md) {
                    min-width: 40rem;
                }
            }

            &:last-child {
                border-bottom: none;
            }
        }

        &.unsorted {
            margin-left: 18px;
            padding: 14px;
            min-width: 10rem;

            overflow: hidden;
            position: relative;

            .unsorted-header {
                border-bottom: 1px solid $secondary-border-color;
                margin-bottom: 8px;
                padding-bottom: 4px;
            }

            // .drop-zone {
            //     position: absolute;
            //     overflow: scroll;

            //     margin-top: 3rem;
            //     height: calc(100% - 3rem)
            // }

            .player-item {
                margin-bottom: 8px;
            }
        }

        .player-item {
            display: flex;
            justify-content: center;

            width: fit-content;

            padding: 6px 8px;
            height: 2.3rem;
            border: 1px solid $secondary-border-color;
            border-radius: 8px;
            color: #d2d2d2;
            font-weight: 600;
            line-height: 1.35rem;

            cursor: grab;
            user-select: none;
            background-color: $darker-bg;

            .bp {
                border-left: 1px solid $secondary-border-color;
            }

            .movement-box {
                margin-top: 0.4rem;
                $scaling: 0.7;

                overflow-x: hidden;
                transition: width 0.2s;
                transition: padding 0.2s;

                padding-right: 1.5rem;

                width: 1rem;
                height: 1rem;

                &.mov-0 {
                    width: 0rem !important;
                    padding: 0 !important;
                
                    .icon-chevron-up {
                        rotate: 90deg;
                        transform: translate(-2px, 0px) scale($scaling);
                    }
                }

                &.up-s .icon-chevron-up {
                    color: rgb(97, 214, 74);
                    transform: translate(0px, -3px) scale($scaling);
                }

                &.up-l .icon-chevron-up {
                    color: rgb(45, 244, 27);
                    transform: translate(0px, 1px) scale($scaling);

                    &.support-chevron {
                        transform: translate(0px, -7px) scale($scaling);
                    }
                }

                &.down-s .icon-chevron-up {
                    color: orange;
                    rotate: 180deg;
                }

                &.down-l .icon-chevron-up {
                    color: red;
                    rotate: 180deg;
                    transform: translate(0px, -4px) scale($scaling);

                    &.support-chevron {
                        transform: translate(0px, 4px) scale($scaling);
                    }
                }    
                
                .icon-chevron-up {
                    $scaling: 0.7;

                    width: 1rem;
                    height: 1rem;

                    position: absolute;

                    top: 0;

                    font-weight: 400;

                    transform: scale($scaling);
                    transition: all 0.2s;
                }
            }

            
        }
    }


    .seperator-h {
        margin-inline: -1.25rem;
        border: none !important;
        background: none;
        border-top: 1px solid $secondary-border-color !important;
    }

    .panel-card {
        border: none;
        padding: 0 !important;
        margin: 0 !important;
    }








</style>
