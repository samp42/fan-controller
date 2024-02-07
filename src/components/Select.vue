<template>
  <div class="select-container">
    <select class="navbar-button">
      <option v-for="(option, index) in selectOptions" :key="index">{{ option }}</option>
    </select>
  </div>
</template>

<style>
/* .select-container {
  position: relative;
  font-family: Arial;
}

.select-container select {
  display: none;
}

.select-selected {
  background-color: DodgerBlue;
}

.select-selected:after {
  position: absolute;
  content: "";
  top: 14px;
  right: 10px;
  width: 0;
  height: 0;
  border: 6px solid transparent;
  border-color: #fff transparent transparent transparent;
} */
</style>

<script setup lang="ts">
// import { onBeforeMount } from 'vue';
const props = defineProps({selectOptions: Array<String>});

// onMounted(() => {
//   console.log(props);
//   console.log(props.selectOptions);
// });

// beforeCreate = () => {
//   console.log(props);
//   console.log(props.selectOptions);
// }
</script>
