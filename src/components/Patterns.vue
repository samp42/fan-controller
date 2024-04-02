<script lang="ts">
import { RouterLink } from "vue-router";
import BackButton from "./BackButton.vue";
</script>
<template>
  <div>
    <!-- <RouterLink :to="{ path: '/', query: { pattern: '' } }" class="back-link">
      <p>Back</p>
    </RouterLink> -->
    <BackButton />
  </div>
  <h1>Patterns</h1>
  <div class="layout">
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Alternate rows' } }">
          <img src="src/images/alt row.png" alt="alternate rows" />
        </RouterLink>
        <div class="name" id="alt_r">Alternate rows</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Alternate columns' } }">
          <img src="src/images/alt col.png" alt="alternate columns" />
        </RouterLink>
        <div class="name" id="alt_c">Alternate columns</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Gradient' } }">
          <img src="src/images/gradient.png" alt="gradient" />
        </RouterLink>
        <div class="name" id="grad">Gradient</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Row on' } }">
          <img src="src/images/row on.png" alt="row on" />
        </RouterLink>
        <div class="name" id="row_on">Row on</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Row off' } }">
          <img src="src/images/row off.png" alt="row off" />
        </RouterLink>
        <div class="name" id="row_off">Row off</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Column on' } }">
          <img src="src/images/col on.png" alt="column off" />
        </RouterLink>
        <div class="name" id="col_on">Column on</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Column off' } }">
          <img src="src/images/col off.png" alt="column off" />
        </RouterLink>
        <div class="name" id="col_off">Column off</div>
      </div>
    </div>

    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Middle on' } }">
          <img src="src/images/middle on .png" alt="middle on" />
        </RouterLink>
        <div class="name" id="mid_on">Middle on</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Middle off' } }">
          <img src="src/images/middle off.png" alt="middle off" />
        </RouterLink>
        <div class="name" id="mid_off">Middle off</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Checkerboard' } }">
          <img src="src/images/checkerboard.png" alt="checkerboard" />
        </RouterLink>
        <div class="name" id="checker">Checkerboard</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Random' } }">
          <img src="src/images/random.png" alt="random" />
        </RouterLink>
        <div class="name" id="random">Random</div>
      </div>
    </div>
    <div class="update">
      <div class="page">
        <RouterLink :to="{ path: '/', query: { pattern: 'Grid' } }">
          <img src="src/images/grid.png" alt="grid" />
        </RouterLink>
        <div class="name" id="grid">Grid</div>
      </div>
    </div>
  </div>

  <div class="update">
    <div class="page">
      <RouterLink :to="{ path: '/', query: { pattern: 'Gaussian' } }">
        <img src="src/images/gaussian.png" alt="gaussian" />
      </RouterLink>
      <div class="name" id="grid">Gaussian</div>
    </div>
  </div>
</template>

/*
<style>
* {
  box-sizing: border-box;
}

.layout {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
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
