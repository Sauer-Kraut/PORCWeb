<template>
  <header>
    <nav class="row justify-content-center text-center h-100">
      <router-link to="/" class="router-link col-2 h-100">Brackets</router-link>
      <router-link to="/qaa" class="router-link col-2 h-100">QaA</router-link>
    </nav>
  </header>

  <div class="row justify-content-center h-100">
    <main class="col-md-8 p-0">
      <router-view></router-view>
    </main>
  </div>
</template>

<style lang="scss" scoped>
$header-color: #5c5c5c;

header {
  height: 50px;
  background-color: $header-color;
}

main {
  min-height: 100%;
  background-color: #323232;
}

nav {
  height: 100%;

  .router-link {
    height: 100%;
    align-content: center;
    color: white;
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
</style>
