import { Principal } from "@dfinity/principal";
import { describe, expect, test } from "@jest/globals";
import {
  tokenA,
  tokenACanisterId,
  tokenB,
  tokenBCanisterId,
  swap,
  swapCanisterId,
  fundIdentity,
} from "./agent";
import { minter, newIdentity } from "./identity";

describe("swap", () => {
  test("happy path: deposit, swap, and withdraw", async () => {
    // Give alice just enough A
    const alice = newIdentity();
    await fundIdentity(tokenA(minter), alice, 100_020_000n);

    // Give bob just enough B
    const bob = newIdentity();
    await fundIdentity(tokenB(minter), bob, 100_020_000n);

    // Check the initial wallet token balances
    expect(
      await Promise.all([
        tokenA(alice).icrc1_balance_of({
          owner: alice.getPrincipal(),
          subaccount: [],
        }),
        tokenB(alice).icrc1_balance_of({
          owner: alice.getPrincipal(),
          subaccount: [],
        }),
        tokenA(bob).icrc1_balance_of({
          owner: bob.getPrincipal(),
          subaccount: [],
        }),
        tokenB(bob).icrc1_balance_of({
          owner: bob.getPrincipal(),
          subaccount: [],
        }),
      ]),
    ).toEqual([100_020_000n, 0n, 0n, 100_020_000n]);

    // Alice approves 1 A + 0.0001 for fees
    const approvalA = await tokenA(alice).icrc2_approve({
      amount: 100_010_000n,
      created_at_time: [],
      expected_allowance: [],
      expires_at: [],
      fee: [],
      from_subaccount: [],
      memo: [],
      spender: { owner: swapCanisterId, subaccount: [] },
    });
    expect(approvalA).toHaveProperty("Ok");

    // Alice deposits 1 A
    const depositA = await swap(alice).deposit({
      amount: 100_000_000n,
      created_at_time: [],
      fee: [],
      from: { owner: alice.getPrincipal(), subaccount: [] },
      memo: [],
      spender_subaccount: [],
      token: tokenACanisterId,
    });
    expect(depositA).toHaveProperty("ok");

    // Bob approves 1 B + 0.0001 for fees
    const approvalB = await tokenB(bob).icrc2_approve({
      amount: 100_010_000n,
      created_at_time: [],
      expected_allowance: [],
      expires_at: [],
      fee: [],
      from_subaccount: [],
      memo: [],
      spender: { owner: swapCanisterId, subaccount: [] },
    });
    expect(approvalB).toHaveProperty("Ok");

    // Bob deposits 1 A
    const depositB = await swap(bob).deposit({
      amount: 100_000_000n,
      created_at_time: [],
      fee: [],
      from: { owner: bob.getPrincipal(), subaccount: [] },
      memo: [],
      spender_subaccount: [],
      token: tokenBCanisterId,
    });
    expect(depositB).toHaveProperty("ok");

    // Check deposited balances
    var balances = await swap(minter).balances();
    var tokenABalances: Record<string, bigint> = Object.fromEntries(
      balances[0],
    );
    var tokenBBalances: Record<string, bigint> = Object.fromEntries(
      balances[1],
    );

    // Alice should have 1 A deposited
    const alicePrincipal = alice.getPrincipal().toString();
    expect(tokenABalances[alicePrincipal]).toBe(100_000_000n);

    // Bob should have 1 B deposited
    const bobPrincipal = bob.getPrincipal().toString();
    expect(tokenBBalances[bobPrincipal]).toBe(100_000_000n);

    // Do the swap
    const swapResult = await swap(minter).swap({
      user_a: alice.getPrincipal(),
      user_b: bob.getPrincipal(),
    });
    expect(swapResult).toHaveProperty("ok");

    // Check deposited balances
    balances = await swap(minter).balances();
    tokenABalances = Object.fromEntries(balances[0]);
    tokenBBalances = Object.fromEntries(balances[1]);

    // Alice should have 1 B deposited
    expect(tokenBBalances[alicePrincipal]).toBe(100_000_000n);

    // Bob should have 1 A deposited
    expect(tokenABalances[bobPrincipal]).toBe(100_000_000n);

    // Alice withdraws TokenB
    const withdrawalA = await swap(alice).withdraw({
      amount: 100_000_000n - 10_000n,
      created_at_time: [],
      fee: [],
      to: { owner: alice.getPrincipal(), subaccount: [] },
      memo: [],
      token: tokenBCanisterId,
    });
    expect(withdrawalA).toHaveProperty("ok");

    // Bob withdraws TokenA
    const withdrawalB = await swap(bob).withdraw({
      amount: 100_000_000n - 10_000n,
      created_at_time: [],
      fee: [],
      to: { owner: bob.getPrincipal(), subaccount: [] },
      memo: [],
      token: tokenACanisterId,
    });
    expect(withdrawalB).toHaveProperty("ok");

    // Check the wallet token balances have changed as expected
    expect(
      await Promise.all([
        tokenA(alice).icrc1_balance_of({
          owner: alice.getPrincipal(),
          subaccount: [],
        }),
        tokenB(alice).icrc1_balance_of({
          owner: alice.getPrincipal(),
          subaccount: [],
        }),
        tokenA(bob).icrc1_balance_of({
          owner: bob.getPrincipal(),
          subaccount: [],
        }),
        tokenB(bob).icrc1_balance_of({
          owner: bob.getPrincipal(),
          subaccount: [],
        }),
      ]),
    ).toEqual([
      0n, // Alice should have 0 A
      99_990_000n, // Alice should have 0.9999 B
      99_990_000n, // Bob should have 0.9999 A
      0n, // Bob should have A B
    ]);
  }, 60_000); // 60 second timeout

  describe("error handling", () => {
    test("deposit with invalid token argument", async () => {
      // Give alice just enough A
      const alice = newIdentity();

      // Alice tries to deposit a token that does not exist
      await swap(alice)
        .deposit({
          amount: 100_000_000n,
          created_at_time: [],
          fee: [],
          from: { owner: alice.getPrincipal(), subaccount: [] },
          memo: [],
          spender_subaccount: [],
          token: Principal.fromText("aaaaa-aa"),
        })
        .catch((e) => {
          expect(e.message).toMatch(/invalid token canister/);
        });
    }, 60_000); // 60 second timeout

    test("deposit fails with insufficient approval", async () => {
      // Give alice just enough A
      const alice = newIdentity();
      await fundIdentity(tokenA(minter), alice, 100_020_000n);

      // Alice approves 0.5 A + 0.0001 for fees
      const approvalA = await tokenA(alice).icrc2_approve({
        amount: 50_010_000n,
        created_at_time: [],
        expected_allowance: [],
        expires_at: [],
        fee: [],
        from_subaccount: [],
        memo: [],
        spender: { owner: swapCanisterId, subaccount: [] },
      });
      expect(approvalA).toHaveProperty("Ok");

      // Check the initial wallet token balances
      expect(
        await Promise.all([
          tokenA(alice).icrc1_balance_of({
            owner: alice.getPrincipal(),
            subaccount: [],
          }),
        ]),
      ).toEqual([100_010_000n]);

      // Alice tries to deposit 1 A. This will fail because only 0.5A has
      // been approved.
      const depositA = await swap(alice).deposit({
        amount: 100_000_000n,
        created_at_time: [],
        fee: [],
        from: { owner: alice.getPrincipal(), subaccount: [] },
        memo: [],
        spender_subaccount: [],
        token: tokenACanisterId,
      });
      expect(depositA).toMatchObject({
        err: {
          TransferFromError: {
            InsufficientAllowance: {
              allowance: 50_010_000n,
            },
          },
        },
      });

      // Check the user's wallet token balance is unchanged
      expect(
        await Promise.all([
          tokenA(alice).icrc1_balance_of({
            owner: alice.getPrincipal(),
            subaccount: [],
          }),
        ]),
      ).toEqual([100_010_000n]);
    }, 60_000); // 60 second timeout

    test("withdrawing more than the user's balance fails with InsufficientFunds", async () => {
      // Give alice just enough A
      const alice = newIdentity();

      // Alice tries to withdrawal 1 A. This will fail because their
      // deposited A balance is 0.
      const withdrawalA = await swap(alice).withdraw({
        amount: 100_000_000n,
        created_at_time: [],
        fee: [],
        to: { owner: alice.getPrincipal(), subaccount: [] },
        memo: [],
        token: tokenACanisterId,
      });
      expect(withdrawalA).toMatchObject({
        err: {
          InsufficientFunds: {
            balance: 0n,
          },
        },
      });
    }, 60_000); // 60 second timeout
  });
});
