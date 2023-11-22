<template>
    <div
      ref="cell"
      class="cell"
      :class="[
        rowClass,
        columnClass,
        {
          userInput,
          isHighlighted,
          hasError,
          isSimilar,
        },
      ]"
      @focus="handleClick"
      @keydown="handleKeydown"
      @keydown.up="moveFocus(-9)"
      @keydown.down="moveFocus(9)"
      @keydown.left="moveFocus(-1)"
      @keydown.right="moveFocus(1)"
      tabIndex="0"
    >
      <span>{{ value }}</span>
    </div>
  </template>
  
  <script>
  export default {
    name: "Cell",
    props: {
      id: String,
      column: Number,
      row: Number,
      block: Number,
      expectedValue: String,
      value: String,
      userInput: Boolean,
      isHighlighted: Boolean,
      isSimilar: Boolean,
      hasError: Boolean,
      hasFocus: Boolean,
    },
    data() {
      return {
        rowClass: `row-${this.row}`,
        columnClass: `col-${this.column}`,
      };
    },
    updated() {
      if (this.hasFocus) {
        this.$refs.cell.focus();
      }
    },
    methods: {
      moveFocus(distance) {
        console.log("moving focus " + distance);
  
        const index = this.$store.grid.findIndex(({ id }) => id === this.id);
  
        console.log(this.$store.grid[index + distance]);
        this.$store.selectCell(this.$store.grid[index + distance]);
      },
      handleClick() {
        this.$store.selectCell(this);
      },
      handleKeydown(e) {
        if (!this.userInput) return;
  
        if (e.key === "Delete" || e.key === "Backspace") {
          this.$store.setCellValue(this, "");
        }
        if (/[1-9]/.test(e.key)) {
          this.$store.setCellValue(this, e.key);
          if (e.key !== this.expectedValue && e.key !== this.value) {
            this.$store.addMistake();
          }
        }
      },
    },
  };
  </script>
  
  <style scoped>
  .cell {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
    width: var(--cell-size);
    height: var(--cell-size);
    border-right: solid 1px var(--border-color);
    border-bottom: solid 1px var(--border-color);
    position: relative;
    text-align: center;
    background: var(--background);
  }
  .cell:focus {
    outline: none;
    background: rgb(197, 237, 255);
    color: black;
  }
  .isHighlighted {
    background: rgb(234, 234, 245);
  }
  .isSimilar {
    background: rgb(213, 213, 241);
  }
  .userInput,
  .userInput:focus {
    color: blue;
  }
  .hasError,
  .hasError:focus {
    color: rgb(221, 54, 54);
  }
  .col-3,
  .col-6 {
    border-right: none;
    margin-right: var(--border-width);
  }
  .row-3,
  .row-6 {
    border-bottom: none;
    margin-bottom: var(--border-width);
  }
  .col-9 {
    border-right: none;
  }
  .row-9 {
    border-bottom: none;
  }
  </style>