<template>
    <div class="page-header p-3">
        <h1>PORC Rules</h1>
    </div>
    <div class="container-fill justify-content-center">
        <div class="part part-text">
            <h2 class="title mt-0">Set rules</h2>
            <ul>
                <li class="rule">

                    The players will play a total of 6 matches on the chosen map (3 as host and 3 as client).
                    Each match consists of a "best of 3 rounds", the players keep track of match wins and total client round wins.
                    If the match wins are tied (the score is 3-3), the player with the most client round wins will win the set at the end of the 6 matches (the score to report will then be 3-4).
                </li>
                <br />
                <li>
                    The players are allowed to decide which map they want to play on (custom maps are also allowed). Alternatively, they may also choose to go to a random map together. If both players
                    do not agree on their preferred map, Pit is chosen as default.
                </li>
                <br />
                <li>After each match, if any client rounds were won, <span class="highlight">the host may decide to switch to the other map.</span></li>
                <br />
                <li>If both players are deadlocked, extended tiebreaker rules may be applied with the agreement of both parties.</li>
                <br />
                <li>No cheating of any kind. If a player cheats, they are banned immediately from the tournament.</li>
                <br />
                <li>Mods that affect any gameplay are not allowed and will be considered as cheating.</li>
            </ul>
            <div class="p-3"></div>
            <div class="under-rules">
                <h4 class="lesser-title">Tiebreaker rules</h4>
                <ul>
                    <li>First to win a client round will set the bar.</li>
                    <br />
                    <li>The opponent will have the right of reply. If they equal the client round, the tiebreaker resets.</li>
                    <br />
                    <li>If they fail to equal, the first client round winner wins. If they win the match, they will win.</li>
                </ul>
            </div>
            <div class="p-3"></div>
            <div class="under-rules">
                <h4 class="lesser-title">Extended tiebreaker rules</h4>
                <ol>
                    <li>Host goes to one shift stone.</li>
                    <br />
                    <li>Host goes to zero shift stones.</li>
                    <br />
                    <li>The players switch to ring if on Pit.</li>
                    <br />
                    <li>The host starts with less HP.</li>
                </ol>
            </div>
        </div>
        <div class="part part-text">
            <h2 class="title mt-0">Scoring</h2>
            <ul>
                <li>A match is defined as a best of 3 rounds with one player as host and the other as client. At the end of a game, scoring is decided as follows:</li>
                <br />
                <li>If the number of matches won is unequal between the players, the final score is simply the matches won by each player during the game.</li>
                <br />
                <li>In case both players won an equal number of matches, the final score is reported as the matches won by both players +1 for the winning party.</li>
                <br />
                <li>If a player forfeits / fails to appear at an agreed time it counts as an automatic 1-0 win for the opposing player.</li>
            </ul>
        </div>
        <div class="part part-text">
            <h2 class="title mt-0">Placement, Ranking and Promotion</h2>
            <ul>
                <li>Your initial placement is determined by your amount of BP as well as your assessed skill.</li>
                <br />
                <li>
                    Before the start of each season, there will be a timeframe to contest your placement after you have been placed in a division. To contest your placement, simply contact one of the
                    moderators and put forward your pitch for your placement.
                </li>
                <br />
                <li>The ranking at the end of each season will be decided by total set wins. Ties will be broken by the advantage score. If somehow after this there is still a tie the placement will be shared
                </li>
                <br/>
                <li>
                    After each season, player performance during the season decides promotion and demotion. As a rule of thumb, one can expect the top third of a division to promote and the bottom
                    third to demote.
                </li>
                <br />
                <li>Promotions over two divisions may be rewarded upon special performance or improvement.</li>
            </ul>
        </div>

        <div class="part part-text">
            <h2 class="title mt-0">Additional Notes</h2>
            <ul>
                <li>Cheaters will be banned from competing, and all their matches will be marked as 0-1 losses.</li>
                <br />
                <li>Competitors are encouraged to record their fights in order to allow both sharing and ensure proof in case of the need to check games.</li>
                <br />
                <li>
                    This is an early access game, so unexpected things can happen. The standard protocol is to reset the match in case a major bug occurs, although treating the occurrences differently
                    is allowed if both players agree to do so.
                </li>
                <br />
                <li>
                    We heavily encourage you to get yourself a timezone role on the PORC dicord server and take full advantage of both discords events and unix time stamps to help you and your fellow
                    rumblers plan matches
                </li>
                <br />
                <li>As is common courtesy in the RUMBLE community, please treat each other with respect and enjoy yourself.</li>
            </ul>
        </div>
        <div class="spacer"></div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.container-fill {
    min-height: 93vh;
}

.part:nth-of-type(n) {
    margin-top: 0 !important;
    background: none !important;
}



.part:nth-of-type(2n) {
    background: none !important;
}

.page-header {
    //background-image: url('@/assets/images/MatchPlannerHeaderNoPorc.png');
}

.title {
    justify-content: center;
    text-align: center;
    margin: 3rem;
    font-style: bold;
    height: fit-content;
}

.lesser-title {
    justify-content: left;
    text-align: left;
    font-style: bold;
    height: fit-content;
    padding-left: 1.6rem;
    margin-bottom: 2rem;
}

.under-rules {
    padding-top: 2rem;
    align-items: first baseline;
    padding-left: 1.5rem;
    padding-right: 1.5rem;
}

.highlight {
    text-decoration: underline;
}

.form-container {
    top: 0;
}

.warning {
    font-style: italic;
    color: rgb(255, 53, 39);
}

.success {
    font-style: italic;
    color: rgb(19, 244, 98);
}

.spacer {
    height: 120px;
}
</style>
