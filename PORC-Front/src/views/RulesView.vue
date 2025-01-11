<template>
    <div class="container-fill justify-content-center">
        <div class="inner-container">
            <h1 class="titel">PORC Rules</h1>
            <div class="rule-container col-10">
                <div class="rule-box">
                    <h2 class="titel">Set rules</h2>
                    <ul>
                        <li class="rule"> 
                            The players will play a total of 6 matches on the chosen map (3 as host and 3 as client). Client round wins do count. 
                            If a player has more client round wins than the other, they will win the game at the end of the 6 matches.
                        </li> <br> <li>
                            The players are allowed to decide which map they want to play on (custom maps are also allowed). Alternatively, they may also choose to go to a random map together. 
                            If both players do not agree on their prefered map, Pit is chosen as default.
                        </li> <br> <li>
                            After each match, if any client rounds were won, <span class="highlight">the host may decide to switch to the other map.</span>
                        </li> <br> <li>
                            If both players are deadlocked, extended tiebreaker rules may be applied with the agreement of both parties.
                        </li> <br> <li>
                            No cheating of any kind. If a player cheats, they are banned immediately from the tournament.
                        </li> <br> <li>
                            Mods that affect any gameplay are not allowed and will be considered as cheating.
                        </li>
                    </ul>
                    <div class="p-3"></div>
                    <div class="under-rules">
                        <h4 class="lesser-titel">Tiebreaker rules</h4>
                        <ul>
                            <li>
                                First to win a client round will set the bar.
                            </li> <br> <li>
                                The opponent will have the right of reply. 
                                If they equal the client round, the tiebreaker resets.
                            </li> <br> <li>
                                If they fail to equal, the first client round winner wins. If they win the match, they will win.
                            </li>
                        </ul>
                    </div>
                    <div class="p-3"></div>
                    <div class="under-rules">
                        <h4 class="lesser-titel">Extended tiebreaker rules</h4>
                        <ol>
                            <li>
                                Host goes to one shift stone.
                            </li> <br> <li>
                                Host goes to zero shift stones.
                            </li> <br> <li>
                                The players switch to ring if on Pit.
                            </li> <br> <li>
                                The host starts with less HP.
                            </li>
                        </ol>
                    </div>
                </div>
                <div class="rule-box">
                    <h2 class="titel">Scoring</h2>
                    <ul>
                        <li> 
                            A match is defined as a best of 3 rounds with one player as host and the other as client. At the end of a game, scoring is decided as follows:
                        </li> <br> <li>
                            If the number of matches won is unequal between the players, the final score is simply the matches won by each player during the game.
                        </li> <br> <li>
                            In case both players won an equal number of matches, the final score is determined by the matches won by both players +1 for the winning party.
                        </li>
                    </ul>
                </div>
                <div class="rule-box">
                    <h2 class="titel">Additional Notes</h2>
                    <ul>
                        <li> 
                            Cheaters will be banned from competing, and all their matches will be marked as 3-0 losses to their opponents.
                        </li> <br> <li>
                            Competitors are encouraged to record their fights in order to allow both sharing and ensure proof in case of the need to check games.
                        </li> <br> <li>
                            This is an early access game, so unexpected things can happen. The standard protocol is to reset the match in case a major bug occurs, 
                            although treating the occurrences differently is allowed if both players agree to do so.
                        </li> <br> <li>
                            As is common courtesy in the RUMBLE community, please treat each other with respect and enjoy yourself.
                        </li>
                    </ul>
                </div>
                <div class="spacer"></div>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
    .container-fill {
        min-height: 93vh;
    }

    .inner-container {
        overflow-x: hidden;
        justify-content: center;
        flex-direction: column;
        align-items: center;
        display: flex;
    }

    .titel {
        justify-content: center;
        text-align: center;
        margin: 3rem;
        font-style: bold;
        height: fit-content;
    }

    .lesser-titel {
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

    .rule-box {
        margin-bottom: 10rem;
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
