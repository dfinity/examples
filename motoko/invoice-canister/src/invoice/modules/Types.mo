import Result "mo:base/Result";
import Time "mo:base/Time";

import SupportedToken "./supported-token/SupportedToken";

/****Types Module of types that are a part of the canister's API.***/
module Types {

  /****Detail types of an invoice.**  
    Includes fields `description : Text` and `meta : Blob`.  
    The meta blob could be a JSON object encoded as UTF-8, providing 
    the particular details related to this invoice such as buyer's 
    contact information, delivery instructions, or the line items 
    this invoice is responsible for rendering as a service or 
    payment. Note that by default neither the description nor the
    meta blob is encrypted, so be aware unless it is encrypted before
    being included as an argument of `create_invoice()` these details 
    could be physically inspected by a node provider. If privacy is 
    needed, encrypt this data before it reaches the invoice canister.  */
  public type Details = {
    description : Text;
    meta : Blob;
  };

  /****Permissions types of an invoice.**  
    Includes fields `canGet : [Principal]` and `canVerify : [Principal]`.  
    Either array can be no longer than 256 principals. Callers with their principals  
    in the `canGet` list have permission to call `get_invoice()` for this invoice  
    while callers with their principals in the `canVerify` list have permission to  
    call either `verify_invoice()` or `recover_invoice_subaccount_balance()` for  
    this invoice.  */
  public type Permissions = {
    canGet : [Principal];
    canVerify : [Principal];
  };

  /****ULID generated literal no longer than 26 characters.***/
  public type InvoiceId = Text;

  /****Invoice type used by the invoice canister used to store invoices.**  
    Has fields:  
      -`token : SupportedToken.UnitType`  
      -`id : InvoiceId;`  
      -`creator : Principal;`  
      -`details : ?Details;`  
      -`permissions : ?Permissions;`  
      -`paymentAddress : Text;`  
      -`amountDue : Nat;`  // Is always in base units (ie for ICP is the e8s value).  
      -`amountPaid : Nat;`  
      -`verifiedPaidAtTime : ?Time.Time;`  
    **Note** this is not the record type returned to a caller, that type  
    also includes all this, `tokenVerbose : TokenVerbose`, and `paid : Bool`  
    which will be true if the invoice is successfully verified as paid as well  
    as `verifiedPaidAtTime` not null but the time of verification in ns.  */
  public type Invoice_ = {
    token : SupportedToken.UnitType;
    id : InvoiceId;
    creator : Principal;
    details : ?Details;
    permissions : ?Permissions;
    paymentAddress : Text;
    amountDue : Nat;
    amountPaid : Nat;
    verifiedPaidAtTime : ?Time.Time;
  };

  /****Invoice record type returned to a caller.**  
    See `Invoice_` type declaration for details.  */
  public type Invoice = Invoice_ and {
    paid : Bool;
    tokenVerbose : SupportedToken.TokenVerbose;
  };

  /****Args of method `add_allowed_creator`.**  
    Has field `who : Principal` which is which principal to add.  */
  public type AddAllowedCreatorArgs = {
    who : Principal;
  };

  /****Result types of method `add_allowed_creator`.**  
    Is equal to `Result.Result<AddAllowedCreatorSuccess, AddAllowedCreatorErr>`.  */
  public type AddAllowedCreatorResult = Result.Result<AddAllowedCreatorSuccess, AddAllowedCreatorErr>;

  /****`ok` result type of method `add_allowed_creator`.**  
    Has field `message : Text` confirming principal was added.  */
  public type AddAllowedCreatorSuccess = {
    message : Text;
  };

  /****`err` result type of method `add_allowed_creator`.**  
    Has field:  
    -`kind : {`  
      `#AlreadyAdded`  
      `#AnonymousIneligible`  
      `#MaxAllowed`  
      `#NotAuthorized`  
    `}`  */
  public type AddAllowedCreatorErr = {
    kind : {
      #AlreadyAdded;
      #AnonymousIneligible;
      #MaxAllowed;
      #NotAuthorized;
    };
  };

  /****Args of method `remove_allowed_creator`.**  
    Has field `who : Principal` which is which principal to remove.  */
  public type RemoveAllowedCreatorArgs = {
    who : Principal;
  };

  /****Result types of method `remove_allowed_creator`.**  
    Is equal to `Result.Result<RemoveAllowedCreatorSuccess, RemoveAllowedCreatorErr>`.  */
  public type RemoveAllowedCreatorResult = Result.Result<RemoveAllowedCreatorSuccess, RemoveAllowedCreatorErr>;

