"use client";

import Child from "./_components/child";
import Parent from "./_components/parent";

export default function ContextProviders() {
  return (
    <>
      <Parent className="bg-orange-200 dark:bg-orange-800">
        <Child className="bg-purple-200 dark:bg-purple-800" />
        <Child className="bg-green-200 dark:bg-green-800" />
      </Parent>

      <Parent className="bg-red-200 dark:bg-red-800">
        <Child className="bg-blue-200 dark:bg-blue-800" />
        <Child className="bg-yellow-200 dark:bg-yellow-800" />
      </Parent>
    </>
  );
}
