export function convertToBigInt(num: number): bigint {
  return BigInt(Math.round(num * 100_000_000));
}