  /****`ok` result type of method `remove_allowed_creator`.**  
    Has field `message : Text` confirming principal was removed.  */
  public type RemoveAllowedCreatorSuccess = {
    message : Text;
  };

  /****`err` result type of method `remove_allowed_creator`.**    
    Has field:  
    -`kind : {`  
      `#NotAuthorized`  
      `#NotFound`  
    `}`  */
  public type RemoveAllowedCreatorErr = {
    kind : {
      #NotAuthorized;
      #NotFound;
    };
  };

  /****Result types of method `get_allowed_creators_list`.**  
    Is equal to `Result.Result<GetAllowedCreatorsListSuccess, GetAllowedCreatorsListErr>`.  */
  public type GetAllowedCreatorsListResult = Result.Result<GetAllowedCreatorsListSuccess, GetAllowedCreatorsListErr>;

  /****`ok` result type of method `get_allowed_creators_list`.**  
    Has field `allowed : [Principal]` is the list of principals allowed to create invoices.  */
  public type GetAllowedCreatorsListSuccess = {
    allowed : [Principal];
  };

  /****`err` result type of method `get_allowed_creators_list`.**  
    Has field:  
    -`kind : {` 
      `#NotAuthorized`  
    `}`  */
  public type GetAllowedCreatorsListErr = {
    kind : {
      #NotAuthorized;
    };
  };

  /****Args of method `create_invoice`.**  
    Has fields:  
      -`tokenAmount : SupportedToken.Amount`  
      -`permissions : ?Permissions`  
      -`details : ?Details`  
    `tokenAmount` will differ for each token type by variant tag name, and also by token amount type.  
    Ie for ICP transactions this would look like:  
      `{ ICP = { e8s = <amount of e8s> } }` (or `ICP_nns`)  
    while for ICRC1 transactions:  
      `{ <ICRC1VariantTagTokenName> = <amount of tokens> }`  
    for instance `{ ICRC1_ExampleToken = 1_000_000_000_000 }`.  */
  public type CreateInvoiceArgs = {
    tokenAmount : SupportedToken.Amount;
    details : ?Details;
    permissions : ?Permissions;
  };

  /****Result types of method `create_invoice`.**  
    Is equal to `Result.Result<CreateInvoiceSuccess, CreateInvoiceErr>`.  */
  public type CreateInvoiceResult = Result.Result<CreateInvoiceSuccess, CreateInvoiceErr>;

  /****`ok` result type of method `create_invoice`.**  
    Has field `invoice : Invoice` is the returned invoice which has all the fields  
    `Invoice_` type  has as well as `paid : Bool` and  `tokenVerbose : TokenVerbose`.  */
  public type CreateInvoiceSuccess = {
    invoice : Invoice;
  };

  /****`err` result type of method `create_invoice`.**  
    Has field:  
    -`kind : {`                    
      `#DescriptionTooLarge`  
      `#InsufficientAmountDue`  
      `#MaxInvoicesCreated`  
      `#MetaTooLarge`  
      `#NotAuthorized`  
      `#TooManyPermissions`  
    `}`  */
  public type CreateInvoiceErr = {
    kind : {
      #DescriptionTooLarge;
      #InsufficientAmountDue;
      #MaxInvoicesCreated;
      #MetaTooLarge;
      #NotAuthorized;
      #TooManyPermissions;
    };
  };

  /****Args of method `get_invoice`.**  
    Has field `id : InvoiceId` which is which invoice with this id to get.  */
  public type GetInvoiceArgs = {
    id : InvoiceId;
  };

  /****Result types of method `get_invoice`.**  
    Is equal to `Result.Result<GetInvoiceSuccess, GetInvoiceErr>`.  */
  public type GetInvoiceResult = Result.Result<GetInvoiceSuccess, GetInvoiceErr>;

  /****`ok` result type of method `get_invoice`.**  
    Has field `invoice : Invoice` is the returned invoice which has all the fields  
    `Invoice_` type  has as well as `paid : Bool` and  `tokenVerbose : TokenVerbose`.  */
  public type GetInvoiceSuccess = {
    invoice : Invoice;
  };

  /****`err` result type of method `get_invoice`.**    
    Has field:  
    -`kind : {`                       
      `#NotFound`  
      `#NotAuthorized`  
    `}`  */
  public type GetInvoiceErr = {
    kind : {
      #NotFound;
      #NotAuthorized;
    };
  };

