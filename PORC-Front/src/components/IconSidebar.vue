<script setup lang="ts">
    import { onMounted, ref, watch } from 'vue';
    import Logo from './svgs/Logo.vue';

    const props = defineProps<{
        items: (string | null)[];
        modelValue?: string | null;
    }>();

    const emit = defineEmits<(e: 'update:modelValue', value: string | null) => void>();


    /* =========================
    Replace icons later
    ========================= */

    // const items: (String | null)[] = [
    //   "⌂" ,
    //   "🔍",
    //   "📥",
    //   null,
    //   "🔔",
    //   null,
    //   "🕘",
    //   "📅",
    //   "👤"
    // ]

    const selectedIndex = ref<number>(0);

    onMounted(() => {
        // initialize selectedIndex from modelValue when provided, otherwise default to first item
        const initial = props.modelValue ?? props.items[0] ?? null;
        const idx = props.items.findIndex(i => i === initial);
        selectedIndex.value = idx >= 0 ? idx : 0;
        // if parent hasn't provided a modelValue, emit initial value so parent and child stay in sync
        if (props.modelValue === undefined && props.items.length > 0) {
            emit('update:modelValue', props.items[selectedIndex.value]);
        }
    });

    watch(() => props.modelValue, (v) => {
        if (v === undefined || v === null) return;
        const idx = props.items.findIndex(i => i === v);
        if (idx >= 0) selectedIndex.value = idx;
    });
</script>

<template>
    <aside class="sidebar">
        <Logo class="header-img col-auto" :primaryColor="'rgb(26, 23, 23)'"></Logo>
        <div class="separator" />
        <div
        v-for="(item, index) in props.items"
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
                @click.stop="( () => { selectedIndex = index; emit('update:modelValue', item) } )()"
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
        background: color-mix(in srgb, var(--primary) 15%, transparent);
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
