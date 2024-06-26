<template>
    <div>
        <br />
        <input class="pattern" id="pattern" type="text" v-model="patternName" placeholder="Pattern Name"
            @input="console.log(patternName)" />
        <div class="layout" style="overflow: hidden">
            <div class="square">
                <div class="content">
                    <table id="table" class="table" aria-describedby="fan inputs" ref="table">
                        <th></th>
                        <tbody>
                            <tr v-for="row in 9" :key="row">
                                <td v-for="col in 9" :key="col">
                                    <input type="number" min="0" max="100" :ref="`r${row}c${col}`"
                                        :id="`r${row}c${col}`" v-model="grid[9 * (row - 1) + col - 1].value"
                                        :class="getColor(row, col)" style="overflow: visible" :disabled="false"
                                        @input="checkInput(row, col, $event.target)" />
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
        <span class="br"></span>
        <div class="input-container">
            <div v-if="displaySize()" class="input-inner">
                <label for="size">Size:</label>
                <input class="patternInput" type="number" id="size" min="1" max="9" v-model="size" />
            </div>
            <div v-if="displaySpeed()" class="input-inner">
                <label for="speed"> Speed:</label>
                <input class="patternInput" type="text" id="speed" v-model="speed" />
            </div>
            <div v-if="displayMinMax()" class="input-inner">
                <label for="min">Minimum:</label>
                <input class="patternInput" type="number" id="min" min="0" max="100" v-model="minimum" />
                <label for="max"> Maximum:</label>
                <input class="patternInput" type="text" id="max" min="0" max="100" v-model="maximum" />
            </div>
            <div v-if="displayReverse()" class="input-inner">
                <input class="checkbox" type="checkbox" id="check" v-model="reverse" />
                <label for="check">Reverse</label>
            </div>
            <div v-if="displayOnOff()" class="input-inner">
                <input class="checkbox" type="checkbox" id="rowOnOff" />
                <label for="rowOnOff">On/Off</label>
            </div>
            <div v-if="displayGaussian()" class="input-inner">
                <label for="mean">Mean:</label>
                <input class="patternInput" type="text" id="mean" v-model="mean" />
                <label for="sigma"> Sigma:</label>
                <input class="patternInput" type="text" id="sigma" v-model="sigma" />
            </div>
        </div>
        <!-- <div v-if="patternName === 'checkerboard' || patternName === 'Checkerboard'">
            <label for="size">Size:</label>
            <input class="patternInput" type="text" id="size" />
            <label for="speed"> Speed:</label>
            <input class="patternInput" type="text" id="speed" />
            <input class="checkbox" type="checkbox" id="check" />
            <label for="check">Reverse</label>
        </div>

        <div v-else-if="patternName === 'Alternate rows' ||
            patternName === 'alternate rows' ||
            patternName === 'Alternate columns' ||
            patternName === 'alternate columns'
            ">
            <label for="size">Size:</label>
            <input class="patternInput" type="text" id="size" />
            <label for="speed"> Speed:</label>
            <input class="patternInput" type="text" id="speed" />
            <input class="checkbox" type="checkbox" id="reverseAlt" />
            <label for="reverseAlt">Reverse</label>
        </div>
        <div v-else-if="patternName === 'Row on' ||
            patternName === 'row on' ||
            patternName === 'Row off' ||
            patternName === 'row off' ||
            patternName === 'Column on' ||
            patternName === 'column on' ||
            patternName === 'Column off' ||
            patternName === 'column off'
            ">
            <label for="size">Position:</label>
            <input class="patternInput" type="text" id="size" />
            <label for="speed"> Speed:</label>
            <input class="patternInput" type="text" id="speed" />
            <input class="checkbox" type="checkbox" id="rowOnOff" />
            <label for="rowOnOff">On/Off</label>
        </div>
        <div v-else-if="patternName === 'gradient' || patternName === 'Gradient'">
            <label for="min">Minimum:</label>
            <input class="patternInput" type="number" id="min" min="0" max="100" v-model="minimum"/>
            <label for="max"> Maximum:</label>
            <input class="patternInput" type="text" id="max" min="0" max="100" v-model="maximum"/>
            <input class="checkbox" type="checkbox" id="rowCol" />
            <label for="rowCol">Row:</label>
            <input class="checkbox" type="checkbox" id="gradientReverse" />
            <label for="gradientReverse">Reverse:</label>
        </div>

        <div v-else-if="patternName === 'random' || patternName === 'Random'"></div>
        <div v-else-if="patternName === 'Middle off' || patternName === 'middle off'">
            <label for="speed">Speed:</label>
            <input class="patternInput" type="text" id="speed" />
            <input class="checkbox" type="checkbox" id="middleOnOff" />
            <label for="middleOnOff">On/Off</label>
        </div>
        <div v-else-if="patternName === 'Middle on' || patternName === 'middle on'">
            <label for="speed">Speed:</label>
            <input class="patternInput" type="text" id="speed" required />
            <input class="checkbox" type="checkbox" id="middleOnOff" checked />
            <label for="middleOnOff">On/Off</label>
        </div>

        <div v-else-if="patternName === 'grid' || patternName === 'Grid'">
            <label for="size">Size:</label>
            <input class="patternInput" type="text" id="size" />
            <label for="speed"> Speed:</label>
            <input class="patternInput" type="text" id="speed" />
            <input class="checkbox" type="checkbox" id="grid" />
            <label for="grid">Reverse</label>
        </div>
        <div v-else-if="patternName === 'gaussian' || patternName === 'Gaussian'">
            <label for="mean">Mean:</label>
            <input class="patternInput" type="text" id="mean" />
            <label for="sigma"> Sigma:</label>
            <input class="patternInput" type="text" id="sigma" />
        </div> -->
        <span class="br"></span>
        <div class="button-container">
            <button type="button" @click="clear()">Clear</button>
            <button type="button" @click="getPatternFromList()">Enter</button>
            <p id="p" class="err" style="color: red; font-weight: bold"></p>
        </div>
        <span class="br"></span>
    </div>
