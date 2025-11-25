"use client";

import { Incrementer, Nullifier } from "./_components/buttons";
import Future from "./_components/future";

export default function Fullstack() {
  const cls =
    "border rounded p-2 flex flex-col gap-2 [&_button]:w-fit items-center";

  return (
    <>
      <div className={cls}>
        <Incrementer
          incrementBy={16}
          title="manipulates the previously seen global signal"
        />

        <Incrementer incrementBy={8} title="same as above" />

        <Incrementer incrementBy={4} title="same as above" />

        <Incrementer incrementBy={2} />

        <Incrementer incrementBy={1} />

        <Nullifier />
      </div>

      <div className={cls}>
        <Future />
      </div>
    </>
  );
}
