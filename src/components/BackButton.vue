<template>
    <RouterLink to="/" class="back-link">
        <div class="back-link-inner" @mouseover="toggleHover" @mouseout="toggleHover">
            <img v-if="!isHovered" src="../assets/chevron-left.svg" alt="Back" class="chevron"/>
            <img v-else src="../assets//chevron-left-hover.svg" alt="Back" class="chevron"/>
            <p>Back</p>
        </div>
    </RouterLink>
</template>

<script setup lang="ts">
import { ref } from 'vue';

let isHovered = ref(false);

function toggleHover(): void {
    isHovered.value = !isHovered.value;
}

</script>

<style>
.back-link {
    width: fit-content;
}

.back-link-inner {
    display: flex;
    flex-direction: row;
    align-items: center;
    margin: 4px;
}

.chevron {
    height: 24px;
}
</style>
