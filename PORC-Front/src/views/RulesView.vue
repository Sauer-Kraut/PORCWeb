<template>
    <div class="container-fill justify-content-center">
        <div class="inner-container">
            <h1 class="titel">PORC Rules</h1>
            <div class="rule-container col-10">
                <div class="rule-box">
                    <h2 class="titel">Set rules</h2>
                    <ul>
                        
                        <li class = "rule"> 
                            The players will play a total of 6 matches on the chosen map (3 as host and 3 as client), client round wins do count, 
                            if a player has more client round wins than the other, they will win the game at the end of the 6 matches.
                        </li> <br> <li>
                            The Players are allowed to decide which map they want to play on (custom maps are also allowed, alternatively they may also choose to go a random map together). 
                            If both players can not come to an agreement pit is choosen as default.
                        </li> <br> <li>
                            After each match, if any client rounds were won, the host may decide to switch to the other map.
                        </li> <br> <li>
                            No cheating of any kind, if a player cheats, they are banned immediately from the tournament.
                        </li> <br> <li>
                            Mods that affect any gameplay are not allowed and will be considered as cheating.
                        </li>
                    </ul>
                    <div class="under-rules">
                        <h4 class="lesser-titel">Tie breaker rules</h4>
                        <ul>
                            <li>
                                First to win a client round will set the bar.
                            </li> <br> <li>
                                The opponent will have the right of reply. 
                                If they equal the client round, the tie breaker resets.
                            </li> <br> <li>
                                If they fail to equal, the first client rounded winner wins. If they win the match, they will win
                            </li>
                        </ul>
                    </div>
                </div>
                <div class="rule-box">
                    <h2 class="titel">Scoring</h2>
                    <ul>
                        
                        <li class = "rule"> 
                            If the amount of matches won is unequal the final score is simply the matches won by each player during the game
                        </li> <br> <li>
                            The Players are allowed to decide which map they want to play on (custom maps are also allowed, alternatively they may also choose to go a random map together). 
                            If both players can not come to an agreement pit is choosen as default.
                        </li> <br> <li>
                            After each match, if any client rounds were won, the host may decide to switch to the other map.
                        </li> <br> <li>
                            No cheating of any kind, if a player cheats, they are banned immediately from the tournament.
                        </li> <br> <li>
                            Mods that affect any gameplay are not allowed and will be considered as cheating.
                        </li>
                    </ul>
                    <div class="under-rules">
                        <h4 class="lesser-titel">Tie breaker rules</h4>
                        <ul>
                            <li>
                                First to win a client round will set the bar.
                            </li> <br> <li>
                                The opponent will have the right of reply. 
                                If they equal the client round, the tie breaker resets.
                            </li> <br> <li>
                                If they fail to equal, the first client rounded winner wins. If they win the match, they will win
                            </li>
                        </ul>
                    </div>
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
        height: 200px;
    }
</style>
