export function formatSats(sats: bigint) {
  return (Number(sats) / 10 ** 8).toFixed(8);
}
