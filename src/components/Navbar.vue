<script setup lang="ts">
//import { RouterLink } from "vue-router";
//import Table from "./Table.vue";
</script>

<template>
    <ul>
  <li class= "dropdown"><a class="dropButton">File</a>
       <div class = "dropContent">
        <a href =loadFile() target="_top">Load File</a>
       <!-- <router-link to="/loadFile">Load File</router-link>
        <router-view></router-view>-->
       <!-- <input type='file' accept=".csv" id="UploadFile">-->
        <a href="C:\Users\" target="_blank">Save Pattern</a>
        <RouterLink to="/pattern" class="help-link">Patterns</RouterLink>
       </div>
</li>
  <li><a>Connect</a></li>
  <li><a >Test</a></li>
  <li class="help"><a >Help</a></li>

</ul>
  </template>

<style>
.help-link {
    text-decoration: none;
    color: black;
    position: absolute;
    right: 8px;
    top: 64px;
    padding: 6px 18px;
    border-radius: 4px;
    background-color: #666;
}
ul {
  list-style-type: none;
  margin: 0;
  padding: 0;
  overflow: hidden;
  background-color: #333;
}

li {
  float: left;
}
li a, .dropButton {
  display: block;
  color: white;
  text-align: center;
  padding: 14px 16px;
  text-decoration: none;
}
.help {
  float:right;
}
li a:hover, .dropdown:hover .dropButton {
  background-color: #111;
}

.dropContent {
  display: none;
  position: absolute;
  background-color: #f9f9f9;
  min-width: 160px;
  box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
  z-index: 1;
}

.dropContent a {
  color: black;
  padding: 12px 16px;
  text-decoration: none;
  display: block;
  text-align: left;
}

.dropContent a:hover {background-color: #f1f1f1;}

.dropdown:hover .dropContent {
  display: block;
}

</style>