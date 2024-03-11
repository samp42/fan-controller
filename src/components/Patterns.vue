<script lang="ts"> 
import { RouterLink } from "vue-router";
import BackButton from "./BackButton.vue";

</script>
<template>
 <div >
        <BackButton></BackButton>  
      </div>   
    <h1>Patterns</h1>
    <div class="update">
        <div class = "page">
                <RouterLink to="/" >
                <img src="src/images/alt.png" alt="alternate rows" width="400" height="400">
                </RouterLink>
                <div class="name">Alternate rows</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <Router-Link to="/" >
                <img src="src/images/gradient.png" alt="gradient" width="400" height="400">
              </Router-Link>
                <div class="name">Gradient</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <Router-Link to="/">
                <img src="src/images/row off.png" alt="row off" width="400" height="400">
              </Router-Link>
                <div class="name">Row off</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <Router-Link to="/">
                <img src="src/images/col off.png" alt="row off" width="400" height="400">
              </Router-Link>
                <div class="name">column off</div>
        </div>
    </div>
    <!--<div class="update">
        <div class = "page">
          <Router-Link to="/">
                <img src="src/images/row off.png" alt="row off" width="400" height="400">
              </Router-Link>
                <div class="name">alternate columns</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <Router-Link to="/">
                <img src="src/images/row off.png" alt="row off" width="400" height="400">
              </Router-Link>
                <div class="name">Middle</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
          <Router-Link to="/">
                <img src="src/images/row off.png" alt="row off" width="400" height="400">
              </Router-Link>
                <div class="name">CheckerBoard</div>
        </div>
    </div>

    <div class="update">
        <div class = "page">
          <Router-Link to="/">
                <img src="src/images/row off.png" alt="row off" width="400" height="400">
              </Router-Link>
                <div class="name">Grid</div>
        </div>
    </div>-->
    </template>
    
    <style>
  *{ box-sizing: border-box; }

  .page{
      border: 1px solid #e1d9d900;
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
  float: left;
  width: 25%;

  }
  @media only screen and (max-width: 700px) {
    .update {
      width: 50%;
      float: left;
    }
  }
  @media only screen and (max-width: 500px) {
  .update {
    width: 100%; 
    float: left;
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