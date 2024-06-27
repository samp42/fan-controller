<template>
  <br />
  <input
    class="pattern"
    id="pattern"
    type="text"
    :value="patternName"
    placeholder="Pattern Name"
  />
  <div class="layout" style="overflow: hidden">
    <div class="square">
      <div class="content">
        <table id="table" class="table" aria-describedby="fan inputs" ref="table">
          <th></th>
          <tbody>
            <tr v-for="row in 9" :key="row">
              <td v-for="col in 9" :key="col">
                <input
                  type="text"
                  :ref="`r${row}c${col}`"
                  :id="`r${row}c${col}`"
                  v-model="grid[9 * (row - 1) + col - 1].value"
                  :class="getColor(row, col)"
                  style="overflow: visible"
                  :disabled="false"
                  @input="checkInput(row, col)"
                />
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
  <span class="br"></span>
  <div v-if="patternName === 'checkerboard' || patternName === 'Checkerboard'">
    <label for="size">Size:</label>
    <input class="patternInput" type="text" id="size" />
    <label for="speed"> Speed:</label>
    <input class="patternInput" type="text" id="speed" />
    <input class="checkbox" type="checkbox" id="check" />
    <label for="check">Reverse</label>
  </div>

  <div
    v-else-if="
      patternName === 'Alternate rows' ||
      patternName === 'alternate rows' ||
      patternName === 'Alternate columns' ||
      patternName === 'alternate columns'
    "
  >
    <label for="size">Size:</label>
    <input class="patternInput" type="text" id="size" />
    <label for="speed"> Speed:</label>
    <input class="patternInput" type="text" id="speed" />
    <input class="checkbox" type="checkbox" id="reverseAlt" />
    <label for="reverseAlt">Reverse</label>
  </div>
  <div
    v-else-if="
      patternName === 'Row on' ||
      patternName === 'row on' ||
      patternName === 'Row off' ||
      patternName === 'row off' ||
      patternName === 'Column on' ||
      patternName === 'column on' ||
      patternName === 'Column off' ||
      patternName === 'column off'
    "
  >
    <label for="size">Position:</label>
    <input class="patternInput" type="text" id="size" />
    <label for="speed"> Speed:</label>
    <input class="patternInput" type="text" id="speed" />
    <input class="checkbox" type="checkbox" id="rowOnOff" />
     <label for="rowOnOff">Reverse</label>
  </div>
  <div v-else-if="patternName === 'gradient' || patternName === 'Gradient'">
    <label for="min">Minimum:</label>
    <input class="patternInput" type="text" id="min" />
    <label for="max"> Maximum:</label>
    <input class="patternInput" type="text" id="max" />
    <input class="checkbox" type="checkbox" id="rowCol" />
    <label for="rowCol">Row:</label>
    <input class="checkbox" type="checkbox" id="gradientReverse" />
    <label for="gradientReverse">Reverse:</label>
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
  <div v-else-if="patternName === 'Middle off' || patternName === 'middle off'">
    <label for="speed">Speed:</label>
    <input class="patternInput" type="text" id="speed" />
    <input class="checkbox" type="checkbox" id="middleOnOff" />
    <label for="middleOnOff">Reverse</label>
  </div>
  <div v-else-if="patternName === 'Middle on' || patternName === 'middle on'">
    <label for="speed">Speed:</label>
    <input class="patternInput" type="text" id="speed" required />
    <input class="checkbox" type="checkbox" id="middleOnOff" />
    <label for="middleOnOff">Reverse</label>
  </div>
  <div v-else-if="patternName === 'Fill' || patternName === 'fill'">
    <label for="speed">Speed:</label>
    <input class="patternInput" type="text" id="speed" required />
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
  </div>
  <span class="br"></span>
  <div class="button-container">
    <button type="button" @click="clear()">Clear</button>
    <button type="button" @click="getPattern()">Enter</button>
    <p id="p" class="err" style="color: red; font-weight: bold"></p>
  </div>
  <span class="br"></span>
</template>

