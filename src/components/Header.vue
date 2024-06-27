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
      <!-- Error message -->
      <span v-if="showFileUploadError" class="error-message">
        Please upload a file
      </span>
    </div>
    <div class="toggle-container">
      <label class="toggle-label">
        <span class="toggle-label-left">Static</span>
        <input type="checkbox" v-model="dynamicMode" class="toggle-input">
        <span class="toggle-slider"></span>
        <span class="toggle-label-right">Dynamic</span>
      </label>
    </div>
    <div class="action-buttons">
      <button class="play-button action-button" @click="handlePlayButtonClick">
        <img src="../assets/play.svg" alt="play" class="img-icon" />
      </button>
      <button class="stop-button action-button" @click="$emit('stop')">
        <img src="../assets/stop.svg" alt="stop" class="img-icon" />
      </button>
    </div>
    <div class="navbar-select-container">
      <Select :select-options="ports" v-if="ports.length !== 0"></Select>
      <img v-else src="../assets/loader.svg" alt="loader" style="height: 50px" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { RouterLink } from 'vue-router';
import { invoke } from '@tauri-apps/api';
import Select from './Select.vue';
import Papa from 'papaparse';
import { InstantProfile, PatternType, useGridStore } from '../store';

const ports = ref([]);
const dynamicMode = ref(false);
const store = useGridStore();
const emit = defineEmits(['run', 'stop']);
let fileUploaded = false;
const showFileUploadError = ref(false); // Reactive variable for error message

function list(): void {
  invoke('list_serial_ports').then((p: any) => {
    ports.value = p;
    console.log(p);
  });
}

function handleFileUpload(event: any) {
  const file = event.target.files[0];

  Papa.parse(file, {
    complete: (results: any) => {
      const parsedData = results.data;

      const instantProfiles: InstantProfile[] = parsedData
        .map(mapRowToInstantProfile)
        .filter((profile: InstantProfile | null) => profile !== null);

      store.pattern = { profiles: instantProfiles };
      store.usePatternType = dynamicMode.value ? PatternType.Dynamic : PatternType.Static;

      console.log('Parsed pattern:', store.pattern);
      fileUploaded = true; // Set fileUploaded flag to true
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
    const fanColumnName = 'fan' + i;
    profile.fans.push(parseInt(row[fanColumnName]));
  }

  return profile;
}

function handlePlayButtonClick() {
  if (dynamicMode.value && !fileUploaded) {
    // Show error message if dynamic mode is selected but no file uploaded
    showFileUploadError.value = true;
  } else {
    // Reset error message state
    showFileUploadError.value = false;

    // Proceed to run pattern
    store.setPatternType(dynamicMode.value ? PatternType.Dynamic : PatternType.Static);
    console.log('Running pattern:', dynamicMode.value ? 'Dynamic' : 'Static');
    console.log('Store pattern:', store.pattern);
    emit('run');
  }
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

.toggle-label {
  position: relative;
  display: inline-block;
  width: 150px; /* Adjust width as needed */
  height: 45px; /* Adjust height as needed */
  /*background-color: #cccccc78; /* Background color of the toggle switch */
  background-color:#65656598;
  border-radius: 4px; /* Rounded corners */
  padding: 2px; /* Adjust padding as needed */
  cursor: pointer;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

.toggle-label-left{
  display: inline-block;
  padding: 9px; /* Adjust padding as needed */
  font-size: 15px; /* Adjust font size as needed */
  color: #171616; /* Text color */
}
.toggle-label-right {
  display: inline-block;
  padding: 9px; /* Adjust padding as needed */
  font-size: 15px; /* Adjust font size as needed */
  color: #171616; /* Text color */
}

.toggle-slider {
  position: absolute;
  top: 1px;
  left: 2px;
  width: calc(50% - 3px);
  height: 100%;
  background-color: rgba(255, 255, 255, 0.516); /* Slider color */
  border-radius: 2px; /* Rounded corners */
  transition: 0.5s; /* Smooth animation */
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

.toggle-input {
  display: none;
}

.toggle-input:checked + .toggle-slider {
  transform: translateX(calc(100% - 4px));
}

.error-message {
  color: red;
  font-size: 14px;
  margin-left: 1px;
}

.toggle-container {
  margin-right: 12px;
}

.navbar-select-container {
  margin-right: 12px;
  display: flex;
}
</style>