  /****Args of method `get_caller_balance`.**  
    Has field `token : SupportedToken.UnitType` which is which supported token type address  
    to get. For instance: `{ token = #ICP };` or `{ token = #ICRC1_ExampleToken };`  */
  public type GetCallerBalanceArgs = {
    token : SupportedToken.UnitType;
  };

  /****Result types of method `get_caller_balance`.**  
    Is equal to `Result.Result<GetCallerBalanceSuccess, GetCallerBalanceErr>`.*/
  public type GetCallerBalanceResult = Result.Result<GetCallerBalanceSuccess, GetCallerBalanceErr>;

  /****`ok` result type of method `get_caller_balance`.**  
    Has field `balance : SupportedToken.Amount` is the token balance.  */
  public type GetCallerBalanceSuccess = {
    balance : SupportedToken.Amount;
  };

  /****`err` result type of method `get_caller_balance`.**    
    Has field:  
    -`kind : {`                         
      `#NotAuthorized;`  
      `#CaughtException : Text;`  
    `}`  */
  public type GetCallerBalanceErr = {
    kind : {
      #NotAuthorized;
      #CaughtException : Text;
    };
  };

  /****Args of method `get_caller_address`.**  
    Has field `token : SupportedToken.UnitType` which is which supported token type balance  
    to get. For instance: `{ token = #ICP };` or `{ token = #ICRC1_ExampleToken };`  */
  public type GetCallerAddressArgs = {
    token : SupportedToken.UnitType;
  };

  /****Result types of method `get_caller_address`.**  
    Is equal to `Result.Result<GetCallerAddressSuccess, GetCallerAddressErr>`.  */
  public type GetCallerAddressResult = Result.Result<GetCallerAddressSuccess, GetCallerAddressErr>;

  /****`ok` result type of method `get_caller_address`.**  
    Has fields:  
    -`asAddress : SupportedToken.Address;`  
    -`asText : Text;`  
    which is both the token's specific address type and its text equivalent. */
  public type GetCallerAddressSuccess = {
    asAddress : SupportedToken.Address;
    asText : Text;
  };

  /****`err` result type of method `get_caller_address`.**    
    Has field:  
    -`kind : {`                   
      `#NotAuthorized;`  
    `}`  */
  public type GetCallerAddressErr = {
    kind : {
      #NotAuthorized;
    };
  };

  /****Args of method `verify_invoice`.**  
    Has field `id : InvoiceId` which is which invoice with this id to verify.  */
  public type VerifyInvoiceArgs = {
    id : InvoiceId;
  };

  /****Result types of method `verify_invoice`.**  
    Is equal to `Result.Result<VerifyInvoiceSuccess, VerifyInvoiceErr>`.  */
  public type VerifyInvoiceResult = Result.Result<VerifyInvoiceSuccess, VerifyInvoiceErr>;

  /****`ok` result type of method `verify_invoice`.**  
    Is a variant:  
    `{`  
    `#VerifiedPaid : { invoice : Invoice };`  
    `#VerifiedAlready : { invoice : Invoice };`  
    `}`  
    which includes the invoice updated as verified so that `paid : Bool` will  
    return true (as well `verifiedPaidAtTime` no longer being null).  */
  public type VerifyInvoiceSuccess = {
    #VerifiedPaid : { invoice : Invoice };
    #VerifiedAlready : { invoice : Invoice };
  };

  /****`err` result type of method `verify_invoice`.**    
    Has field:  
    -`kind : {`                       
      `#InProgress;`  
      `#IncompletePayment : { partialAmountPaid : SupportedToken.Amount };`  
      `#NotAuthorized;`  
      `#NotFound;`  
      `#Unpaid : { partialAmountPaid : SupportedToken.Amount };`  
      `#SupportedTokenTransferErr : SupportedToken.TransferErr;`  
      `#CaughtException : Text;`  
    `}`  */
  public type VerifyInvoiceErr = {
    kind : {
      #InProgress;
      #IncompletePayment : { partialAmountPaid : SupportedToken.Amount };
      #NotAuthorized;
      #NotFound;
      #Unpaid;
      #SupportedTokenTransferErr : SupportedToken.TransferErr;
      #CaughtException : Text;
    };
  };

