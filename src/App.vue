<script setup lang="ts">
import Header from "./components/Header.vue";
import { RouterView } from "vue-router";
</script>

<template>
<Header/>
<!-- <h2 class="header">PatternName</h2>
<div style="overflow-x:auto;">
  <form>
    <div class="center">
    <Table></Table>
    <div class="">
      <button @click="clear()">Clear</button>
    </div>
    </div>
  </form>
</div> -->
<RouterView></RouterView>
</template>

<style>
.layout {  
  min-width:450px; 
}
.header {
text-align: center;
}
.center {
  min-width:600px;
  margin: auto;
  width: 40%;
  text-align: center;
  box-sizing: border-box;
}
.column{
  float:center; 
}
.button{
justify-content: flex-end;
text-align: center;
}
.row::after {
  content: "";
  display: table;
  clear: both;
}
</style>
