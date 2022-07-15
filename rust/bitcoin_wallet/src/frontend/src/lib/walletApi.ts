import type { BackendActor } from './actor';

export interface WalletApi {
  getBalance: () => Promise<bigint>;
  getAddress: () => Promise<string>;

  getFees: () => Promise<{ low: bigint; std: bigint; high: bigint }>;
  send: (address: string, amount: bigint, fee: bigint) => Promise<boolean>;
  deriveNewAddress: () => Promise<void>;
}

export function createIcApi(actor: BackendActor): WalletApi {
  return {
    getBalance: actor.get_balance,
    getAddress: actor.get_principal_address_str,
    getFees: async () => {
      const fees = await actor.get_fees();
      return {
        high: fees[2] / BigInt(1000),
        std: fees[1] / BigInt(1000),
        low: fees[0] / BigInt(1000),
      };
    },
    send: () => sleep(1000).then(() => true),
    deriveNewAddress: () => sleep(1000),
  };
}

function sleep(ms: number, successRate = 1) {
  return new Promise<void>((resolve, reject) =>
    setTimeout(() => {
      if (Math.random() > 1 - successRate) {
        resolve();
      } else {
        reject();
      }
    }, ms)
  );
}

export function createMockApi(successRate = 1): WalletApi {
  return {
    getBalance: () => sleep(1000, successRate).then(() => BigInt(123456789)),
    getAddress: () =>
      sleep(1000, successRate).then(() => 'thisisamockbitcoinaddress'),
    getFees: () =>
      sleep(1000, successRate).then(() => ({
        high: BigInt(27),
        std: BigInt(22),
        low: BigInt(16),
      })),
    send: () => sleep(1000, successRate).then(() => true),
    deriveNewAddress: () => sleep(1000, successRate),
  };
}
