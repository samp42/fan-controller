<template>
  <div class="header-container">
    <div>
      <div class="button-container">
        <RouterLink to="/" class="header-button">Home</RouterLink>
        <RouterLink to="/patterns" class="header-button">Patterns</RouterLink>
        <RouterLink to="/help" class="header-button">Help</RouterLink>
      </div>
    </div>
    <div>
      <input type="file" accept=".csv" @change="handleFileUpload" />
    </div>
    <div class="navbar-select-container">
      <button class="play-button action-button" @click="$emit('run')">
        <img src="../assets/play.svg" alt="play" class="img-icon" />
      </button>
      <button class="stop-button action-button" @click="$emit('stop')">
        <img src="../assets/stop.svg" alt="stop" class="img-icon" />
      </button>
      <button @click="time()">Time</button>
      <!-- <button class="refresh-button action-button" @click="list">
        <img src="../assets/arrow-clockwise.svg" alt="refresh" class="img-icon"/>
      </button> -->
      <Select :select-options="ports" v-if="ports.length !== 0"></Select>
      <img v-else src="../assets/loader.svg" alt="loader" style="height: 50px" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api";
import Select from "./Select.vue";
import Papa from "papaparse";
import { InstantProfile, PatternType, useGridStore } from "../store";

const ports = ref([]);

function list(): void {
  invoke("list_serial_ports").then((p: any) => {
    ports.value = p;
  });
}

function time(): void {
  const store = useGridStore();
  invoke("get_timing", { port: store.port }).then((p: any) => {
    console.log(p);
  });
}

function handleFileUpload(event: any) {
  const file = event.target.files[0];

  Papa.parse(file, {
    complete: (results: any) => {
      // Extracting data from parsed CSV
      const parsedData = results.data;

      const instantProfiles: InstantProfile[] = parsedData
        .map(mapRowToInstantProfile)
        .filter((profile: InstantProfile | null) => profile !== null);

      const store = useGridStore();
      store.pattern = { profiles: instantProfiles };
      store.usePatternType = PatternType.Dynamic;

      console.log(store.pattern);
    },
    header: true, // Set to true if your CSV file has headers
  });
}

function mapRowToInstantProfile(row: any): InstantProfile | null {
  if (!row.dt || !row.fan1 || !row.fan81) {
    // If any required column is missing, return null (invalid)
    return null;
  }

  const profile: InstantProfile = {
    dt: parseInt(row.dt), // Assuming dt is a number
    fans: [],
  };

  for (let i = 1; i <= 81; i++) {
    const fanColumnName = "fan" + i;
    profile.fans.push(parseInt(row[fanColumnName]));
  }

  return profile;
}

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
