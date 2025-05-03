<script lang="ts" setup>
import config from '@/config';

const signUpURL = `${config.getBackendUrl()}`;
const discordServerURL = 'https://discord.gg/TUQd26DTzg';
const discordTimeStamps = 'https://www.geeksforgeeks.org/how-to-make-timestamps-on-discord/#what-is-a-discord-timestamp';
</script>

<template>
    <div class="page-header p-3">
        <h1>FAQ</h1>
    </div>
    <div class="container-fill justify-content-center">
        <div class="part part-text">
            <h2 class="lesser-title">How can I participate in PORC?</h2>
            <label class="answer">
                To compete in PORC, you need to sign up before the start of the season you wish to participate in and join the
                <a class="link" :href="discordServerURL" target="_other">Discord server</a>. You can sign up <a class="link" :href="signUpURL" target="_self">here</a>.
            </label>

            <h2 class="lesser-title">How will my matches work?</h2>
            <label class="answer">
                You can play your matches against opponents in any order. <span class="highlight-text">The goal is for everyone to complete all their matches</span> by the end of the season, so please
                plan accordingly and ahead of time. It is your responsibility to coordinate when and how to play your matches with each opponent.
            </label>

            <h2 class="lesser-title">How should I plan my matches?</h2>
            <label class="answer">
                With the help of the <span class="highlight-text">Match Planner</span> , which you can easily access from the top menu if you are logged in, you can set your own schedule, including
                your availability and special notes. You can also view your opponents' schedules and challenge them. <br />
                When you challenge an opponent, <span class="highlight-text">they will be contacted via Discord by PORC Bot</span> with a prompt to accept or decline the request. They can respond
                directly on the website in their schedule. <br />
                Once your opponent reacts to your request, you will automatically be informed of their response via Discord. If a request is accepted, an event will be created on the PORC Discord for
                the planned time, allowing other competitors to observe your match live in one of the stage channels on the PORC Discord server.
            </label>

            <h2 class="lesser-title">What happens if I don't play all my matches?</h2>
            <label class="answer"> You may feel immense guilt, but other than that, it's fine—we are here to have fun, after all. </label>

            <h2 class="lesser-title">Which division will I be in?</h2>
            <label class="answer">
                Your initial division placement is <span class="highlight-text">determined by your BP and estimated capabilities</span>. If you're unhappy with your placement, you can contest it
                before the season starts.
            </label>

            <h2 class="lesser-title">How do I enter a score?</h2>
            <label class="answer">
                To enter a score <span class="highlight-text">you have to be logged</span> in as the discord account you signed up with and press on the pencil icon next to the match of yours you want
                to enter a score for. You can only enter scores for your own matches.
            </label>

            <h2 class="lesser-title">My opponent isn't responding/showing up/forfeited - what now?</h2>
            <label class="answer">
                Congratulations, you've won your match! It's up to you whether you'd like to enjoy the victory or give your opponent another chance. If you choose to claim the win, you can enter the
                match score as 3-0 in your favor.
            </label>

            <h2 class="lesser-title">I want to leave PORC - how?</h2>
            <label class="answer">
                If you want to leave PORC, simply enter all your remaining matches as 0-3 in your opponents' favor. Ideally, you should also announce your resignation to your fellow rumblers in your
                division to avoid confusion.
            </label>
        </div>
        <div class="spacer"></div>
    </div>
</template>

<style lang="scss" scoped>
@import '@/assets/scss/styles.scss';

.container-fill {
    min-height: 93vh;
}

.page-header {
    //background-image: url('@/assets/images/MatchPlannerHeaderNoPorc.png');
}

.part:nth-of-type(2n) {
    background: linear-gradient(135deg, darken(#343232, 5%), darken(#23252b, 5%));
}

.titel {
    justify-content: center;
    text-align: center;
    margin: 3rem;
    font-style: bold;
    height: fit-content;
}

.lesser-title {
    margin-bottom: 0.4rem;
}

.highlight {
    text-decoration: underline;
}

.warning {
    font-style: italic;
    color: rgb(255, 53, 39);
}

.success {
    font-style: italic;
    color: rgb(19, 244, 98);
}

.answer {
    margin-bottom: 6rem;
}

.spacer {
    height: 100px;
}

.link {
    color: rgb(0, 191, 255);
    text-decoration: none;
}

.highlight-text {
    font-weight: 750;
}
</style>
