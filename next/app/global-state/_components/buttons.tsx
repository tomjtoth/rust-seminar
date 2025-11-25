import { useGlobalCounter } from "@/app/_lib/store";

export function Incrementer({ incrementBy }: { incrementBy: number }) {
  const cx = useGlobalCounter();

  const op = incrementBy > 0 ? "+" : "-";
  const disabled =
    cx.count + incrementBy > 127 || cx.count + incrementBy < -128;

  return (
    <button
      {...{
        disabled,

        ...(disabled
          ? {
              className: "cursor-not-allowed! text-gray-200 dark:text-gray-800",
            }
          : {}),

        onClick() {
          cx.inc(incrementBy);
        },
      }}
    >
      {cx.count} {op} {Math.abs(incrementBy)}
    </button>
  );
}

export function Nullifier() {
  const cx = useGlobalCounter();

  return (
    <button className="my-5" onClick={() => cx.inc(0)}>
      null it
    </button>
  );
}
