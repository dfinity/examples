import type { BackendActor } from './actor';
import type { TransferResult, TransferError } from './backend';

export interface WalletApi {
  getBalance: () => Promise<bigint>;
  getAddress: () => Promise<string>;

  getFees: () => Promise<{ low: bigint; std: bigint; high: bigint }>;
  send: (
    address: string,
    amount: bigint,
    fee: bigint
  ) => Promise<TransferResult>;
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
    send: (address: string, amount: bigint, fee: bigint) => {
      return actor.transfer(address, amount, fee, false);
    },
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

function sample<T>(arr: T[]): T {
  return arr[Math.floor(Math.random() * arr.length)];
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
    send: (_address: string, _amount: bigint, fee: bigint) =>
      sleep(1000, 1.0).then(() => {
        if (Math.random() > 1 - successRate) {
          return {
            Ok: {
              fee,
              utxos_addresses: [],
              id: `52b73e2d4d30521d86ce1b4191ce3236fca8dc95811e713f0742e39a84646192`,
              size: 5,
              timestamp: BigInt(Date.now()) * BigInt(1000),
            },
          };
        } else {
          return sample<{ Err: TransferError }>([
            { Err: { InsufficientBalance: null } },
            { Err: { InvalidPercentile: null } },
            { Err: { MalformedDestinationAddress: null } },
            { Err: { ManagementCanisterReject: null } },
            { Err: { MinConfirmationsTooHigh: null } },
            { Err: { UnsupportedSourceAddressType: null } },
          ]);
        }
      }),
    deriveNewAddress: () => sleep(1000, successRate),
  };
}
