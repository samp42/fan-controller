<template>
  <div style="overflow-x: auto">
    <!--<input type="text" placeholder="Pattern Name" />-->
    <div class="center">
      <Table class="table" :patternName="defaultPattern"></Table>
    </div>
  </div>
</template>

<script setup lang="ts">
import Table from "./Table.vue";
// import { onMounted, on } from "vue";
// import mitt from "mitt";

// const emitter = mitt();
const defaultPattern = "";

// onMounted(() => {
//   // on("run_pattern", () => {
//   //   // Handle the emitted event
//   //   console.log("run Pattern");
//   // });
//   this.emitter.on("run", () => {
//     console.log("RUN FROM MITT");
//   })
// });
</script>

