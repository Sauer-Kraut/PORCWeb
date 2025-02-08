<script lang="ts" setup>
  import { ref } from 'vue';
  import DiscordUserComponent from './components/DiscordUserComponent.vue';

  const isMenuOpen = ref(false)

  function toggleMenu() {
    isMenuOpen.value = !isMenuOpen.value
  }
</script>

<template>
  <header>
    <!-- Burger Icon -->
    <div class="burger-icon" @click="toggleMenu">
      <span class="bar" :class="{ 'open': isMenuOpen }"></span>
      <span class="bar" :class="{ 'open': isMenuOpen }"></span>
      <span class="bar" :class="{ 'open': isMenuOpen }"></span>
    </div>

    <!-- Navigation -->
    <nav class="justify-content-center text-center h-100 custom" :class="{ 'open': isMenuOpen, 'row': !isMenuOpen }">
      <router-link to="/" :class="{'col-12': isMenuOpen, 'hidden': !isMenuOpen}" class="router-link col-2 h-100 custom">Tournament</router-link>
      <router-link to="/signup" :class="{'col-12': isMenuOpen, 'hidden': !isMenuOpen}" class="router-link col-2 h-100 custom">Sign Up</router-link>
      <router-link to="/rules" class="router-link col-2 h-100 custom" :class="{'col-12': isMenuOpen, 'hidden': !isMenuOpen}">Rules</router-link>
      <router-link to="/faq" class="router-link col-2 h-100 custom" :class="{'col-12': isMenuOpen, 'hidden': !isMenuOpen}">FAQ</router-link>
    </nav>

    <!-- Discord User Component -->
    <div class="custom discordUser">
      <DiscordUserComponent></DiscordUserComponent>
    </div>
  </header>

  <div class="row justify-content-center h-100 custom backgorund">
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
  background: linear-gradient(135deg, #3a3938, #4a4f5b);
  overflow-x: hidden;
  scrollbar-color: #242424;
}

.background {
  background: linear-gradient(135deg, #ff8306, #4a4f5b);
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

.burger-icon {
  display: none;
  cursor: pointer;
  padding: 10px;
  width: 100px;
  position: absolute;
}

.burger-icon .bar {
  display: block;
  width: 25px;
  height: 3px;
  margin: 5px 0;
  background-color: #333;
  transition: 0.3s;
}

/* When the menu is open, rotate bars */
.burger-icon .bar.open:nth-child(1) {
  transform: rotate(45deg) translate(5px, 5px);
}

.burger-icon .bar.open:nth-child(2) {
  opacity: 0;
}

.burger-icon .bar.open:nth-child(3) {
  transform: rotate(-45deg) translate(5px, -5px);
}

/* Mobile Menu Styles */
@media (max-width: 768px) {

  .open {
    margin-top: 0.15rem;
    width: 100vw;
  }

  header {
    height: auto;
    min-height: 60px;
    background-color: $header-color;
    overflow-x: hidden;
  }

  .discordUser {
    top: 0%;
    right: 3%;
    width: 50px;
    position: absolute;
    z-index: 10000;
    display: flex;
    justify-content: center;
    align-items: center;
    height: 50px;
    overflow-x: hidden;
  }

  /* Hide the regular menu on mobile and show the burger */
  nav {
    height: 100%;
    overflow-x: hidden;

    .hidden {
      height: 100%;
      align-content: center;
      color: rgb(0, 0, 0);
      text-decoration: none;
      display: none;
    }
  }

  .burger-icon {
    display: block;  // Show burger icon on small screens
  }

  /* Display the menu when open */
  .row.open {
    display: block;  // Show the navigation items when the menu is open
    width: 100%;
    text-align: center;
  }

  .router-link {
    display: block;  // Make the links stack vertically
    padding: 10px 0;
  }
}

/* Desktop menu (default behavior) */
@media (min-width: 769px) {
  .burger-icon {
    display: none;  // Hide burger icon on large screens
  }

  .row {
    display: flex;
    justify-content: center;
  }

  .router-link {
    padding: 10px 15px;
    text-align: center;
  }
}
</style>
