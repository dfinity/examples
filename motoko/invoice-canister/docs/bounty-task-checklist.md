## BNT-2 - Invoice Bounty Task Check Sheet ## 

##### This is a list of all the tasks specified as part of the bounty to be completed before submission for approval. #####

They can be originally found at [BNT-2: Invoice Canister #2](https://github.com/dfinity/grant-rfps/issues/2).

This was created to make it easier to know what needed to be done as to verify everything has been completed.

All the referenced line numbers should be correct to within one or two lines (some revisions were completed after this was "finally" drafted). 

In addition to these bounty tasks, two other non-trivial changes include using the literal of a generated ULID for an invoice's id (instead of the invoice creation counter value). ULID was chosen for its timestamp encoding and human copy and paste friendly dash-lacking format; there's an example of decoding the timestamp from an invoice's ulid in the frontend of the `motoko-seller-client` example. Also note a monotonic invoice creation counter is still used and available if needed for an invoice record. The other change was upgrading from the volatile hashmap to the stable compatible trie to remove the need for using the pre and postupgrade system hooks. 

In addition to this, there's also a [Testing Glossary](./docs/TestingGlossay.md) showing an example output of all the unit and E2E tests, as well as an example of the [startup script's output](./docs/clean-startup-console-output.md) showing a shell running the `clean-startup-mjs` script with the `deployForTesting` flag.

---

