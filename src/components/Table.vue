<template>
	<br>
    <input class ="pattern" id="pattern" type ="text" :value="patternName" placeholder="Pattern Name">  
    <div class= "layout" style= "overflow: auto">
        <div class = "square">
            <div class = "content">
                <table id="table" class="table" aria-describedby="fan inputs" ref="table">
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
    <br>
    <!--    <div v-if="patternName.toLowerCase()  === ''">
        </div>
    -->
    <div v-if="patternName  === 'checkerboard' || patternName  === 'Checkerboard'">
        <label for="size">Size:</label>
        <input class = "patternInput" type="text" id="size" >
        <label for="speed"> Speed:</label>
        <input class = "patternInput" type="text" id="speed" >
        <input class="checkbox" type="checkbox" id="check" >
        <label for="check">Reverse</label>
    </div>

    <div v-else-if="patternName === 'Alternate rows' || patternName === 'alternate rows' || patternName === 'Alternate columns' || patternName === 'alternate columns'">
        <label for="size">Size:</label>
        <input class = "patternInput" type="text" id="size" >
        <label for="speed"> Speed:</label>
        <input class = "patternInput" type="text" id="speed" >  
        <input class="checkbox" type="checkbox" id="reverseAlt" >
        <label for="reverseAlt">Reverse</label>     
    </div>
    <div v-else-if="patternName === 'Row on' || patternName === 'row on' || patternName === 'Row off'|| patternName === 'row off'|| patternName === 'Column on' || patternName === 'column on' || patternName === 'Column off' || patternName === 'column off'">
        <label for="size">Position:</label>
        <input class = "patternInput" type="text" id="size" >
        <label for="speed"> Speed:</label>
        <input class = "patternInput" type="text" id="speed" >
        <input class="checkbox" type="checkbox" id="rowOnOff" >
        <label for="rowOnOff">On/Off</label>
    </div>
    <div v-else-if="patternName === 'gradient' || patternName === 'Gradient'">
        <label for="min">Minimum:</label>
        <input class = "patternInput" type="text" id="min" >
        <label for="max">   Maximum:</label>
        <input class = "patternInput" type="text" id="max" >
        <label for="rowCol">Row/Col:</label>
        <input type="checkbox" id="rowCol" >
        <label for="gradientReverse">Reverse:</label>
        <input type="checkbox" id="gradientReverse" >
    </div>

    <div v-else-if="patternName === 'random' || patternName === 'Random'">
    <!--    <label for="min">Minimum:</label>
        <input class = "patternInput" type="text" id="min" >
        <label for="max">Maximum:</label>
        <input class = "patternInput" type="text" id="max" >
        <label for="speed">Value:</label>
        <input class = "patternInput" type="text" id="speed" >
        <label for="randomReverse">Reverse:</label>
        <input type="checkbox" id="randomReverse" >
    -->
    </div>
    <div v-else-if="patternName === 'Middle on' || patternName === 'middle on' || patternName === 'Middle off' ||patternName === 'middle off'">
        <label for="speed">Speed:</label>
        <input class = "patternInput" type="text" id="speed" >
        <input class="checkbox" type="checkbox" id="middleOnOff" >
        <label for="middleOnOff">On/Off</label>
    </div>

    <div v-else-if="patternName === 'grid' || patternName === 'Grid'">
      <label for="size">Size:</label>
        <input class = "patternInput" type="text" id="size" >
        <label for="speed"> Speed:</label>
        <input class = "patternInput" type="text" id="speed" >  
        <input class="checkbox" type="checkbox" id="grid" >
        <label for="grid">Reverse</label> 
    </div>
    <div v-else-if="patternName === 'gaussian' || patternName === 'Gaussian'">
      <label for="mean">Mean:</label>
        <input class = "patternInput" type="text" id="mean" >
        <label for="sigma"> Sigma:</label>
        <input class = "patternInput" type="text" id="sigma" >   
    </div>
    <br>
    <button type="button" @click="clear()">Clear</button>
    <button type="button" @click="getPattern()">Enter</button>

