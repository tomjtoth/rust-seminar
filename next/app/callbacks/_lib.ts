import { useState } from "react";

export function useExternalLogic() {
  const [index, setIndex] = useState(0);

  return function handler(callback: (index: number) => void) {
    const incremented = index + 1;
    setIndex(incremented);

    callback(incremented);
  };
}
