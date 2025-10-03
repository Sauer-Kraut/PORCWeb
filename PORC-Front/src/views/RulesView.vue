<script setup lang="ts">
    import { computed, onMounted, ref, watch } from 'vue';


    const sections = ref<HTMLElement[]>([] as HTMLElement[]);

    const sectionMarkers = ref([] as { name: string; offsetPercent: number }[]);

    function getElementPosY(el: HTMLElement): number {
        let y = el.getBoundingClientRect().top + window.pageYOffset ;
        return y;
    }

    function calcOffsets(): { name: string; offsetPercent: number }[] {
        const offsets: { name: string; offsetPercent: number }[] = [];
        const viewportHeight = window.outerHeight;

        sections.value.forEach((section) => {
            const posY = getElementPosY(section);
            const offsetPercent = (posY / viewportHeight) * 40;
            offsets.push({ name: section.id, offsetPercent });
        });

        return offsets;
    }

    const scrollPercent = ref(0)

    function getScrollPercent(): number {
        const scrollTop = window.scrollY;
        const docHeight = document.body.scrollHeight - window.innerHeight;
        return (scrollTop / docHeight) * 100;
    }

    onMounted(() => {
        sections.value = Array.from(document.querySelectorAll<HTMLElement>(".rule-section"));
        sectionMarkers.value = calcOffsets();
        window.addEventListener('resize', () => {
            sectionMarkers.value = calcOffsets();
        });
        window.addEventListener('scroll', () => {
            scrollPercent.value = getScrollPercent();
            console.log(scrollPercent.value);
        });
        console.log("Section Markers:" + sectionMarkers.value);
    });


    function scrollToY(targetY: number, duration = 400) {
        const startY = window.scrollY;
        const diff = targetY - (window.innerHeight / 2.5) - startY;
        const startTime = performance.now();

        function step(time: number) {
            const elapsed = time - startTime;
            const progress = Math.min(elapsed / duration, 1);

            // easeInOutCubic easing
            const eased = progress < 0.5
            ? 4 * progress * progress * progress
            : 1 - Math.pow(-2 * progress + 2, 3) / 2;

            window.scrollTo(0, startY + diff * eased);

            if (progress < 1) {
            requestAnimationFrame(step);
            }
        }

        requestAnimationFrame(step);
    }


    function handleSectionMarkerClick(sectionName: string) {
        const el = document.getElementById(sectionName);
        if (el) {
            scrollToY(getElementPosY(el));
        }
    }

    function placeScrollMarker(): number {
        let distance = 1000000;
        let spot = 0;
        for (const section of sectionMarkers.value) {
            const diff = Math.abs(section.offsetPercent - scrollPercent.value);
            if (diff < distance) {
                distance = diff;
                spot = section.offsetPercent;
            }
        }

        return spot;
    }
</script>


