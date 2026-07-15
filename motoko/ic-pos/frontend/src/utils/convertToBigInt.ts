/**
 * Converts a decimal token amount into raw base units, given the token's
 * decimal precision.
 */
export function convertToBigInt(num: number, decimals: number): bigint {
  return BigInt(Math.round(num * 10 ** decimals));
}
