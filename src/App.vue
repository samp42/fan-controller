<script setup lang="ts">
import Table from "./components/Table.vue";
import Navbar from "./components/Navbar.vue";

function clear(){
  Table.fill(0);
}
</script>

<template >
<Navbar/>
  <h2 class="header">PatternName</h2>
  <br>
  <div style="overflow-x:auto;">
  <form>
    <div class ="center">
    <Table></Table>
    <div class="button">
      <button onclick="clear()">Clear</button>
    </div>
    </div>
  </form>
</div>

 <br>  
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
