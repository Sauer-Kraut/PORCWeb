<script lang="ts" setup>
  import DiscordUserComponent from './components/DiscordUserComponent.vue';
</script>

<template>
  <header>
    <nav class="row justify-content-center text-center h-100 custom">
      <router-link to="/" class="router-link col-s-3 col-xl-2 col-xll-2 h-100 custom">Tournament</router-link>
      <router-link to="/signup" class="router-link col-s-3 col-xl-2 col-xll-2 h-100 custom">Sign Up</router-link>
      <router-link to="/rules" class="router-link col-s-3 col-xl-2 col-xll-2 h-100 custom">Rules</router-link>
      <router-link to="/qaa" class="router-link col-s-3 col-xl-2 col-xll-2 h-100 custom">QaA</router-link>
    </nav>
    <div class="col-s-3 col-xl-2 col-xll-2 custom discordUser">
    <DiscordUserComponent></DiscordUserComponent>
    </div>
  </header>

  <div class="row justify-content-center h-100 custom">
    <main class="col-xxl-4 col-xl-6-cust col-l-8-cust col-lg-8 col-md-9 col-11 p-0 custom">
      <router-view class="custom"></router-view>
    </main>
  </div>
</template>

<style lang="scss" scoped>
$header-color: rgb(241, 241, 241);

header {
  height: 90px;
  background-color: $header-color;
  overflow-x: hidden;
}

main {
  min-height: 100%;
  background-color: #323232;
  overflow-x: hidden;
  scrollbar-color: #242424;
}

nav {
  height: 100%;
  overflow-x: hidden;

  .router-link {
    height: 100%;
    align-content: center;
    color: rgb(0, 0, 0);
    text-decoration: none;

    &.router-link-active {
      background-color: darken($header-color, 4%);
      font-weight: bolder;
    }

    &:hover {
      background-color: darken($header-color, 5%);
      font-weight: bolder;
    }
  }
}

.custom {
  overflow-x: hidden;
  overflow-y: hidden;
}

.discordUser {
  top: 0%;
  right: 3%;
  width: 100px;
  position: absolute;
  z-index: 10000;
  display: flex;
  justify-content: center;
  align-items: center;
  height: 90px;
  overflow-x: hidden;
}

@media (max-width: 2400px) and (min-width: 1699px) {
  .col-xl-6-cust {
  width: 50%;
  }
}

@media (max-width: 1699px) and (min-width: 1400px) {
  .col-l-8-cust {
    width: 66%;
  }
}

@media (max-width: 1699px) {
  .col-s-3 {
    width: 25%;
  }
}
</style>