### Support for ICRC-1 fungible token standard ###  
  ㅤ Generally for every operation involving an ICP address or transaction there needs to be an equally working operation for an ICRC1 address or transaction.  
  #### Main subtasks: ####  

 - [x] **Incorporate the actual ICRC1 token-ledger canister(s).**  
  ✶ With the development of `SupportedToken` to demonstrate it works, two ICRC1 token-canister ledgers are installed and deployed from a downloaded wasm and did of the offical Dfinity Rosetta repository for ICRC1. 
    - [x] `src/token-ledger-canisters/icrc1/` (`icrc1.did` & `ledger.wasm`) (template shell script also provided).  
    - [x] Updated `dfx.json` `icrc1_token_ledger_canister_ex1` & `..._ex2` (lines [29](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/dfx.json#L29)-40).  
    - [x] Update the install script to deploy both and prepare for E2E testing. 
      - [x] `clean-startup.mjs::deploy_icrc1_token_canister()` (lines [189](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/clean-startup.mjs#L189)-214, [370](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/clean-startup.mjs#L370)-374).  
      - [x] `clean-startup.mjs::disburse_funds_to_nnsFundedSecp256k1Identity_creator_subaccounts()` (lines [250](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/clean-startup.mjs#L250)-270, 377).  
  
- [x] **Adding the required ICRC1 token-ledger canister typings:**  
    ✶ [src/invoice/modules/supported-token/token-specific/icrc1/Types.mo](./src/invoice/modules/supported-token/token-specific/icrc1/Types.mo)  

- [x] **For consistency and integration adding an ICRC1 token-ledger canister actor supertype:**  
    ✶ [src/invoice/modules/supported-token/token-specific/icrc1/ActorSupertype.mo](./src/invoice/modules/supported-token/token-specific/icrc1/ActorSupertype.mo)  

- [x] **Adding the logic for ICRC1 addressing computations:**  
  - [x] [src/invoice/modules/supported-token/token-specific/icrc1/Adapter.mo](./src/invoice/modules/supported-token/token-specific/icrc1/Adapter.mo)  
     - [x] `isValidSubaccount()`  
     - [x] `isValidAddress()`  
     - [x] `encodeAddress()`  
     - [x] `decodeAddress()`  
     - [x] `computeInvoiceSubaccount()`  
     - [x] `computeInvoiceSubaccountAddress()`  
     - [x] `computeCreatorSubaccount()`  
     - [x] `computeCreatorSubaccountAddress()`  
  - [x] Each has at least one unit test in `test/unit/Test.mo`  
      ✶ `describe("ICRC1 Adapter Account and Subaccount Computations"...` (lines [270](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/unit/Test.mo#L270)-422).  

- [x] **Adding the logic connecting those addressing transformations to the invoice canister's API methods:**
  - [x] [src/invoice/modules/supported-token/SupportedToken.mo](./src/invoice/modules/supported-token/SupportedToken.mo).  
        _ㅤVariant tags mapping the two ICRC1 tokens:_  
    - [x] `SupportedToken<T1, T2>` (`#ICRC1_ExampleToken` & `#ICRC1_ExampleToken2` (lines [63](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/supported-token/SupportedToken.mo#L63)-64).  
        _ㅤRelated `SupportedToken`'s types each with corresponding ICRC1 type:_  
    - [x] `UnitType = SupportedToken<(), ()>`  
    - [x] `Amount = SupportedToken<TokenSpecific.ICP.Tokens, TokenSpecific.ICRC1.Tokens>`  
    - [x] `Address = SupportedToken<TokenSpecific.ICP.AccountIdentifier, TokenSpecific.ICRC1.Account>`  
    - [x] `TransferArgs = SupportedToken<TokenSpecific.ICP.TransferArgs, TokenSpecific.ICRC1.TransferArgs>`  
    - [x] `TransferResult = SupportedToken<TokenSpecific.ICP.TransferResult, TokenSpecific.ICRC1.TransferResult>`  
    - [x] `TransferSuccess = SupportedToken<TokenSpecific.ICP.BlockIndex, TokenSpecific.ICRC1.TxIndex>`  
    - [x] `TransferErr = SupportedToken<TokenSpecific.ICP.TransferError, TokenSpecific.ICRC1.TransferError>`    
        _ㅤRelated `SupportedToken`'s methods each with corresponding ICRC1 cases:_  
    - [x] `getTransactionFee()`  
    - [x] `unwrapTokenAmount()`   
    - [x] `wrapAsTokenAmount()`  
    - [x] `getTokenVerbose()`  
    - [x] `encodeAddress()`  
    - [x] `encodeAddressOrUnitErr()`  
    - [x] `getAddressOrUnitErr()`  
    - [x] `getInvoiceSubaccountAddress()`  
    - [x] `getEncodedInvoiceSubaccountAddress()`  
    - [x] `getCreatorSubaccountAddress()`  
    - [x] `getTransferArgsFromInvoiceSubaccount()`  
    - [x] `getTransferArgsFromCreatorSubaccount()`  
    - [x] `rewrapTransferResults()`  
    - [x] `getDefaultSubaccountAddress()`  

  - [x] **Unit testing for each of the above methods in `test/unit/Test.mo`.**  
          _ㅤEach includes its own subsuite-set of test cases ("describe">"it") for each token type._  
      - [x] `describe("Supported Token Types' and Methods"...` (full set omitted here, lines [426](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/unit/Test.mo#L426)-1651).  

  - [x] **Implementing the actual use of the above methods in the invoice canister's API methods in [src/invoice/Invoice.mo](./src/invoice/Invoice.mo):**  
    - [x] `create_invoice()`  
    - [x] `get_caller_address()`  
    - [x] `get_caller_balance()`  
    - [x] `get_invoice()`  
    - [x] `verify_invoice()`  
    - [x] `transfer()`  
    - [x] `recover_invoice_subaccount_balance()`  
    - [x] `to_other_address_format()`  
  
  - [x] **E2E testing for each above the methods in `test/e2e/src/tests/`:**  
        _ㅤEach of the following includes its own subsuite-sets of test cases ("describe">"it") for each token type and other test case conditions where appropiate (full set omitted here, it's a long list)._  
    - [x] [create_invoice.test.js](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/create_invoice.test.js)   
    - [x] [get_caller_address.test.js](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/get_caller_address.test.js)   
    - [x] [get_caller_balance.test.js](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/get_caller_balance.test.js)   
    - [x] [get_invoice.test.js](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/get_invoice.testjs)  
    - [x] [verify_invoice.test.js](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/verify_invoice.test.js)  
    - [x] [transfer.test.js](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/transfer.test.js)  
    - [x] [recover_invoice_subaccount_balance.test.js](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/recover_invoice_subaccount_balance.test.js)  
    - [x] [to_other_address_format.test.js](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/to_other_address_format.test.js)  
  
- [x] **Updating `motoko-seller-client` project to demonstrate seller flow also integrating ICRC1 compatable invoice canister:**  
        _ㅤProject scope change: instead of four, only two two tokens and corresponding `SupportedToken` variant tags are used: `#ICP` and `#ICRC1`._  

    - [x] **Adding mock token-canister ledgers[^6].**   
      ✶ `examples/motoko-seller-client/src/backend/modules/MockTokenLedgerCanisters.mo...`  
          _ㅤShould correctly return every #Ok/#Err result of balance/transfer except ICRC1's Generic/TempUnavailable Err._   
      - [x] [...ICP.MockLedger](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/backend/modules/MockTokenLedgerCanisters.mo#L96) (lines 96-238; also with [deposity_free_money()](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/backend/modules/MockTokenLedgerCanisters.mo#L211)).  
      - [x] [...ICRC1.MockLedger](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/backend/modules/MockTokenLedgerCanisters.mo#L315) (lines 315-450; also with [deposity_free_money()](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/backend/modules/MockTokenLedgerCanisters.mo#L433)).  

    - [x] **Updating the backend**
      - [x] Updating [examples/motoko-seller-client/src/backend/modules/SupportedToken.mo](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/backend/modules/SupportedToken.mo):   
        - [x] For each corresponding methods (and [SupportedToken<> variant](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/backend/modules/SupportedToken.mo#L523)) two cases instead of the previous four.  
      - [x] Updating [examples/motoko-seller-client/src/backend/Invoice.mo](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/backend/Invoice.mo):  
        - [x] Editing to do two instead of four cases for each API method.  
        - [x] Adding the seller as an authorized allowed creator.  
        - [x] Updating the `deposit_free_money` logic to handle both ICP and ICRC1 token types.  
      - [x] Updatting [examples/motoko-seller-client/src/backend/Seller.mo](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/backend/Seller.mo):  
        - [x] Correctly importing `Invoice` canister now that it's a class actor.  
        - [x] Updating each method to still do expected functionality.  
  
    - [x] **Updating the frontend**  
      ✶ `examples/motoko-seller-client/src/frontend/...`:  
        - [x] Adding the needed known identities [.../src/identity.js](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/frontend/src/identity.js) to correctly create actor types.  
        - [x] Updating `.../src/components/Invoice.jsx`:   
          - [x] to handle accepting both ICP and ICRC1 token types (lines [93](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/frontend/src/components/Invoice.jsx#L93)-140).   
          - [x] to display payment address correctly (line [147](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/frontend/src/components/Invoice.jsx#L147)).  
          - [x] to display creation timestamp decoded from ULID (lines [62](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/frontend/src/components/Invoice.jsx#L62)-91, [123](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/frontend/src/components/Invoice.jsx#L123), [153](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/frontend/src/components/Invoice.jsx#L153)-156). 
        - [x] Updating `.../src/components/InvoicePayDialog.jsx`: (lines 68, 71).
          - [x] [to allow selection of payment token type](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/frontend/src/components/InvoicePayDialog.jsx#L28) (lines 14, 15, 17-24, 28-39, 43-50). 
        - [x] Updating `.../src/components/InvoiceManager.jsx`:  
          - [x] to handle creating, getting both token types (lines [19-36](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/frontend/src/components/InvoiceManager.jsx#L30)).  
          - [x] fixed bug showing previous invoice while new one is being created between displaying different invoices (lines [13-17](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/frontend/src/components/InvoiceManager.jsx#L13), 47).  
        - [x] Updating `.../src/components/Payment.jsx`:
          - [x] to process initiating payment for either token type (lines [10-18](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/examples/motoko-seller-client/src/frontend/src/components/Payment.jsx#L10), 22-25).  

### Invoice Canister Cleanup Tasks #292 ###  
 - [x] https://github.com/dfinity/examples/issues/292  
    ㅤPre-existing issues that remained to be resolved. In particular:  
    - [x] Add access control for creating new invoices (see [SEC-F20] & [SEC-F21] below).    
    - [x] Refactor permission checks to a method.  
      ✶ `src/invoice/Invoice.mo::getInvoiceIfAuthorized()` ([lines 100-142](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L100)).  
    - [x] Additionally, when first starting this bounty independently of any work I was doing, the startup scripting was being migrated to use zx which coincidentally at the time I had just become interested in. There's likely an ideal niche for dfx cli and zx for example in making dynamic canister deployment easier particularly as the javascript can console log out the arg as a literal without the explicit need of using it with zx. In any case as a result this migration was completed in the form of [`clean-startup.mjs`](./clean-startup.mjs).  
  
### Prevent arithmetic overflow when amount in TransferArgs is below 10_000 #35 ###  
  - [x] https://github.com/dfinity/invoice-canister/issues/35  
    ㅤThis became a two part issue with a follow up. With the added refund logic, there are three scenarios when the invoice canister calls the transfer logic of a token-ledger canister and before that call can take place it can result in an arithmetic overflow or underflow (if it is `Nat64` or `Nat`)[^1] when the transfer fee is subtracted from the amount to transfer. These three scenarios are:  

      1) User calls to transfer a specific amount from their creator subaccount.  
      2) User calls for a partial refund or recovery of missent funds from an invoice subaccount.  
      3) During the normal life cycle of an invoice upon successful verification of payment when the proceeds are transferred from the invoice's subaccount to the subaccount of that invoice's creator.  
  
    ㅤTo account for the first two cases the `#err kind #InsufficientTransferAmount` is added; to account for the third `#err kind #InsufficientAmountDue` is added since proceeds of invoices with an amount due less than the transfer fee are effectively irrecoverable[^2] if each invoice has it's own subaccount as a payment address. _Although ICRC1 token-ledger canisters can handle the fee automatically (as an opt transfer arg), it was easier to normalize preventing the error than handling its return which, in addition to also including the necessary support for ICP ledgers, provides a more uniform API for the user._   
    ㅤAs this issue is specifically resolved in the code:  
    - [x] **`create_invoice()`:**  
        ✶ `src/Invoice/modules/Types.mo` ([line 204: `#InsufficientAmountDue;`](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/Types.mo#L204)).  
        ✶ `src/Invoice/Invoice.mo` ([lines 229-235](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L232)).  
        ✶ `test/e2e/src/tests/create_invoice.test.js` ([lines 520-549 all four token types tested](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/create_invoice.test.js#L520)).  
    - [x] **`transfer()`:**  
        ✶ `src/Invoice/modules/Types.mo` ([line 390: `#InsufficientTransferAmount;`](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/Types.mo#L390)).  
        ✶ `src/Invoice/Invoice.mo` ([lines 661-667](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L665)).  
        ✶ `test/e2e/src/tests/transfer.test.js` ([lines 308-356, all four token types tested](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/transfer.test.js#L308)).  
    - [x] **`recover_invoice_subaccount_balance()`:**  
        ✶ `src/Invoice/modules/Types.mo` ([line 436: `#InsufficientTransferAmount;`](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/Types.mo#L436)).  
        ✶ `src/Invoice/Invoice.mo` ([lines 825-832](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L828)).  
        ✶ `test/e2e/src/tests/transfer.test.js`:  
        ㅤ✶ [lines 313-349 (#ICP)](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/recover_invoice_subaccount_balance.test.js#L313).  
        ㅤ✶ [lines 539-575 (#ICP_nns)](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/recover_invoice_subaccount_balance.test.js#L313).  
        ㅤ✶ [lines 769-805 (#ICRC1_ExampleToken)](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/recover_invoice_subaccount_balance.test.js#L313).  
        ㅤ✶ [lines 1004-1041 (#ICRC1_ExampleToken2)](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/recover_invoice_subaccount_balance.test.js#L313).  

### [SEC-F27] principalToSubaccount uses no domain separator #28 ###  
  - [x] https://github.com/dfinity/invoice-canister/issues/28  
    For both ICP and ICRC1 computed subaccounts:  
    - [x] ICP:  
      ✶ `src/invoice/modules/supported-token/token-specific/icp/Adapter.computeCreatorSubaccount()` ([line 99](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/supported-token/token-specific/icp/Adapter#L99)).  
      ㅤㅤ↳was previousily `src/invoice/Account.mo::principalToSubaccount()` 
    - [x] ICRC1:  
      ✶ `src/invoice/modules/supported-token/token-specific/icrc1/Adapter.computeCreatorSubaccount()` ([line 114](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/supported-token/token-specific/icp/Adapter#L114)).  
 
### [SEC-F21] Anonymous principal has an account #25 ###  
  - [x] https://github.com/dfinity/invoice-canister/issues/25  
    ㅤRepresented by the logic of three different methods, depending on where the check is occuring; all the canister's API methods checks against the anonymous principal by calling at least one of the following:  
    - [x] Preventing the canister installer from adding an allowed creator as the anonymous principal.  
      ✶ `src/invoice/Invoice.mo` ([line 286](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L286) and `src/invoice/modules/Types.mo` [line 107](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/Types.mo#L107)).  
      _ㅤ(which in turn prevents the anonymous principal from calling any other method, specifically because checks done in all the other methods by one or the other of the following two then prevent)_  
    - [x] Unauthorized calls by principals not on allowed creators list when the call is not invoice specific.  
      ✶ `src/invoice/Invoice.mo::hasCallPermission_()` ([lines 92-94](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L92).    
      _ㅤㅤ(and if it is)_  
      ✶ `src/invoice/Invoice.mo::getInvoiceIfAuthorized_()` ([lines 100-142](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L100)).  
      _ㅤㅤ(which uses the previous `hasCallPermission_()` method)_   

    ✶ [test/e2e/src/tests/disallowAnonymous.test.js](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/disallowAnonymous.test.js) (entire file) demonstrates verified coverage for each API method.  

### [SEC-F05] TOCTOU in verify_invoice #21 ###  
  - [x] https://github.com/dfinity/invoice-canister/issues/21  
    ㅤThis does not have an explicit test at this time. That being said, a means to resolving this issue is implemented by a lock synchronizing access of an invoice by its id when either the `verify_invoice()` or `recover_invoice_subaccount_balance()` is called. In turn either method could trigger a transfer from that invoice's subaccount which with ungaurded concurrent access could lead to problems as discussed in that issue (and potentially more with the added recovery of funds functionality).  
    ㅤTo ensure the lock itself does not become a problem, it is implemented with an auto-expiring timeout; all inter-canister calls are wrapped with a try/catch; other code in the scope of the lock has been tested and accounted for (preventing trapping from subtracting amounts less than a transfer fee, for example). This means of resolution was brought up on the forums as well, and this approach given tentantive approval (the timeout may itself prevent problems, but if either method takes longer than  ten minutes--the currently set expiration time--either method would likely need to be called again); while there might be a better built-in solution available for Motoko, it is not yet available.  
    ㅤAs invoices are already access controlled (only callers on verify permission list could call either method for a given invoice) and are only linked as a sender from their own subaccount, this issue is even more of an edge case. The auto-expiring lock prevents it and other potential issues from concurrent calls to verify and recover causing problems. To see the specific code:  
      ✶ `src/invoice/Invoice.mo` ([map and timeout declarations line 74 & 78](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L74)).  
      ✶ `src/invoice/Invoice.mo::verify_invoice()` ([each branch covered lines 472-629](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L427)).  
      ✶ `src/invoice/Invoice.mo::recover_invoice_subaccount_balance()` ([each branch covered lines 773-894](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L773)).  

### [SEC-F12] Copied libraries #20 ###  
  - [x] https://github.com/dfinity/invoice-canister/issues/20  
    ㅤUpgrading the invoice canister to use libraries as opposed to hard-coded copying of sha256, crc32, and hex libraries. This is done by adding [Aviate Labs](https://github.com/aviate-labs) [Internet Computer Open Services](https://github.com/internet-computer/) package set as an additional upstream in `package-set.dhall` to make the `"array", "crypto", "hash", "encoding", "principal"` dependencies available in `vessel.dhall`. The existing addressing computations for account identifiers is updated as well as 1-1 Motoko unit tests with existing tests to show equivalence between the two implementations (added a tag to jump to that commit which is no longer a part of visible code base)[^3]. Most of those same methods as they are now:
     
    - [x] `src/invoice/modules/supported-token/token-specific/icp/Adapter.computeInvoiceSubaccount()` ([lines 64-78](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/supported-token/token-specific/icp/Adapter.mo#L64)).    
         ㅤ↳was previousily `src/invoice/Utils.mo::generateInvoiceSubaccount()`  

    - [x] `src/invoice/modules/supported-token/token-specific/icp/Adapter.computeCreatorSubaccount()` ([lines 94-106](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/supported-token/token-specific/icp/Adapter.mo#L94)).  
         ㅤ↳was previousily `src/invoice/Account.mo::principalToSubaccount()`  

    - [x] `src/invoice/modules/supported-token/token-specific/icp/Adapter.computeInvoiceSubaccountAddress()` ([lines 82-91](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/supported-token/token-specific/icp/Adapter.mo#L82)).  
         ㅤ↳was previousily `src/invoice/Account.mo::accountIdentifier()`   

    - [x] `src/invoice/modules/supported-token/token-specific/icp/Adapter.computeCreatorSubaccountAddress()` ([lines 110-118](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/supported-token/token-specific/icp/Adapter.mo#L110)).  
         ㅤ↳was previousily `src/invoice/Account.mo::accountIdentifier()`   

    - [x] `src/invoice/modules/supported-token/token-specific/icp/Adapter.isValidAddress()`) ([lines 37-44](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/supported-token/token-specific/icp/Adapter.mo#L37)).  
         ㅤ↳was previousily `src/invoice/Account.mo::validateAccountIdentifier()`   

    ㅤNote that the `ICRC1.Adapter` module also uses these libraries for its `computeCreatorSubaccount()` method.  
    There's also coverage of the entire `ICP.Adapter` module (same as `ICRC1.Adapter` above) using these library dependencies at:  
    - [x] `test/unit/Test.mo`  
      ㅤ✶ `describe("ICP Adapter AccountIdentifier and Subaccount Computations")` ([lines 109-269](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/unit/Test.mo#L109)).   
    ✶ There's also a git tag (816165a-LibrariesEquivalence) showing equivalence between previous implementation and current implementation (although creator subaccount addresses changed as a result of adding the new separator).

### [SEC-F17] Uncertified Queries #16 ###  
  - [x] https://github.com/dfinity/invoice-canister/issues/16  
    ㅤWhile adding the `CertifiedMap` library is a potential option for future development, all the calls have been made update calls (as well as discussion regarding why in the non-generated developer docs):   
     
    - [x] `src/Invoice.mo::get_invoice()` ([line 347](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L347)).  
    - [x] `src/Invoice.mo::get_caller_balance()` ([line 336](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L336), previousily `get_balance`).  
    - [x] `src/Invoice.mo::get_caller_address()` ([line 422](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L422), previousily `get_account_identifier`).  
    - [x] `src/Invoice.mo::to_other_address_format()` ([line 922](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L922)).  

### [SEC-F30] Funds can get stuck in invoice accounts #13 ###  
  - [x] https://github.com/dfinity/invoice-canister/issues/13  
    ㅤNow any amount more than the transfer fee cost can be transferred out of an invoice subaccount by its creator or those on its verify permission list. If the invoice has not yet been verified, this could serve as a refund; if the invoice has already been verified, this method can be used to recover those funds sent after payment has already been made. It should be noted this is not a refund for invoices already verified as those funds are moved to the creator's subaccount when the balance paid is confirmed to be greater or equal to the invoice's amount due. Since the verification and balance recovery methods are synchronized by invoice id, a call to recover funds will only happen after the invoice verification is complete (see [SEC-F05] above). The `recover_invoice_subaccount_balance()` also has extensive E2E testing and demonstrates testing of nearly all the invoice canister's functionality. 
    - [x] `src/Invoice.mo::recover_invoice_subaccount_balance()` ([lines 751-910](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L751)).  
    - [x] Related `src/modules/Types.mo` declarations ([lines 416-458](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/Types.mo#L402)).   
    - [x] [recover_invoice_subaccount_balance.test.js](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/recover_invoice_subaccount_balance.test.js) 
     ㅤ✶ (entire file, each token has its own test subsuite, in addition to non-token specific tests).  

### [SEC-F20] Controller of canister could take all funds by upgrading 12 ###  
  - [x] https://github.com/dfinity/invoice-canister/issues/12  
    ㅤImplementating access control can get very complicated, particularly if the objective is to use the invoice canister for managing the finances of a DAO. While not impossible, it is beyond the scope of this bounty. That being said an allowed creators list is added such that the original invoice canister installer has the unique right to add or remove principals from the allowed creators list. It should be noted other than this (and the fact they are the original installer) they have no special rights: for instance if they are not an invoice's creator or on its get or verify permissions list, they cannot get or verify or recover funds of that invoice by calling the canister's API; similarly they cannot arbitrarily transfer funds out of any subaccount with the code base as it is. As for principals on the allowed creators list, they too can call every method of the invoice canister API except adding, removing or getting the allowed creators list. In addition to the logic added in code there is extension dicussion of this in the `docs/DesignDocs.md` (lines 33-54). 
    _ㅤThere is a simple implementation of adding a 'delegatedAdministrator' who would also have the ability to add and remove allowed creators, originally as an optional deployment argument and then with an "official" getter and setter in the API list, as this might be a useful feature for the invoice canister to have (a developer could setup and maintain the canister for a storefront, and give the storefront the "administrator" access control while still being able to do DX tech support/dev ops; or for blackholing the canister), but this is left as an exercise for the developer to implement as there are libraries that can do that and much more out there if that is a needed feature (that code is tag commited[^4])._   

    ㅤTo see all the related code:  
    - [x] `src/module/Types.mo` ([lines 82-161, add/remove/get allowed creators list API types](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/modules/Types.mo#L68)).  
    - [x] `src/Invoice.mo::allowedCreatorsList_` ([line 65, stable principal array](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L65)).  
    - [x] `src/Invoice.mo::add_allowed_creator()` ([lines 278-307](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L278)).  
    - [x] `src/Invoice.mo::remove_allowed_creator()` ([lines 311-334](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L311)). 
    - [x] `src/Invoice.mo::get_allowed_creators_list()` ([lines 338-343](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/src/invoice/Invoice.mo#L338)). 
    - [x] [test/e2e/src/tests/allowedCreatorsList.test.js](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/test/e2e/src/tests/allowedCreatorsList.test.js) (entire file, test subsuites for all the above methods).
    - [x] `test/e2e/src/tests/*.test.js` (used by most other test suites as well).

### [SEC-F29] Incomplete design documentation #19 ###
  - [x] https://github.com/dfinity/invoice-canister/issues/19
    - [x] Extensive Motokodoc and in-method-body comments literally everywhere / generated `dev docs`.
    - [x] All testing is also extensively commented.
    - [x] Standalone (non-generated) developer doc [DesignDoc.md](./docs/DesignDoc.md) covering specifications, API, security and critical aspects.  
    - [x] Commented [invoice.did](./invoice.did) at project's root.  
    - [x] Coverage in [README](./README.md) including integration of invoice canister.   
    - [x] [Diagram flow chart](./docs/invoice-payment-flow.png) of typical "ok" invoice lifecycle.  

### [SEC-F22] Potentially sensitive invoice details are stored in plain text on the canister #26 ###
  - [x] https://github.com/dfinity/invoice-canister/issues/26  
    ㅤThreshold encryption for E2E processing in Motoko not yet reliably available; implications of this aspect of canister memory documented in Motokodoc and discussed in developer doc in [DesignDoc.mo](https://github.com/atengberg/examples/blob/ashton/invoice-bnt2/motoko/invoice-canister/docs/DesignDoc.md#the-extent-of-an-invoices-privacy).  

### [SEC Cleanup] Incomplete design documentation #29 ###  
  - [x] https://github.com/dfinity/invoice-canister/issues/29  
    - [x] **(1) "redundant argument in verify_invoice:..."**  
      ✶ Method completely redone for ICRC1 integration, and this argument is no longer there[^5].  
    - [x] **(2) "In verify_invoice the from_subaccount is re-computed..."**    
      ✶ If all the addresses are to be computed once, then there is going to be lot of redundant data as each invoice would need it's own subaccount blob, creator subaccount blob, and creator subaccount address. The latter two _could_ be put into their own hashmap or collection, but all the addresses will still require computation, which in turn requires testing to verify functionality. In other words as long as the methods to do this are clearly defined with a single responsibility (as well as well tested), it represents less of a potential problem particularly if there's extensive documentation/commentary explaining what and how does what and how. If at a later point a developer wants to add this as an extra layer of security (through redundancy) as _this is_ an invoice bearing smart contract, they can implement what's needed with the given implementation which as clearly defined and well tested (the `ICP` and `ICRC1` `SupportedToken.Adapter` modules).  
    - [x] **(3) "verify_invoice still has a TODO that should be removed..."**    
      ✶ Method redone for ICRC1 integration and this is no longer there; as well as the `motoko-seller-client`'s implementation of the invoice canister has also been updated as well.  
    - [x] **(4) "in refund_invoice let replaced = invoices.put(i.id,..."**  
      ✶ This was never a part of the code base that was a part of the offial branch in the dfinity examples repo afaik; in any case it is no longer there as the refund method was also redone.  
    - [x] **(5) "get_invoice returns invoice by default:..."**  
      ✶ Also did not make it into the official branch of the dfinity examples code base and is no longer an issue.  
    - [x] **(6) "get_invoice permissions: IIUC one always..."**  
      ✶ A buyer's principal does _need_ to be in the permissions to either get or verify an invoice; it is not technically required though. In any case it is clearly mentioned in the non-generated developer docs. 
    - [x] **(7) "unused code: defaultSubAccount..."**  
      ✶ No longer an issue. When a default subaccount is needed, it is done by the the `principal` library introduced with [SEC-F12] to generate the default subaccount account identifier. 

---

###  About Testing ###  

ㅤ**Almost every test title literal (or its parent test title literal) will have the involved method and/or expected input/output annotated as suffix to make it easier to parse at a glance.**

ㅤIn addition to these tasks, there is unit and E2E testing with as complete coverage as could be reasonably implemented (every public method of the `SupportedToken` module (`ICP.Adapter`, `ICRC1.Adapter` and the set of common methods at that's file module scope; as well as the invoice canister's API methods) so that in general every well defined input and output should have its own test case. 

ㅤThe exception to this is _all_ the possible return types from the token-ledger canisters, although all the `#Ok` and many of `#Err` types are tested by integration within the rest of the E2E test suites. For all the other token-ledger canister's transfer `#Err` results, that are not part of the normally expected flow of logic for a given invoice canister API method, they are returned as the argument of that supported token's `SupportedToken` variant tag as the invoice canister's returned `#SupportedTokenTransferErr: SupportedToken.TransferErr` `#err kind` result type. This can be verified in the E2E `transfer.test.js` suite in the subsuite `describe("Prevent Caller from Transferring More than their Balance...")` for each of the four tokens added, as the invoice canister transfer call result contains the corresponding token-ledger canister's own `#Err` result type when this occurs. In other words, when a user calls the invoice canister's `transfer` method such that the actual call to the token-canister ledger finds the transfer amount requested is less than the available balance, the invoice canister returns to the caller the `#err kind` result: 

ㅤ`#SupportedTokenTransferErr : #InsufficientFunds : <#ICPx : { balance: { e8s : Nat64 } } | #ICRC1x : { balance : Tokens}>` 

ㅤThis demonstrates this should work as expected for the other `#Err` result types as well. Additionally, each inter-canister call is wrapped in its own `try/catch` so that if those token-ledger canister's trap unexpectantly, it should be returned to the original caller of the invoice canister as the `#err kind #CaughtException : Text` where the `Text` argument is the caught error's `Error.message(e)` value. 

ㅤThe motivation for this is to give the developer the choice in deciding how to handle what to do in each such case--and this is only for when there is a problem, otherwise the token-ledger canister's returned results are rewrapped by the invoice canister to give the expected API result type. 

ㅤFinally on that note the two mock token-ledger canisters created should return most of those `#Err` types correctly (as well as the `#Ok` ones), with the exception of the ICRC1's `#TemporarilyUnavailable` and `#GenericError` variant tags, if a developer wants to target specific conditions producing error results. Adding a callback to auto-return either of those or any of the others, or trigger an exception to be caught should not be an unreasonable exercise if such is needed. 

ㅤ**One final note is that the E2E test suite `recover_invoice_subaccount_balance.test.js` involves enough different functionality it demonstrates the majority of the required functionality for completing the work required of this bounty.**


[^1]: The invoice canister now does all its amounts arithmetic in `Nat` before setting any token specific transfer args.
[^2]: In the event zero payment invoices are needed, this would likely be a byproduct of implementing a 'status' field for invoices to handle other features like full refund functionality and is left as an exercise for the developer.
[^3]: Git commit tag 816165a-LibrariesEquivalence 816165a7ecd07760548570dc7e2e32a579f788b2 - "Created new utility module of equivalent ICP related functionality and demonstrated working equivalence in unit tests" - `test/unit/Test.mo`.
[^4]: Git commit tag fecdaa1-delegatedAdministrator fecdaa184ab94416b87a70881fc1db686dbbd98e - "Not very complicated code adding a setting/getter to add a delegated administrator. E2E tests also exist in the next commit." -(lines 133-164 in `src/invoice/main.mo`)
[^5]: Before the major refactoring, it was specifically removed see git commit tag 942f066-redundantargremoved 942f066d1a2be48b585788b417d3e5d8edb0636d "Just a tag showing the arg was specifically removed at one point." `src/invoice/ICPLedger.mo` line 143-147  
[^6]: While a hashmap would have sufficed, these should help at least with more robust testing if needed.


