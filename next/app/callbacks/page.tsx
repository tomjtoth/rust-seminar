"use client";

import { useState } from "react";

import { useExternalLogic } from "./_lib";

export default function Callbacks() {
  const [text, setText] = useState("initial");
  const externalLogic = useExternalLogic();

  // DEMO: rm number typing
  const localLogic = (idx: number) => setText(`done-${idx}`);

  return (
    <>
      <div>currently: "{text}"</div>

      <button
        onClick={() => {
          externalLogic(localLogic);
        }}
      >
        trigger callback
      </button>
    </>
  );
}