  /****Args of method `transfer`.**  
    Has fields:  
     -`tokenAmount : SupportedToken.Amount`  
     -`destination : SupportedToken.RecipientAddress`  
    An example looks like:  
    `{`  
    `  tokenAmount = #ICP({ e8s = 1_000_000_000_000 }) };`  
    `  destination = #HumanReadable("904a62f479c999d141c3c3d98d1066dfbc1864ec92dce98ed9f33d0f951479d1");`  
    `}`  
    or  
    `{`  
    `  tokenAmount = #ICRC1_ExampleToken(1_000_000_000_000) };`  
    `  destination = #CanisterExpected(#ICRC1_ExampleToken({`  
    `    owner = Principal.fromText("q4eej-kyaaa-aaaaa-aaaha-cai");`  
    `    subaccount = null;`  
    `   }))`  
    `}`  
    Note if `CanisterExpected` `RecipientAddress` is passed it must match the  
    token type `tokenAmount` is or err kind #InvalidDestination is returned. */
  public type TransferArgs = {
    tokenAmount : SupportedToken.Amount;
    destination : SupportedToken.RecipientAddress;
  };

  /****Result types of method `transfer`.**  
    Is equal to `Result.Result<SupportedToken.TransferSuccess, TransferError>`.  */
  public type TransferResult = Result.Result<SupportedToken.TransferSuccess, TransferError>;

  /****`err` result type of method `transfer`.**  
    Has field:  
    -`kind : {`  
      `#NotAuthorized;`  
      `#InsufficientTransferAmount;`  
      `#InvalidDestination;`  
      `#SupportedTokenTransferErr : SupportedToken.TransferErr;`         
      `#CaughtException : Text;`  
    `}`  */
  public type TransferError = {
    kind : {
      #NotAuthorized;
      #InsufficientTransferAmount;
      #InvalidDestination;
      #SupportedTokenTransferErr : SupportedToken.TransferErr;
      #CaughtException : Text;
    };
  };

  /****Args of method `recover_invoice_subaccount_balance`.**  
    Has fields:  
     -`id : InvoiceId`  
     -`destination : SupportedToken.RecipientAddress`  
    An example looks like:  
    `{`  
    `  id = "6GNGGRXAKGTXG070DV4GW2JKCJ";`  
    `  destination = #HumanReadable("904a62f479c999d141c3c3d98d1066dfbc1864ec92dce98ed9f33d0f951479d1");`  
    `}`  
    or  
    `{`  
    `  id = "6GNGGRXAKGTXG070DV4GW2JKCJ";`  
    `  destination = #CanisterExpected(#ICRC1_ExampleToken({`  
    `    owner = Principal.fromText("q4eej-kyaaa-aaaaa-aaaha-cai");`  
    `    subaccount = null;`  
    `   }))`  
    `}`  
    Note if `CanisterExpected` `RecipientAddress` is passed it must match the  
    token type of the invoice or err kind #InvalidDestination is returned.  */
  public type RecoverInvoiceSubaccountBalanceArgs = {
    id : InvoiceId;
    destination : SupportedToken.RecipientAddress;
  };

  /****Result types of method `recover_invoice_subaccount_balance`.**  
    Is equal to `Result.Result<RecoverInvoiceSubacccountBalanceSuccess, RecoverInvoiceSubacccountBalanceErr>`. */
  public type RecoverInvoiceSubaccountBalanceResult = Result.Result<RecoverInvoiceSubacccountBalanceSuccess, RecoverInvoiceSubacccountBalanceErr>;

  /****`ok` result type of method `recover_invoice_subaccount_balance`.**  
    Has fields:  
    -`transferSuccess : SupportedToken.TransferSuccess;`  
    -`balanceRecovered : SupportedToken.Amount`  
    which is both the token's specific address type and its text equivalent. */
  public type RecoverInvoiceSubacccountBalanceSuccess = {
    transferSuccess : SupportedToken.TransferSuccess;
    balanceRecovered : SupportedToken.Amount;
  };

  /****`err` result type of method `recover_invoice_subaccount_balance`.**    
    Has field:  
    -`kind : {`                   
      `#InProgress;`  
      `#InsufficientTransferAmount;`  
      `#InvalidDestination;`  
      `#NotAuthorized;`  
      `#NoBalance;`  
      `#NotFound;`  
      `#SupportedTokenTransferErr : SupportedToken.TransferErr;`  
      `#CaughtException : Text;`  
    `}`  */
  public type RecoverInvoiceSubacccountBalanceErr = {
    kind : {
      #InProgress;
      #InsufficientTransferAmount;
      #InvalidDestination;
      #NotAuthorized;
      #NoBalance;
      #NotFound;
      #SupportedTokenTransferErr : SupportedToken.TransferErr;
      #CaughtException : Text;
    };
  };

