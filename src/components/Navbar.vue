<template>
  <div class="navbar-container">
    <div class="navbar-elements">
      <div class="dropdown">
        <button class="navbar-button dropButton">File</button>
        <div class="dropContent">
          <button class="dropdown-button">Load File</button>
          <button class="dropdown-button">Patterns</button>
          <button class="dropdown-button">Save Pattern</button>
        </div>
      </div>
      <button class="navbar-button">Help</button>
    </div>
    <div class="navbar-select-container">
      <Select></Select>
    </div>
    <button @click="list()" class="navbar-button">List</button>
  </div>
</template>

<script lang="ts" setup>
import { invoke } from "@tauri-apps/api";
import Select from "./Select.vue";

function list() {
  invoke('list_serial_ports').then((p: any) => {
    console.log(p);
  });
  // console.log("list");
}
</script>

<style>
.navbar-container {
  overflow: hidden;
  background-color: #333;
  height: 54px;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
}

.navbar-elements {
  display: flex;
  flex-direction: row;
}

.navbar-button {
  background: none;
  border: none;
  box-shadow: none;
  border-radius: 0;
  color: white;
  height: 100%;
}

.navbar-button:hover {
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
}
</style>