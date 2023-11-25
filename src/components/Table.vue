<template>
<table id="table" class="table" aria-describedby="fan inputs">
	<th></th>
	<tbody>
		<!-- 9x9 grid of inputs using v-for -->
		<tr v-for="row in 9" :key="row">
			<td v-for="col in 9" :key="col">
				<input
					type="text"
					:ref="`r${row}c${col}`"
					:id="`r${row}c${col}`"
					v-model="grid[9*(row - 1) + col - 1]"
					:class="{ 'red': grid[9*(row - 1) + col - 1] == 1, 'green': grid[9*(row - 1) + col - 1] == 2, 'blue': grid[9*(row - 1) + col - 1] == 3 }"
				/>
			</td>
		</tr>
	</tbody>
</table>

<table class="table" aria-describedby="fan display">
	<th></th>
	<tbody>
		<!-- 9x9 grid of inputs using v-for -->
		<tr v-for="row in 9" :key="row">
			<td v-for="col in 9" :key="col">
				<span
					:ref="`r${row}c${col}`"
					:id="`r${row}c${col}`"
					v-text="grid[9*(row - 1) + col - 1]"
					/>
			</td>
		</tr>
	</tbody>
</table>
</template>

<script lang="ts">
export default {
	data() {
		return {
			grid: Array<Number>(81).fill(0),
		};
	},
	mounted() {
		console.log(this.grid);
	},
	onUpdated() {
		console.log(this.grid);
	},
};
</script>

<style scoped>
* { box-sizing: border-box; }
input {
    background-color: white;
	box-shadow: none;
	width: 72px;
	padding: 12px 4px;
	text-align: end;
}

table {
    border: 1px solid black; 
    justify-content: center;
	border-spacing: 0;
}

th {
	display: none;
}

tr {
    border: 1px solid black;
}

td {
    border: 1px solid black; 
	text-align: center;
	padding: none;
}

[data-color="red"] {
  background-color: color(xyz 45 20 0);
}

[data-color="green"] {
  background-color: color(xyz-d50 0.3 80 0.3);
}

[data-color="blue"] {
  background-color: color(xyz-d65 5 0 50);
}

.red {
	background-color: color(xyz 45 20 0);
}

.green {
	background-color: color(xyz-d50 0.3 80 0.3);
}

.blue {
	background-color: color(xyz-d65 5 0 50);
}
</style>