  /****Args of method `to_other_address_format`.**  
    Has fields:  
     -`address : ?SupportedToken.RecipientAddress`  
     -`token : ?SupportedToken.UnitType`  
    An example looks like:  
    `{`  
    `  token = #ICP_nns;`  
    `  destination = #HumanReadable("904a62f479c999d141c3c3d98d1066dfbc1864ec92dce98ed9f33d0f951479d1");`  
    `}`  
    or  
    `{`  
    `  token = null;`  
    `  destination = #CanisterExpected(#ICRC1_ExampleToken({`  
    `    owner = Principal.fromText("q4eej-kyaaa-aaaaa-aaaha-cai");`  
    `    subaccount = null;`  
    `  }))`  
    `}`  
    Or just a `tokenType` can be passed into to get the default subaccount  
    icp accountIdentifier blob or icrc1 account of the caller's principal.  
    Note this method will return both an address type and its text equivalent.  */
  public type ToOtherAddressFormatArgs = {
    token : ?SupportedToken.UnitType;
    address : ?SupportedToken.RecipientAddress;
  };

  /****Result types of method `to_other_address_format`.**  
    Is equal to `Result.Result<ToOtherAddressFormatSuccess, ToOtherAddressFormatErr>`. */
  public type ToOtherAddressFormatResult = Result.Result<ToOtherAddressFormatSuccess, ToOtherAddressFormatErr>;

  /****`ok` result type of method `to_other_address_format`.**  
    Has fields:  
    -`asAddress : SupportedToken.Address;`  
    -`asText : Text;`  
    which is both the token's specific address type and its text equivalent.  */
  public type ToOtherAddressFormatSuccess = {
    asAddress : SupportedToken.Address;
    asText : Text;
  };

  /****`err` result type of method `to_other_address_format`.**  
    Has field:  
    -`kind : {`                  
      `#NotAuthorized;`  
      `#MissingTokenType;`  
      `#InvalidDestination;`  
    `}`  */
  public type ToOtherAddressFormatErr = {
    kind : {
      #NotAuthorized;
      #MissingTokenType;
      #InvalidDestination;
    };
  };

  /****Interface for instantiating an invoice canister
    actor in another canister using the Invoice Canister API.**  
    When using the invoice canister in another canister an actor declaration of  
    the invoice canister's actor type will be needed, which is this type declaration.  
    _Remember to add that canister as an allowed creator so it is authorized to make calls._  
    An example of using of this:  
    
    ```
    import Types "./modules/Types.mo"
    // Other imports.

    actor CanisterRequiringPaymentProcessing {

      // Other state, type and method declarations.

      let invoiceCanister : Types.InvoiceCanisterAPI = fromActor("<invoice canister id>");

      // Now the API methods can be called such as
      // await invoiceCanister.create_invoice(<CreateInvoiceArgs>);

      // rest of canister's code...
    };
    ``` */
  public type InvoiceCanisterAPI = {
    // The following three methods are only permitted for the invoice canister installer.
    add_allowed_creator : shared AddAllowedCreatorArgs -> async AddAllowedCreatorResult;
    remove_allowed_creator : shared RemoveAllowedCreatorArgs -> async RemoveAllowedCreatorResult;
    get_allowed_creators_list : shared () -> async GetAllowedCreatorsListResult;
    // The following methods are permitted for any caller whose principal is on the
    // allowed creators list, and the invoice canister installer
    create_invoice : shared CreateInvoiceArgs -> async CreateInvoiceResult;
    get_caller_balance : shared GetCallerBalanceArgs -> async GetCallerBalanceResult;
    get_caller_address : shared GetCallerAddressArgs -> async GetCallerAddressResult;
    transfer : shared TransferArgs -> async TransferResult;
    to_other_address_format : shared ToOtherAddressFormatArgs -> async ToOtherAddressFormatResult;
    // The following three methods are only authorized for the invoice's creator and
    // those on its get or verify permission list (recover is permitted by `canVerify` permission).
    get_invoice : shared GetInvoiceArgs -> async GetInvoiceResult;
    verify_invoice : shared VerifyInvoiceArgs -> async VerifyInvoiceArgs;
    recover_invoice_subaccount_balance : shared RecoverInvoiceSubaccountBalanceArgs -> async RecoverInvoiceSubaccountBalanceResult;
  };
};
