export function formatCkBtc(amount: bigint | number | null | undefined) {
  if (amount === undefined) return "0";
  amount = typeof amount === "number" ? BigInt(amount) : amount;
  if (!amount) return "0";
  const integerPart = amount / 100000000n;
  const fractionalPart = amount % 100000000n;
  const fractionalPartString = fractionalPart.toString().padStart(8, "0");
  const fractionalPartTrimmed = fractionalPartString.replace(/0+$/, ""); // Removes trailing zeroes
  return `${integerPart.toLocaleString()}.${fractionalPartTrimmed}`;
}