<script lang="ts">
import { RouteLocationNormalizedLoaded } from "vue-router";
import { PatternType, useGridStore } from "../store";

export default {
  data() {
    return {
      patternName: "",
      grid: [] as { value: number; disabled: boolean }[],
    };
  },
  // watch: {
  //   grid(newValue) {
  //     const gridStore = useGridStore();
  //     gridStore.grid = newValue.map((cell: any) => ({
  //       value: parseInt(cell.value),
  //       disabled: cell.disabled,
  //     }));
  //     gridStore.usePatternType = PatternType.Static;
  //     console.log(newValue);
  //   },
  // },
  // onUpdated() {
  //   console.log("updated");
  //   console.log(this.grid);
  // },
  created() {
    const route = this.$route as RouteLocationNormalizedLoaded;
    this.patternName = route.query.pattern as string;
    const gridStore = useGridStore();
    this.grid = gridStore.grid;
  },
  methods: {
    getValue() {
      var value = (<HTMLInputElement>document.getElementById("pattern")).value;
      return value;
    },
    getPattern() {
      var input = (<HTMLInputElement>(
        document.getElementById("pattern")
      )).value.toLowerCase();
      const message = document.getElementById("p");
      message!.innerHTML = "";
      try {
        switch (input) {
          case "random":
            this.randomFill();
            break;
          case "gradient":
            var check = (<HTMLInputElement>document.getElementById("gradientReverse"))
              .checked;
            var min = (<HTMLInputElement>document.getElementById("min")).value;
            var max = (<HTMLInputElement>document.getElementById("max")).value;

            if (min == "" || max == "") throw "empty";
            if (!min.match(/^[0-9]+$/) || !max.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(min)) || isNaN(parseInt(max))) throw "not a number";
            if (parseInt(min) < 0) throw "too low";
            if (parseInt(max) > 100) throw "too high";

            var rowCol = (<HTMLInputElement>document.getElementById("rowCol")).checked;
            this.gradient(parseInt(min), parseInt(max), rowCol, check);
            break;
          case "alternate rows":
            var check = (<HTMLInputElement>document.getElementById("reverseAlt")).checked;
            var speed = (<HTMLInputElement>document.getElementById("speed")).value;
            var size = (<HTMLInputElement>document.getElementById("size")).value;

            if (speed == "" || size == "") throw "empty";
            if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
            if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
            if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

            this.altRows(parseInt(size), parseInt(speed), check);
            break;
          case "row on":
            var check = (<HTMLInputElement>document.getElementById("rowOnOff")).checked;
            var speed = (<HTMLInputElement>document.getElementById("speed")).value;
            var size = (<HTMLInputElement>document.getElementById("size")).value;

            if (speed == "" || size == "") throw "empty";
            if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
            if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
            if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

            //this.singleRow(parseInt(size), parseInt(speed), check);
            this.RowOn(parseInt(size), parseInt(speed), check);
            break;
          case "row off":
            var check = (<HTMLInputElement>document.getElementById("rowOnOff")).checked;
            var speed = (<HTMLInputElement>document.getElementById("speed")).value;
            var size = (<HTMLInputElement>document.getElementById("size")).value;

            if (speed == "" || size == "") throw "empty";
            if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
            if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
            if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

            //this.singleRow(parseInt(size), parseInt(speed), check);
            this.RowOff(parseInt(size), parseInt(speed), check);
            break;
          case "alternate columns":
            var check = (<HTMLInputElement>document.getElementById("reverseAlt")).checked;
            var speed = (<HTMLInputElement>document.getElementById("speed")).value;
            var size = (<HTMLInputElement>document.getElementById("size")).value;

            if (speed == "" || size == "") throw "empty";
            if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
            if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
            if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

            this.altCols(parseInt(size), parseInt(speed), check);
            break;
          case "column on":
            var check = (<HTMLInputElement>document.getElementById("rowOnOff")).checked;
            var speed = (<HTMLInputElement>document.getElementById("speed")).value;
            var size = (<HTMLInputElement>document.getElementById("size")).value;

            if (speed == "" || size == "") throw "empty";
            if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
            if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
            if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

            //this.singleCol(parseInt(size), parseInt(speed), check);
            this.colOn(parseInt(size), parseInt(speed), check);
            break;
          case "column off":
            var check = (<HTMLInputElement>document.getElementById("rowOnOff")).checked;
            var speed = (<HTMLInputElement>document.getElementById("speed")).value;
            var size = (<HTMLInputElement>document.getElementById("size")).value;

            if (speed == "" || size == "") throw "empty";
            if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
            if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
            if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

            //this.singleCol(parseInt(size), parseInt(speed), check);
            this.colOff(parseInt(size), parseInt(speed), check);
            break;
          case "middle on":
            var check = (<HTMLInputElement>document.getElementById("middleOnOff"))
              .checked;
            var speed = (<HTMLInputElement>document.getElementById("speed")).value;

            if (speed == "") throw "empty";
            if (!speed.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(speed))) throw "not a number";
            if (parseInt(speed) < 0) throw "too low";
            if (parseInt(speed) > 100) throw "too high";

            this.middle(parseInt(speed), check);
            break;
          case "middle off":
            var check = (<HTMLInputElement>document.getElementById("middleOnOff"))
              .checked;
            var speed = (<HTMLInputElement>document.getElementById("speed")).value;

            if (speed == "") throw "empty";
            if (!speed.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(speed))) throw "not a number";
            if (parseInt(speed) < 0) throw "too low";
            if (parseInt(speed) > 100) throw "too high";

            this.middle(parseInt(speed), check);
            break;
          case "grid":
            var speed = (<HTMLInputElement>document.getElementById("speed")).value;
            var size = (<HTMLInputElement>document.getElementById("size")).value;
            var check = (<HTMLInputElement>document.getElementById("grid")).checked;

            if (speed == "" || size == "") throw "empty";
            if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
            if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
            if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

            this.gridPattern(parseInt(speed), parseInt(size), parseInt(size), check);
            break;
          case "checkerboard":
            var check = (<HTMLInputElement>document.getElementById("check")).checked;
            var speed = (<HTMLInputElement>document.getElementById("speed")).value;
            var size = (<HTMLInputElement>document.getElementById("size")).value;

            if (speed == "" || size == "") throw "empty";
            if (!speed.match(/^[0-9]+$/) || !size.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(speed)) || isNaN(parseInt(size))) throw "not a number";
            if (parseInt(speed) < 0 || parseInt(size) < 0) throw "too low";
            if (parseInt(speed) > 100 || parseInt(size) > 9) throw "too high";

            this.checkerBoard(parseInt(size), parseInt(speed), check);
            break;
          // TODO: Implement jetflow
          // case "jet":
          //   var check = (<HTMLInputElement>document.getElementById("check")).checked;
          //   var speed = (<HTMLInputElement>document.getElementById("speed")).value;
          //   var size = (<HTMLInputElement>document.getElementById("size")).value;
          //   this.jetflow(55);
          //   break;
          case "gaussian":
            var mean = (<HTMLInputElement>document.getElementById("mean")).value;
            var sigma = (<HTMLInputElement>document.getElementById("sigma")).value;

            if (mean == "" || sigma == "") throw "empty";
            if (!mean.match(/^[0-9]+$/) || !sigma.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(mean)) || isNaN(parseInt(sigma))) throw "not a number";
            if (parseInt(mean) < 0 || parseInt(sigma) < 0) throw "too low";
            if (parseInt(mean) > 100 || parseInt(sigma) > 9) throw "too high";

            this.gaussian(parseInt(mean), parseInt(sigma));
            break;
          case "fill":
            var speed = (<HTMLInputElement>document.getElementById("speed")).value;

            if (speed == "") throw "empty";
            if (!speed.match(/^[0-9]+$/)) throw "invalid";
            if (isNaN(parseInt(speed))) throw "not a number";
            if (parseInt(speed) < 0) throw "too low";
            if (parseInt(speed) > 100) throw "too high";
            this.fill(parseInt(speed));
            break;
          default:
            (<HTMLInputElement>document.getElementById("pattern")).value = "";
        }
      } catch (error) {
        message!.innerHTML = "<b style='color: red;'>Input is " + error + "</b>";
      }
    },
    getColor(row: number, column: number) {
      const gridValue = this.grid[9 * (row - 1) + column - 1];
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
    },
    randomFill() {
      this.clear();
      for (let i = 0; i < 9; i++) {
        for (let j = 0; j < 9; j++) {
          this.grid[9 * i + j] = {
            value: Math.floor(Math.random() * 101),
            disabled: false,
          };
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

          this.grid[9 * i + j] = {
            value: isEvenSquare ? (rev ? val : 0) : rev ? 0 : val,
            disabled: isEvenSquare,
          };
        }
      }
    },
    fill(speed: number){
      for (let i = 0; i < 9; i++) {
        for (let j = 0; j < 9; j++) {
          this.grid[9 * i + j] = { value: speed, disabled: false };
        }
      }
    },
    singleRow(row: number, val: number, on: boolean) {
      this.clear();

      for (let i = 0; i < 9; i++) {
        const index = 9 * (row - 1) + i;
        this.grid[index] = {
          value: on ? (i === val - 1 ? val : 0) : (i === val - 1 ? 0 : val),
          disabled: this.grid[index].disabled,
        };
      }
    },
  RowOn(row: number, val: number, rev: boolean) {
  this.clear();
  if (!rev){
  for (let i = 0; i < 9; i++) {
    const index = 9 * (row - 1) + i;
    this.grid[index] = {
      value: val,
      disabled: this.grid[index].disabled,
    };
  }
}
else{
for (let r = 0; r < 9; r++) {
    for (let c = 0; c < 9; c++) {
      const index = 9 * r + c;
      if (r === (row - 1)) {
        this.grid[index] = {
          value: 0,
          disabled: this.grid[index].disabled,
        };
      } else {
        this.grid[index] = {
          value: val,
          disabled: this.grid[index].disabled,
        };
      }
    }
  }
}
},

