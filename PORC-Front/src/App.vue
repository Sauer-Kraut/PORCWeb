<template>
  <header></header>

  <main>
    <h3 class="text-primary">Hello PORC</h3>
    <nav class="row">
      <router-link to="/" class="col">Brackets</router-link>
      <router-link to="/qaa" class="col">QaA</router-link>
    </nav>
  </main>

  <router-view></router-view>
</template>
