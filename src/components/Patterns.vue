<template>
  <div>
    <BackButton />
  </div>
  <h1>Patterns</h1>
  <div class="layout">
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Alternate rows' } }">
          <img :src="alt_row" alt="alternate rows" />
        </RouterLink>
        <div class="name" id="alt_r">Alternate rows</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Alternate columns' } }">
          <img :src="alt_col" alt="alternate columns" />
        </RouterLink>
        <div class="name" id="alt_c">Alternate columns</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Gradient' } }">
          <img :src="gradient" alt="gradient" />
        </RouterLink>
        <div class="name" id="grad">Gradient</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Row(s) on/off' } }">
          <img :src="row_on" alt="row on" />
        </RouterLink>
        <div class="name" id="row_on">Row(s) on/off</div>
      </div>
    </div>
    <!-- <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Row off' } }">
          <img :src="row_off" alt="row off" />
        </RouterLink>
        <div class="name" id="row_off">Row off</div>
      </div>
    </div> -->
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Column(s) on/off' } }">
          <img :src="col_on" alt="column on" />
        </RouterLink>
        <div class="name" id="col_on">Column(s) on/off</div>
      </div>
    </div>
    <!-- <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Column off' } }">
          <img :src="col_off" alt="column off" />
        </RouterLink>
        <div class="name" id="col_off">Column off</div>
      </div>
    </div> -->
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Jet Flow' } }">
          <img :src="middle_on" alt="jet flow" />
        </RouterLink>
        <div class="name" id="jet_flow">Jet Flow</div>
      </div>
    </div>
    <!-- <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Middle off' } }">
          <img :src="middle_off" alt="middle off" />
        </RouterLink>
        <div class="name" id="mid_off">Middle off</div>
      </div>
    </div> -->
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Checkerboard' } }">
          <img :src="checkerboard" alt="checkerboard" />
        </RouterLink>
        <div class="name" id="checker">Checkerboard</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Random' } }">
          <img :src="random" alt="random" />
        </RouterLink>
        <div class="name" id="random">Random</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Grid' } }">
          <img :src="grid" alt="grid" />
        </RouterLink>
        <div class="name" id="grid">Grid</div>
      </div>
    </div>
    <div class="update">
    <div class="page">
      <RouterLink :to="{ path: '/', query: { pattern: 'Gaussian' } }">
        <img :src="gaussian" alt="gaussian" />
      </RouterLink>
      <div class="name" id="grid">Gaussian</div>
    </div>
  </div>
  </div>
</template>


<style scoped>
* {
  box-sizing: border-box;
}

.layout {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  margin-left: 10px;
  margin-right: 10px;
}

.page {
  overflow: auto;
}

.page:hover {
  border: 1px solid #060000;
}

.page img {
  display: block;
  margin: 0 auto;
  width: 100%;
  height: auto;
}

.name {
  text-align: center;
  padding: 5px;
}

.update {
  width: 23%;
  margin-bottom: 20px;
}

@media only screen and (max-width: 800px) {
  .update {
    width: 48%;
  }
}

@media only screen and (max-width: 400px) {
  .update {
    width: 100%;
  }
}

.back-link {
  display: flex;
  flex-direction: column;
  text-decoration: none;
  color: black;
}
</style>

<script setup lang="ts">
import { RouterLink } from "vue-router";
import BackButton from "./BackButton.vue";
import alt_row from "../assets/images/alt_row.png";
import alt_col from "../assets/images/alt_col.png";
import col_on from "../assets/images/col_on.png";
import checkerboard from "../assets/images/checkerboard.png";
import row_on from "../assets/images/row_on.png";
import gradient from "../assets/images/gradient.png";
import random from "../assets/images/random.png";
import grid from "../assets/images/grid.png";
import middle_on from "../assets/images/middle_on.png";
import gaussian from "../assets/images/gaussian.png";
</script>
