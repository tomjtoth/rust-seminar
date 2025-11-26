import { create } from "zustand";

type CounterState = {
  count: number;
  inc: (num: number) => void;
};

/**
 * DEMO: closest thing to global signals in Dx
 */
export const useGlobalCounter = create<CounterState>((set) => ({
  count: 0,
  inc(num) {
    set((s) => ({ count: num ? s.count + num : 0 }));
  },
}));
