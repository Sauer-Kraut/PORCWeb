<script lang="ts" setup>
import type { MatchModel } from '@/models/matchplan/MatchModel';
import type { PlayerPerformance } from '@/models/matchplan/PlayerPerformancModel';
import { filter_str } from '@/util/stringFilter';
import { defineProps, onMounted, ref, watch } from 'vue';

const props = defineProps<{
    divisionName: String;
    performances: PlayerPerformance[];
    matches?: MatchModel[];
    minimal?: boolean
}>();

const highlightedPlayerId = defineModel<string>('highlightedPlayerId', { default: '' });
const highlightPin = defineModel<boolean>('highlightPing', { default: false });

const internalPerformances = ref<PlayerPerformance[]>([]);

function selectPlayer(p: PlayerPerformance) {
    if (!highlightPin.value) {
        highlightedPlayerId.value = p.player.id;
    }
}

function unselectPlayer(p: PlayerPerformance) {
    if (highlightedPlayerId.value == p.player.id && !highlightPin.value) {
        highlightedPlayerId.value = '';
    }
}

function pin_player(p: PlayerPerformance) {
    if(props.minimal) {return}
    if (highlightedPlayerId.value == p.player.id) {
        highlightPin.value = !highlightPin.value;
    } else {
        highlightedPlayerId.value = p.player.id;
        highlightPin.value = true;
    }
}

// Watch for changes in the performances prop
watch(
    () => props.performances,
    (newPerformances) => {
        console.log('Performances updated:', newPerformances);
        internalPerformances.value = newPerformances;
    },
    { immediate: true }, // Ensure the watcher runs immediately on mount
);

function avgAdvantage(player: PlayerPerformance) {
    const val = player.cumulative_match_difference;
    const save_val = isNaN(val) ? 0 : val;

    const out = save_val.toString();

    if (save_val > 0) {
        return ("+" + out)
    } else {
        return out
    }
}


interface MatchInfo {
    win: boolean,
    opponent: string,
    score: [number, number]
}

function getMatches(pId: string): MatchInfo[] {
    let matchInfos = []
    for (let match of props.matches ?? []) {
        if (!((match.p1score ?? 0) == 0 && (match.p2score ?? 0) == 0)){
            if (match.p1.id == pId) {
                const info = {
                    win: ((match.p1score ?? 0) > (match.p2score ?? 0)),
                    opponent: match.p2.tag,
                    score: [match.p1score, match.p2score]
                } as MatchInfo
                matchInfos.push(info);
            }
            if (match.p2.id == pId) {
                const info = {
                    win: ((match.p2score ?? 0) > (match.p1score ?? 0)),
                    opponent: match.p1.tag,
                    score: [match.p2score, match.p1score]
                } as MatchInfo
                matchInfos.push(info);
            }
        }
    }
    return matchInfos;
}

onMounted(async () => {
    internalPerformances.value = props.performances;
});
</script>

