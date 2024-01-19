<script setup lang="ts">
import Table from "./components/Table.vue";
import Navbar from "./components/Navbar.vue";

function Clear() {
  Table.fill(0);
}
</script>

<template >
<Navbar/>
  <h2 class="header">PatternName</h2>
  <br>
  <div class="row">
  <div class="column" style="overflow-x:auto;">
  <form>
  <Table></Table>
  <div class="button">
  <button onclick="Clear()">Clear</button>
</div>
  </form>
 <br>  
  <div>
  <div class="header">
  <label > Runtime:</label>
  <input type="text" placeholder="Time"> 

  <!--<div class="button">-->
    <button type="button">run/pause</button>
  </div>
    <br>
  </div>
 <!-- </div>  -->
</div>
 <!--<div class="column right">
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
  </div>-->
  
  </div>
</template>

<style>
.header {
text-align: center;
}
.column{
  float:left; 
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
text-align: right;
}
.row::after {
  content: "";
  display: table;
  clear: both;
}
</style>
