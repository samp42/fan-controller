<template>
  <div class="header">
    <Header @run="runPattern" @stop="stopPattern"/>
  </div>

  <div class="app-content">
    <RouterView ref="homePage"></RouterView>
  </div>
</template>

<script setup lang="ts">
import Header from "./components/Header.vue";
import { RouterView } from "vue-router";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";

const homePage = ref();

function runPattern(): void {
  // console.log('run')
  // homePage.value.$emit("run_pattern");

  invoke('run_pattern').then((p: any) => {
    console.log(p);
  });
}

function stopPattern(): void {
  console.log('stop')
}
</script>

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
