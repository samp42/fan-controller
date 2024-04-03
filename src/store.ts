import { defineStore } from 'pinia';

export interface InstantProfile {
  fans: number[];
  dt: number;
}

export interface Pattern {
  profiles: InstantProfile[];
}

export enum PatternType {
  Static,
  Dynamic,
}

export const useGridStore = defineStore('grid', {
  state: () => ({
    grid: Array(81).fill(0).map(() => ({ value: 0, disabled: false })),
    pattern: { profiles: [] } as Pattern,
    patternName: '',
    port: "/dev/cu.usbmodem149464201",
    usePatternType: PatternType.Static,
  }),
});
