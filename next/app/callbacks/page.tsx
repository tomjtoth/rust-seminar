"use client";

import { useState } from "react";

import { useComplexEternalLogic } from "./_lib";

export default function Callbacks() {
  const [text, setText] = useState("initial");
  const handler = useComplexEternalLogic();

  return (
    <>
      <div>currently: "{text}"</div>

      <button
        onClick={() => {
          handler((idx) => setText(`done-${idx}`));
        }}
      >
        trigger callback
      </button>
    </>
  );
}
