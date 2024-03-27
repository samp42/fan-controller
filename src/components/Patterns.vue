<script lang="ts"> 
import { RouterLink } from "vue-router";
import BackButton from "./BackButton.vue";

</script>
<template>
    <div class="page-title">
      <BackButton class="back-button"></BackButton>
      <h1>Patterns</h1>
    </div>
    <div class="layout">
    <div class="update">
        <div class = "page">
          <RouterLink :to="{path: '/', query: {pattern: 'Alternate rows'}}">
          <img src="src/images/alt row.png" alt="alternate rows" >
          </RouterLink>
          <div class="name" id="alt_r">Alternate rows</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <RouterLink :to="{path: '/', query: {pattern: 'Alternate columns'}}">
          <img src="src/images/alt col.png" alt="row off" >
          </RouterLink>
          <div class="name" id="alt_c" >Alternate columns</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <RouterLink :to="{path: '/', query: {pattern: 'Gradient'}}" >
          <img src="src/images/gradient.png" alt="gradient" >
          </RouterLink>
          <div class="name" id="grad">Gradient</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <RouterLink :to="{path: '/', query: {pattern: 'Row on'}}">
          <img src="src/images/row on.png" alt="row on" >
          </RouterLink>
          <div class="name" id="row_on">Row on</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <RouterLink :to="{path: '/', query: {pattern: 'Row off'}}">
          <img src="src/images/row off.png" alt="row off" >
          </RouterLink>
          <div class="name" id="row_off">Row off</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <RouterLink :to="{path: '/', query: {pattern: 'Column on'}}">
          <img src="src/images/col on.png" alt="col off" >
          </RouterLink>
          <div class="name" id="col_on">Column on</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <RouterLink :to="{path: '/', query: {pattern: 'Column off'}}">
          <img src="src/images/col off.png" alt="col off" >
          </RouterLink>
          <div class="name" id="col_off">Column off</div>
        </div>
    </div>
    
    <div class="update">
        <div class = "page">
          <RouterLink :to="{path: '/', query: {pattern: 'Middle on'}}">
          <img src="src/images/middle on .png" alt="row off" >
          </RouterLink>
          <div class="name" id="mid_on">Middle on</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <RouterLink :to="{path: '/', query: {pattern: 'Middle off'}}">
          <img src="src/images/middle off.png" alt="row off" >
          </RouterLink>
          <div class="name" id="mid_off">Middle off</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <RouterLink :to="{path: '/', query: {pattern: 'Checkerboard'}}">
          <img src="src/images/checkerboard.png" alt="row off" >
          </RouterLink>
          <div class="name" id="checker">Checkerboard</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <RouterLink :to="{path: '/', query: {pattern: 'Random'}}">
          <img src="src/images/random.png" alt="random" >
          </RouterLink>
          <div class="name" id="random">Random</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <RouterLink :to="{path: '/', query: {pattern: 'Grid'}}">
          <img src="src/images/row off.png" alt="row off" >
          </RouterLink>
          <div class="name" id="grid">Grid</div>
        </div>
    </div>
  </div>
  
</template>
    
<style scoped>
.back-button {
  /* TODO */
}

  *{ box-sizing: border-box; }

  .page{
     /* border: px solid #e1d9d900;*/
      float: left; 
      overflow: auto;
  }
  .page:hover{
      border: 1px solid #060000
  }
  .page img{
    display: block;
    margin-left: auto;
    margin-right: auto;
    width: 90%;
    height: auto;  
  }
  .name{
      text-align:center;
      padding: 5px; 
  }
  .update {
  width: 20%; /* Adjusted width to ensure four items fit in a row */
   /* Added margin for spacing between items */
  float: left; /* Allow items to float next to each other */
  
  /* Optional: Adjusting for responsiveness */
  @media only screen and (max-width: 800px) {
    width: 50%; /* Two items per row on smaller screens */
    
  }
  @media only screen and (max-width: 400px) {
    width: 100%; /* Single item per row on even smaller screens */
  }
}

  .back-link {
    display: flex;
    flex-direction: column;
    /* align-items: center; */
    /* gap: 5px; */
    /* margin-bottom: 20px; */
    text-decoration: none;
    color: black;
}
    </style>