<template>
    <div class="leaderboard-cont d-flex flex-column">




        <!-- <div class="leaderboard-row head-row">
            <span>Player</span>
            <span>Matches</span>
            <span title="This shows the average match score difference over all played sets.">Advantage</span>
        </div> -->
            
        <div class="d-flex flex-column p-0 m-0" :style="{overflowY: 'scroll'}">
            <div v-for="(player, index) in internalPerformances" :key="player.player.id" class="leaderboard-row"
                @mouseover="selectPlayer(player)"
                @mouseleave="unselectPlayer(player)"
                @click="pin_player(player)"
                :class="{'pinned': highlightPin && highlightedPlayerId == player.player.id, 'minimal': minimal}"
                >
                <span class="row-index">0{{ index +1 }}</span>
                <div class="d-flex row-name">
                    <span :class="[index === 0 ? 'pedestal first-place' : index === 1 ? 'pedestal second-place' : index === 2 ? 'pedestal third-place' : '']">{{ filter_str(player.player.tag, 12) }}</span>
                </div>

                <!-- Max 5 -->
                <div class="d-flex flex-row gap-2">
                    <div v-for="(matchInfo, miIndex) of (getMatches(player.player.id) ?? [])" :key="miIndex" class="match-marker" :class="{ win: matchInfo.win }">
                        {{ matchInfo.win ? 'W' : 'L' }} {{ (matchInfo.score[0] ?? 0) + ':' + (matchInfo.score[1] ?? 0) }} vs {{ filter_str(matchInfo.opponent, 7) }}
                    </div>
                    <!-- <div class="match-marker" :class="{win: false}">L 2:4 vs {{ filter_str("Savitarian", 7) }}</div>
                    <div class="match-marker" :class="{win: true}">W 5:4 vs {{ filter_str("The edj", 7) }}</div>
                    <div class="match-marker" :class="{win: true}">W 4:2 vs {{ filter_str("Omlette du Fromage", 7) }}</div>
                    <div class="match-marker" :class="{win: false}">L 6:7 vs {{ filter_str("Atrain", 7) }}</div> -->
                </div>
                <span class="row-score">{{ player.wins }}-{{ player.matches - player.wins }}</span>
                <span class="row-advantage"> {{ avgAdvantage(player) }}</span>
            </div>
        </div>

        <!-- ß-1 class -->
        <div class="ß-1"></div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.leaderboard-cont {
    width: 100%;
    height: fit-content;

    padding: 0rem !important;
    padding-inline: 0rem !important;

    transition: all 0.6s ease !important;

    .leaderboard-row {
        display: grid;
        grid-template-columns: 0.5fr 1.25fr 4fr 0.75fr 0.5fr;

        justify-content: flex-start;
        align-items: baseline;
        text-align: start;

        height: 3.5rem;
        width: auto;

        padding: 0.25rem 1rem !important;
        margin: 0.25rem 1rem;

        overflow: hidden;
        align-items: center;
        justify-content: center;
        text-align: center;

        // border-top: $border-color solid 1px;
        border-radius: 6px;

        // margin-bottom: 5px;
        color: $text-color;
        font-size: 0.925rem;
        text-wrap: none;
        clip: auto;

        transition: all 0.1s, height 0s, margin 0s !important;

        * {
            text-align: start;
            height: 1.5rem;
        }

        &.head-row {
            background-color: rgba(255, 255, 255, 0.0);
            height: 3.75rem !important;

            font-weight: bold;
            font-size: 1.15rem;
            color: $secondary-text;
            text-align: center;
            border: 0px !important;
        }

        &:hover {
            //background-color: rgba(255, 255, 255, 0.05);
            background: color-mix(in srgb, white 5%, rgba(255, 255, 255, 0));
            // color: black;
            // cursor: pointer;

            // .match-marker {
            //     --marker-color: black !important;
            //     background: color-mix(in srgb, var(--marker-color) 10%, transparent);
            //     border: 1px solid color-mix(in srgb, var(--marker-color) 30%, transparent);
            // }
        }

        &.pinned {
            //background-color: rgba(255, 255, 255, 0.05);
            background: color-mix(in srgb, var(--primary) 80%, rgb(255, 255, 255));
            color: black;
            cursor: pointer;

            height: 3rem;
            margin: 0.5rem 1rem;

            .match-marker {
                --marker-color: black !important;
                background: color-mix(in srgb, var(--marker-color) 10%, transparent);
                border: 1px solid color-mix(in srgb, var(--marker-color) 30%, transparent);

                &.win {
                    background: color-mix(in srgb, var(--marker-color) 80%, transparent);
                    color: var(--primary);
                }
            }
        }

        &.minimal:not(.pinned) {
            display: none;
            pointer-events: none;
            cursor: default;

            &:hover {
                background: transparent !important;
            }
        }

        &.minimal {
            transition: none !important;
        }

        @include media-breakpoint-down(md) {
            display: grid;
            grid-template-columns: 1fr 1fr;

            :nth-child(3) {
                display: none !important;
            }

            // &.head-row {
            //     :nth-child(n + 2) {
            //         display: none !important;
            //     }
            // }
        }

        .row-index {
            font-weight: 600;
        }

        .row-name {
            font-weight: 600;
        }

        .row-score, .row-advantage {
            font-size: 0.8rem;
            letter-spacing: 2px;
            font-weight: 400;
            font-family: monospace;
        }

        .match-marker {
            --marker-color: rgb(231, 71, 71);

            display: flex;
            flex-direction: row;
            flex-wrap: nowrap;

            justify-content: center;
            text-align: start;

            overflow: hidden;

            width: fit-content;
            min-width: 3rem;
            max-width: 9rem;

            height: 1.4rem;
            padding-inline: 0.4rem;

            font-size: 0.65rem;
            font-weight: 700;
            line-height: 1.25rem;

            letter-spacing: 1px;

            background: color-mix(in srgb, var(--marker-color) 5%, transparent);
            border: 1px solid color-mix(in srgb, var(--marker-color) 10%, transparent);
            border-radius: 4px;
            color: var(--marker-color);

            &.win {
                --marker-color: rgb(100, 206, 111);
            }
        }
    }
}

.pedestal {
    // display: flex;
    // height: 1.5rem !important;

    // color: black;
    // font-weight: bold;
    // border-radius: 8px;
    // width: fit-content;
    // padding-left: 0.5rem;
    // padding-right: 0.5rem;


    // &.first-place {
    //     background-color: $trophy-color-gold;
    // }

    // &.second-place {
    //     background-color: $trophy-color-silver;
    // }

    // &.third-place {
    //     background-color: $trophy-color-bronze;
    // }
}

@media (max-width: 575px) {
    .add-col {
        display: none;
    }
}
</style>