</template>

<script setup lang="ts">
import { PatternType, useGridStore } from "../store";
import { ref } from "vue";
import { useRoute } from "vue-router";
import { Cell } from "../cell";
import { initEmptyGrid, checkerBoard, gaussian, gradient, randomFill } from "../patterns";

const route = useRoute();
let gridStore = useGridStore();

let patternName = ref<string>(route.query.pattern as string || "");
let grid = ref<Cell[]>(initEmptyGrid());
let errorMessage = ref<string>("");

// pattern parameters
let minimum = ref<number>(0);
let maximum = ref<number>(100);
let size = ref<number>(1);
let speed = ref<number>(0);
let position = ref<number>(5);
let mean = ref<number>(0);
let sigma = ref<number>(0);
let rowCol = ref<boolean>(false);
let reverse = ref<boolean>(false);
let middleOnOff = ref<boolean>(false);

// functions
function displayMinMax(): boolean {
    return patternName.value.toLowerCase() === "gradient";
}

function displaySize(): boolean {
    return patternName.value.toLowerCase() === "checkerboard";
}

function displayReverse(): boolean {
    return patternName.value.toLowerCase() === "checkerboard";
}

function displaySpeed(): boolean {
    const pattern = patternName.value.toLowerCase();

    return pattern === "checkerboard" || pattern === "alternate rows" || pattern === "alternate columns" ||
        pattern === "middle on" || pattern === "middle off" || pattern === "grid" || pattern === "row on" ||
        pattern === "row off" || pattern === "row on" || pattern === "column on" || pattern === "column off";
}

function displayPosition(): boolean {
    return true;
}

function displayRowCol(): boolean {
    return true;
}

function displayOnOff(): boolean {
    return patternName.value.toLowerCase() === "middle off" || patternName.value.toLowerCase() === "middle on";
}

