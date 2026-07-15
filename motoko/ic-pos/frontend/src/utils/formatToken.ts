/**
 * Formats a raw token amount (in base units) into a human-readable decimal
 * string, given the token's decimal precision.
 *
 * Pure and symbol-agnostic — the caller appends the token symbol.
 */
export function formatToken(
  amount: bigint | number | null | undefined,
  decimals: number
): string {
  if (amount === undefined || amount === null) return "0";
  amount = typeof amount === "number" ? BigInt(amount) : amount;
  if (!amount) return "0";
  const base = 10n ** BigInt(decimals);
  const integerPart = amount / base;
  const fractionalPart = amount % base;
  const fractionalPartString = fractionalPart
    .toString()
    .padStart(decimals, "0");
  const fractionalPartTrimmed = fractionalPartString.replace(/0+$/, ""); // Removes trailing zeroes
  return `${integerPart.toLocaleString()}.${fractionalPartTrimmed}`;
}
