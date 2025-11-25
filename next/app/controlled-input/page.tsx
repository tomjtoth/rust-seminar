"use client";

import { useState } from "react";

export default function ControlledInput() {
  const [value, setValue] = useState("");

  return (
    <form
      className="flex flex-row gap-2"
      onSubmit={() => console.debug("you shall not PASS/be printed ever")}
    >
      <label>
        click me to focus the input{" "}
        <input
          type="text"
          className="ml-2"
          placeholder="text here"
          value={value}
          onChange={(ev) => setValue(ev.target.value)}
        />
      </label>

      <button
        onClick={(ev) => {
          ev.preventDefault();
          setValue("CHEESE");
        }}
      >
        say "CHEESE"
      </button>
    </form>
  );
}