RowOff(row: number, val: number, rev: boolean) {
  this.clear();
  if (!rev){
  for (let r = 0; r < 9; r++) {
    for (let c = 0; c < 9; c++) {
      const index = 9 * r + c;
      if (r === (row - 1)) {
        this.grid[index] = {
          value: 0,
          disabled: this.grid[index].disabled,
        };
      } else {
        this.grid[index] = {
          value: val,
          disabled: this.grid[index].disabled,
        };
      }
    }
  }
}
else{
  for (let i = 0; i < 9; i++) {
    const index = 9 * (row - 1) + i;
    this.grid[index] = {
      value: val,
      disabled: this.grid[index].disabled,
    };
  }
}
},


    singleCol(col: number, val: number, on: boolean) {
      this.clear();
      for (let i = 0; i < 9; i++) {
        const index = 9 * i + (col - 1);
        this.grid[index] = {
          value: on ? (i === val - 1 ? val : 0) : (i === val - 1 ? 0 : val),
          disabled: this.grid[index].disabled,
        };
      }
    },
colOn(col: number, val: number, rev: boolean) {
  this.clear();
  if (!rev) {
    for (let r = 0; r < 9; r++) {
      const index = 9 * r + (col - 1);
      this.grid[index] = {
        value: val,
        disabled: this.grid[index].disabled,
      };
    }
  } else {
    for (let r = 0; r < 9; r++) {
      for (let c = 0; c < 9; c++) {
        const index = 9 * r + c;
        if (c === (col - 1)) {
          this.grid[index] = {
            value: 0,
            disabled: this.grid[index].disabled,
          };
        } else {
          this.grid[index] = {
            value: val,
            disabled: this.grid[index].disabled,
          };
        }
      }
    }
  }
},

