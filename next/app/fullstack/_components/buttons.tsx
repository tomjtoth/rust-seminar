import { useGlobalCounter } from "@/app/_lib/store";
import { roundtrip } from "../_lib";

export function Incrementer({
  incrementBy,
  title,
}: {
  incrementBy: number;
  title?: string;
}) {
  const globalState = useGlobalCounter();

  const op = incrementBy > 0 ? "+" : "-";
  const disabled =
    globalState.count + incrementBy > 127 ||
    globalState.count + incrementBy < -128;

  return (
    <button
      {...{
        disabled,
        title,

        ...(disabled
          ? {
              className: "cursor-not-allowed! text-gray-200 dark:text-gray-800",
            }
          : {}),

        async onClick() {
          const res = await roundtrip(incrementBy);
          globalState.inc(res);
        },
      }}
    >
      {globalState.count} {op} {Math.abs(incrementBy)}
    </button>
  );
}

export function Nullifier() {
  const cx = useGlobalCounter();

  return (
    <button
      className="my-5"
      onClick={async () => {
        const res = await roundtrip(0);
        cx.inc(res);
      }}
    >
      null it
    </button>
  );
}
