for this test
 * I got the stable memory from our installation on
   `messaging`
 * Installed that (using a very small wasm module that copies
   the install arg to stable memory)
 * Upgraded to our real code
 * Maybe upgraded once more
 * Tested that for various users `lookup` still returns the right data
 * Prepared a delegation to see if the user’s identity is still the
   same, which proves that the secret salt was recovered successfully

This test serves two purposes

 * Ensure backward compatibility with our stable memory format going
   forward.
 * Show that we can indeed fully recover in case of a desaster, assuming
   we have a backup of the stable memory.

To create the user with the key that our tests can log into, I had to

 * Use `wireshark` on a local intaction of `dfx` to figure out the
   DER-encoded public key corresponding to
   `~/.config/dfinity/dfx/identity/default/identity.pem`
 * Used
   ```
   dfx canister --no-wallet --network messaging call internet_identity register '(record {pubkey = blob "(publickeyfromwireshark)"; alias = "dfx"}, record {nonce = 0; timestamp = 0})'
   ```
   the error message tells me the current time stamp
 * Used
   ```
   npx ts-node pow.ts rdmx6-jaaaa-aaaaa-aaadq-cai 1619677860554976404
   ```
   to geneate a PoW nonce
 * Registered
   ```
   dfx canister --no-wallet --network messaging call internet_identity register '(record {pubkey = blob "(publickeyfromwireshark)"; alias = "dfx"}, record {nonce = 6872029500088652; timestamp = 1619677860554976404})'
   ```
 * Added the `webauthPK` used from testing:
   ```
   dfx canister --no-wallet --network messaging call internet_identity add '(10030, record {pubkey = blob "0^0\0c\06\n+\06\01\04\01\83\b8C\01\01\03N\00\a5\01\02\03& \01!X lR\be\ad]\f5, \8a\9b\1c{\e0\a6\08GW>[\e4\acO\e0\8e\a4\806\d0\ba\1d*\cf\"X \b3=\ae\b8;\c9\c7}\8a\d7b\fdh\e3\ea\b0\86\84\e4c\c4\93Q\b3\ab*\14\a4\00\13\83\87"; alias = "testkey"})'
   ```
 * Got a delegation so see my public key there:
   ```
   dfx canister --no-wallet --network messaging call internet_identity prepare_delegation '(10030, "example.com", blob "dummykey", null)'
   ```

To get the stable memory, I logged into a messaging testnet replica,
become root, and copied
```
/var/lib/dfinity-node/ic_state/tip/canister_states/00000000000000070101/stable_memory.bin
```
