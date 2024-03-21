<script setup lang="ts">
import Header from "./components/Header.vue";
import { RouterView } from "vue-router";
</script>

<template>
  <div class="header">
    <Header />
  </div>

  <div class="app-content">
    <RouterView></RouterView>
  </div>
</template>

<style>
.header {
  text-align: center;
  position: fixed;
  width: 100%;
  left: 0;
  top: 0;
  z-index: 100;
}

.app-content {
  margin-top: 54px;
  overflow: scroll;
  margin-bottom: 50px;
}

.center {
  min-width:600px;
  margin: auto;
  width: 40%;
  text-align: center;
  box-sizing: border-box;
}
</style>
