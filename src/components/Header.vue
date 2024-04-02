<template>
  <div class="header-container">
    <div>
      <!--class="header-elements"-->
      <div class="button-container">
        <!-- <button class="header-button dropButton">File</button>
        <div class="dropContent">
          <button class="dropdown-button" @click="openFileInput">Load File</button>
          <input type="file" ref="fileInput" style="display: none" /> -->
        <!-- <button class="dropdown-button">Save Pattern</button> -->
        <!-- </div> -->
        <RouterLink to="/" class="header-button">Home</RouterLink>
        <RouterLink to="/patterns" class="header-button">Patterns</RouterLink>
        <RouterLink to="/help" class="header-button">Help</RouterLink>
      </div>
    </div>
    <div class="navbar-select-container">
      <button class="play-button action-button" @click="$emit('run')">
        <img src="../assets/play.svg" alt="play" class="img-icon" />
      </button>
      <button class="stop-button action-button" @click="$emit('stop')">
        <img src="../assets/stop.svg" alt="stop" class="img-icon" />
      </button>
      <!-- <button class="refresh-button action-button" @click="list">
        <img src="../assets/arrow-clockwise.svg" alt="refresh" class="img-icon"/>
      </button> -->
      <Select :select-options="ports"></Select>
      <!-- <img
        src="../assets/loader.svg"
        alt="refresh"
        class="refresh-button"
        style="height: 50px"
      /> -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api";
import Select from "./Select.vue";

const ports = ref([]);
// const fileInputRef = ref<HTMLInputElement | null>(null);

function list(): void {
  invoke("list_serial_ports").then((p: any) => {
    ports.value = p;
  });
}

// function openFileInput(): void {
//   // fileInputRef.value?.click();
//   // openFile();
// }

onMounted(() => {
  window.setInterval(list, 3000);
});
</script>

<style>
.header-container {
  overflow: hidden;
  background-color: #333;
  height: 54px;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 -2px 8px 4px #333;
}

.header-elements {
  display: flex;
  flex-direction: row;
}

.button-container {
  margin-left: 12px;
}

.header-button {
  background: none;
  border: none;
  box-shadow: none;
  border-radius: 0;
  color: white;
  height: 100%;
  margin-right: 12px;
}

.refresh-button {
  border: none;
  box-shadow: none;

  color: white;
  height: 100%;
  margin-right: 12px;
}

.action-button {
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 20px;
  margin-right: 8px;
  padding: 0;
  padding-top: 6px;
  padding-left: 1px;
  background-color: #1a1a1a;
}

.img-icon {
  height: 24px;
}

.header-button:hover {
  color: rgb(0, 208, 255);
}

.dropdown-button {
  width: 100%;
  background: none;
  border: none;
  box-shadow: none;
  text-align: left;
  border-radius: 0;
}

.dropdown-button:hover {
  background-color: #eee;
}

.dropdown-button:active {
  background-color: #aaa;
}

ul {
  list-style-type: none;
  margin: 0;
  padding: 0;
}

li {
  float: left;
}

.dropContent {
  display: none;
  position: absolute;
  background-color: #f9f9f9;
  min-width: 160px;
  box-shadow: 0px 8px 16px 0px rgba(0, 0, 0, 0.2);
  z-index: 1;
  top: 54px;
}

.dropContent a {
  color: black;
  padding: 12px 16px;
  text-decoration: none;
  display: block;
  text-align: left;
}

.dropContent a:hover {
  background-color: #eee;
}

.dropdown:hover .dropContent {
  display: block;
}

.navbar-select-container {
  margin-right: 12px;
  display: flex;
}
</style>
