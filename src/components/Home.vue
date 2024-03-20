<script setup lang="ts">
import { RouterLink } from "vue-router";
import Table from "./Table.vue";

const defaultPattern = ''; 
</script>

<template>
    <RouterLink to="/help" class="help-link">Help</RouterLink>
    <div style="overflow-x:auto;">
        <!--<input type="text" placeholder="Pattern Name" />-->
        <div class="center">
            <Table class="table" :patternName="defaultPattern"></Table>
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
