<template>
  <header>
    <nav class="row justify-content-center text-center h-100 custom">
      <router-link to="/" class="router-link col-2 h-100 custom">Tournament</router-link>
      <router-link to="/rules" class="router-link col-2 h-100 custom">Rules</router-link>
      <router-link to="/qaa" class="router-link col-2 h-100 custom">QaA</router-link>
      <router-link to="/sign-up" class="router-link col-2 h-100 custom">Sign up</router-link>
    </nav>
  </header>

  <div class="row justify-content-center h-100 custom">
    <main class="col-xxl-4 col-xl-6 col-md-8 p-0 custom">
      <router-view></router-view>
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
}
</style>
