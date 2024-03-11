<script setup lang="ts">
import Table from "./Table.vue";
</script>

<template>
    <div class="home-layout">
        <!--<input type="text" placeholder="Pattern Name" />-->
        <div class="table-container">
            <Table class="table"></Table>
        </div>
    </div>
</template>

<style>
.home-layout {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 100%;
    margin-top: 24px;
}

.table-container {
    width: 60%;
    margin-top: 24px;
    
    .table {
        /* width: 60%; */
        /* height:  */
    }
}
</style>
