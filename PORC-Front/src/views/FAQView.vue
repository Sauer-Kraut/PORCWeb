<script lang="ts" setup>
import config from '@/config';

const signUpURL = `${config.getBackendUrl()}/signup`;
const discordServerURL = 'https://discord.gg/TUQd26DTzg';
const discordTimeStamps = 'https://www.geeksforgeeks.org/how-to-make-timestamps-on-discord/#what-is-a-discord-timestamp';
</script>

<template>
    <div class="container-fill justify-content-center">
        <div class="inner-container">
            <h1 class="titel">FAQ</h1>
            <div class="rule-container col-10">
                <div class="rule-box">
                    <h2 class="lesser-title">How can I participate in PORC?</h2>
                    <label class="answer">
                        To compete in PORC, you need to sign up before the start of the season you wish to participate in and join the
                        <a class="link" :href="discordServerURL" target="_other">Discord server</a>. You can sign up <a class="link" :href="signUpURL" target="_self">here</a>.
                    </label>

                    <h2 class="lesser-title">How will my matches work?</h2>
                    <label class="answer">
                        You can play your matches against opponents in any order. The goal is for everyone to complete all their matches by the end of the season, so please plan accordingly and ahead
                        of time. It is your responsibility to coordinate when and how to play your matches with each opponent.
                    </label>

                    <h2 class="lesser-title">How should I plan a match?</h2>
                    <label class="answer">
                        Simply contact your opponent via our Discord server or Discord DMs to agree on a date and map for your match. Keep in mind that your opponent might be in a different time zone,
                        so reach out well in advance. Feel free to use both <a class="link" :href="discordTimeStamps" target="_other">Discord's Unix timestamp feature</a> and Discord events to avoid
                        misunderstandings and make planning as easy as possible for both you and your opponent.
                    </label>

                    <h2 class="lesser-title">What happens if I don't play all my matches?</h2>
                    <label class="answer"> You may feel immense guilt, but other than that, it's fine—we are here to have fun, after all. </label>

                    <h2 class="lesser-title">Which division will I be in?</h2>
                    <label class="answer">
                        Your initial division placement is determined by your BP and estimated capabilities. If you're unhappy with your placement, you can contest it before the season starts.
                    </label>

                    <h2 class="lesser-title">How do I enter a score?</h2>
                    <label class="answer">
                        To enter a score you have to be logged in as the discord account you signed up with 
                        and press on the pencil icon next to the match of yours you want to enter a score for.
                        You can only enter scores for your own matches.
                    </label>

                    <h2 class="lesser-title">My opponent isn't responding/showing up/forfeited - what now?</h2>
                    <label class="answer">
                        Congratulations, you've won your match! It's up to you whether you'd like to enjoy the victory or give your opponent another chance. If you choose to claim the win, you can
                        enter the match score as 3-0 in your favor.
                    </label>

                    <h2 class="lesser-title">I want to leave PORC - how?</h2>
                    <label class="answer">
                        If you want to leave PORC, simply enter all your remaining matches as 0-3 in your opponents' favor. Ideally, you should also announce your resignation to your fellow rumblers
                        in your division to avoid confusion.
                    </label>
                    <div class="p-3"></div>
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

.lesser-title {
    margin-bottom: 0.4rem;
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
    margin-top: 1rem;
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
</style>
