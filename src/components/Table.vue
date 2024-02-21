<template>
<div class= "layout">
	<div class = "square">
		<div class = "content">
			<table id="table" class="table" aria-describedby="fan inputs">
				<th></th>
				<tbody>
					<tr v-for="row in 9" :key="row">
						<td v-for="col in 9" :key="col">
							<input
								type="text"
								:ref="`r${row}c${col}`"
								:id="`r${row}c${col}`"
								v-model="grid[9*(row - 1) + col - 1].value"
								:class="getColor(row, col)"
								style="overflow:visible;"
								:disabled="false"
								@input="checkInput(row,col)"
							/>
						</td>
					</tr>
				</tbody>
			</table>
		</div>
	</div>
</div>
<div class = "select">
<button type="button" @click="clear()">Clear</button>
<!--<button type="button" @click="randomFill()">Random Fill</button>
<button type="button" @click="checkerBoard(3, true)">checkboard even</button>
<button type="button" @click="checkerBoard(3, false)">reverse checkboard odd</button>
<button type="button" @click="singleRow(2, 50, true)">row on</button>
<button type="button" @click="singleRow(2, 50,false)">row off</button>
<button type="button" @click="singleCol(2, 50, true)">col on</button>
<button type="button" @click="singleCol(2, 50, false)">row on</button>
<button type="button" @click="altRows(2, 50, true)">alt rows ON/OFF</button>
<button type="button" @click="altRows(2, 50, false)">alt rows OFF/ON</button>
<button type="button" @click="altCols(2, 50, true)">alt col ON/OFF</button>
<button type="button" @click="altCols(2, 50, false)">alt col OFF/ON</button>
<button type="button" @click="middle(50, true)">middle ON</button>
<button type="button" @click="middle(50, false)">middle OFF</button>
<button type="button" @click="gridPattern(50,1,2, false)">grid 2 on 2</button>
<button type="button" @click="gradient(5, 95, true, true)">row gradient</button>
<button type="button" @click="gradient(0, 81, true, false)">inv row gradient</button>
<button type="button" @click="gradient(3, 99, false, true)">col gradient</button>
<button type="button" @click="gradient(14, 78, false, false)">inv col gradient</button>
-->
</div>

</template>

