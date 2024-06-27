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

async function runPattern(): Promise<void> {
  const grid = useGridStore();
  console.log('Store pattern type:', grid.usePatternType);
  console.log('Store grid:', grid.grid);
  console.log('Store pattern:', grid.pattern);

  if (grid.usePatternType === PatternType.Static) {
    const gridValue = grid.grid.map((row) => row.value);
    console.log('Grid value:', gridValue);

    try {
      const response = await invoke("run_pattern", { port: grid.port, gridValue: gridValue });
      console.log('Response from run_pattern (Static):', response);
    } catch (error) {
      console.error('Error in run_pattern (Static):', error);
    }
  } else {
    const profiles = grid.pattern.profiles;
    console.log('Profiles:', profiles);

    try {
      const response = await invoke("run_pattern", {
        port: grid.port,
        gridValue: null,
        profiles: profiles,
      });
      console.log('Response from run_pattern (Dynamic):', response);
    } catch (error) {
      console.error('Error in run_pattern (Dynamic):', error);
    }
  }
}

async function stopPattern(): Promise<void> {
  const grid = useGridStore();

  try {
    const response = await invoke("stop_pattern", { port: grid.port });
    console.log('Response from stop_pattern:', response);
  } catch (error) {
    console.error('Error in stop_pattern:', error);
  }
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
