"use client";

import { Incrementer, Nullifier } from "./_components/buttons";

export default function GlobalState() {
  return (
    <>
      <Incrementer incrementBy={20} />
      <Incrementer incrementBy={10} />
      <Incrementer incrementBy={5} />
      <Incrementer incrementBy={1} />

      <Nullifier />

      <Incrementer incrementBy={-1} />
      <Incrementer incrementBy={-5} />
      <Incrementer incrementBy={-10} />
      <Incrementer incrementBy={-20} />
    </>
  );
}
