<template>
  <div class="header">
    <Header @run="runPattern" @stop="stopPattern" />
  </div>

  <div class="app-content" style="overflow: hidden">
    <RouterView ref="homePage"></RouterView>
  </div>
</template>

<script setup lang="ts">
import Header from "./components/Header.vue";
import { RouterView } from "vue-router";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { PatternType, useGridStore } from "./store";

const homePage = ref();

function runPattern(): void {
  const grid = useGridStore();

  if (grid.usePatternType === PatternType.Static) {
    // map grid by taking only the value attribute
    const gridValue = grid.grid.map((row) => row.value);

    invoke("run_pattern", { port: grid.port, gridValue: gridValue }).then((p: any) => {
      console.log(p);
    });
  } else {
    invoke("run_pattern", {
      port: grid.port,
      gridValue: null,
      profiles: grid.pattern.profiles,
    }).then((p: any) => {
      console.log(p);
    });
  }
}

function stopPattern(): void {
  const grid = useGridStore();

  invoke("stop_pattern", { port: grid.port }).then((p: any) => {
    console.log(p);
  });
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
  min-width: 600px;
  margin: auto;
  width: 45%;
  text-align: center;
  box-sizing: border-box;
}
</style>
