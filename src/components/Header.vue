<template>
  <div class="header-container">
    <div class="header-elements">
      <div class="dropdown">
        <button class="header-button dropButton">File</button>
        <div class="dropContent">
          <button class="dropdown-button">Load File</button>
          <button class="dropdown-button">Save Pattern</button>
        </div>
        <RouterLink to="/patterns" class="header-button">Patterns</RouterLink>
        <RouterLink to="/help" class="header-button">Help</RouterLink>
      </div>
      
    </div>

    <div class="navbar-select-container">
      <button class="refresh-button" @click="list">Refresh</button>
      <Select :select-options="ports"></Select>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onBeforeMount } from "vue";
import { invoke } from "@tauri-apps/api";
import Select from "./Select.vue";

const ports = ref([]);

function list() {
  invoke('list_serial_ports').then((p: any) => {
    ports.value = p;
  });
}

onBeforeMount(() => {
  list();
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

.header-button {
  background: none;
  border: none;
  box-shadow: none;
  border-radius: 0;
  color: white;
  height: 100%;
}

.refresh-button {
  border: none;
  box-shadow: none;

  color: white;
  height: 100%;
  margin-right: 12px;

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
}

.dropContent a {
  color: black;
  padding: 12px 16px;
  text-decoration: none;
  display: block;
  text-align: left;
}

.dropContent a:hover {
  background-color: #f1f1f1;
}

.dropdown:hover .dropContent {
  display: block;
}

.navbar-select-container {
  margin-right: 12px;
  display: flex;
}
</style>
