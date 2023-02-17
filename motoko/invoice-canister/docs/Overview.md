
# Payments - Invoice Canister  

ㅤAs the Internet Computer becomes more capable, both within its own ecosystem as well as in connecting to not only the yet to be blockchained powered web, but also in directly signing to the smart contracts of other blockchains, we look to refine the developer experience around payments in canisters.  

ㅤWe concluded that requiring a developer to always handle all the "low level" operations necessary for interfacing to either the ICP ledger canister or any ICRC1 token-ledger canister just to process a transaction using one of those tokens is counter-productive to everyone's interest and time.  

ㅤFor example, a canister that would like to access or implement a payment system would need to implement from scratch things like protection against double spending, conversion to and from and validation of each kind of address format, or the multi-step token-canister ledger specific, and potentially trapping, intercanister calls necessary for the accounting of payments. For that reason and motivated by the expectation a developer's toolset should simply be reliably effective, we propose to design an interface that will make it easier for a typical canister to integrate payment functionality.  

## Goals  

Goals for this project are as follows:   

1. Solution should be simple to include and develop against locally.  
2. Canister should have at least some access control so that different callers can be authorized to use its service methods.  
3. Canister should be capable of easily checking the balance of an authorized caller.  
4. Canister should be capable of verifying a payment has been satisfied in either ICP or a token of the ICRC1 standard.  
5. Canister should be capable of reliably transferring funds that it holds on behalf of an authorized caller.  
6. Canister should be capable of also recovering funds from the subaccounts it creates on behalf its authorized callers.  
7. Canister should accept different address formats as if they were the same so fewer steps are required of callers to process or relay transactions.  
8. Canister's interface for connecting to other token-ledger canisters should be standardized so supporting an additional ICRC1 tokens is as simple.  
9. This design standard should also be compatible with BTC, ETH and other token-ledgers as they become available.  

## Getting to Know the Invoice Canister  
#### and the Choices of its Design  

##### Premise   

ㅤThe goal here is to design a flow where a client application such as a webpage could initiate a payment flow that could be used to gate services or transfer ownership of assets with as few canister calls as possible. 

ㅤIn this codebase and in the rest of this document, whenever the term 'API' is used, it is used to mean the `shared` methods of the Invoice Canister. Whenever the term 'address' is used, it means the source or destination of funds sent or received. While the naming strategy for fields, variables, methods and files may be a bit verbose, it is intended to make as much of this codebase as unambigious as possible. 

##### Access Control as a Starting Point  

ㅤThe Invoice Canister will provide a standard API for each authorized caller without any special privileges to a specific caller if possible. As the question of access control can easily become complicated, to provide a starting point for developers to integrate a payment flow, the original deployer or canister installer retains the permission to add and remove principals from a list of allowed creators. Any caller whose principal is on this list will have the same permission of access in calling on the Invoice Canister[^1]. 

ㅤAn analogy that might be useful is that the installer is like an adminstrator, while allowed creators are like the staff (there's even a Motoko library for just this kind of thing[^2]). While not implemented currently, explicitly declaring a field at the Invoice Canister's actor scope to set a principal as the designated or delegated administrator (or making it mutable so it can be dynamically assigned; or even as list of select principals) could be a useful addition. For instance a developer who wanted to provide development operations management and technical support to a web storefront looking to integrate crypto payments could set their command and control canister's id as this delegated administrator principal and then add all the sellers from that storefront to that invoice canister's allowed creators list. 

ㅤAlternatively that delegated administrator could be set as the principal of the controller of a DAO if the idea is to black-hole the Invoice Canister so it can perform as a decentralized payment processor. If this the case, it brings up an important and critical point to be aware of: *all funds managed by the Invoice Canister, until they are transferred out of its custody, could be either misappropriated by the canister's installer (or if changed, the current controller) or even irrecoverably lost if the Invoice Canister is black-holed*. That being said, with the current API any transfer of funds is only possible for funds that are linked to a specific authorized caller or the invoices that they create or are associated with (that is, their principal is on one of the invoice's permissions list).

##### Equally Authorized Caller Representation of an Invoice's Permissions     

