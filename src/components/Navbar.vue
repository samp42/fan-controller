<template>
    <ul>
  <li><a class="dropdown">File</a>
       <div class = "dropContent">
        <p>Load File</p>
        <p>Save Pattern</p>
        <p>Patterns</p>
       </div>
</li>
  <li><a >Test</a></li>

</ul>
  </template>

<style>
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

li a {
  display: block;
  color: white;
  text-align: center;
  padding: 14px 16px;
  text-decoration: none;
}

li a:hover {
  background-color: #111;
}
.dropdown:hover {
  position: relative;
  display: inline-block;
}

.dropContent {
  display: none;
  position: absolute;
  background-color: #f9f9f9;
  min-width: 160px;
  box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
  padding: 12px 16px;
  z-index: 1;
}

.dropdown:hover .dropdown-content {
  display: block;
}

</style>