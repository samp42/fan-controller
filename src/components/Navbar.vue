<template>
  <ul>
    <li class="dropdown">
      <a class="dropButton">File</a>
      <div class = "dropContent">
        <button class="navbar-button">Load File</button>
        <button class="navbar-button">Patterns</button>
        <button class="navbar-button">Save Pattern</button>
      </div>
    </li>
    <li><a>Connect</a></li>
    <li><a >Test</a></li>
  </ul>
</template>

<style>
.navbar-button {
  width: 100%;
  background: none;
  border: none;
  box-shadow: none;
  text-align: left;
  border-radius: 0;
}

.navbar-button:hover {
  background-color: #eee;
}

.navbar-button:active {
  background-color: #aaa;
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