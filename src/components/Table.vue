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

			/*if(value <= 0 || value>100) {
				return 'white';
			} else if(value > 0 && value <= 25) {
				return 'green';
			} else if(value > 25 && value <= 50) {
				return 'yellow';
			}
			else if(value > 50 && value <= 75) {
				return 'orange';
			}

			return 'red';*/

			if(value <= 0 || value>100) {
				return 'white';
			} else if(value > 0 && value <= 15) {
				return 'lightBlue';
			} else if(value > 15 && value <= 30) {
				return 'blue';
			} else if(value > 30 && value <= 45) {
				return 'darkBlue';
			} else if(value > 45 && value <= 60) {
				return 'darkPurple';
			} else if(value > 60 && value <= 75) {
				return 'purple';
			}
			else if(value > 75 && value <= 90) {
				return 'darkPink';
			}
			return 'pink';
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
.white {
	background-color: rgb(255, 255, 255);
}
/*.lightBlue {
	background-color: rgb(28, 98, 249);
}
.blue {
	background-color: rgb(23, 57, 230);
}
.darkBlue {
	background-color: rgb(6, 30, 218);
}
.darkPurple{
	background-color: rgb(68, 1, 226);
}
.purple{
	background-color: rgb(129, 4, 255);
}
.darkPink {
	background-color: rgb(169, 2, 247);
}
.pink {
	background-color: rgb(227, 2, 247);
}

*/
.lightBlue {
	background-color: rgba(28, 98, 249, 0.621);
}
.blue {
	background-color: rgba(23, 58, 230, 0.687);
}
.darkBlue {
	background-color: rgba(6, 31, 218, 0.696);
}
.darkPurple{
	background-color: rgba(68, 1, 226, 0.676);
}
.purple{
	background-color: rgba(130, 4, 255, 0.687);
}
.darkPink {
	background-color: rgba(169, 2, 247, 0.644);
}
.pink {
	background-color: rgba(227, 2, 247, 0.667);
}
</style>