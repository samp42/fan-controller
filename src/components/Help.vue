<script setup lang="ts">
import BackButton from "./BackButton.vue";
</script>

<template>
  <div>
    <!-- <RouterLink to="/" class="back-link"> -->
    <!-- <img src="../assets/chevron-left.svg" alt="Back" /> -->
    <!-- <p>Back</p> -->
    <!-- </RouterLink> -->
    <BackButton class="back-button"></BackButton>
    <h1>Help</h1>
    <div class="help-content-container">
      <h2>Running a pattern</h2>
      <p>
        To run a pattern, either a dynamic (CSV) pattern or a static pattern and a port
        must be selected. This section explains how to do both. Once a pattern and a port
        have been selected, you can click on the play button in the header to run the
        pattern. The pattern will run in a loop. To stop the pattern, click on the stop
        button in the header. It may take a few seconds between the time you click on the
        stop button and the time the pattern actually stops running. To restart the
        pattern, simply click on the play button again and the pattern will start from the
        beginning.
      </p>
      <h3>Selecting a port</h3>
      <p>
        To select a port, you can use the dropdown in the header. It may take up to 3
        seconds to see a new device connected to a port to appear in the options. If the
        dropdown is blank, it means that no port has been selected. If you see the pacman,
        it means that it is loading the available ports or no port is available.
      </p>
      <h3>Selecting a pattern</h3>
      <h4>Static pattern</h4>
      <p>
        To select a static pattern, go into the Patterns page by clicking on the link in
        the header. Then, choose the desired pattern. You will be redirected to the home
        page with the grid. You will then be able to select different paramters of the
        patterns like the size and the speed.
      </p>
      <h4>Dynamic pattern</h4>
      <p>To select a dynamic pattern, simply choose a CSV file from the header.</p>
    </div>
  </div>
</template>

<style>
.help-content-container {
  margin: 0 48px;
  text-align: justify;
}
</style>
