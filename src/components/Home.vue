<script setup lang="ts">
import { RouterLink } from "vue-router";
import Table from "./Table.vue";
</script>

<template>
    <RouterLink to="/help" class="help-link">Help</RouterLink>
    <div class="home-layout">
        <!--<input type="text" placeholder="Pattern Name" />-->
        <div class="table-container">
            <Table class="table"></Table>
        </div>
    </div>
</template>

<style>
.help-link {
    text-decoration: none;
    color: black;
    position: absolute;
    right: 8px;
    top: 64px;
    padding: 6px 18px;
    border-radius: 4px;
    background-color: #666;
}

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
