## Testing Glossary Complete List  
#### Contains the console output from the unit and E2E testing. 

Running `make all` will run both of these.  

---
### Unit Testing  

To run for yourself, use the command `make test`

#### `/test/unit/Test.mo`  
```
[INFO] Installing 8 packages
[INFO] Installation complete.
.vessel/.bin/0.7.3/moc -r --package array .vessel/array/v0.2.1/src --package base  
  .vessel/base/aafcdee0c8328087aeed506e64aa2ff4ed329b47/src   --package base-0.7.3   .vessel/base-0.7.3/aafcdee0c8328087aeed506e64aa2ff4ed329b47/src --package crypto .vessel/crypto/v0.3.1/src   --package encoding .vessel/encoding/v0.4.1/src --package hash .vessel/hash/v0.1.1/src   --package matchers .vessel/matchers/v1.2.0/src --package principal .vessel/principal/v0.2.6/src -wasi-system-api test/unit/Test.mo  
```
#### Supported Token Tests: Failed: 0, Passed: 104, Pending: 0, Skipped: 0  

##### Token Standard Specific Addressing Computations: Failed: 0, Passed: 34, Pending: 0, Skipped: 0  

###### ICP Adapter AccountIdentifier and Subaccount Computations: Failed: 0, Passed: 18, Pending: 0, Skipped: 0   
```
Recognizing Faulty ICP Subaccounts | isValidSubaccount -> false: Failed: 0, Passed: 3, Pending: 0, Skipped: 0  
  should return false if it is an empty subaccount: Passed  
  should return false if it is an incomplete subaccount: Passed  
  should return false if it is an excessive subaccount: Passed  

Recognizing Acceptable ICP Subaccounts | isValidSubaccount -> true: Failed: 0, Passed: 2, Pending: 0, Skipped: 0
  should return true if it is default subaccount blob of 32 0s: Passed  
  should return true if it is a known acceptable non trivial subaccount blob: Passed  

Recognizing Faulty Account Identifiers | isValidAddress -> false: Failed: 0, Passed: 4, Pending: 0, Skipped: 0  
  should return false if it is an empty account identifier blob: Passed  
  should return false if it is an incomplete account identifier blob: Passed  
  should return false if it is an account identifier blob of excessive length: Passed  
  should return false if it as an account identifier blob with incorrect crc32 hash: Passed  

Recognizing Acceptable Account Identifiers | isValidAddress -> true: Failed: 0, Passed: 2, Pending: 0, Skipped: 0
  should return true if it is the known default subaccount of an account identifier: Passed  
  should return true if it is a known valid account identifier blob: Passed  

should encode acceptable account identifier | encodeAddress AcountIdentifier -> Text: Passed  
should reject faulty text to be decoded into an account identifier | decodeAddress #err: Passed  
should correctly decode an account identifier after checking if source text valid | decodeAddress #ok: Passed  
should compute an invoice subaccount from an id and creator's principal | computeInvoiceSubaccount: Passed  
should compute an invoice subaccount's account identifier from an id, principal and canister id | computeInvoiceSubaccountAddress: Passed  
should compute an invoice creator's subaccount from a principal | computeCreatorSubaccount: Passed  
should compute an invoice creator's subaccount account identifier from their principal and a canister id | computeCreatorSubaccountAddress: Passed  
```

###### ICRC1 Adapter Account and Subaccount Computations: Failed: 0, Passed: 16, Pending: 0, Skipped: 0 ##### 
```
Recognizing Faulty ICRC1 Subaccounts | isValidSubaccount -> false: Failed: 0, Passed: 3, Pending: 0, Skipped: 0
  should return false if it is an empty subaccount blob: Passed
  should return false if it is an incomplete subaccount blob: Passed
  should return false if it is a subaccount blob of excessive length: Passed

Recognizing Acceptable ICRC1 Subaccounts | isValidSubaccount -> true: Failed: 0, Passed: 2, Pending: 0, Skipped: 0
  should return true if it is default subaccount blob of 32 0s: Passed
  should return true if it is a known non trivial acceptable subaccount blob: Passed

Recognizing Faulty ICRC1 Accounts | isValidAddress -> false: Failed: 0, Passed: 2, Pending: 0, Skipped: 0
  should return false if its subaccount blob is empty: Passed
  should return false if its subaccount blob is of excessive length: Passed

Recognizing Acceptable ICRC1 Accounts | isValidAddress -> true: Failed: 0, Passed: 2, Pending: 0, Skipped: 0
  should return true if it is the known default subaccount of an icrc1 account: Passed
  should return true if it is a known valid icrc1 account: Passed

should encode acceptable icrc1 account | encodeAddress Acount -> Text: Passed

should reject faulty text to be decoded into an icrc1 account | decodeAddress #err: Passed

should correctly decode an account after checking if source text valid | decodeAddress #ok: Passed

should compute an invoice subaccount from an id and creator's principal | computeInvoiceSubaccount: Passed

should compute an invoice subaccount's icrc1 account from an id, principal and canister id | computeInvoiceSubaccountAddress: Passed

should compute an invoice creator's subaccount from a principal | computeCreatorSubaccount: Passed

should compute an invoice creator's subaccount icrc1 account from a principal and canister id | computeCreatorSubaccountAddress: Passed
```

