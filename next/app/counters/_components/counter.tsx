"use client";

import { useState } from "react";

export default function Counter() {
  const [count, setCount] = useState(0);

  return (
    <div>
      current count:{" "}
      <button
        className="w-fit inline-block"
        title="increment by one"
        onClick={() => setCount(count + 1)}
      >
        {count}++
      </button>
    </div>
  );
}
