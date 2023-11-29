<template>
<table id="table" class="table" aria-describedby="fan inputs">
	<th></th>
	<tbody>
		<tr v-for="row in 9" :key="row">
			<td v-for="col in 9" :key="col">
				<input
					type="text"
					:ref="`r${row}c${col}`"
					:id="`r${row}c${col}`"
					v-model="grid[9*(row - 1) + col - 1]"
					:class="getColor(row, col)"
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
	methods: {
		getColor(row: number, column: number) {
			const gridValue = this.grid[9 * (row - 1) + column - 1];

			// check if gridValue contains only numbers
			if(isNaN(gridValue as number)) {
				return '';
			}

			const value = gridValue as number;

			if(value < 25) {
				return 'green';
			} else if(value < 50) {
				return 'yellow';
			} else if(value < 75) {
				return 'orange';
			}

			return 'red';
		}
	}
};
</script>

<style scoped>
* { box-sizing: border-box; }
input {
    background-color: white;
	border-radius: 0;
	border: 0;
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
	padding: 0;
}

.green {
	background-color: green;
}

.yellow {
	background-color: yellow;
}

.orange {
	background-color: orange;
}

.red {
	background-color: red;
}
</style>