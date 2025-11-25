import { createContext, Dispatch, SetStateAction } from "react";

export type TailwindClasses =
  `${string}-${string}-${string} dark:${string}-${string}-${string}`;

export const CxParent = createContext<
  Dispatch<SetStateAction<TailwindClasses>>
>(() => {});
