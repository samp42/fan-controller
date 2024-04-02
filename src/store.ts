import { defineStore } from 'pinia';

export const useGridStore = defineStore('grid', {
  state: () => ({
    grid: Array(81).fill(0).map(() => ({ value: 0, disabled: false })),
  }),
  actions: {
    
  },
});