</template>

    <script lang="ts">  
    import {RouteLocationNormalizedLoaded } from 'vue-router'; 
    import { useGridStore } from '../store';

    //const grid = useGridStore();
    //const patternName = ''; // Assuming it's predefined or comes from props

    export default {
        data() {
            return {
                //grid: Array<Number>(81).fill(0)
                    patternName: '',
                    grid: [] as { value: number; disabled: boolean }[],                    
                };
        },
        mounted() {
            console.log(this.grid);
        },
        onUpdated() {
            console.log(this.grid);
        },
        created() {
        const route = this.$route as RouteLocationNormalizedLoaded;
        this.patternName = route.query.pattern as string;
        const gridStore = useGridStore();
        this.grid = gridStore.grid;
        },
        methods: {
            getValue(){
                var value = (<HTMLInputElement>document.getElementById("pattern")).value; 
                return value; 
            },
            getPattern(){
                var input = (<HTMLInputElement>document.getElementById("pattern")).value.toLowerCase(); 
                switch(input){
                    case "random":
                        this.randomFill() ; 
                        break; 
                    case "gradient":
                        var check = (<HTMLInputElement>document.getElementById("gradientReverse")).checked;
                        var min = (<HTMLInputElement>document.getElementById("min")).value;
                        var max = (<HTMLInputElement>document.getElementById("max")).value;
                        var rowCol = (<HTMLInputElement>document.getElementById("rowCol")).checked;
                        this.gradient(parseInt(min), parseInt(max), rowCol,  check);
                        break; 
                    case "alternate rows": 
                        var check = (<HTMLInputElement>document.getElementById("reverseAlt")).checked;
                        var speed = (<HTMLInputElement>document.getElementById("speed")).value;
                        var size = (<HTMLInputElement>document.getElementById("size")).value;
                        this.altRows(parseInt(size), parseInt(speed), check); 
                        break; 
                    case "row on":
                        var check = (<HTMLInputElement>document.getElementById("rowOnOff")).checked;
                        var speed = (<HTMLInputElement>document.getElementById("speed")).value;
                        var size = (<HTMLInputElement>document.getElementById("size")).value;
                        this.singleRow(parseInt(size), parseInt(speed), check); 
                        break;
                    case "row off":
                        var check = (<HTMLInputElement>document.getElementById("rowOnOff")).checked;
                        var speed = (<HTMLInputElement>document.getElementById("speed")).value;
                        var size = (<HTMLInputElement>document.getElementById("size")).value;
                        this.singleRow(parseInt(size), parseInt(speed), check); 
                        break; 
                    case "alternate columns":
                        var check = (<HTMLInputElement>document.getElementById("reverseAlt")).checked;
                        var speed = (<HTMLInputElement>document.getElementById("speed")).value;
                        var size = (<HTMLInputElement>document.getElementById("size")).value;
                        this.altCols(parseInt(size), parseInt(speed), check); 
                        break; 
                    case "column on":
                        var check = (<HTMLInputElement>document.getElementById("rowOnOff")).checked;
                        var speed = (<HTMLInputElement>document.getElementById("speed")).value;
                        var size = (<HTMLInputElement>document.getElementById("size")).value;
                        this.singleCol(parseInt(size), parseInt(speed), check); 
                        break; 
                    case "column off":
                        var check = (<HTMLInputElement>document.getElementById("rowOnOff")).checked;
                        var speed = (<HTMLInputElement>document.getElementById("speed")).value;
                        var size = (<HTMLInputElement>document.getElementById("size")).value;
                        this.singleCol(parseInt(size), parseInt(speed), check); 
                        break;
                    case "middle on":
                        var check = (<HTMLInputElement>document.getElementById("middleOnOff")).checked;
                        var speed = (<HTMLInputElement>document.getElementById("speed")).value;
                        this.middle(parseInt(speed), check); 
                        break;
                    case "middle off":
                        var check = (<HTMLInputElement>document.getElementById("middleOnOff")).checked;
                        var speed = (<HTMLInputElement>document.getElementById("speed")).value;
                        this.middle(parseInt(speed), check); 
                        break;
                    case "grid":
                        var speed = (<HTMLInputElement>document.getElementById("speed")).value;
                        var size = (<HTMLInputElement>document.getElementById("size")).value;
                        var check = (<HTMLInputElement>document.getElementById("grid")).checked;
                        this.gridPattern(parseInt(speed),parseInt(size),parseInt(size), check);
                        break; 
                    case "checkerboard":
                        var check = (<HTMLInputElement>document.getElementById("check")).checked;
                        var speed = (<HTMLInputElement>document.getElementById("speed")).value;
                        var size = (<HTMLInputElement>document.getElementById("size")).value;
                        this.checkerBoard(parseInt(size),parseInt(speed), check);
                        break;  
                    case "jet":
                        var check = (<HTMLInputElement>document.getElementById("check")).checked;
                        var speed = (<HTMLInputElement>document.getElementById("speed")).value;
                        var size = (<HTMLInputElement>document.getElementById("size")).value;
                        this.jetflow(55);
                        break; 
                    case "gaussian":
                        var mean = (<HTMLInputElement>document.getElementById("mean")).value;
                        var sigma = (<HTMLInputElement>document.getElementById("sigma")).value;
                        this.gaussian(mean, sigma);
                        break;  
                    default: 
                        (<HTMLInputElement>document.getElementById("pattern")).value = '';
                }  
            },
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
    checkerBoard(size: number, val: number, rev: boolean) {
      this.clear();
      for (let i = 0; i < 9; i++) {
        for (let j = 0; j < 9; j++) {
          const isEvenRow = Math.floor(i / size) % 2 === 0;
          const isEvenCol = Math.floor(j / size) % 2 === 0;
    
          const isEvenSquare = (isEvenRow && isEvenCol) || (!isEvenRow && !isEvenCol);
    
          this.grid[9 * i + j] = { value: isEvenSquare ? (rev ? val : 0) : (rev ? 0 : val), disabled: isEvenSquare };
        }
      }
    },
    singleRow(row: number, val: number, on: boolean){ //NUMBERING FROM 0 TO 8 OR FROM 1 TO 9 (RN: 1 TO 0)
        this.grid = this.grid.map((cell, index) => ({
        value: on ? (Math.floor(index / 9)+1 === row ? val : 0) : (Math.floor(index / 9)+1 === row ? 0 : val),
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
    gridPattern(val: number, lines: number, cols: number, on: boolean) {
        this.grid = this.grid.map((cell, index) => {
            const row = Math.floor(index / 9);
            const col = index % 9;
            
            const isAltRow = Math.floor(row / lines) % 2 === (on ? 0 : 1);
            const isAltCol = Math.floor(col / cols) % 2 === (on ? 0 : 1);
            
            const displayValue = (isAltRow || isAltCol) ? val : 0;
            
            return { value: displayValue, disabled: cell.disabled };
        });
    },
    gradient(min: number, max: number, row: boolean, on: boolean) {
      //this.clear(); // Clear the grid
    
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
    },
  gaussian(mean: number, sigma: number) {
  // Define the Gaussian function
  const gaussian = function(row, col) {
    return Math.exp(-((row - mean) ** 2 + (col - mean) ** 2) / (2 * sigma ** 2));
  };
  
  // Clear the grid before applying the pattern
  this.clear();
  
  // Apply the Gaussian pattern to the grid
  for (let i = 1; i < 10; i++) {
    for (let j = 1; j < 10; j++) {
      const value = Math.floor(gaussian(i, j) * 100); // Scale to [0, 100]
      this.grid[9 * (i - 1) + (j - 1)].value = value; // Adjusted row and column numbering
    }
  }
},
        clear(){
            //this.grid.fill({ value: 0, disabled: false });
            //(<HTMLInputElement>document.getElementById("speed")).value = '';
        for (let i = 0; i < 9; i++) {
        for (let j = 0; j < 9; j++) {
            this.grid[9 * i + j] = { value: 0, disabled: false };
        }
    }
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
    .checkbox {
        vertical-align: middle;
        width: 15px;
        height: 15px;
        padding-right: 100px;
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
    .pattern {
        color: black;
        background-color: #65656598;
        border-radius: 8px;
      border: 1px solid transparent;
      box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
	  font-weight: bold;
    }
    .button{
    justify-content: flex-end;
    text-align: center;
    }
    .patternInput{
        background-color: #f2f2f2;
    border: 1px solid #ccc;
    border-radius: 5px;
    padding: 8px 12px;
    margin-bottom: 10px;
    font-size: 16px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    width: 100px;
    }
    .white {
        background-color: rgb(255, 255, 255);
    }
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
    
