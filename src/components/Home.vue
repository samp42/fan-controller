<script setup lang="ts">
import Table from "./Table.vue";
import { useGridStore } from '../store';

const grid = useGridStore(); 
</script>

<template>
    <div style="overflow-x:auto;">
        <!--<input type="text" placeholder="Pattern Name" />-->
        <div class="center">
            <Table class="table"></Table>
        </div>
    </div>
</template>

<style>
	* { box-sizing: border-box; }

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