function displayGaussian(): boolean {
    return patternName.value.toLowerCase() === "gaussian";
}

function getColor(row: number, column: number): string {
    const gridValue = grid.value[9 * (row - 1) + column - 1];

    // check if gridValue contains only numbers
    if (isNaN(gridValue.value)) {
        return "";
    }
    const value = gridValue.value;

    if (value == 0) {
        return "white";
    } else if (value < 0 || value > 100) {
        return "red";
    } else if (value > 0 && value <= 15) {
        return "lightBlue";
    } else if (value > 15 && value <= 30) {
        return "blue";
    } else if (value > 30 && value <= 45) {
        return "darkBlue";
    } else if (value > 45 && value <= 60) {
        return "darkPurple";
    } else if (value > 60 && value <= 75) {
        return "purple";
    } else if (value > 75 && value <= 90) {
        return "darkPink";
    }
    return "pink";
}

function checkInput(row: number, col: number, target: any): void {
    const index = 9 * (row - 1) + col - 1;

    const tempGrid = grid.value;
    tempGrid.at(index)!.value = target.value.replace(/[^0-9]/g, ''); // Remove non-numeric characters

    grid.value = tempGrid;

    // grid.value[index].value = target.value;
    // const inputValue = grid.value[index].value;

    // const num = parseInt(`${inputValue}`) | 0;

    // if (
    //     typeof inputValue === "string" ||
    //     isNaN(inputValue) ||
    //     inputValue < 0 ||
    //     inputValue > 100
    // ) {
    //     const numOnly = parseInt(inputValue.toString().replace(/[^0-9]/g, "")) | 0;

    //     grid.value[index] = { ...grid.value[index], value: numOnly, disabled: false };
    //     gridStore.usePatternType = PatternType.Static;
    //     gridStore.grid = grid.value;
    // }

    grid.value[index].value
    gridStore.grid = grid.value;
}

function clear(): void {
    grid.value = initEmptyGrid();
    errorMessage.value = "";
}

function getPatternFromList(): void {
    switch (patternName.value.toLowerCase()) {
        case "random":
            grid.value = randomFill();
            break;
        case "gradient":
            grid.value = gradient(minimum.value, maximum.value, rowCol.value, reverse.value);
            break;
        case "alternate rows":
            break;
        case "row on":
            break;
        case "column on":
            break;
        case "column off":
            break;
        case "middle on":
            break;
        case "middle off":
            break;
        case "grid":
            break;
        case "checkerboard":
            grid.value = checkerBoard(size.value, speed.value, reverse.value);
            break;
        case "gaussian":
            grid.value = gaussian(mean.value, sigma.value);
    }
}

// getPattern() {
//     var input = (<HTMLInputElement>(
//         document.getElementById("pattern")
//     )).value.toLowerCase();
//     const message = document.getElementById("p");
//     message!.innerHTML = "";
//     try {
//         switch (input) {
//             case "random":
//                 this.randomFill();
//                 break;
//             case "gradient":
//                 var check = (<HTMLInputElement>document.getElementById("gradientReverse"))
//                     .checked;
//                 var min = (<HTMLInputElement>document.getElementById("min")).value;
//                 var max = (<HTMLInputElement>document.getElementById("max")).value;

//                 if (min == "" || max == "") throw "empty";
//                 if (!min.match(/^[0-9]+$/) || !max.match(/^[0-9]+$/)) throw "invalid";
//                 if (isNaN(parseInt(min)) || isNaN(parseInt(max))) throw "not a number";
//                 if (parseInt(min) < 0) throw "too low";
//                 if (parseInt(max) > 100) throw "too high";

//                 var rowCol = (<HTMLInputElement>document.getElementById("rowCol")).checked;
//                 this.gradient(parseInt(min), parseInt(max), rowCol, check);
//                 break;
//             case "alternate rows":
//                 var check = (<HTMLInputElement>document.getElementById("reverseAlt")).checked;
//                 var speed = (<HTMLInputElement>document.getElementById("speed")).value;
//                 var size = (<HTMLInputElement>document.getElementById("size")).value;

