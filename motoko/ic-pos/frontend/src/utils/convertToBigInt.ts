/**
 * Converts a decimal token amount (given as a string) into raw base units,
 * using string/bigint math to avoid the floating-point rounding that
 * `parseFloat` + `Math.round` would introduce — important for a payment amount,
 * where the wrong base-unit value must never be sent.
 */
export function convertToBigInt(value: string, decimals: number): bigint {
  const trimmed = value.trim();
  if (trimmed === "" || trimmed === "." || !/^\d*\.?\d*$/.test(trimmed)) {
    throw new Error("Amount must be a number.");
  }

  const [whole = "", frac = ""] = trimmed.split(".");
  if (frac.length > decimals) {
    throw new Error(`Amount supports at most ${decimals} decimal places.`);
  }

  return BigInt(`${whole}${frac.padEnd(decimals, "0")}` || "0");
}
