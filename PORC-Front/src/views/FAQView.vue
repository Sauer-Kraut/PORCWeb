<script lang="ts" setup>
    import config from '@/config';
import { ref } from 'vue';

    const signUpURL = `${config.getBackendUrl()}`;
    const discordServerURL = 'https://discord.gg/TUQd26DTzg';
    const discordTimeStamps = 'https://www.geeksforgeeks.org/how-to-make-timestamps-on-discord/#what-is-a-discord-timestamp';

    interface FAQItem {
        question: string;
        answer: string;
    }

    const faqs: FAQItem[] = [
    {
        question: "How can I participate in PORC?",
        answer:
        "To compete in PORC, you need to sign up before the start of the season you wish to participate in and join the Discord server. You can sign up here.",
    },
    {
        question: "How will my matches work?",
        answer:
        "You can play your matches against opponents in any order. The goal is for everyone to complete all their matches by the end of the season, so please plan accordingly and ahead of time. It is your responsibility to coordinate when and how to play your matches with each opponent.",
    },
    {
        question: "How should I plan my matches?",
        answer:
        "With the help of the Match Planner, which you can easily access from the top menu if you are logged in, you can set your own schedule, including your availability and special notes. You can also view your opponents' schedules and challenge them. When you challenge an opponent, they will be contacted via Discord by PORC Bot with a prompt to accept or decline the request. They can respond directly on the website in their schedule. Once your opponent reacts to your request, you will automatically be informed of their response via Discord. If a request is accepted, an event will be created on the PORC Discord for the planned time, allowing other competitors to observe your match live in one of the stage channels on the PORC Discord server.",
    },
    {
        question: "brk",
        answer: ""
    },
    {
        question: "What happens if I don't play all my matches?",
        answer:
        "You may feel immense guilt, but other than that, it's fine—we are here to have fun, after all.",
    },
    {
        question: "Which division will I be in?",
        answer:
        "Your initial division placement is determined by your BP and estimated capabilities. If you're unhappy with your placement, you can contest it before the season starts.",
    },
    {
        question: "brk",
        answer: ""
    },
    {
        question: "How do I enter a score?",
        answer:
        "To enter a score you have to be logged in as the discord account you signed up with and press on the pencil icon next to the match of yours you want to enter a score for. You can only enter scores for your own matches.",
    },
    {
        question: "My opponent isn't responding/showing up/forfeited - what now?",
        answer:
        "Congratulations, you've won your match! It's up to you whether you'd like to enjoy the victory or give your opponent another chance. If you choose to claim the win, you can enter the match score as 3-0 in your favor.",
    },
    {
        question: "I want to leave PORC - how?",
        answer:
        "If you want to leave PORC, simply enter all your remaining matches as 0-3 in your opponents' favor. Ideally, you should also announce your resignation to your fellow rumblers in your division to avoid confusion.",
    },
    ];


    const activeIndex = ref<number | null>(0);

    function toggle(index: number) {
        activeIndex.value = activeIndex.value === index ? null : index;
    }
</script>

<template>
    <div class="col-10 col-lg-6 col-xl-4 mt-6 faq-body">

        <header class="mb-5 mt-3 text-center pb-4">
            <h1 class="decor-title primary">FAQ</h1>
            <p class="content-subtitle">Common Questions and their Awnsers</p>
        </header>

        <div
            v-for="(item, index) in faqs"
            :key="index"
            class="faq-item"
            :class="{ active: activeIndex === index , hidden: item.question == 'brk'}"
            >

            <div class="faq-question" @click="toggle(index)" v-if="item.question != 'brk'">
                <span>{{ item.question }}</span>
                <span>{{ activeIndex === index ? "−" : "+" }}</span>
            </div>
            <div class="m-4 p-2" v-else></div>
            <div v-if="activeIndex === index && item.question != 'brk'" class="faq-answer">
                {{ item.answer }}
            </div>
        </div>

    </div>
</template>

<style scoped>
@import '@/assets/scss/styles.scss';
@import '@/assets/scss/global.scss';

.faq-body {
}

.faq-item {
    margin-bottom: 2.5rem;
    border-bottom: 1px solid #333;
    padding-bottom: 1rem;

    &.hidden {
        border: none !important;
    }
}

.faq-question {
  cursor: pointer;
  font-weight: bold;
  font-size: 1.2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: color 0.2s ease;
}

.faq-question:hover {
  color: var(--primary);
}

.faq-answer {
  margin-top: 0.5rem;
  font-size: 0.95rem;
  color: #ccc;
  line-height: 1.4;
}

.faq-item.active .faq-question {
  color: var(--primary);
}

.mt-6 {
    margin-top: 5rem !important;

    @media (max-width: 768px) {
        margin-top: 2rem !important;
    }
}
</style>