ㅤThe Invoice Canister acts as the custodian of funds its processes for each of its authorized callers independently from all the other callers. Other than the three API methods associated with adding or removing or getting the allowed creators list:

* `add_allowed_creator()`   
* `remove_allowed_creator()`   
* `get_allowed_creators_list()`   
  
all other API calls work the same for each authorized caller. This is even true for the canister's installer: for example it is not possible for the canister installer to use any of the existing API methods to arbitrarily transfer funds from any of the addresses the Invoice Canister controls. 

ㅤThis is equally true for the other allowed creators as well. Each allowed creator has the same permission of access only to the invoices they create and the funds that end up in the Invoice Canister's custody as the result of those invoices. They also have the same authorization to add access control per invoice they create: specifically, the creation of an invoice allows for two lists to be included of no more than 256 principals each, such that one grants authorization for callers who can get that same invoice (`canGet`), and the other authorizes callers who can verifying that invoice or recover funds associated with its payment address (`canVerify`). 

ㅤIn other words only authorized callers can create invoices. For each invoice created, only callers authorized to either get or verify that invoice, other than the invoice's creator, can all the related Invoice Canister's API methods successfully. This is equally true for the invoice canister installer as is the code currently implemented. This is the extent of built-in access control the Invoice Canister provides[^3]. 

##### The Extent of an Invoice's Privacy    

ㅤWhile it is true an invoice is protected from unauthorized callers as exampled above, it is important to be aware of the fact invoice records, in particular the `meta` and `description` `details` fields, as they are saved in the memory of an invoice canister, are not by default encrypted or otherwise impervious to physical inspection by a node provider. If such privacy is needed, consider implementing a cryptographic strategy for encrypting invoice data before it reaches the canister's `create_invoice` method. One available strategy is demonstrated by the [Encrypted Notes Dapp](https://internetcomputer.org/docs/current/samples/encrypted-notes/) while another option, highly anticipated, is the upcoming E2E encryption utilizing [Threshold ECDSA Signatures](https://forum.dfinity.org/t/threshold-ecdsa-signatures/6152).  

##### Reasonably Certifiable Results     

ㅤRelated to this is the issue of whether results returned to the caller by the Invoice Canister are certified or not. That is to say whether the integrity of those result's data are validated by the Internet's Computer blockchain or not (technically called a subnet as the Internet Computer is actually a number of blockchains ('subnets') running together). Unless declared otherwise in a canister's code, when an API method of a canister is called, its execution is replicated by all the nodes in a subnet (hence the term replica) before that method's resulting value is returned to the caller. This is also called going through "a round of consensus", which is much like a network of indpendent computers only running a certain file if each of those computers evaluates that file's checksum and then compares their results so as to only run that file if enough of them have ended up calculating the same checksum value. 

ㅤAnother way to say this is by default the Internet Computer's runtime enviroment automatically has built in data integrity that is certifiable. The trade-off for this, as most familiar with blockchains know, is it necessarily takes a bit longer for a function to run on the network this way. In terms of canisters and their API methods, the term for this is 'update' as in 'calls to update methods', or update calls. An update call typically takes at least two and a half seconds. The current implementation of the Invoice Canister has all of its API methods as update calls so that all of the results it returns to a caller have the benefit of going through a round of consensus. To balance for this as few awaits as possible are used in each API method's implementation. 

