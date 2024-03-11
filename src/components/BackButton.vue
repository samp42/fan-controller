<template>
    <RouterLink to="/" class="back-link" @mouseenter="toggleHover" @mouseleave="toggleHover">
        <div class="back-link-inner">
            <img v-if="!isHovered" src="../assets/chevron-left.svg" alt="Back" class="chevron"/>
            <img v-else src="../assets/chevron-left-hover.svg" alt="Back" class="chevron"/>
            <p class="back-label">Back</p>
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
    padding: 0 4px;
}

.chevron {
    height: 24px;
}

.back-label {
    margin: 4px;
}
</style>
