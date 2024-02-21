<template>
    <div>
        <RouterLink to="/" class="back-link">
            <!-- <img src="../assets/chevron-left.svg" alt="Back" /> -->
            <p>Back</p>
        </RouterLink>
        <h1>Help</h1>
        <p>Help page</p>
        
    </div>
</template>

<style>
.back-link {
    display: flex;
    flex-direction: column;
    /* align-items: center; */
    /* gap: 5px; */
    /* margin-bottom: 20px; */
    text-decoration: none;
    color: black;

    img: {
        height: 20px;
    }
}
</style>