ㅤHowever, it is also possible to declare a method as a being `query` call if it causes no change or mutation of state in that canister. If a method is declared `query`, its execution does not go through a round of subnet consensus because its results are returned directly to the caller by a single responding node. As a result its execution completes far more quickly than that of an update call, but the responding node can manipulate or corrupt the data being returned before it reaches that method's caller. It's also possible to imperatively encode within the body of declared `query` method the process of going through a round of subnet consensus for specific data that needs to be returned as if it were from an update call. Such data or results are typically called "certified assets". Care must be given when deciding what methods to declare `query` and wehn the data of those methods should be certified so be sure to review the [Security best practices](https://internetcomputer.org/docs/current/developer-docs/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security) article at the Developer Docs.

ㅤIn a nutshell, the certification of assets works by using a specialized directed acyclic graph called a Merkle tree and progressively assigning a hash in that tree to each asset needing to be certified as its state changes so that when an asset returned to a caller needs to be certified, it's returned with a value that can be used to check if it matches the expected values assigned with the tree (very much an oversimplification). For a better explanation check out [Inside the Internet Computer: Certified Variables](https://www.youtube.com/watch?v=mZbFhRIHIiY)). In other words this requires action by both the backend and the frontend as the the certification of assets must be implemented in the backend canister as well as the calling client must confirm the certified values they were returned. 

ㅤAs there are four API methods of the Invoice Canister that require no change of state--getting an invoice, getting a caller's balance or the address of that balance, or converting a given address into its other format--these could be declared `query`. However doing so without also implementing those method's returned values as certified would make the actual values the caller receives vulnerable to the manipulation by the single responding node who can then return incorrect or fraudulent values instead of the expected ones intended. In particular it's worth considering that since an Invoice Canister caller must be authorized to even make the call in the first place, they may assume whatever results they are getting come with certified integrity automatically.

ㅤIf greater optimization is needed such that implementing these methods as `query` and certifying their results becomes a requirement, check out the [Certified Variable](https://github.com/dfinity/examples/tree/master/motoko/cert-var) Motoko example from this project's parent repository. There's also a tutorial[^4] as well as the example of the ICP Ledger Rust canister[^5] which certifies its data so that the method to get the balance of ICP address can be a `query` call and be returned to a caller more quickly while still being secure. There's also an example of validating certified results from the point of view of Typescript client[^6].  Also consider if the scope of requirements involves implementating `CertifiedAssets`, this could be combined to create more sophisticated access control[^7] for turning an invoice canister into a fully featured web3 powered payment processor.

##### Compulsory Constants     

ㅤWhile on the subject of the Invoice Canister's operational requirements, to keep the Invoice Canister functionally reliable, the following constraints are imposed:  
1) An invoice cannot be created with more than 256 principals on either its verify permission list or its get permissions list.  
2) An invoice cannot be created with a description literal longer than 256 characters.  
3) An invoice cannot be created with a meta blob larger than 32000 bytes (or ~ 16k UTF-8 latin-alphabet characters[^8]).  
4) No more than 30000 invoices can be created (technically stored) in the same invoice canister.  
5) No more than 256 allowed creators per invoice canister.
6) Another magic number to be aware of is the auto timeout expiration of the lock used to synchronize the verification and recovery of funds of a particular invoice which is currently set to ten minutes. That is, after ten minutes the lock for a given invoice's id will always be released to garuntee an authorized caller is not locked out of verifying an invoice or recovering the funds from that invoice's payment address.
7) It should also be noted an invoice cannot be created with an amount due less than twice the transfer fee of that token's ledger canister. This is so that an invoice can at least cover the cost of when it successfully verified as paid and its proceeds are imperatively transferred by the Invoice Canister from its payment address to the address created for that invoice's creator.   
   
##### How to Address the Life Cycle of an Invoice    

##### `create_invoice()`    
 
ㅤThe Invoice Canister is designed to consolidate the proceeds of invoices it successfully verifies as paid from the payment address of that payment into the address it creates for that invoice's creator. That is, when an authorized caller successfully creates an invoice having called `create_invoice()` with acceptable inputs (see above), a payment address is created for that invoice by computing a subaccount from the invoice's id and that caller's principal; when that invoice subaccount is combined with the Invoice's Canister own id, the actual ICP account identifier or ICRC1 account is created. The Invoice Canister does not store the token specific address types in the records of its invoices, it only stores the invoice payment addresses in their text encoded form which it computes when an invoice is created.

##### `Subaccounts and account identifiers`     

ㅤA subaccount is any sequence of 32 bytes; for both ICP and ICRC1 there is no other condition for what determines a valid subaccount. An account identifier is also a sequence of 32 bytes, such that the first 4 bytes are the CRC32 encoded checksum hash of the remaining 28 bytes, which are the SHA224 hash of the length of the domain separator concatenated with the domain separator literal "account-id", concatenated with the bytes of the caller's principal, and finally concatenated with the subaccount. In the event there is no subaccount (which is a valid option) what's called the "default subaccount" is used, which is a sequence of 32 zeros.  

