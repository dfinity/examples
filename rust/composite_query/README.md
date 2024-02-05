# Composite queries

## Building the example

We first need to build the data partition backend canister.

```bash
cd rust/composite_query
dfx start --background
dfx canister create data_partition
dfx build data_partition
```

During compilation of the fronted canister, the canister's wasm code will be inlined in the frontend canister's wasm code.

```bash
dfx canister create kv_frontend
dfx build kv_frontend
dfx canister install kv_frontend
```

## Using the canister

Now we add some key value pairs to via the frontend canister.

```bash
dfx canister call kv_frontend put '(1:nat, 1337:nat)'
(null)
dfx canister call kv_frontend put '(1:nat, 42:nat)'
(opt (1_337 : nat))
```

Note that the first call to `put` is slow, since the data partitions have to be created first.

```bash
dfx canister call kv_frontend get '(1:nat)'
(opt (42 : nat))
```

We can also query it via a (duplicate) method doing update calls:

```bash
dfx canister call kv_frontend get_update '(1:nat)'
(opt (1_337 : nat))
```

It's also possible to do *two* query calls, first into the frontend and then into the data partition:

```bash
$ dfx canister call kv_frontend lookup '(1: nat)'
(1 : nat, "dmalx-m4aaa-aaaaa-qaanq-cai")
$ dfx canister call dmalx-m4aaa-aaaaa-qaanq-cai get '(1: nat)' --query
(1_337 : nat)
```
