<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Logo from './svgs/Logo.vue';

const props = defineProps<{
    items: (String | null)[];
}>();

const selecteditem = defineModel<String | null>('selectedItem');


/* =========================
   Replace icons later
   ========================= */

const items: (String | null)[] = [
  "⌂" ,
  "🔍",
  "📥",
  null,
  "🔔",
  null,
  "🕘",
  "📅",
  "👤"
]

const selectedIndex = ref<number>(0);

onMounted(() => {
    selectedIndex.value = 0;
    selecteditem.value = items[0];
});
</script>

<template>
    <aside class="sidebar">
        <Logo class="header-img col-auto" :primaryColor="'rgb(26, 23, 23)'"></Logo>
        <div class="separator" />
        <div
        v-for="(item, index) in items"
        :key="index"
        class="sidebar-item"
        >
            <!-- Separator -->
            <div
                v-if="item === null"
                class="separator"
            />

            <!-- Icon -->
            <button
                v-else
                class="icon-button"
                :class="{'active': index === selectedIndex}"
                @click.stop="selectedIndex = index; selecteditem = item"
                type="button"
            >
                <span class="icon">{{ item }}</span>
            </button>
        </div>
    </aside>
</template>

<style scoped lang="scss">
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

/* =========================
   Sidebar shell
   ========================= */

    .sidebar {
        width: 72px;
        height: 100%;
        background: #0b0b0b;
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 16px 0;
        gap: 14px;
    }

/* =========================
   Icons
   ========================= */

.icon-button {
    width: 44px;
    height: 44px;
    border-radius: 14px;
    background: transparent;
    border: none;
    color: #a0a0a0;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: 0.15s ease;

    &:hover {
        background: rgba(181,108,255,0.12);
        color: #eaeaea;
    }

    &.active {
        background: var(--primary);
        color: #000000;
    }
}

.icon {
  font-size: 18px;
  line-height: 1;
}

/* =========================
   Separator
   ========================= */

.separator {
  width: 32px;
  height: 1px;
  background: rgba(255,255,255,0.08);
  margin: 8px 0;
}

    .header-img {
        padding: 0.5rem !important;

        width: 3rem;
        height: 3rem;
        object-fit: cover;

        border-radius: 16px;
        // box-shadow: 0 0 20px rgba(0, 0, 0, 0.518);
        // border: 1px solid $border-color;

        align-self: center;

        background-color: var(--primary);
    }
</style>