##### `Invoice subaccounts and addressing computations`     

ㅤThe Invoice Canister uses a similar covention, computing the subaccount for an invoice as SHA224 hash of the sequence of bytes of the length of domain separator concatenated with the domain separator literal "invoice-id", followed by the bytes of the id, followed by the bytes of the principal of the creator. Then the CRC32 checksum hash is computed and prefixed to the SHA224 hash. In this way each created invoice has its own created payment address. 

ㅤAs mentioned before, the Invoice Canister also creates an address for each authorized caller aka invoice creator. It is much the same computation as for invoices, except it is only with the length of the domain separator, the domain separator literal being "creator-id" (coincidentally the same length as the previous two), finally concatenated with the creator's principal bytes. 

ㅤNote that the CRC32 checksum hash is dropped when computing ICRC1 subaccounts, but otherwise the process is the same. An ICRC1 address is a record of two fields: a principal and an optional subaccount. Like ICP, a null subaccount is functionally evaluated as the default subaccount of 32 zeros.  

##### `get_invoice()`   

ㅤOnce created, the invoice record is stored in a hashmap and remains there. If an authorized caller (an invoice's creator or a caller with a principal on that invoice's get permission list) calls `get_invoice()` the invoice is retrieved by its id from this map. It should also be noted that each invoice is not stored with their associated `tokenVerbose : TokenVerbose` record data or `paid : Bool` field, these are added before the invoice record is returned to the caller (this is true for all API methods).  

##### `verify_invoice()`     

ㅤAnytime after an invoice has been created, `verify_invoice()` can be called by its creator or someone on its verify permission list to trigger the Invoice Canister to query the balance of that invoice's subaccount address ("payment address"--note often times "subaccount" is used synonymousily to represent its associated address, however address is always added in this code base to keep things unambigious as possible). If the balance is confirmed to be greater or equal to that invoice's amount due, then the Invoice Canister proceeds by transferring the balance from that invoice's subaccount address to the subaccount address created for that invoice's creator. Because of this, it is required that all invoices be created with an amount due at least twice the transfer fee cost of that token type's ledger-canister (as mentioned before). Note that the invoice is locked by its id during verification to prevent concurrent calls to verification (or invoice subaccount balance recovery) from interfering with eachother. If the transfer succeeds, a new invoice record will be created as a copy of the existing invoice except its `verifiedPaidAtTime` will be updated as a non-null opt `Time` stamp as well as updating the `amountPaid` which will be the amount received (note that the amount deposited into the invoice creator's subaccount address will be less by one transfer fee as defined by that token ledger's canister). Also note if partial payment has been paid, `verify_invoice()` will return this information to the caller but take no other action. 

##### `transfer()`   

ㅤNow the proceeds are available to the invoice's creator and available to be transferred from out of the custody of the Invoice Canister whenever the creator calls the `transfer()` method. All the subaccount addresses discussed so far belong to the invoice canister, so it should be understood the invoice canister processes its invoices by custody and is the custodian of any funds sent for payment until the creator transfers funds out of their subaccount address. A transfer call will fail if the authorized caller requests an insufficient amount to transfer, that is an amount equal or less than the transfer fee (at least one token needs to end up in the specified destination). 

##### `get_caller_balance() & get_caller_address()`     