//                 if (speed == "" || size == "") throw "empty";
//                 if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
//                 if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
//                 if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
//                 if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

//                 this.altRows(parseInt(size), parseInt(speed), check);
//                 break;
//             case "row on":
//                 var check = (<HTMLInputElement>document.getElementById("rowOnOff")).checked;
//                 var speed = (<HTMLInputElement>document.getElementById("speed")).value;
//                 var size = (<HTMLInputElement>document.getElementById("size")).value;

//                 if (speed == "" || size == "") throw "empty";
//                 if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
//                 if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
//                 if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
//                 if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

//                 this.singleRow(parseInt(size), parseInt(speed), check);
//                 break;
//             case "row off":
//                 var check = (<HTMLInputElement>document.getElementById("rowOnOff")).checked;
//                 var speed = (<HTMLInputElement>document.getElementById("speed")).value;
//                 var size = (<HTMLInputElement>document.getElementById("size")).value;

//                 if (speed == "" || size == "") throw "empty";
//                 if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
//                 if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
//                 if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
//                 if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

//                 this.singleRow(parseInt(size), parseInt(speed), check);

//                 break;
//             case "alternate columns":
//                 var check = (<HTMLInputElement>document.getElementById("reverseAlt")).checked;
//                 var speed = (<HTMLInputElement>document.getElementById("speed")).value;
//                 var size = (<HTMLInputElement>document.getElementById("size")).value;

//                 if (speed == "" || size == "") throw "empty";
//                 if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
//                 if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
//                 if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
//                 if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

//                 this.altCols(parseInt(size), parseInt(speed), check);
//                 break;
//             case "column on":
//                 var check = (<HTMLInputElement>document.getElementById("rowOnOff")).checked;
//                 var speed = (<HTMLInputElement>document.getElementById("speed")).value;
//                 var size = (<HTMLInputElement>document.getElementById("size")).value;

//                 if (speed == "" || size == "") throw "empty";
//                 if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
//                 if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
//                 if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
//                 if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

//                 this.singleCol(parseInt(size), parseInt(speed), check);
//                 break;
//             case "column off":
//                 var check = (<HTMLInputElement>document.getElementById("rowOnOff")).checked;
//                 var speed = (<HTMLInputElement>document.getElementById("speed")).value;
//                 var size = (<HTMLInputElement>document.getElementById("size")).value;

//                 if (speed == "" || size == "") throw "empty";
//                 if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
//                 if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
//                 if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
//                 if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

//                 this.singleCol(parseInt(size), parseInt(speed), check);
//                 break;
//             case "middle on":
//                 var check = (<HTMLInputElement>document.getElementById("middleOnOff"))
//                     .checked;
//                 var speed = (<HTMLInputElement>document.getElementById("speed")).value;

//                 if (speed == "") throw "empty";
//                 if (!speed.match(/^[0-9]+$/)) throw "invalid";
//                 if (isNaN(parseInt(speed))) throw "not a number";
//                 if (parseInt(speed) < 0) throw "too low";
//                 if (parseInt(speed) > 100) throw "too high";

//                 this.middle(parseInt(speed), check);
//                 break;
//             case "middle off":
//                 var check = (<HTMLInputElement>document.getElementById("middleOnOff"))
//                     .checked;
//                 var speed = (<HTMLInputElement>document.getElementById("speed")).value;

//                 if (speed == "") throw "empty";
//                 if (!speed.match(/^[0-9]+$/)) throw "invalid";
//                 if (isNaN(parseInt(speed))) throw "not a number";
//                 if (parseInt(speed) < 0) throw "too low";
//                 if (parseInt(speed) > 100) throw "too high";

//                 this.middle(parseInt(speed), check);
//                 break;
//             case "grid":
//                 var speed = (<HTMLInputElement>document.getElementById("speed")).value;
//                 var size = (<HTMLInputElement>document.getElementById("size")).value;
//                 var check = (<HTMLInputElement>document.getElementById("grid")).checked;