##### Supported Token Types' and Methods: Failed: 0, Passed: 68, Pending: 0, Skipped: 0
```
Correctly get the transfer fee of each supported token type regardless of variant's argument type | getTransactionFee
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should for ICP token type: Passed
  should for ICP_nns token type: Passed
  should for ICRC1_ExampleToken type: Passed
  should for ICRC1_ExampleToken2 type: Passed

Correctly unwrap each supported token amount type into unit type and base units | unwrapTokenAmount
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should for ICP token type: Passed
  should for ICP_nns token type: Passed
  should for ICRC1_ExampleToken type: Passed
  should for ICRC1_ExampleToken2 type: Passed

Correctly wrap base unit amounts into each supported token amount type | wrapAsTokenAmount
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should for ICP token type: Passed
  should for ICP_nns token type: Passed
  should for ICRC1_ExampleToken type: Passed
  should for ICRC1_ExampleToken2 type: Passed

Correctly get each supported verbose token type | getTokenVerbose
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should for ICP token type: Passed
  should for ICP_nns token type: Passed
  should for ICRC1_ExampleToken type: Passed
  should for ICRC1_ExampleToken2 type: Passed

Correctly encode valid addresses | encodeAddress Address -> Text
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should for ICP token type: Passed
  should for ICP_nns token type: Passed
  should for ICRC1_ExampleToken type: Passed
  should for ICRC1_ExampleToken2 type: Passed

Correctly check if valid and encode an address if so | encodeAddress #ok Address -> Text
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should as a result #ok argument for ICP token type: Passed
  should as a result #ok argument for ICP_nns token type: Passed
  should as a result #ok argument for ICRC1_ExampleToken type: Passed
  should as a result #ok argument for ICRC1_ExampleToken2 type: Passed

Reject invalid addresses that cannot be encoded | encodeAddress #err Address -> Text
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should as a result #err for ICP token type: Passed
  should as a result #err for ICP_nns token type: Passed
  should as a result #err for ICRC1_ExampleToken type: Passed
  should as a result #err for ICRC1_ExampleToken2 type: Passed

Correctly get an address decoded from checking valid text | getAddressOrUnitErr #ok Text -> Address
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should as a result #ok argument for ICP token type: Passed
  should as a result #ok argument for ICP_nns token type: Passed
  should as a result #ok argument for ICRC1_ExampleToken type: Passed
  should as a result #ok argument for ICRC1_ExampleToken2 type: Passed

Reject invalid text to be decoded as an address | getAddressOrUnitErr #err Text -> Address
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0  
  should as a result #err for ICP token type: Passed  
  should as a result #err for ICP_nns token type: Passed  
  should as a result #err for ICRC1_ExampleToken type: Passed  
  should as a result #err for ICRC1_ExampleToken2 type: Passed  

Correctly get an invoice subaccount address | getInvoiceSubaccountAddress
                                      : Failed: 0, Passed: 4,Pending: 0, Skipped: 0  
  should for ICP token type: Passed  
  should for ICP_nns token type: Passed  
  should for ICRC1_ExampleToken type: Passed  
  should for ICRC1_ExampleToken2 type: Passed  

Correctly get an encoded invoice subaccount address | getEncodedInvoiceSubaccountAddress
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should for ICP token type: Passed  
  should for ICP_nns token type: Passed  
  should for ICRC1_ExampleToken type: Passed  
  should for ICRC1_ExampleToken2 type: Passed  

Correctly get a creator's subaccount address | getInvoiceSubaccountAddress
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should for ICP token type: Passed
  should for ICP_nns token type: Passed
  should for ICRC1_ExampleToken type: Passed
  should for ICRC1_ExampleToken2 type: Passed

Correctly get the transfer args from as invoice subaccount, to as invoice creator subaccount | getTransferArgsFromInvoiceSubaccount (to creator's subaccount)
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should for ICP token type: Passed
  should for ICP_nns token type: Passed
  should for ICRC1_ExampleToken type: Passed
  should for ICRC1_ExampleToken2 type: Passed

Correctly get transfer args from invoice subaccount, to as an arbitrary address | getTransferArgsFromInvoiceSubaccount (to any address)
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should for ICP token type: Passed
  should for ICP_nns token type: Passed
  should for ICRC1_ExampleToken type: Passed
  should for ICRC1_ExampleToken2 type: Passed

Correctly get the transfer args from creator subaccount, to an arbitrary address | getTransferArgsFromCreatorSubaccount (to any address)
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should for ICP token type: Passed
  should for ICP_nns token type: Passed
  should for ICRC1_ExampleToken type: Passed
  should for ICRC1_ExampleToken2 type: Passed

Correctly wrap each specific supported token types transfer result into invoice transfer result | rewrapTransferResults
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should for ICP token type: Passed
  should for ICP_nns token type: Passed
  should for ICRC1_ExampleToken type: Passed
  should for ICRC1_ExampleToken2 type: Passed

Correctly get default subaccount address | getDefaultSubaccountAddress
                                      : Failed: 0, Passed: 4, Pending: 0, Skipped: 0
  should for ICP token type: Passed
  should for ICP_nns token type: Passed
  should for ICRC1_ExampleToken type: Passed
  should for ICRC1_ExampleToken2 type: Passed
```