ㅤIf an invoice creator aka authorized caller aka caller whose principal is on the allowed creators list needs to check their (creator's) subaccount address balance they can call `get_caller_balance()` at any time. Similarly, if they want to find out what that address is they can call `get_caller_address()` which will return both the specific token address type (account or account identifier) as the argument of the `SupportedToken` variant tag for that token type `(#ICP{ balance = { e8s = 10000000000000000000000000 }})`. Note that `#ICP` and `#ICRC1` are not special tags, the actual variant tag corresponds to the literal given for that particular token's `SupportedToken` variant tag. In the event the Invoice Canister is processing only ICP mainnet ledger and a single ICRC1 token-ledger canister transactions, this can resolve as simply as this. 

##### `recover_invoice_subaccount_balance()`     

ㅤAn invoice can only be verified if the balance of its subaccount address is at least as much as its amount due. In the current implementation invoices have no status, and refunds for verified invoices are not supported. That being said, if partial payment has been made, or payment is sent to the invoice's subaccount (payment) address after that invoice has already been verified, those funds can be recovered by the invoice creator, or an authorized principal on that invoice's verify permission list, by calling `recover_invoice_subaccount_balance()` which will transfer a non-zero balance of an invoice subaccount address to a given valid destination address as long as that balance is greater than the transfer fee cost of token's ledger canister. **Do not** consider this a means to refund invoices that have been verified paid as those balances have already been moved into the creator's subaccount address. Adding such functionality would likely involve incorporating a designated invoice status field[^9]. 

##### `to_other_address_format()`     

ㅤIf an authorized caller needs to know either their default subaccount address (in ICP or ICRC1), or convert a valid token type specific address to its text encoded format (or vica versa) they can call `to_other_address_format()` to do so. If neither text nor address is given, the default subaccount will be computed according the given token type passed with the caller's principal. If only text is given, so must to be given the token type. An address type is encoded as text. In all cases both formats are returned the caller as the `asAddress` and `asText` fields of the returned record object.  

#### Module at the Core : SupportedToken   

ㅤBuilding on the work and motivation Kyle started with the Invoice Canister, what really brings the current implementation together with the introduction of ICRC1 compatibility is the `SupportedToken` module and its generic variant field. This construct and its associated methods act much like a facade design pattern bridging the addressing computations of the two adapter modules (`ICP.Adapter` and `ICRC1.Adapter`) to the API methods of the Invoice Canister while abstracting through encapsulation the lower level operations unique to each token-canister ledger type. This field looks like:
```
type SupportedToken<T1, T2> = {
#ICP : T1;
#ICP_nns : T1;
#ICRC1_A : T2;
#ICRC1_B : T2;
// etc
}
```
with the associated fields such as:

`type Amount : SupportedToken<ICP.ICP, ICRC1.Tokens> // <{ e8s : Nat64}, Nat>`
`type Address : SupportedToken<ICP.AccountIdentifier, ICRC1.Account>;`
`// etc`

ㅤThese declarations make it possible to provide a normalized and unified type for interacting with token-ledger canisters provided the set of addressing computations are well defined (eg `ICP.Adapter`/`ICRC.Adapter`) and which in turn are used by the Invoice Canister through the set of common transformation methods defined at the file module scope of `SupportedToken.mo`. Combined with the the use of the token-ledger canister ICP and ICRC1 supertype actors (also defined `SupportedToken.mo`), any additional ICP or ICRC1 token-ledger canisters to be supported can quite literally be copy and pasted, with no modification necessary other than updating the corresponding variant tags in each case. An easy way to do this is edit the `SupportedToken` generic variant to add or remove a member, and observe where the Motoko VSCode extension highlights all the corresponding switch cases that need updating. 

ㅤAlso observe that the implementation of `Invoice.mo` in the `motoko-seller-client` and its `SupportedToken.mo` module only contains two members of this generic variant `#ICP` and `#ICRC1` (this and the use of the `MockTokenLedgerCanister.mo` mock ledgers, `deposit_free_money()` API method, and adding the seller canister id as an allowed creator are the only difference between the two `Invoice.mo` files), which may be easier to start working with (also, no in-body comments). 

ㅤBecause bounded generics aren't a thing in Motoko yet, and at some point a line had to be drawn to prioritize the invoice canister bounty being completed, this is not yet as developed as it could be. However, it should make integrating additional token-ledger canisters quite easy. The reason there are four in the main project's `Invoice.mo` with such extensive testing is to demonstrate the practical functionality of these modules. 

##### An Important Functionality Implication     

ㅤWhile the Invoice Canister itself uses `Nat` as a normalized base unit for all token type's amounts, whenever this value is returned to the caller it is always returned as the type as it is defined in the specification of its token-ledger canister as the argument of the `SupportedToken` variant tag for that token type. In other words, when querying an ICP creator subaccount balance, the returned result (in Motoko syntax) would be:  

 `#ok({ balance = #ICP{ e8s = 1000000000000000 }});`   

 or if an ICRC1 token with it's specific `SupportedToken` variant tag defined as `ICRC1_XDRxckBTC` the returned result (in Motoko syntax) would be:  

`#ok({ balance = #ICRC1_XDRxckBTC(1000000000000000));`  

ㅤAll the `SupportedToken` types work the same way, and in particular be aware of that for addresses (`SupportedToken.Address`). For example, `to_other_address_format()` returns its `#ok` argument as a record of the form:

`{ asText; asAddress }` 

where that address is not simply an account or account identifier, but that account or account identifier is the argument of the `SupportedToken` variant tag defined for that token (ie in the example immediately above, instead of the `{ e8s }` as the argument, it'd be `#ICP(<account identifier>`). The same is true for when a transfer call doesn't fail, but returns `#Err`. The specific token ledger-canister error result is rewrapped and returned as the `SupportedToken.TransferErr` which is that `#Err`'s argument but as the corresponding `SupportedToken` variant tag's argument, (or technically then also wrapped by the API method's error result type, ie `#err{kind : SupportedTokenTransferErr}`). In other words if the authorized caller tried to use the `transfer()` API method to move more ICP e8s out of their creator subaccount address than they have as balance, the caller would be returned (in Motoko syntax):  

`#err({ kind = #ICP(#InsufficientFunds { balance = { e8s = 1000} } )})`

ㅤWhile not totally ideal, hopefully this still provides more utility than a headache. The motivation was to, in the event an intercanster called did not return the expected ok type, pass back the error directly to the caller so they could decide what to do with it. A similar technique is used when returning caught errors literalized as the argument of the `#err kind = CaughtException : ErrorLiteral`. If it really presents a problem, it should not be difficult to unwrap the argument from the variant's tag immediately before returning to the caller. In the spirit of Motoko's strongly functional static typing (and that a variant is required to return either ICP or ICRC1 address types in the same method as the same result), this is not done by default.  

## Future Proofing    

ㅤAs handy as variants are, they can pose quite a headache if used incorrectly. Specifically they pose a challenge in upgrading safely: if a caller cannot correctly handle switching on a variant that has an added tag since they last called, this can be a breaking change. It is well explained in this forum post: [using variant variables in the canister public interface return value breaks composability](https://forum.dfinity.org/t/using-variant-variables-in-the-canister-public-interface-return-value-breaks-composability/16960/5). 

ㅤFortunately there is a simple solution that now has official support in Candid: [wrapping the whole variant in an optional type](https://forum.dfinity.org/t/new-candid-version-and-catching-send-failures-motoko-updates/18410/3). If the client has not upgraded their switch case handler, it will safely fall back to the null case. While this may seem to present more trouble than it is worth, in a world where smart contracts are capable of handling vast sums of currency with signficant real world value, there's no reason not to take advantage of the functional exactitude type safety provides a smart contract language.

ㅤIf this does present an intractable problem or is prohibited by technical requirements, an alternative is to only use the `SupportedToken` variant type privately and create a converter to map a `Text` type that represents which token is being specified when processing a caller's input and preparing what they'll expect as output. If using a mapping collection other than the one already provided, if it requires serialization with the pre and post upgrade system hooks, also be sure to update your migration logic with it. 

ㅤIn the event an invoice canister to be deployed is going to support only a fixed number of token-ledger canisters, the Invoice Canister can be used as is without the additional optional wrapping of returned types. It may also be worth exploring the possibilities of the new async*/await* syntax to restructure this code's implementation to take advantage of the reusable modularity without the previousily necessary added delay when it comes to making the actual calls to token-ledger canisters.

## Test Coverage  

ㅤUnit and E2E testing coverage includes most all the well defined inputs and expected outputs for each method of the modules and of the invoice canister API. In particular the complete list of actual testing output for both unit and E2E testing can be viewed in `docs/TestingGlossay.md`. Note that for the most part, `#ICP` and `#ICP_nns`, and `#ICRC1_ExampleToken1` and `#ICRC1_ExampleToken2` consist of the same actual tests except for the change in variant tag name. Each is included however to demonstrate fully functional expected operation. 

To run both tests use the command `make all`
To run unit tests run `make test`
To run E2E tests run `make e2e`

ㅤAs there are a lot of E2E tests, it might be more convient run `npm run deploy4Testing`, and once complete navigate to the `test/e2e/` directory and run `npm run test:ui` which will display the results in web browser in a more organized way so they are easier to peruse.  

## Deploying 

ㅤZx is used to script the dfx cli required to get the local replica up and running configured correctly with all the correct canisters deployed and other actions necessary. For ease of use two npm run scripts have been added:

`npm run deployAll` 
`npm run deploy4Testing`  

which will run the `clean-spinup.mjs` script with the different flags for either running for testing or not. 

## Non-goals  

* We do not intend to change the ICP ledger or ICRC1 token-ledger canisters.  
* This interface won't specifically handle minting cycles or other secondary ledger features.  
* Handling escrow payments.  
* Automating recurring payments.  
* Balancing an order book or operating as a dex.  

## Open Questions  

* Should this be a new canister type in `dfx`, a single centralized canister on the NNS subnet, or its own module based library?  
* What's the best way to connect the front-end user experience, from the buyer's perspective, to complete the payment flow? 
* What custodial roles are acceptable for managing finances in smart contracts? What are their conditions?  
* What metadata should be required when processing cryptocurrency payments?

## Basic Payment Flow ( hypothetical ) 

ㅤA canister based storefront can receive a request to purchase included with the principal of the buyer or authorized distributor. From this the call to create an invoice can be made including the principal, if provided, on that invoice's create arg's get and verify permissions list. Once successfuly created, that storefront can save that principal, if provided, along with the id returned for that invoice, and the payment address can be returned so that the invoice's balance can be paid.

ㅤOnce the payment has been satisfied, either the storefront canister or the buyer can initiate checking the status of the payment by calling `verify_invoice`. Once the invoice canister has finished confirming the payment has been made, it can notify the storefront canister that the verified status can then be presented to the buyer, satisfying the payment flow.  

## Additional Helpful Links

[Payments - Invoice Canister Design Review](https://forum.dfinity.org/t/payments-invoice-canister-design-review/)  
[ICRC-1 Official Dfinity Repository](https://github.com/dfinity/ICRC-1)  
[Fungible Tokens 101](https://mmapped.blog/posts/09-fungible-tokens-101.html)  
[Good Practices for Canister Smart Contract Development in Motoko](https://www.bitcoininsider.org/article/187552/good-practices-canister-smart-contract-development-motoko)  
[Internet Computer Wiki](https://wiki.internetcomputer.org/wiki/Internet_Computer_wiki)  

[^1]: Note that the installer's principal is not by default put on the allowed creators list.   
[^2]: Motoko Library [Users with Roles](https://github.com/aviate-labs/auth.mo) by Aviate-Labs.   
[^3]: Also note that the anonymous principal is not authorized to call any of the Invoice Canister's API methods (which includes being in the allowed creators list).
[^4]: [Certified Assets from Motoko PoC/Tutorial](https://forum.dfinity.org/t/certified-assets-from-motoko-poc-tutorial/7263) from the Dfinity developer forums.
[^5]: [Certification by the Ledger Canister in Rust](https://github.com/dfinity/sdk/blob/master/src/dfx/src/lib/operations/ledger.rs)  
[^6]: [An example of validating certified assets by a Typescript client](https://github.com/dfinity/ic/blob/master/typescript/service-worker/src/sw/validation.ts) 
[^8]: [Internet Computer Interface Specification: Certified Data](https://internetcomputer.org/docs/current/references/ic-interface-spec#system-api-certified-data)
[^7]: [Access Control on the Internet Computer](https://github.com/domwoe/access_control) - A demonstration and comparison of two approaches to provide access control that integrates certified assets.
[^8]: [Text compression Motoko](https://forum.dfinity.org/t/text-compression-in-motoko/10306/4)  
[^9]: For some inspiration, here's some example of what [commercially](https://support.google.com/corporate-suppliers/answer/9989647) [successful](https://stripe.com/docs/invoicing/overview#invoice-statuses) [companies](https://www.zoho.com/us/subscriptions/kb/invoices/different-invoice-status.html) [use](https://www.ibm.com/docs/en/control-desk/7.6.1?topic=overview-invoice-statuses). 