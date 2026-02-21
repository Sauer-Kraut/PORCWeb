<script lang="ts" setup>
    import type { PubAccountInfo } from '@/models/pub_account_info/PubAccountInfo';
    import { ref, computed, watch, onMounted } from 'vue';

    const props = defineProps<{
        account: PubAccountInfo;
    }>();

    let avatarUrl = ref<string>(`https://cdn.discordapp.com/avatars/${props.account.id}/${props.account.avatar}.png`);

    watch(() => props.account, (newValue) => {
        
        avatarUrl.value = `https://cdn.discordapp.com/avatars/${newValue.id}/${newValue.avatar}.png`;
        
    });

    onMounted(async () => {
        console.log(props.account);
        // getSelectorHeight();
    });
</script>

<template>
    <img
        class="avatar"
        :src="avatarUrl"
        alt="Discord User Image"
        @error="avatarUrl = 'https://cdn.discordapp.com/embed/avatars/0.png'"
    />
</template>

<style lang="scss" scoped>
    @import '@/assets/scss/styles.scss';

    .avatar {
        border-radius: 50%;
    }
</style>