<script lang="ts">
export default {
	data() {
		return {
			//grid: Array<Number>(81).fill(0)
				grid: Array(81).fill(0).map(() => ({ value: 0, disabled: false })),
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
			if(isNaN(gridValue.value)) {
				return '';
			}

			const value = gridValue.value;

			if(value == 0) {
				return 'white';
			}
			else if(value < 0 || value > 100) {
				return 'red';
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
		},
randomFill() {
  this.clear();

  for (let i = 0; i < 9; i++) {
    for (let j = 0; j < 9; j++) {
      this.grid[9 * i + j] = { value: Math.floor(Math.random() * 101), disabled: false };
    }
  }
},
checkerBoard(size: number, rev: boolean) {
  this.clear();
  for (let i = 0; i < 9; i++) {
    for (let j = 0; j < 9; j++) {
      const isEvenRow = Math.floor(i / size) % 2 === 0;
      const isEvenCol = Math.floor(j / size) % 2 === 0;

      const isEvenSquare = (isEvenRow && isEvenCol) || (!isEvenRow && !isEvenCol);

      this.grid[9 * i + j] = { value: isEvenSquare ? (rev ? 100 : 0) : (rev ? 0 : 100), disabled: isEvenSquare };
    }
  }
},

  singleRow(row: number, val: number, on: boolean){	//NUMBERING FROM 0 TO 8 OR FROM 1 TO 9 (RN: 1 TO 0)
	this.grid = this.grid.map((cell, index) => ({
    value: on ? (Math.floor(index / 9) === row ? val : 0) : (Math.floor(index / 9) === row ? 0 : val),
    disabled: cell.disabled
  }));  },
  
  singleCol(col: number, val: number, on: boolean){
	this.grid = this.grid.map((cell, index) => ({
    value: on ? (index % 9 === col - 1 ? val : 0) : (index % 9 === col - 1 ? 0 : val),
    disabled: cell.disabled
  }));  },

  altRows(lines: number, val: number, on: boolean) {
	this.grid = this.grid.map((cell, index) => ({
    value: (Math.floor(index / 9 / lines) % 2 === (on ? 0 : 1)) ? val : 0,
    disabled: cell.disabled
  }));
},
altCols(cols: number, val: number, on: boolean) {
  this.grid = this.grid.map((cell, index) => ({
    value: (Math.floor(index % 9 / cols) % 2 === (on ? 0 : 1)) ? val : 0,
    disabled: cell.disabled
  }));
},
middle(val: number,on: boolean) {
  const middleIndex = 40; // Index of the middle cell in a 9x9 grid

  if (on) {
    this.grid = this.grid.map((cell, index) => ({
      value: index === middleIndex ? val : 0,
      disabled: cell.disabled,
    }));
  } else {
    this.grid = this.grid.map((cell, index) => ({
      value: index === middleIndex ? 0 : val,
      disabled: cell.disabled,
    }));
  }
},
gridPattern(val: number, lines: number, square: number, on: boolean) {
  this.clear(); // Clear the grid

  const size = 9; // Size of the grid

  for (let i = 0; i < size; i++) {
    for (let j = 0; j < size; j++) {
      const isSquare = Math.floor(i / square) % 2 === 0 && Math.floor(j / square) % 2 === 0;
      const isLine = (Math.floor(i / lines) % 2 === 0 || Math.floor(j / lines) % 2 === 0) && !isSquare;

      const displayValue = on ? (isSquare ? val : 0) : (isLine ? val : 0);

      this.grid[size * i + j] = { value: displayValue, disabled: false };
    }
  }
},
gradient(min: number, max: number, row: boolean, on: boolean) {
  this.clear(); // Clear the grid

  const size = 9; // Size of the grid

  for (let i = 0; i < size; i++) {
    for (let j = 0; j < size; j++) {
      const position = row ? i : j;
      const invertedPosition = row ? size - 1 - i : size - 1 - j;

      const gradientValue = Math.floor((max - min) * (position / (size - 1)) + min);
      const invertedGradientValue = Math.floor((max - min) * (invertedPosition / (size - 1)) + min);

      const displayValue = on ? gradientValue : invertedGradientValue;

      this.grid[size * i + j] = { value: displayValue, disabled: false };
    }
  }
}

,
	clear(){
		this.grid.fill({ value: 0, disabled: false });
	}, 
	checkInput(row: number, col: number) {
    const index = 9 * (row - 1) + col - 1;
    const inputValue = this.grid[index].value;

    // Ensure the input value is between 0 and 100
    if (isNaN(inputValue) || inputValue < 0 || inputValue > 100) {
		const numOnly = inputValue.toString().replace(/[^0-9]/g, ''); 
        this.grid[index] = { ...this.grid[index], value: numOnly, disabled: false };
    }
}

}

};


</script>

<style scoped>
* { box-sizing: border-box; }

.layout{
	position: relative;
	width:100%;
}
.square{
	padding-bottom: 100%;
}
.content {
	position: absolute;
	top: 0; 
	bottom: 0; 
	left: 0; 
	right: 0; 
}
input {
    background-color: white;
	border-radius: 0;
	border: 0;
	text-align: center;
	box-shadow: none;
	width: 100%;
	height:100%;
	top:50%;
	left:50%;
}
table{
	border: 1px solid black; 
    justify-content: center;
	border-spacing: 0;
	width:100%;
	height:100%;
	padding: 0; 
}
th { display: none;}
tr { border: 1px solid black;}
td {
    border: 1px solid black; 
	text-align: center;
	padding: 0;
}
.button{
justify-content: flex-end;
text-align: center;
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
.red{
	background-color: rgba(213, 20, 20, 0.814);
}
</style>
