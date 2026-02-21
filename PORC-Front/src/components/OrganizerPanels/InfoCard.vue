<script lang="ts" setup>

    const props = defineProps<{
        titel: string,
        value: string,
        subtitle: string,
    }>();
</script>

<template>
    <div class="config-card">
        <div class="mini-title">{{ props.titel }}</div>
        <div class="mini-value">{{ props.value }}</div>
        <div class="mini-sub muted">{{ props.subtitle }}</div>
    </div>
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';
    @import '@/assets/scss/global.scss';

    .config-card { 
        background: linear-gradient(180deg, rgba(255,255,255,0.01), transparent); 
        border: 1px solid rgba(255,255,255,0.02); 
        padding: 10px; 
        border-radius: 8px; 

        .mini-title { font-size: 12px; color: #a0a0a0; font-weight: 600; }
        .mini-value { font-size: 18px; font-weight: 700; margin-top: 6px; color: #eaeaea; }
        .mini-sub { font-size: 12px; color: #9aa0a6; margin-top: 6px; }
    }

    .muted {
        color: #a0a0a0;
        font-size: 13px;
        line-height: 1.5rem;
    }
</style>