<template>
    <div class="container-fluid d-flex flex-row mt-6 hidescroll">

        <div class="sidebar col-2 m-4 ms-5 d-none d-lg-flex">

            <div
                class="section-marker"
                v-for="section in sectionMarkers"
                :key="section.name"
                :style="{ top: section.offsetPercent + '%' }"
                @click="handleSectionMarkerClick(section.name)"
            >
                <label class="marker-label">{{ section.name }}</label>
            </div>
            <div
                class="section-marker-bg"
                v-for="section in sectionMarkers"
                :key="section.name"
                :style="{ top: section.offsetPercent + '%' }"
            ></div>

            <div class="sidebar-limit" :style="{ top: '0%' }"></div>
            <div class="sidebar-limit" :style="{ top: '100%' }"></div>
            

            <div class="scroll-marker" :style="{top: placeScrollMarker() + '%'}"></div>
        </div>

        <div class="col-2 d-none d-lg-block"></div>



        <div class="d-flex flex-column col-lg-10 col-xl-8">

            <div class="d-flex flex-column col-10 col-xl-7 ms-auto me-auto">
            

                <!-- Header -->
                <header class="mb-4 mt-0 mt-md-4 text-center">
                    <h1 class="decor-title primary">PORC Rules</h1>
                    <p class="content-subtitle">Official guidelines for fair play & competition</p>
                </header>

                <!-- Rules Content -->

                <!-- Section -->
                <div id="set-rules" class="rule-section">
                    <h1>Set Rules</h1>
                    <div class="seperator-h"></div>
                    <h2>1. Match Format</h2>
                    <ol class="rules">
                        <li>The players will play a total of 6 matches on the chosen map (3 as host and 3 as client).</li>
                        <li>Each match consists of a "best of 3 rounds". Players must keep track of match wins and total client round wins.</li>
                        <li>If the match wins are tied (3-3), the player with the most client round wins will win the set (the score to report will then be 3-4).</li>
                    </ol>

                    <h2>2. Map Selection</h2>
                    <ol class="rules">
                        <li>The players are allowed to decide which map they want to play on (custom maps are also allowed).</li>
                        <li>They may also choose to go to a random map together.</li>
                        <li>If both players cannot agree, Pit is chosen as the default map.</li>
                        <li>After each match, <span class="text-highlight">if any client rounds were won, the host may decide to switch to another map</span>.</li>
                    </ol>

                    <h2>3. Tiebreakers</h2>
                    <ol class="rules">
                        <li>If both players are deadlocked, extended tiebreaker rules may be applied with the agreement of both parties.</li>
                    </ol>

                    <h2>4. Fair Play</h2>
                    <ol class="rules">
                        <li>No cheating of any kind. Cheating results in <span class="text-highlight">immediate tournament disqualification</span>.</li>
                        <li>Mods that affect gameplay are not allowed and will be considered cheating.</li>
                    </ol>
                </div>

                <div id="tiebreaker-rules" class="rule-section">
                    <h1>Tiebreaker Rules</h1>
                    <div class="seperator-h"></div>
                    <h2>1. Procedure</h2>
                    <ol class="rules">
                        <li>First to win a client round sets the bar.</li>
                        <li>The opponent has the right of reply:
                            <ul class="sub-rules">
                                <li>If they equal the client round, the tiebreaker resets.</li>
                                <li>If they fail, the first client round winner wins.</li>
                            </ul>
                        </li>
                        <li>If the tiebreaker winner wins the match, they win the set.</li>
                    </ol>
                </div>

                <div id="extended-tiebreaker" class="rule-section">
                    <h1>Extended Tiebreaker Rules</h1>
                    <div class="seperator-h"></div>
                    <h2>1. Adjustments</h2>
                    <ol class="rules">
                        <li>Host goes to one shift stone.</li>
                        <li>Host goes to zero shift stones.</li>
                        <li>If on Pit, players switch to Ring.</li>
                        <li>The host starts with less HP.</li>
                    </ol>
                </div>

                <div id="scoring" class="rule-section">
                    <h1>Scoring</h1>
                    <div class="seperator-h"></div>
                    <h2>1. Match Definition</h2>
                    <ol class="rules">
                        <li>A match is defined as a best of 3 rounds with one player as host and the other as client.</li>
                    </ol>

                    <h2>2. Scoring Outcomes</h2>
                    <ol class="rules">
                        <li>If the number of matches won is unequal, the final score is simply the matches won by each player.</li>
                        <li>If both players win an equal number of matches, the final score is reported as matches won by both players +1 for the winner.</li>
                        <li>If a player forfeits or fails to appear at an agreed time, it counts as an automatic 3-0 win for the opposing player.</li>
                    </ol>
                </div>

                <div id="placement-ranking" class="rule-section">
                    <h1>Placement, Ranking and Promotion</h1>
                    <div class="seperator-h"></div>
                    <h2>1. Placement</h2>
                    <ol class="rules">
                        <li>Your initial placement is determined by <span class="text-highlight">your BP amount and assessed skill</span>.</li>
                        <li>Before each season starts, you may contest your placement by contacting a moderator with your pitch.</li>
                    </ol>

                    <h2>2. Ranking</h2>
                    <ol class="rules">
                        <li>Ranking at the end of each season is decided by total set wins.</li>
                        <li>Ties are broken by advantage score. (Advantage = total round wins - total round losses)</li>
                        <li>If still tied, placement will be shared.</li>
                    </ol>

                    <h2>3. Promotion and Demotion</h2>
                    <ol class="rules">
                        <li>Player performance determines promotion and demotion.</li>
                        <li>As a rule of thumb:
                            <ul class="sub-rules">
                                <li>Top third of a division promotes.</li>
                                <li>Bottom third of a division demotes.</li>
                            </ul>
                        </li>
                        <li>Exceptional performance may result in promotion over two divisions.</li>
                    </ol>
                </div>

                <div id="additional-notes" class="rule-section">
                    <h1>Additional Notes</h1>
                    <div class="seperator-h"></div>
                    <h2>1. Enforcement</h2>
                    <ol class="rules">
                        <li>Cheaters are banned from competing, and all their matches are marked as 0-3 losses.</li>
                    </ol>

                    <h2>2. Match Proof</h2>
                    <ol class="rules">
                        <li>Competitors are encouraged to record their fights for both sharing and proof in case verification is needed.</li>
                    </ol>

                    <h2>3. Game Stability</h2>
                    <ol class="rules">
                        <li>This is an early access game, so unexpected issues may occur.</li>
                        <li>The standard protocol is to reset the match in case of a major bug, unless both players agree on a different solution.</li>
                    </ol>

                    <h2>4. Scheduling</h2>
                    <ol class="rules">
                        <li>You can use <span class="text-highlight">the Match Planner accessible on the webiste</span> in order to see your opponents availabilities and suggest times.</li>
                        <li>PORC Bot will automatically contact you over discord if someone requested a match with you. You can accept match request both via discord and the Match Planner</li>
                    </ol>

                    <h2>5. Community Conduct</h2>
                    <ol class="rules">
                        <li>Treat each other with respect and enjoy yourself, as is customary in the RUMBLE community.</li>
                    </ol>
                </div>

                <div class="mt-5 mb-5 pb-3 pt-3 ms-auto me-auto detail-title">Just be nice, you'll figure it out ^^</div>

            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';
