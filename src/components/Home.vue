<script setup lang="ts">
import Table from "./Table.vue";

const defaultPattern = ''; 
</script>

<template>
    <div class="home-layout">
        <!--<input type="text" placeholder="Pattern Name" />-->
        <div class="center">
            <Table class="table" :patternName="defaultPattern"></Table>
        </div>
    </div>
</template>

<style>
.home-layout {
    min-width: 600px;  
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    margin-top: 24px;
}

.table-container {
    width: 60%;
    margin-top: 24px;
    display: center; 
    justify-content: center;
    align-items: center;
    .table {
        width: 100%;
         height: 100%
    }
}
</style>
