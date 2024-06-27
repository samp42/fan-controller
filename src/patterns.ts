import { Cell } from './cell';

export function initEmptyGrid(): Cell[] {
    return Array(81).fill({ value: 0, disabled: false });
}

export function randomFill(): Cell[] {
    let grid: Cell[] = initEmptyGrid();

    for (let i = 0; i < 9; i++) {
        for (let j = 0; j < 9; j++) {
            grid[9 * i + j] = {
                value: Math.floor(Math.random() * 101),
                disabled: false,
            };
        }
    }

    return grid;
}

export function checkerBoard(size: number, speed: number, rev: boolean): Cell[] {
    let grid: Cell[] = initEmptyGrid();

    for (let i = 0; i < 9; i++) {
        for (let j = 0; j < 9; j++) {
            const isEvenRow = Math.floor(i / size) % 2 === 0;
            const isEvenCol = Math.floor(j / size) % 2 === 0;

            const isEvenSquare = (isEvenRow && isEvenCol) || (!isEvenRow && !isEvenCol);

            grid[9 * i + j] = {
                value: isEvenSquare ? (rev ? speed : 0) : rev ? 0 : speed,
                disabled: isEvenSquare,
            };
        }
    }

    return grid;
}

export function singleRow(row: number, speed: number, on: boolean): Cell[] {
    let grid: Cell[] = initEmptyGrid();

    return grid.map((cell, index) => ({
        value: on
            ? Math.floor(index / 9) + 1 === row
                ? speed
                : 0
            : Math.floor(index / 9) + 1 === row
                ? 0
                : speed,
        disabled: cell.disabled,
    }));
}

export function singleCol(col: number, val: number, on: boolean): Cell[] {
    let grid: Cell[] = initEmptyGrid();

    return grid.map((cell, index) => ({
        value: on ? (index % 9 === col - 1 ? val : 0) : index % 9 === col - 1 ? 0 : val,
        disabled: cell.disabled,
    }));
}

export function altRows(lines: number, speed: number, on: boolean): Cell[] {
    let grid: Cell[] = initEmptyGrid();

    return grid.map((cell, index) => ({
        value: Math.floor(index / 9 / lines) % 2 === (on ? 0 : 1) ? speed : 0,
        disabled: cell.disabled,
    }));
}

export function altCols(cols: number, speed: number, on: boolean): Cell[] {
    let grid: Cell[] = initEmptyGrid();

    return grid.map((cell, index) => ({
        value: Math.floor((index % 9) / cols) % 2 === (on ? 0 : 1) ? speed : 0,
        disabled: cell.disabled,
    }));
}

export function middle(speed: number, on: boolean): Cell[] {
    let grid: Cell[] = initEmptyGrid();

    const middleIndex = 40; // Index of the middle cell in a 9x9 grid
    if (on) {
        grid = grid.map((cell, index) => ({
            value: index === middleIndex ? speed : 0,
            disabled: cell.disabled,
        }));
    } else {
        grid = grid.map((cell, index) => ({
            value: index === middleIndex ? 0 : speed,
            disabled: cell.disabled,
        }));
    }

    return grid;
}

export function gridPattern(speed: number, lines: number, cols: number, on: boolean): Cell[] {
    let grid: Cell[] = initEmptyGrid();

    return grid.map((cell, index) => {
        const row = Math.floor(index / 9);
        const col = index % 9;

        const isAltRow = Math.floor(row / lines) % 2 === (on ? 0 : 1);
        const isAltCol = Math.floor(col / cols) % 2 === (on ? 0 : 1);

        const displayValue = isAltRow || isAltCol ? speed : 0;

        return { value: displayValue, disabled: cell.disabled };
    });
}

export function gradient(min: number, max: number, row: boolean, on: boolean): Cell[] {
    let grid: Cell[] = initEmptyGrid();

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

            grid[size * i + j] = { value: displayValue, disabled: false };
        }
    }

    return grid;
}

export function gaussian(mean: number, sigma: number): Cell[] {
    let grid: Cell[] = initEmptyGrid();

    const gaussian = function (row: number, col: number) {
        return Math.exp(-((row - mean) ** 2 + (col - mean) ** 2) / (2 * sigma ** 2));
    };

    for (let i = 1; i < 10; i++) {
        for (let j = 1; j < 10; j++) {
            const value = Math.floor(gaussian(i, j) * 100);
            grid[9 * (i - 1) + (j - 1)].value = value;
        }
    }

    return grid;
}
