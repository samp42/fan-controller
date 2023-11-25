<script setup lang="ts">
import Table from "./components/Table.vue";
import Navbar from "./components/Navbar.vue";
</script>

<template >
<Navbar/>
  <h2>PatternName</h2>
  <div class="row">
  <div class="column left" style="overflow-x:auto;">
  <form>
  <Table></Table>
  <button type="reset">Clear</button>
  </form>
</div>
<div class="column right">
  <label> Fan speed:</label>
  <input type="text" placeholder="Percentage"> 
  <div>
  <input type="checkbox" placeholder="Percentage"> <label> All fans</label>
  </div>
  <button type="button">Enter</button>
  <div>
  <label> Runtime:</label>
  <input type="text" placeholder="Time"> 
  </div>  
  <div class="button">
  <button type="button" >Connect</button>
  <button type="button">run/pause</button>
  </div>
  </div>
  </div>
</template>

<style>
.column{
  float:left; 
}
.left {
  padding: 10px;
  width: 75%;

}
.right {
  float:left;
  padding: 10px;
  width: 25%;
  justify-content: center;
}
.button{
justify-content: flex-end;
}
.row::after {
  content: "";
  display: table;
  clear: both;
}
</style>
