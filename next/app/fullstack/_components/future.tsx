import { useEffect, useState } from "react";

import { roundtrip } from "../_lib";

export default function Future() {
  const [value, setValue] = useState("");
  const [trip, setTrip] = useState(0);

  useEffect(() => {
    roundtrip("value from initial roundtrip", 1000).then(setValue);
  }, [trip]);

  return (
    <>
      <label>
        useEffect for async ops:{" "}
        <input
          type="text"
          placeholder="type here"
          value={value}
          onChange={(ev) => roundtrip(ev.target.value).then(setValue)}
        />
      </label>

      <button className="inline-block" onClick={() => setTrip(1 - trip)}>
        reset slowly
      </button>
    </>
  );
}
