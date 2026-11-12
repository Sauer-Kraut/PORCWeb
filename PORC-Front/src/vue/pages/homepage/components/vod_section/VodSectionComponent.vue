<script lang="ts" setup>
    import type { VideoReference } from '@/logic/models/discord/VideoReference';
    // import type { Division } from '@/logic/models/matchplan/division/Division';
    // import VideoCarousel from '../video_carousel/VideoCarousel.vue';
    import YoutubeVideo from '@/vue/global/assets/YoutubeVideo.vue';
    // import AccountCard from '@/vue/global/account/profile/md/AccountCard.vue';
    import { computed, onMounted, ref, watch } from 'vue';
    import { waitForAppReady } from '@/appReady';
    // import { matchplanStore } from '@/logic/storage/st_matchplan';
    // import { divisionNames } from '@/logic/storage/defaults';
    // import { stripAfterFirstSpace } from '@/logic/util/formating/StripAfterSpace';
    import { getDivisionColor } from '@/logic/util/resources/get/GetDivisionColor';
    // import { getDivisionImage } from '@/logic/util/resources/get/ImageHelper';
    import { accountsStore } from '@/logic/storage/st_accounts.ts';
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo.ts';
    import DiscordAvatarComponent from '@/vue/global/account/avatar/DiscordAvatarComponent.vue';
    import AccountCardSMVertical from '@/vue/global/account/profile/sm/AccountCardSMVertical.vue';
    import { InfoPopover } from '@/vue/global/system/Popover/PopoverDesign/InfoPopover.ts';
    import { clickEvaluator } from '@/vue/global/system/Popover/PopoverDisplayLogic/ClickPopover.ts';
    import PopoverComponent from '@/vue/global/system/Popover/PopoverComponent.vue';
    import { hoverEvaluator } from '@/vue/global/system/Popover/PopoverDisplayLogic/HoverPopover.ts';
    import { clickFleetingEvaluator } from '@/vue/global/system/Popover/PopoverDisplayLogic/ClickFleetingPopover';
    import { getStringWidth } from '@/logic/util/formating/CalcStringWidth';

    const props = defineProps<{
        videos: VideoReference[];
    }>();

    const PopoverContainer = ref<HTMLElement | null>(null);
    const playerAccountReferences = ref<string[]>(["306467062530965514", "1237958983348654151", "244833436450291722", "309001128929001472"]);
    const chipHoverTick = ref(0);

    function onChipHover() {
        chipHoverTick.value++;
    }

    const chipElements = computed(() => {
        chipHoverTick.value; // force recompute after hover
        return playerAccountReferences.value.map((id) =>
            document.getElementById(`chipElement${id}`)
        );
    });
    const selectedChipId = ref<string | null>(null);
    const activeChipData = computed(() =>{
        return playerAccountReferences.value
            .map((id): [HTMLElement | null, boolean, string] => [
                document.getElementById(`chipElement${id}`),
                selectedChipId.value === id,
                id
            ])
            .filter(([, hover]) => hover)
        
    });
    
    const activeChipElement = computed(() => activeChipData.value[0]?.[0] ?? null);
    const activeChipId = computed(() => activeChipData.value[0]?.[2] ?? null);

    const activeChipAccount = ref<PubAccountInfo | null>(null);
    watch(activeChipId, async (newId) => {
        if (newId) {
            try {
                const account = await accountsStore().get_accounts_min([newId]);
                activeChipAccount.value = account[0] ?? null;
            } catch (error) {
                console.error('Error fetching account for active chip:', error);
                activeChipAccount.value = null;
            }
        } else {
            activeChipAccount.value = null;
        }
    }, { immediate: true });

    // Legacy filtered/carousel source, currently unused in active template.
    // const parsedVideos = ref<VideoReference[]>(props.videos);

    // Sentinel filter value for uploaders that are not in any division of the matchplan
    const OTHER = '__other__';

    // Legacy filter/sort/date formatting logic (toolbar + carousel) is currently not in use.
    // const query = ref('');
    // const selectedCreator = ref<string | null>(null);
    // const selectedDivision = ref<string | null>(null);
    //
    // const SORTS = ['Newest', 'Oldest', 'Title'] as const;
    // const sortIndex = ref(0);
    // const sortLabel = computed(() => SORTS[sortIndex.value % SORTS.length]);
    //
    // function cycleSort() {
    //     sortIndex.value = (sortIndex.value + 1) % SORTS.length;
    // }
    //
    // function timeOf(video: VideoReference): number {
    //     const t = new Date(video.date).getTime();
    //     return isNaN(t) ? 0 : t;
    // }
    //
    // function formatDate(date?: string | number | Date): string {
    //     if (!date) return '';
    //     const d = new Date(date);
    //     return isNaN(d.getTime()) ? '' : d.toLocaleDateString(undefined, { year: 'numeric', month: 'long', day: 'numeric' });
    // }

    // Divisions

    // Account id -> division, built from the players of the matchplan
    // const divisions = ref<Division[]>([]);
    // const divisionByPlayer = computed(() => {
    //     const map = new Map<string, Division>();
    //     for (const division of divisions.value) {
    //         for (const player of division.players ?? []) {
    //             if (!map.has(player.id)) map.set(player.id, division);
    //         }
    //     }
    //     return map;
    // });

    onMounted(async () => {
        await waitForAppReady();
        await accountsStore().get_accounts_min(playerAccountReferences.value);
        // const plan = await matchplanStore().get_matchplan();
        // divisions.value = [...(plan?.divisions ?? [])].sort((a, b) => a.order - b.order);
    });

    // Division key of a video: the name of the division its uploader plays in, or OTHER
    function divisionKeyOf(video: VideoReference): string {
        // Legacy division mapping data is currently disabled, so this falls back to OTHER.
        // const id = video.creator?.id;
        // return (id && divisionByPlayer.value.get(id)?.name) || OTHER;
        void video;
        return OTHER;
    }

    function colorOf(name: string): string | undefined {
        return name === OTHER ? undefined : getDivisionColor(name.toLowerCase()).trim();
    }

    // Legacy division/filter/carousel logic is currently not in use.
    // function baseName(name: string): string {
    //     return stripAfterFirstSpace(name).trim().toLowerCase();
    // }
    //
    // function hasLogo(name: string): boolean {
    //     return (divisionNames as readonly string[]).includes(baseName(name));
    // }
    //
    // function matchesBase(video: VideoReference): boolean {
    //     if (selectedCreator.value && video.creator?.id !== selectedCreator.value) return false;
    //     const tokens = query.value.toLowerCase().split(/\s+/).filter(Boolean);
    //     const haystack = `${video.title} ${video.creator?.username ?? ''}`.toLowerCase();
    //     return tokens.every(t => haystack.includes(t));
    // }
    //
    // const divisionCounts = computed(() => {
    //     const counts = new Map<string, number>();
    //     for (const v of props.videos) {
    //         if (!matchesBase(v)) continue;
    //         const key = divisionKeyOf(v);
    //         counts.set(key, (counts.get(key) ?? 0) + 1);
    //     }
    //     return counts;
    // });
    //
    // const baseCount = computed(() => props.videos.filter(matchesBase).length);
    //
    // const hasOther = computed(() => props.videos.some(v => divisionKeyOf(v) === OTHER));
    //
    // const divisionChips = computed(() => {
    //     let chips = divisions.value.map(d => ({
    //         key: d.name,
    //         label: d.name,
    //         count: divisionCounts.value.get(d.name) ?? 0,
    //     }));
    //     if (hasOther.value || selectedDivision.value === OTHER) {
    //         chips.push({ key: OTHER, label: 'Other', count: divisionCounts.value.get(OTHER) ?? 0 });
    //     }
    //     return chips;
    // });
    //
    // const filtered = computed(() => {
    //     const out = parsedVideos.value.filter(v =>
    //         matchesBase(v) && (selectedDivision.value === null || divisionKeyOf(v) === selectedDivision.value)
    //     );
    //
    //     switch (sortLabel.value) {
    //         case 'Oldest':
    //             out.sort((a, b) => timeOf(a) - timeOf(b));
    //             break;
    //         case 'Title':
    //             out.sort((a, b) => a.title.localeCompare(b.title));
    //             break;
    //         default:
    //             out.sort((a, b) => timeOf(b) - timeOf(a));
    //     }
    //     return out;
    // });
    //
    // const selectedCreatorName = computed(() =>
    //     props.videos.find(v => v.creator?.id === selectedCreator.value)?.creator?.username ?? ''
    // );
    //
    // const isFiltering = computed(() =>
    //     query.value.trim() !== '' || selectedCreator.value !== null || selectedDivision.value !== null
    // );
    //
    // const carouselTitle = computed(() =>
    //     isFiltering.value
    //         ? `${filtered.value.length} of ${parsedVideos.value.length} VODs`
    //         : `${filtered.value.length} VODs`
    // );
    //
    // function resetFilters() {
    //     query.value = '';
    //     selectedCreator.value = null;
    //     selectedDivision.value = null;
    // }
    //
    // function toggleCreator(id: string) {
    //     selectedCreator.value = selectedCreator.value === id ? null : id;
    // }
    //
    // function toggleDivision(key: string) {
    //     selectedDivision.value = selectedDivision.value === key ? null : key;
    // }

    // Viewer

    const selectedId = ref<string | null>(null);
    const viewerRef = ref<HTMLElement | null>(null);

    // Keep selected video valid for the current active (non-carousel) view.
    watch(
        () => props.videos,
        () => {
            if (props.videos.length === 0) return;
            if (!props.videos.some(v => v.youtube_id === selectedId.value)) {
                selectedId.value = props.videos[0].youtube_id;
            }
        },
        { immediate: true }
    );

    // Legacy uploader enrichment flow used by filtered carousel data.
    // const rawVideos = computed(() => props.videos);
    // watch(async () => rawVideos.value, async (newVideos) => {
    //     const uploaderIds = new Set((await newVideos).map(v => v.creator?.id).filter(Boolean));
    //     let uploaderAccounts: PubAccountInfo[] = [];
    //     try {
    //         uploaderAccounts = await accountsStore().get_accounts_min([...uploaderIds] as string[]);
    //     } catch (error) {
    //         const accountQueryHandles = [];
    //         for (const id of uploaderIds) {
    //             accountQueryHandles.push(accountsStore().get_competitor(id));
    //         }
    //         await Promise.all(accountQueryHandles).then((results) => {
    //             uploaderAccounts = results.filter((res) => res !== null);
    //         }).catch((error) => {
    //             console.error('Error fetching uploader accounts:', error);
    //         });
    //     }
    //
    //     const filteredVideos = (await newVideos)
    //         .map(v => {
    //             const account = uploaderAccounts.find(a => a.id === v.creator?.id);
    //             const updatedVideo = v;
    //             v.creator = account ?? v.creator;
    //             return [updatedVideo, account != undefined];
    //         })
    //         .filter(([, keep]) => keep)
    //         .map(([video]) => video as VideoReference);
    //
    //     parsedVideos.value = filteredVideos;
    // }, { immediate: true });

    const current = computed<VideoReference | null>(() =>
        props.videos.find(v => v.youtube_id === selectedId.value) ?? null
    );

    const highlightVideo = ref<VideoReference>({
        title: 'Be The Best.',
        youtube_id: 'h0iLa9kyyt0',
        creator: accountsStore().get_placeholder_account(),
        date: new Date('2026-10-28T00:00:00Z'),
    });

    const currentDivision = computed(() => (current.value ? divisionKeyOf(current.value) : OTHER));

    // The viewer takes on the colour of the uploader's division, like the event cards do
    const viewerStyle = computed(() => {
        const color = colorOf(currentDivision.value);
        return color ? { '--primary': color } : undefined;
    });

    // Legacy carousel callback (unused while results carousel is disabled).
    // function selectVideo(video: VideoReference) {
    //     selectedId.value = video.youtube_id;
    //     viewerRef.value?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
    // }