##### Unit Test for Migration Module: Failed: 0, Passed: 2, Pending: 0, Skipped: 0
```
should correctly convert an unpaid original \
  invoice canister invoice record type with no permissions or details to Invoice_ record type: Passed

should correctly convert a paid original \
  invoice canister invoice record type with permissions and details to Invoice_ record type: Passed
```  
#### Failed: 0, Passed: 104, Pending: 0, Skipped: 0  

---

### E2E Testing  

To run for yourself use the command `make e2e`  

_Note listed times do not reflect typical calls times of the actual API methods, usually a test will involve a couple of different calls to verify intended operation._  

```
 RUN  v0.28.4 ~/examples/motoko/invoice-canister/test/e2e

✓ src/tests/recover_invoice_subaccount_balance.test.js (22) 193868ms
  ✓ Test recover_invoice_subaccount_balance Functionality (22) 193868ms
    ✓ Test Token Non-Specific #err Results Returned From recover_invoice_subaccount_balance (2) 4137ms
      ✓ should return invoice not found if no invoice exists for given id and caller is authorized | -> err #NotAuthorized 2063ms
      ✓ should reject and return err kind #NotAuthorized if caller not authorized | -> err #NotAuthorized 2073ms
    ✓ Test recover_invoice_subaccount_balance Functionality for #ICP Type (5) 47482ms
      ✓ should correctly recover partial amount paid before invoice has been verified | #ICP -> #ok (case 1: partial refund) 12396ms
      ✓ should correctly recover amount mistakenly sent after invoice already verified | #ICP -> #ok (case 2: recover lost funds) 16513ms
      ✓ should reject if invoice subaccount balance is zero | #ICP -> err #NoBalance 4125ms
      ✓ should reject if invoice subaccount balance not enough to cover transfer fee | #ICP -> err #InsufficientTransferAmount 8258ms
      ✓ should reject if given invalid destination | #ICP -> err #InvalidDestination 6190ms
    ✓ Test recover_invoice_subaccount_balance Functionality for #ICP_nns Type (5) 47419ms
      ✓ should correctly recover partial amount paid before invoice has been verified | #ICP_nns -> #ok (case 1: partial refund) 12368ms
      ✓ should recover amount mistakenly sent after invoice already verified | #ICP_nns -> #ok (case 2: recover lost funds) 16496ms
      ✓ should reject if invoice subaccount balance is zero | #ICP_nns -> err #NoBalance 4124ms
      ✓ should reject if invoice subaccount balance not enough to cover transfer fee | #ICP_nns -> err #InsufficientTransferAmount 8249ms
      ✓ should reject if given invalid destination | #ICP_nns -> err #InvalidDestination 6182ms
    ✓ Test recover_invoice_subaccount_balance Functionality for #ICRC1_ExampleToken Type (5) 47411ms
      ✓ should correctly recover partial amount paid before invoice has been verified | #ICRC1_ExampleToken -> #ok (case 1: partial refund)   12363ms
      ✓ should recover amount mistakenly sent after invoice already verified | #ICRC1_ExampleToken -> #ok (case 2: recover lost funds) 16494ms
      ✓ should reject if invoice subaccount balance is zero | #ICRC1_ExampleToken -> err #NoBalance 4126ms
      ✓ should reject if invoice subaccount balance not enough to cover transfer fee | #ICRC1_ExampleToken -> err #InsufficientTransferAmount 8246ms
      ✓ should reject if given invalid destination | #ICRC1_ExampleToken -> err #InvalidDestination 6182ms
    ✓ Test recover_invoice_subaccount_balance Functionality for #ICRC1_ExampleToken2 (5) 47419ms
      ✓ should correctly recover partial amount paid before invoice has been verified | #ICRC1_ExampleToken2 -> #ok (case 1: partial refund)  12382ms
      ✓ should recover amount mistakenly sent after invoice already verified | #ICRC1_ExampleToken2 -> #ok (case 2: recover lost funds) 16489ms
      ✓ should reject if invoice subaccount balance is zero | #ICRC1_ExampleToken2 -> err #NoBalance 4123ms
      ✓ should reject if invoice subaccount balance not enough to cover transfer fee | #ICRC1_ExampleToken2 -> err #InsufficientTransferAmount 8243ms
      ✓ should reject if given invalid destination | #ICRC1_ExampleToken2 -> err #InvalidDestination 6182ms

✓ src/tests/verify_invoice.test.js (26) 210470ms
  ✓ Test verify_invoice Functionality (26) 210469ms
    ✓ Test Token-Non Specific #err Results Return From verify_invoice (2) 4140ms
      ✓ should reject and return err kind #NotFound if no invoice for given id exists and caller authorized 2063ms
      ✓ should reject and return err kind #NotAuthorized if no invoice for given id exists and caller not authorized 2070ms
    ✓ Test verify_invoice Functionality for #ICP Type (6) 51575ms
      ✓ should reject if zero amount has been paid | #ICP -> err #Unpaid 6193ms
      ✓ should reject invoice balance has only been partially paid | #ICP -> err #IncompletePayment 8247ms
      ✓ should correctly mark & transfer proceeds to creator if amount due is paid | #ICP -> ok #VerifiedPaid 10310ms
      ✓ should return already verified if invoice already verified | #ICP -> ok #VerifiedAlready 8260ms
      ✓ should allow for someone on verify permissions list to verify | #ICP -> ok (caller on verify permissions) 6186ms
      ✓ should reject if unauthorized caller | #ICP -> err #NotAuthorized 12379ms
    ✓ Test verify_invoice Functionality for #ICP_nns Type (6) 51633ms
      ✓ should reject if zero amount has been paid | #ICP_nns -> err #Unpaid 6186ms
      ✓ should reject invoice balance has only been partially paid | #ICP_nns -> err #IncompletePayment 8299ms
      ✓ should correctly mark & transfer proceeds to creator if amount due is paid | #ICP_nns -> ok #VerifiedPaid 10309ms
      ✓ should return already verified if invoice already verified | #ICP_nns -> ok #VerifiedAlready 8244ms
      ✓ should allow for someone on verify permissions list to verify | #ICP_nns -> ok (caller on verify permissions) 6195ms
      ✓ should reject if unauthorized caller | #ICP_nns -> err #NotAuthorized 12400ms
    ✓ Test verify_invoice Functionality for #ICRC1_ExampleToken Type (6) 51553ms
      ✓ should reject if zero amount has been paid | #ICRC1_ExampleToken -> err #Unpaid 6187ms
      ✓ should reject invoice balance has only been partially paid | #ICRC1_ExampleToken -> err #IncompletePayment 8247ms
      ✓ should correctly mark & transfer proceeds to creator if amount due is paid | #ICRC1_ExampleToken -> ok #VerifiedPaid 10310ms
      ✓ should return already verified if invoice already verified | #ICRC1_ExampleToken -> ok #VerifiedAlready 8246ms
      ✓ should allow for someone on verify permissions list to verify | #ICRC1_ExampleToken -> ok (caller on verify permissions) 6183ms
      ✓ should reject if unauthorized caller | #ICRC1_ExampleToken -> err #NotAuthorized 12380ms
    ✓ Test verify_invoice Functionality for #ICRC1_ExampleToken2 Type (6) 51568ms
      ✓ should reject if zero amount has been paid | #ICRC1_ExampleToken2 -> err #Unpaid 6179ms
      ✓ should reject invoice balance has only been partially paid | #ICRC1_ExampleToken2 -> err #IncompletePayment 8258ms
      ✓ should correctly mark & transfer proceeds to creator if amount due is paid | #ICRC1_ExampleToken2 -> ok #VerifiedPaid 10301ms
      ✓ should return already verified if invoice already verified | #ICRC1_ExampleToken2 -> ok #VerifiedAlready 8261ms
      ✓ should allow for someone on verify permissions list to verify | #ICRC1_ExampleToken2 -> ok (caller on verify permissions) 6183ms
      ✓ should reject if unauthorized caller | #ICRC1_ExampleToken2 -> err #NotAuthorized 12384ms

✓ src/tests/create_invoice.test.js (22) 57830ms
  ✓ Test create_invoice Functionality (22) 57830ms
    ✓ Test Token Specific #ok Results Returned From create_invoice (16) 33025ms
      ✓ Test create_invoice Functionality for Invoices using #ICP Type (4) 8269ms
        ✓ should correctly create an invoice | #ICP -> ok 2062ms
        ✓ should correctly create an invoice with permission | #ICP -> ok 2063ms
        ✓ should correctly create an invoice with details | #ICP -> ok 2063ms
        ✓ should allow a caller on allowed creators list to correctly create an invoice | #ICP -> ok 2062ms
      ✓ Test create_invoice Functionality for Invoices using #ICP Type (4) 8244ms
        ✓ should correctly create an invoice | #ICP -> ok 2059ms
        ✓ should correctly create an invoice with permission | #ICP -> ok 2063ms
        ✓ should correctly create an invoice with details | #ICP -> ok 2062ms
        ✓ should allow a caller on allowed creators list to correctly create an invoice | #ICP -> ok 2060ms
      ✓ Test create_invoice Functionality for Invoices using #ICRC1_ExampleToken Type (4) 8245ms
        ✓ should correctly create an invoice | #ICRC1_ExampleToken -> ok 2060ms
        ✓ should correctly create an invoice with permission | #ICRC1_ExampleToken -> ok 2063ms
        ✓ should correctly create an invoice with details | #ICRC1_ExampleToken -> ok 2061ms
        ✓ should allow a caller on allowed creators list to correctly create an invoice | #ICRC1_ExampleToken -> ok 2061ms
      ✓ Test create_invoice Functionality for Invoices using #ICRC1_ExampleToken2 Type (4) 8267ms
        ✓ should correctly create an invoice | #ICRC1_ExampleToken2 -> ok 2063ms
        ✓ should correctly create an invoice with permission | #ICRC1_ExampleToken2 -> ok 2067ms
        ✓ should correctly create an invoice with details | #ICRC1_ExampleToken2 -> ok 2061ms
        ✓ should allow a caller on allowed creators list to correctly create an invoice | #ICRC1_ExampleToken2 -> ok 2076ms
    ✓ Test #err Results Returned from create_invoice (6) 24805ms
      ✓ should reject if given meta too large | -> err #MetaTooLarge 2084ms
      ✓ should reject if given description too large | -> err #DescriptionTooLarge 2059ms
      ✓ should reject if given too many principals for verify permissions list | -> err #TooManyPermissions 2072ms
      ✓ should reject if given too many principals for get permissions list | -> err #TooManyPermissions 2071ms
      ✓ should reject if given amount due less than transfer fee for each token type | -> err #InsufficientAmountDue 8240ms
      ✓ should reject for creating invoice of each token type if caller not authorized | -> err #NotAuthorized 8279ms

✓ src/tests/transfer.test.js (24) 90769ms
  ✓ Test transfer Functionality (24) 90769ms
    ✓ Test #ok Results Returned From transfer (4) 49508ms
      ✓ should correctly transfer e8s to an address specified as an account identifier & as text | #ICP 12363ms
      ✓ should correctly transfer e8s to an address specified as an account identifier & as text | #ICP_nns 12364ms
      ✓ should correctly transfer icrc1 tokens to an address specified as an icrc1 account & as text | #ICRC1_ExampleToken 12407ms
      ✓ should correctly transfer icrc1 tokens to an address specified as an icrc1 account & as text | #ICRC1_ExampleToken2 12366ms
    ✓ Test #err Results Return From transfer (20) 41261ms
      ✓ When Caller is Not Authorized | -> err kind #NotAuthorized (4) 8277ms
        ✓ should reject when caller is not authorized | #ICP 2067ms
        ✓ should reject when caller is not authorized | #ICP_nns 2073ms
        ✓ should reject when caller is not authorized | #ICRC1_ExampleToken 2066ms
        ✓ should reject when caller is not authorized | #ICRC1_ExampleToken2 2071ms
      ✓ When Caller Tries to Transfer Amount Less Than Transfer Fee | -> err kind #InsufficientTransferAmount (4) 8252ms
        ✓ should reject when the caller is using insufficient e8s | #ICP 2063ms
        ✓ should reject when the caller is using insufficient e8s | #ICP_nns 2063ms
        ✓ should reject when the caller is using insufficient icrc1 tokens | #ICRC1_ExampleToken 2065ms
        ✓ should reject when the caller is using insufficient icrc1 tokens | #ICRC1_ExampleToken2 2061ms
      ✓ When the Destination is Invalid | -> err kind #InvalidDestination (8) 16491ms
        ✓ should reject if given invalid text as an account identifier destination | #ICP 2061ms
        ✓ should reject if given an invalid account identifier as destination | #ICP 2061ms
        ✓ should reject if given invalid text as an account identifier destination | #ICP_nns 2062ms
        ✓ should reject if given an invalid account identifier as destination | #ICP_nns 2067ms
        ✓ should reject if given invalid text (reserved principal) as an account destination | #ICRC1_ExampleToken 2058ms
        ✓ should reject if given an invalid account as destination | #ICRC1_ExampleToken 2062ms
        ✓ should reject if given invalid text as an icrc1 account destination | #ICRC1_ExampleToken2 2059ms
        ✓ should reject if given an invalid account as destination | #ICRC1_ExampleToken2 2061ms
      ✓ Prevent Caller from Transferring More than their Balance | -> err kind #SupportedTokenTransferErr (4) 8241ms
        ✓ should reject if caller tries transferring more e8s than they have | #ICP 2060ms
        ✓ should reject if caller tries transferring more e8s than they have | #ICP_nns 2061ms
        ✓ should reject if caller tries transferring more e8s than they have | #ICRC1_ExampleToken 2060ms
        ✓ should reject if caller tries transferring more e8s than they have | #ICRC1_ExampleToken2 2060ms

✓ src/tests/to_other_address_format.test.js (27) 55676ms
  ✓ Test to_other_address_format Functionality (27) 55676ms
    ✓ Test #ok Results Returned From to_other_address_format (14) 28871ms
      ✓ Test to_other_address_format Functionality for ICP Token Type Addressing (3) 6198ms
        ✓ should correctly encode an account identifier into text 2063ms
        ✓ should correctly decode acceptable text into an account identifier 2060ms
        ✓ should correctly compute the default subaccount account identifier from a principal 2062ms
      ✓ Test to_other_address_format Functionality for ICP_nns Token Type Addressing (3) 6181ms
        ✓ should correctly encode an account identifier into text 2059ms
        ✓ should correctly decode acceptable text into an account identifier 2061ms
        ✓ should correctly compute the default subaccount account identifier from a principal 2061ms
      ✓ Test to_other_address_format Functionality for ICRC1_ExampleToken Type Addressing (4) 8251ms
        ✓ should correctly encode an icrc1 account into text 2069ms
        ✓ should correctly encode an icrc1 account with non-trivial subaccount into text 2060ms
        ✓ should correctly decode acceptable text into an icrc1 account 2061ms
        ✓ should correctly compute the default subaccount icrc1 account from a principal 2061ms
      ✓ Test to_other_address_format Functionality for ICRC1_ExampleToken2 Type Addressing (4) 8241ms
        ✓ should correctly encode an icrc1 account to text 2061ms
        ✓ should correctly encode an icrc1 account with non-trivial subaccount into text 2061ms
        ✓ should correctly decode acceptable text into an icrc1 account 2059ms
        ✓ should correctly compute the default subaccount icrc1 account from a principal 2060ms
    ✓ Test #err Results Returned From to_other_address_format (13) 26805ms
      ✓ should reject and return err kind #NotAuthorized if caller not authorized 2068ms
      ✓ If Missing which Token Type to Convert Address Type of | -> #err kind #MissingTokenType (2) 4122ms
        ✓ should reject when calling for default subaccount address but missing given token type 2059ms
        ✓ should reject when given acceptable encoded address text but missing token type 2063ms
      ✓ If Given an Invalid Destination | #err kind #InvalidDestination (10) 20615ms
        ✓ Such as Invalid Text to Decode | Text -> #err kind #InvalidDestination (6) 12366ms
          ✓ should reject if given invalid text to be decoded into an account identifier | #ICP 2060ms
          ✓ should reject if given invalid text to be decoded into an account identifier | #ICP_nns 2060ms
          ✓ should reject if given invalid text to be decoded into an icrc1 account | #ICRC1_ExampleToken 2065ms
          ✓ should reject if given invalid text to be decoded into an icrc1 account | #ICRC1_ExampleToken2 2063ms
          ✓ should reject if given text to decode into icrc1 account is just a reserved principal | #ICRC1_ExampleToken 2058ms
          ✓ should reject if given text to decode into icrc1 account is just a reserved principal | #ICRC1_ExampleToken2 2060ms
        ✓ Such as an Invalid Address to Encode | Address -> #err kind #InvalidDestination (4) 8249ms
          ✓ should reject if given an invalid account identifier | #ICP 2061ms
          ✓ should reject if given an invalid account identifier | #ICP_nns 2060ms
          ✓ should reject if given an invalid icrc1 account | #ICRC1_ExampleToken 2061ms
          ✓ should reject if given an invalid icrc1 account | #ICRC1_ExampleToken2 2067ms

✓ src/tests/allowedCreatorsList.test.js (9) 63932ms
  ✓ Test Functionality of Allowed Creators List (9) 63932ms
    ✓ Test add_allowed_creator Functionality (4) 18589ms
      ✓ should allow invoice canister deployer to add a non-anonymous principal to allowed creators list and that works correctly | -> ok 10306ms
      ✓ should reject if principal to add already on the list | -> err #AlreadyAdded 4124ms
      ✓ should reject if principal to add is anonymous | -> err #AnonymousIneligible  2061ms
      ✓ should reject if caller unauthorized | -> err #NotAuthorized 2089ms
    ✓ Test remove_allowed_creator Functionality (3) 14427ms
      ✓ should allow invoice canister deployer to remove a principal from the allowed creators list | -> ok 6182ms
      ✓ should reject if principal to remove not on the list | -> err #NotFound 6180ms
      ✓ should reject if caller unauthorized | -> #NotAuthorized 2065ms
    ✓ Test get_allowed_creators_list Functionality (2) 30916ms
      ✓ should get the list of allowed creators for the invoice canister deployer correctly | -> ok 28844ms
      ✓ should reject if caller not authorized (ie not the canister installer) | -> #NotAuthorized 2072ms

✓ src/tests/disallowAnonymous.test.js (11) 47364ms
  ✓ Test Anonymous Principal is Disallowed as a Caller (11) 47364ms
    ✓ should reject anonymous caller | create_invoice (all token kinds) -> err #NotAuthorized 8236ms
    ✓ should reject anonymous caller | add_allowed_creator -> err #NotAuthorized 2058ms
    ✓ should reject anonymous caller | remove_allowed_creator -> err #NotAuthorized 2057ms
    ✓ should reject anonymous caller | get_allowed_creators_list -> err #NotAuthorized 2057ms
    ✓ should reject anonymous caller | get_invoice -> err #NotAuthorized 2058ms
    ✓ should reject anonymous caller | get_caller_balance -> err #NotAuthorized 8231ms
    ✓ should reject anonymous caller | get_caller_address (any token type) -> err #NotAuthorized 8241ms
    ✓ should reject anonymous caller | verify_invoice -> err #NotAuthorized 2060ms
    ✓ should reject anonymous caller | transfer (all token kinds) -> err #NotAuthorized 8233ms
    ✓ should reject anonymous caller | to_other_address_format -> err #NotAuthorized 2059ms
    ✓ should reject anonymous caller | recover_invoice_subaccount_balance -> err #NotAuthorized 2064ms

✓ src/tests/get_invoice.test.js (10) 22699ms
  ✓ Test get_invoice Functionality (10) 22698ms
    ✓ Test Token-Non Specific #err Results From get_invoice (2) 6213ms
      ✓ should reject if no invoice exists for given id and caller authorized | -> err #NotFound 4138ms
      ✓ should reject if caller not authorized | -> #NotAuthorized 2066ms
    ✓ Test Token Specific #ok Results Returned From get_invoice (8) 16485ms
      ✓ Test get_invoice Functionality for Invoices using #ICP Type (2) 4121ms
        ✓ should correctly get an existing invoice by given id | #ICP -> ok (caller = creator) 2060ms
        ✓ should correctly get an existing invoice by given id | #ICP -> ok (caller on get permissions) 2061ms
      ✓ Test get_invoice Functionality for Invoices using #ICP_nns Type (2) 4122ms
        ✓ should correctly get an existing invoice by given id | #ICP_nns -> ok (caller = creator) 2062ms
        ✓ should correctly get an existing invoice by given id | #ICP_nns -> ok (caller on get permissions) 2060ms
      ✓ Test get_invoice Functionality for Invoices using #ICRC1_ExampleToken Type (2) 4121ms
        ✓ should correctly get an existing invoice by given id | #ICRC1_ExampleToken -> ok (caller = creator) 2060ms
        ✓ should correctly get an existing invoice by given id | #ICRC1_ExampleToken -> ok (caller on get permissions) 2061ms
      ✓ Test get_invoice Functionality for Invoices using #ICRC1_ExampleToken2 Type (2) 4121ms
        ✓ should correctly get an existing invoice by given id | #ICRC1_ExampleToken2 -> ok (caller = creator) 2060ms
        ✓ should correctly get an existing invoice by given id | #ICRC1_ExampleToken2 -> ok (caller on get permissions) 2061ms

✓ src/tests/get_caller_balance.test.js (12) 24763ms
  ✓ Test get_caller_balance Functionality (12) 24763ms
    ✓ When Caller Wants their #ICP Type Balance (3) 6199ms
      ✓ should get the balance of the invoice canister installer's creator subaccount | #ICP -> ok 2063ms
      ✓ should correctly get the balance of an authorized caller's creator subaccount | #ICP -> ok 2060ms
      ✓ should reject if the caller not authorized | #ICP -> err #NotAuthorized 2066ms
    ✓ When Caller Wants their #ICP_nns Type Balance (3) 6188ms
      ✓ should get the balance of the invoice canister installer's creator subaccount | #ICP_nns -> ok 2060ms
      ✓ should correctly get the balance of an authorized caller's creator subaccount | #ICP_nns -> ok 2062ms
      ✓ should reject if the caller not authorized | #ICP_nns -> err #NotAuthorized 2066ms
    ✓ When Caller Wants their #ICRC1_ExampleToken Type Balance (3) 6193ms
      ✓ should get the balance of the invoice canister installer's creator subaccount | #ICRC1_ExampleToken -> ok 2062ms
      ✓ should correctly get the balance of an authorized caller's creator subaccount | #ICRC1_ExampleToken -> ok 2060ms
      ✓ should reject if the caller not authorized | #ICRC1_ExampleToken -> err #NotAuthorized 2071ms
    ✓ When Caller Wants their #ICRC1_ExampleToken2 Type Balance (3) 6183ms
      ✓ should get the balance of the invoice canister installer's creator subaccount | #ICRC1_ExampleToken2 -> ok 2059ms
      ✓ should correctly get the balance of an authorized caller's creator subaccount | #ICRC1_ExampleToken2 -> ok 2059ms
      ✓ should reject if the caller not authorized | #ICRC1_ExampleToken2 -> err #NotAuthorized 2065ms

✓ src/tests/get_caller_address.test.js (8) 16525ms  
  ✓ Test get_caller_address Functionality (8) 16525ms  
    ✓ When Calling get_caller_address to get an #ICP Type Address (2) 4141ms
      ✓ should get the account identifer and as encoded text of the caller's creator subaccount | #ICP -> ok 2062ms
      ✓ should reject if the caller not authorized | #ICP -> err #NotAuthorized 2068ms
    ✓ When Calling get_caller_address to get an #ICP_nns Type Address (2) 4128ms
      ✓ should get the account identifer and as encoded text of the caller's creator subaccount | #ICP_nns -> ok 2060ms
      ✓ should reject if the caller not authorized | #ICP_nns -> err #NotAuthorized 2068ms
    ✓ When Calling get_caller_address to get an #ICRC1_ExampleToken Type Address (2) 4128ms
      ✓ should get the account and as encoded text of the caller's creator subaccount | #ICRC1_ExampleToken -> ok 2060ms
      ✓ should reject if the caller not authorized | #ICRC1_ExampleToken -> err #NotAuthorized 2068ms
    ✓ When Calling get_caller_address to get an #ICRC1_ExampleToke2n Type Address (2) 4128ms
      ✓ should get the account and as encoded text of the caller's creator subaccount | #ICRC1_ExampleToken2 -> ok 2061ms
      ✓ should reject if the caller not authorized | #ICRC1_ExampleToken2 -> err #NotAuthorized 2067ms
```
#### Test Files  10 passed (10)
#### Tests  171 passed (171)
#### Start at  13:11:45
#### Duration  844.77s (transform 242ms, setup 28ms, collect 60.37s, tests 783.90s)