//                 if (speed == "" || size == "") throw "empty";
//                 if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
//                 if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
//                 if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
//                 if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

//                 this.gridPattern(parseInt(speed), parseInt(size), parseInt(size), check);
//                 break;
//             case "checkerboard":
//                 var check = (<HTMLInputElement>document.getElementById("check")).checked;
//                 var speed = (<HTMLInputElement>document.getElementById("speed")).value;
//                 var size = (<HTMLInputElement>document.getElementById("size")).value;

//                 if (speed == "" || size == "") throw "empty";
//                 if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
//                 if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
//                 if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
//                 if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

//                 this.checkerBoard(parseInt(size), parseInt(speed), check);
//                 break;
//             // TODO: Implement jetflow
//             // case "jet":
//             //   var check = (<HTMLInputElement>document.getElementById("check")).checked;
//             //   var speed = (<HTMLInputElement>document.getElementById("speed")).value;
//             //   var size = (<HTMLInputElement>document.getElementById("size")).value;
//             //   this.jetflow(55);
//             //   break;
//             case "gaussian":
//                 var mean = (<HTMLInputElement>document.getElementById("mean")).value;
//                 var sigma = (<HTMLInputElement>document.getElementById("sigma")).value;

//                 if (mean == "" || sigma == "") throw "empty";
//                 if (!mean.match(/^[0-9]+$/) || !sigma.match(/^[0-9]+$/)) throw "invalid";
//                 if (isNaN(parseInt(mean)) || isNaN(parseInt(sigma))) throw "not a number";
//                 if (parseInt(mean) < 0 || parseInt(sigma) < 0) throw "too low";
//                 if (parseInt(mean) > 100 || parseInt(sigma) > 9) throw "too high";

//                 this.gaussian(parseInt(mean), parseInt(sigma));
//                 break;
//             default:
//                 (<HTMLInputElement>document.getElementById("pattern")).value = "";
//         }
//     } catch (error) {
//         message!.innerHTML = "<b style='color: red;'>Input is " + error + "</b>";
//     }
// },
</script>

<style scoped>
* {
    box-sizing: border-box;
}

/* Chrome, Safari, Edge, Opera */
input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

/* Firefox */
input[type=number] {
    -moz-appearance: textfield;
}

.input-container {
    display: flex;
    flex: 1;
    justify-content: center;
    align-items: center
}

.input-inner {
    margin: 4px 4px 0;
}

.br {
    display: block;
    margin-bottom: 0em;
}

.layout {
    position: relative;
    width: 100%;
}

.square {
    padding-bottom: 85%;
}

.content {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
}

.button-container {
    position: relative;
}

.err {
    position: absolute;
    top: -10px;
    right: 80px;
    padding-bottom: 55px;
    color: red;
    font-weight: bold;
}

input {
    background-color: white;
    border-radius: 0;
    border: 0;
    text-align: center;
    box-shadow: none;
    width: 100%;
    height: 100%;
    top: 50%;
    left: 50%;
}

.checkbox {
    vertical-align: middle;
    width: 15px;
    height: 15px;
    padding-right: 100px;
}

table {
    border: 1px solid black;
    justify-content: center;
    border-spacing: 0;
    width: 100%;
    height: 100%;
    padding: 0;
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

.pattern {
    color: black;
    background-color: #65656598;
    border-radius: 8px;
    border: 1px solid transparent;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    font-weight: bold;
}

.button {
    justify-content: flex-end;
    text-align: center;
}

.patternInput {
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

.darkPurple {
    background-color: rgba(68, 1, 226, 0.676);
}

.purple {
    background-color: rgba(130, 4, 255, 0.687);
}

.darkPink {
    background-color: rgba(169, 2, 247, 0.644);
}

.pink {
    background-color: rgba(227, 2, 247, 0.667);
}

.red {
    background-color: rgba(213, 20, 20, 0.814);
}
</style>
