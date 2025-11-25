"use server";

export async function roundtrip<T>(value: T, msDelay?: number) {
  if (msDelay) await new Promise((done) => setTimeout(done, msDelay));

  return value;
}