colOff(col: number, val: number, rev: boolean) {
  this.clear();
  if (!rev) {
    for (let r = 0; r < 9; r++) {
      for (let c = 0; c < 9; c++) {
        const index = 9 * r + c;
        if (c === (col - 1)) {
          this.grid[index] = {
            value: 0,
            disabled: this.grid[index].disabled,
          };
        } else {
          this.grid[index] = {
            value: val,
            disabled: this.grid[index].disabled,
          };
        }
      }
    }
  } else {
    for (let r = 0; r < 9; r++) {
      const index = 9 * r + (col - 1);
      this.grid[index] = {
        value: val,
        disabled: this.grid[index].disabled,
      };
    }
  }
},

    altRows(lines: number, val: number, on: boolean) {
      this.clear();
      for (let i = 0; i < 81; i++) {
        const rowIndex = Math.floor(i / 9);
        this.grid[i].value = Math.floor(rowIndex / lines) % 2 === (on ? 0 : 1) ? val : 0;
      }
    },

    altCols(cols: number, val: number, on: boolean) {
      this.clear();
      for (let i = 0; i < 81; i++) {
        const colIndex = i % 9;
        this.grid[i].value = Math.floor(colIndex / cols) % 2 === (on ? 0 : 1) ? val : 0;
      }
    },

    middle(val: number, on: boolean) {
      this.clear();
      const middleIndex = 40; // Index of the middle cell in a 9x9 grid
      this.grid[middleIndex].value = on ? val : 0;
    },

    gridPattern(val: number, lines: number, cols: number, on: boolean) {
      this.clear();
      for (let i = 0; i < 81; i++) {
        const row = Math.floor(i / 9);
        const col = i % 9;

        const isAltRow = Math.floor(row / lines) % 2 === (on ? 0 : 1);
        const isAltCol = Math.floor(col / cols) % 2 === (on ? 0 : 1);

        const displayValue = isAltRow || isAltCol ? val : 0;

        this.grid[i].value = displayValue;
      }
    },

    gradient(min: number, max: number, row: boolean, on: boolean) {
      const size = 9; // Size of the grid

      for (let i = 0; i < size; i++) {
        for (let j = 0; j < size; j++) {
          const position = row ? i : j;
          const invertedPosition = row ? size - 1 - i : size - 1 - j;

          const gradientValue = Math.floor((max - min) * (position / (size - 1)) + min);
          const invertedGradientValue = Math.floor(
            (max - min) * (invertedPosition / (size - 1)) + min
          );

          const displayValue = on ? gradientValue : invertedGradientValue;

          this.grid[size * i + j] = { value: displayValue, disabled: false };
        }
      }
    },
    gaussian(mean: number, sigma: number) {
      const gaussian = function (row: number, col: number) {
        return Math.exp(-((row - mean) ** 2 + (col - mean) ** 2) / (2 * sigma ** 2));
      };

      this.clear();

      for (let i = 1; i < 10; i++) {
        for (let j = 1; j < 10; j++) {
          const value = Math.floor(gaussian(i, j) * 100);
          this.grid[9 * (i - 1) + (j - 1)].value = value;
        }
      }
    },
    clear() {
      const message = document.getElementById("p");
      message!.innerHTML = "";
      for (let i = 0; i < 9; i++) {
        for (let j = 0; j < 9; j++) {
          this.grid[9 * i + j] = { value: 0, disabled: false };
        }
      }
    },
    checkInput(row: number, col: number) {
      const gridStore = useGridStore();

      const index = 9 * (row - 1) + col - 1;
      const inputValue = this.grid[index].value;

      console.log(inputValue + typeof inputValue);

      if (
        typeof inputValue === "string" ||
        isNaN(inputValue) ||
        inputValue < 0 ||
        inputValue > 100
      ) {
        const numOnly = parseInt(inputValue.toString().replace(/[^0-9]/g, ""));

        this.grid[index] = { ...this.grid[index], value: numOnly, disabled: false };
        gridStore.usePatternType = PatternType.Static;
        gridStore.grid = this.grid;
      }
    },
  },
};
</script>

<style scoped>
* {
  box-sizing: border-box;
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