</script>

<template>
    <div class="vod-section d-flex flex-column align-items-center justify-content-center">

        <!-- Heading -->
        <div class="vod-heading d-flex flex-column justify-content-center align-items-center col-12 mb-5 pb-5">
            <h2 class="decor-title text-center justify-content-center w-auto">Content <span class="text-highlight">Highlights</span></h2>
            <h3 class="content-subtitle justify-content-center w-auto mt-2 mb-0">Watch the best PORC has to offer, whenever you want</h3>
        </div>

        <!-- Browser -->
        <div class="d-flex vod-browser col-9 h-100 justify-content-center" v-if="videos.length > 0" ref="PopoverContainer">

            <!-- Viewer: selected video + uploader -->
            <div class="d-flex flex-row vod-viewer" ref="viewerRef" :style="viewerStyle" v-if="current">
                <div class="viewer-main">
                    <div class="viewer-player">
                        <YoutubeVideo :key="current.youtube_id" :videoId="highlightVideo.youtube_id" :width="860" maxres></YoutubeVideo>
                    </div>

                    <!-- <div class="viewer-info">
                        <div class="d-flex flex-row">
                            <h3 class="viewer-title">{{ current.title }}</h3>

                            <a
                                class="ms-auto"
                                :href="`https://www.youtube.com/watch?v=${current.youtube_id}`"
                                target="_blank"
                                rel="noopener noreferrer"
                            >Open on YouTube</a>
                        </div>

                        <div class="viewer-meta">
                            <span class="meta-item" v-if="current.creator">
                                Uploaded by <strong class="viewer-uploader">{{ current.creator.username }}</strong>
                            </span>
                            <span class="meta-item" v-if="current.date">{{ formatDate(current.date) }}</span>
                            <span class="division-badge" v-if="currentDivision !== OTHER">
                                <img v-if="hasLogo(currentDivision)" :src="getDivisionImage(baseName(currentDivision))" :alt="`${currentDivision} logo`" />
                                {{ currentDivision }}
                            </span>
                        </div>
                    </div> -->
                </div>

                <div class="seperator-v h-100"></div>

                <div class="d-felx flex-column viewer-side info" v-if="current.creator">
                    <!-- <AccountCard :key="current.creator.id" :account="current.creator" :listHeight="0"></AccountCard> -->
                    <div class="label-row">
                        <span class="eyebrow quiet">About this video</span>
                        <a class="ext" href="https://www.youtube.com/watch?v=h0iLa9kyyt0">Open on YouTube ↗</a>
                    </div>
    
                    <div class="info-body">
                        <div>
                            <h4 class="v-title">Explaining every Shiftstone in VR Earthbending</h4>
                            <div class="v-meta">PORC<span>/</span>1 Sep 2026<span>/</span>15:07</div>
                        </div>
    
                        <div class="v-desc">
                            In RUMBLE VR, Earthbending comes down to precise spacing, ruthless tempo, and mastering your mechanics. But out of every Shiftstone option available, why does competitive play ignore half of the roster?
                            <br>
                            <br>
                            Before the next major update flips the game on its head, we sat down with some of RUMBLE’s highest-ranked players to investigate why the current Shiftstone meta is what it is—and what you should actually be running if you want to win.
                        </div>
    
                        <blockquote class="note">
                            <span class="mini">About this section</span>
                            One or two sentences from you on
                        </blockquote>
    
                        <!-- <div class="chapters">
                            <span class="mini">Chapters</span>
                            <ol id="chapters">
                                <li><button type="button" class="on"><span class="t">0:00</span><span class="c">Intro</span></button></li>
                                <li><button type="button"><span class="t">0:18</span><span class="c">First glimpse</span></button></li>
                                <li><button type="button"><span class="t">0:42</span><span class="c">Movement tech</span></button></li>
                                <li><button type="button"><span class="t">1:15</span><span class="c">The duel</span></button></li>
                                <li><button type="button"><span class="t">1:48</span><span class="c">Final shot</span></button></li>
                            </ol>
                        </div> -->
    
                        <div class="featured layer-featured" id="featured-chips">
                            <span class="mini">In this video</span>
                            <div class="d-flex flex-row chips">
                                <div v-for="[index, playerid] in playerAccountReferences.entries()" @mouseenter="onChipHover()">    
                                    <div
                                        class="chip"
                                        :id="`chipElement${playerid}`"
                                        :style="{ '--chip-width-extended': `${getStringWidth(accountsStore().get_competitor_entry(playerid)[0]?.username ?? '')*1.5 +48 + 8}px` }"
                                        @click="selectedChipId = playerid"
                                    ><DiscordAvatarComponent class="avatar" :account="accountsStore().get_competitor_entry(playerid)[0] ?? accountsStore().get_placeholder_account()"/>
                                        <span class="chip-tag">{{ accountsStore().get_competitor_entry(playerid)[0]?.username ?? '' }}</span>
                                    </div>
                                    <PopoverComponent
                                        v-if="chipElements[index]"
                                        :anchor="chipElements[index]"
                                        :update="InfoPopover"
                                        :display-logic="clickFleetingEvaluator"
                                        :parent="'#featured-chips'"
                                        :fade-in="false"
                                        :popover-boundary="PopoverContainer ?? undefined"
                                        :hide-delay="200"
                                        :exclusive-group="'VideoHighlightChips'"
                                        class="p-0"
                                    >
                                        <AccountCardSMVertical :account="accountsStore().get_competitor_entry(playerid)[0] ?? accountsStore().get_placeholder_account()"></AccountCardSMVertical>
                                    </PopoverComponent>
                                </div>
                            </div>
                        </div>

                    </div>

                </div>
            </div>

            <!-- Toolbar: search + sort -->
            <!-- <div class="vod-toolbar">
                <label class="vod-search">
                    <svg class="vod-search-icon" viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                        <circle cx="11" cy="11" r="7"></circle>
                        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
                    </svg>
                    <input
                        v-model="query"
                        type="text"
                        class="vod-search-input"
                        placeholder="Search by title or creator"
                        aria-label="Search VODs"
                        @keydown.esc="query = ''"
                    />
                    <button v-if="query" type="button" class="vod-search-clear" aria-label="Clear search" @click="query = ''">&times;</button>
                </label>

                <button
                    v-if="selectedCreator"
                    type="button"
                    class="vod-tag"
                    title="Show VODs from everyone again"
                    @click="selectedCreator = null"
                >
                    Uploader: <strong>{{ selectedCreatorName }}</strong> <span aria-hidden="true">&times;</span>
                </button>

                <h5 class="sort-option primary mb-0 ms-auto" role="button" @click="cycleSort">Sorted by {{ sortLabel }} &#9662;</h5>
            </div> -->

            <!-- Division filter -->
            <!-- <div class="vod-divisions" role="group" aria-label="Filter by division">
                <button
                    type="button"
                    class="division-chip"
                    :class="{ active: selectedDivision === null }"
                    @click="selectedDivision = null"
                >
                    All <span class="chip-count">{{ baseCount }}</span>
                </button>
                <button
                    v-for="chip in divisionChips"
                    :key="chip.key"
                    type="button"
                    class="division-chip"
                    :class="{ active: selectedDivision === chip.key, empty: chip.count === 0 }"
                    :style="{ '--chip-color': colorOf(chip.key) }"
                    :disabled="chip.count === 0 && selectedDivision !== chip.key"
                    :aria-pressed="selectedDivision === chip.key"
                    @click="toggleDivision(chip.key)"
                >
                    <img v-if="chip.key !== OTHER && hasLogo(chip.key)" class="chip-logo" :src="getDivisionImage(baseName(chip.key))" alt="" />
                    {{ chip.label }} <span class="chip-count">{{ chip.count }}</span>
                </button>
            </div> -->

            <!-- Results -->
            <!-- <VideoCarousel
                v-if="filtered.length > 0"
                :videos="filtered"
                :selectedId="selectedId"
                :sectionTitle="carouselTitle"
                :width="270"
                :gap="16"
                @select="selectVideo"
                @select-creator="toggleCreator"
            />

            <div class="vod-empty" v-else>
                <span class="vod-empty-title">No VODs match your filters</span>
                <span class="vod-empty-sub">Try a different title, creator or division.</span>
                <button type="button" class="btn btn-small btn-outline-primary mt-2" @click="resetFilters">Clear filters</button>
            </div> -->
        </div>

        <!-- Nothing uploaded -->
        <div class="vod-empty" v-else>
            <span class="vod-empty-title">No VODs yet</span>
            <span class="vod-empty-sub">Recorded matches will show up here as soon as they are uploaded.</span>
        </div>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .vod-section {
        width: 100%;
        .vod-viewer {
            align-items: start;
            gap: 2rem;
            margin-bottom: 1.5rem;

            .viewer-player {
                position: relative;
                width: 65rem;
                aspect-ratio: 16 / 9;
                background: #000;
                border: 1px solid $border-color;
                border-radius: 8px;
                overflow: hidden;

                :deep(.youtube-preview) {
                    width: 100% !important;
                    height: 100% !important;
                }
            }

            .viewer-side {
                display: flex;
                justify-content: center;
                height: 100%;
            }

            .seperator-v {
                mask-image: linear-gradient(to bottom, transparent 0%, rgba(255, 255, 255, 0.739) 50%, transparent 100%);
            }

            .info {
                position: relative;
                min-height: 0;
                max-width: 22rem;
            }

            .info-inner {
                position: absolute;
                inset: 0;
                display: flex;
                flex-direction: column;
            }

            .label-row {
                display: flex;
                align-items: center;
                justify-content: space-between;
                height: 1.6rem;
                margin-bottom: 0.9rem;
            }

            .eyebrow {
                display: inline-flex;
                align-items: center;
                gap: 0.55rem;
                font-size: 0.85rem;
                font-weight: 700;
                letter-spacing: 0.06em;
                text-transform: uppercase;
                color: var(--primary);

                &::before {
                    content: '';
                    width: 0.55rem;
                    height: 0.55rem;
                    border-radius: 50%;
                    background: var(--primary);
                }

                &.quiet {
                    color: var(--faint);
                    letter-spacing: 0.16em;

                    &::before {
                        display: none;
                    }
                }
            }

            .ext {
                font-size: 0.85rem;
                font-weight: 600;
                color: color-mix(in srgb, var(--primary) 80%, #fff 0%);
                text-decoration: none;
                opacity: 0.8;
                transition: opacity 0.2s ease;

                &:hover {
                    opacity: 1;
                }
            }

            .info-body {
                flex: 1;
                min-height: 0;
                display: flex;
                flex-direction: column;
                gap: 1.35rem;
                padding-top: 0.1rem;
            }

            .v-title {
                margin: 0;
                font-size: 1.45rem;
                font-weight: 600;
                line-height: 1.25;
                color: #e7e7ea;
            }

            .v-meta {
                margin-top: 0.45rem;
                // font-family: var(--mono);
                font-size: 0.72rem;
                letter-spacing: 0.1em;
                text-transform: uppercase;
                color: var(--faint);

                span {
                    margin: 0 0.45rem;
                    opacity: 0.6;
                }
            }

            .v-desc {
                margin: 0;
                font-size: 0.92rem;
                line-height: 1.65;
                color: $weak-text;
                overflow: hidden;
            }

            .mini {
                display: block;
                margin-bottom: 0.5rem;
                // font-family: var(--mono);
                font-size: 0.64rem;
                letter-spacing: 0.2em;
                text-transform: uppercase;
                color: var(--faint);
            }

            .note {
                margin: 0;
                padding: 0.1rem 0 0.1rem 1rem;
                border-left: 2px solid color-mix(in srgb, var(--primary) 45%, transparent);
                font-size: 0.9rem;
                line-height: 1.6;
                color: $weak-text;
            }

            .chapters {
                flex: 1;
                min-height: 0;
                display: flex;
                flex-direction: column;

                ol {
                    list-style: none;
                    margin: 0;
                    padding: 0 0.4rem 0 0;
                    flex: 1;
                    min-height: 0;
                    overflow-y: auto;
                    scrollbar-width: thin;
                    scrollbar-color: rgba(255, 255, 255, 0.12) transparent;
                    -webkit-mask-image: linear-gradient(to bottom, #000 calc(100% - 1.5rem), transparent);
                    mask-image: linear-gradient(to bottom, #000 calc(100% - 1.5rem), transparent);
                }

                li + li {
                    border-top: 1px solid rgba(255, 255, 255, 0.05);
                }

                button {
                    appearance: none;
                    width: 100%;
                    display: grid;
                    grid-template-columns: 3.2rem 1fr;
                    align-items: baseline;
                    gap: 0.5rem;
                    padding: 0.55rem 0.6rem;
                    font: inherit;
                    text-align: left;
                    color: inherit;
                    background: transparent;
                    border: 0;
                    border-radius: 6px;
                    cursor: pointer;
                    transition: background-color 0.2s ease;

                    &:hover {
                        background: rgba(255, 255, 255, 0.03);

                        .c {
                            color: #fff;
                        }
                    }

                    &.on {
                        .t {
                            color: var(--primary);
                        }

                        .c {
                            color: #e7e7ea;
                        }
                    }
                }

                .t {
                    // font-family: var(--mono);
                    font-size: 0.75rem;
                    color: var(--faint);
                    transition: color 0.2s ease;
                }

                .c {
                    font-size: 0.88rem;
                    color: #9aa1a9;
                    transition: color 0.2s ease;
                }
            }

            .chips {
                display: flex;
                flex-wrap: wrap;
                gap: 0.45rem;
            }

            .chip {
                display: inline-flex;
                align-items: center;
                overflow: hidden;
                width: 2.5rem;
                // gap: 0.5rem;
                // padding: 0.25rem 0.7rem 0.25rem 0.3rem;
                border: 1px solid $border-color;
                border-radius: 999px;
                background: $darker-bg;
                
                transition: all 0.3s ease;

                font-size: 0.9rem;
                font-weight: 600;
                color: black;

                .chip-tag {
                    &:hover {
                        text-decoration: underline;
                    }
                }



                &:hover {
                    // color: #fff;
                    // border-color: color-mix(in srgb, var(--primary) 40%, $border-color);
                    background-color: var(--primary);
                    z-index: 100 !important;

                    width: var(--chip-width-extended);
                }

                &:not(&:last-child) {
                    margin-right: -1.0rem !important;
                }

                @for $i from 1 through 50 {
                    &:nth-child(#{$i}) {
                        z-index: #{100 - $i};
                    }
                }

                .avatar {
                    width: 2.5rem;
                    height: 2.5rem;
                    border-radius: 999px;

                    margin-right: 0.5rem;
                }
            }
        }

        .vod-empty {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 0.5rem;
            padding: 2rem 1rem;
            text-align: center;

            .vod-empty-title {
                color: whitesmoke;
                font-size: 1.3rem;
                font-weight: 600;
            }

            .vod-empty-sub {
                color: $muted;
            }
        }

        @media (max-width: 62rem) {
            .vod-viewer {
                .info-inner {
                    position: static;
                }

                .chapters {
                    ol {
                        max-height: 16rem;
                    }
                }
            }
        }
    }

</style>
