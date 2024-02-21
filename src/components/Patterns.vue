<template>
    <h1>Patterns</h1>
    <div class="update">
        <div class = "page">
                <a target="_blank" href="">
                <img src="./images/alt.png" alt="alternate rows" width="400" height="400">
                </a>
                <div class="name">Alternate rows</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
                <a target="_blank" href="">
                <img src="./images/row off.png" alt="gradient" width="400" height="400">
                </a>
                <div class="name">Gradient</div>
        </div>
    </div>
    <div class="update">
        <div class = "page">
                <a target="_blank" href="">
                <img src="./images/alt.png" alt="alternate rows" width="400" height="400">
                </a>
                <div class="name">Alternate rows</div>
        </div>
    </div>
    </template>
    
    <style>
  *{ box-sizing: border-box; }
  .page{
      border: 1px solid #e1d9d900;
  }
  .page:hover{
      border: 1px solid #060000
  }
  .page img{
    display: block;
    margin-left: auto;
    margin-right: auto;
    width: 80%;
    height: auto;  
  }
  .name{
      text-align:center;
      padding: 5px; 
  }
  .update {
  float: left;
  width: 25%;
  padding: 0 5px; 
  }
  @media only screen and (max-width: 700px) {
    .update {
      width: 50%;
      margin: 5px 0;
    }
  }
  @media only screen and (max-width: 500px) {
  .update {
    width: 100%; 
  }
  }
  
    </style>