h2, h4 {
  font-weight: 600;
}
ul, ol {
  margin-top: 1rem;
}
.card {
  border: 1px solid rgba(255,255,255,0.05);
}





.sidebar {
    position: fixed;
    width: 1px;
    height: 90vh;


    border-left: 3px dashed rgb(195, 195, 195);


    $marker-size: 1rem;

    .section-marker {
        position: absolute;
        transform: translate(calc(-50% - 1.5px), -50%);

        height: $marker-size;
        width: $marker-size;

        border-radius: 50%;
        border: 3px solid rgb(195, 195, 195);

        display: flex;
        align-items: center;
        gap: 0.5rem;

        z-index: 4;

        cursor: pointer;

        .marker-label {
            cursor: pointer;

            padding: 0.2rem 0.5rem;

            margin-bottom: 0.1rem;
            margin-left: 1.5rem;

            border-radius: 8px;
            font-size: 0.85rem;
            white-space: nowrap;
            text-transform: capitalize;
            border: 1px solid rgba(255,255,255,0.05);

            font-weight: 600;
        }
    }

    .section-marker-bg {
        position: absolute;
        transform: translate(calc(-50% - 1.5px), -50%);

        height: $marker-size;
        width: $marker-size;

        border-radius: 50%;
        background-color: $background-color;

        display: flex;
        align-items: center;
        gap: 0.5rem;

        z-index: 1;

        cursor: pointer;
    }

    .scroll-marker {
        transition: all 0.3s ease-in-out;

        position: absolute;
        transform: translate(calc(-50% - 1.5px), -50%);

        height: calc($marker-size * 3);
        width: calc($marker-size * 0.8);

        border-radius: 50%;
        border-radius: calc($marker-size / 2);
        border: 3px solid var(--primary);
        background-color: var(--primary);

        z-index: 2;
    }

    .sidebar-limit {
        position: absolute;
        transform: translate(calc(-50% - 1.5px), -50%);
        
        width: 1rem;
        height: 3px;
        background-color: rgb(195, 195, 195);
    }
}

.mt-6 {
    margin-top: 5rem !important;

    @media (max-width: 768px) {
        margin-top: 2rem !important;
    }
}

.hidescroll {
    scrollbar-width: none !important; /* For Firefox */
    -ms-overflow-style: none !important;  /* For Internet Explorer and Edge */
    &::-webkit-scrollbar {
        display: none !important; /* For Chrome, Safari, and Opera */
    }
}



.rule-section {
    margin-bottom: 7rem !important;


    h1 {
        color: var(--primary);
        font-size: 2.5rem;
        font-weight: 700;


        @include media-breakpoint-down(md) {
            font-size: 2rem;
        }
    }

    h2 {
        font-size: 1.75rem;
        font-weight: 600;
        margin-top: 1.5rem !important;
        margin-bottom: 1rem !important;

        text-decoration: underline;
        text-decoration-thickness: 0.5px;
        text-decoration-color: color-mix(in srgb, rgb(118, 116, 116), var(--primary) 20%);;
        text-underline-offset: 0.3rem;

        @include media-breakpoint-down(md) {
            font-size: 1.5rem;
        }
    }

    .rules {
        font-size: 1rem !important;
        font-weight: 400 !important;
        line-height: 1.5 !important;

        * {
            margin-top: 0.5rem !important;
            margin-bottom: 0.5rem !important;
        }
    }

    .text-highlight {
        text-decoration: underline;
        text-decoration-color: var(--primary);
        text-underline-offset: 0.3rem;
        text-decoration-thickness: 0.15rem;
    }
}

.seperator-h {
    background-color: color-mix(in srgb, rgb(118, 116, 116), var(--primary) 30%);
}
</style>
