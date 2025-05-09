# Token Standard Spec



A fungible token standard for the DFINITY Internet Computer.

## Abstract

A standard token interface is a basic building block for many applications on the Internet Computer, such as wallets and decentralized exchanges, in this specification we propose a standard token interface for fungible tokens on the IC. This standard provides basic functionality to transfer tokens, allow tokens to be approved so they can be spent by a third-party, it also provides interfaces to query history transactions.

## Specification

### 1. Data Structures

1. Metadata: basic token information

   ```js
   type Metadata = {
       logo : Text; // base64 encoded logo or logo url
       name : Text; // token name
       symbol : Text; // token symbol
       decimals : Nat8; // token decimal
       totalSupply : Nat; // token total supply
       owner : Principal; // token owner
       fee : Nat; // fee for update calls
   }
   ```

2. TxReceipt: receipt for update calls, contains the transaction index or an error message

   ```js

   public type TxReceipt = {
       #Ok: Nat;
       #Err: {
            #InsufficientAllowance;
            #InsufficientBalance;
            #ErrorOperationStyle;
            #Unauthorized;
            #LedgerTrap;
            #ErrorTo;
            #Other;
            #BlockUsed;
            #AmountTooSmall;
       };
   };

   when the Transaction status is #failed, an error should be returned instead of a transaction id

   ```

3. TxRecord: history transaction record

   ```js
   public type Operation = {
       #approve;
       #mint;
       #transfer;
       #transferFrom;
   };
   public type TransactionStatus = {
       #succeeded;
       #failed;
   };
   public type TxRecord = {
       caller: ?Principal;
       op: Operation; // operation type
       index: Nat; // transaction index
       from: Principal;
       to: Principal;
       amount: Nat;
       fee: Nat;
       timestamp: Time.Time;
       status: TransactionStatus;
   };
   ```

   `caller` in TxRecord is optional and only need to be non-empty for `transferFrom` calls

### 2. Basic Interfaces

#### Update calls

The update calls described in this section might choose to charge `fee` amount of tokens to prevent DDoS attack, this is necessary because of the reverse gas model of the IC.
All update functions are allowed to trap, instead of returning an error in order to take advantage of the canisters automatic, atomic state rollback.

Please keep in mind that as of now the canisters' stable memory is limited to 8GB. This forces token implementations to come up with their own scalable transaction storage implementation that offloads the data to separate canisters. An other limitation of the Dfinity blockchain is that currently inter-canister query calls are not supported. These limitations together mean that getTransaction and getTransactions functions temporarily have to be update functions.

##### transfer

Transfers `value` amount of tokens to user `to`, returns a `TxReceipt` which contains the transaction index or an error message.

```javascript
public shared(msg) func transfer(to: Principal, value: Nat) : async TxReceipt
```

##### transferFrom

Transfers `value` amount of tokens from user `from` to user `to`, this method allows canster smart contracts to transfer tokens on your behalf, it returns a `TxReceipt` which contains the transaction index or an error message.

```js
public shared(msg) func transferFrom(from: Principal, to: Principal, value: Nat) : async TxReceipt
```

##### approve

Allows `spender` to withdraw tokens from your account, up to the `value` amount. If it is called again it overwrites the current allowance with `value`. There is no upper limit for `value`.

```js
public shared(msg) func approve(spender: Principal, value: Nat) : async TxReceipt
```

##### getTransaction

Returns transaction detail of the transaction identified by `index`. If the `index` is out of range, the execution traps. Transactions are indexed from zero.

```js
public query func getTransaction(index: Nat) : async TxRecord
```

##### getTransactions

Returns an array of transaction records in the range `[start, start + limit)`. To fend off DoS attacks, this function is allowed to trap, if limit is greater than the limit allowed by the token. This function is also allowed to trap if `start + limit > historySize()`

```js
public query func getTransactions(start: Nat, limit: Nat) : async [TxRecord]
```

#### Query calls

##### logo

Returns the logo of the token.

```js
public query func logo() : async Text
```

##### name

Returns the name of the token.

```js
public query func name() : async Text
```

##### symbol

Returns the symbol of the token.

```js
public query func symbol() : async Text
```

##### decimals

Returns the decimals of the token.

```js
public query func decimals() : async Nat8
```

##### totalSupply

Returns the total supply of the token.

```js
public query func totalSupply() : async Nat
```

##### balanceOf

Returns the balance of user `who`.

```js
public query func balanceOf(who: Principal) : async Nat
```

##### allowance

Returns the amount which `spender` is still allowed to withdraw from `owner`.

```js
public query func allowance(owner: Principal, spender: Principal) : async Nat
```

##### getMetadata

Returns the metadata of the token.

```js
public query func getMetadata() : async Metadata
```

##### historySize

Returns the history size.

```js
public query func historySize() : async Nat
```


### 3. Optional interfaces

#### Update calls

The following update calls should be authorized, only the `owner` of the token canister can call these functions.

##### mint

Mint `value` number of new tokens to user `to`, this will increase the token total supply, only `owner` is allowed to mint new tokens.

```js
public shared(msg) func mint(to: Principal, value: Nat): async TxReceipt
```

##### burn

Burn `value` number of new tokens from user `from`, this will decrease the token total supply, only `owner` or the user `from` him/herself can perform this operation.

```js
public shared(msg) func burn(from: Principal, value: Nat): async TxReceipt
```

`aaaaa-aa` is the IC management canister id, it's not a real canister, just an abstraction of system level management functions, it can be used as blackhole address.

##### setName

Change the name of the token, no return value needed.

```js
public shared(msg) func setName(name: Text)
```

##### setLogo

Change the logo of the token, no return value needed. The `logo` can either be a base64 encoded text of the logo picture or an URL pointing to the logo picture.

```js
public shared(msg) func setLogo(logo: Text)
```

##### setFee

Set fee to `newFee` for update calls(`approve`, `transfer`, `transferFrom`), no return value needed.

```javascript
public shared(msg) func setFee(newFee: Nat)
```

##### setFeeTo

Set fee receiver to `newFeeTo` , no return value needed.

```javascript
public shared(msg) func setFeeTo(newFeeTo: Principal)
```

##### setOwner

Set the owner of the token to `newOwner`, no return value needed.

```javascript
public shared(msg) func setOwner(newOwner: Principal)
```

#### Query calls

##### getUserTransactions

Returns an array of transaction records in range `[start, start + limit)` related to user `who` . Unlike `getTransactions`
function, the range [start, start + limit) for getUserTransactions is not the global range of all transactions.
The range [start, start + limit) here pertains to the transactions of user `who`.
Implementations are allowed to return less TxRecords than requested to fend off DoS attacks.

```js
public query func getUserTransactions(who: Principal, start: Nat, limit: Nat) : async [TxRecord]
```

##### getUserTransactionAmount

Returns total number of transactions related to the user `who`.

```js
public query func getUserTransactionAmount(who: Principal) : async Nat
```

### 4. Change log
