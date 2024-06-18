<template>
  <div class="select-container">
    <select class="select-elem" v-model="selectedPort">
      <option v-for="(option, index) in selectOptions" :key="index">{{ option }}</option>
    </select>
  </div>
</template>

<style>
.select-elem {
  min-width: 200px;
}
</style>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useGridStore } from "../store";
defineProps({ selectOptions: Array<String> });

const selectedPort = ref<string | null>(null);

watch(selectedPort, () => {
  const grid = useGridStore();
  grid.port = selectedPort.value!;
});
</script>
