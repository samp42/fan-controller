<script setup lang="ts">
import Table from "./components/Table.vue";
import Header from "./components/Header.vue";
import { RouterView } from "vue-router";

function clear(){
  Table.fill(0);
}
</script>

<template >
<Header/>
<!-- <h2 class="header">PatternName</h2>
<div style="overflow-x:auto;">
  <form>
    <div class ="center">
    <Table></Table>
    <div class="">
      <button @click="clear()">Clear</button>
    </div>
    </div>
  </form>
</div>  -->
<RouterView></RouterView>
</template>

<style>
template {  min-width:1000px; }
.header {
text-align: center;
}
.center {
  min-width:500px;
  margin: auto;
  width: 50%;
  text-align: center;
  box-sizing: border-box;
}
.column{
  float:center; 
}
.left {
  padding: 50px;
  width: 75%;
}
.right {
  padding:10px;
  width: 25%;
  justify-content: center;
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
