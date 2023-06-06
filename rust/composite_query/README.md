# Installing

```
cd rust/composite_query/src
dfx start
dfx canister create data_partition
```

Take not of the canister ID of the `data_partition` canister and put it in `kv_frontend/src/main.rs` file in constant `CANISTER_IDS`.

```
dfx canister create kv_frontend
dfx build --all
dfx canister install --all
```

# Using the canister

```
$ dfx canister call data_partition put '(1:nat, 1337:nat)'
(null)
$ dfx canister call data_partition put '(1:nat, 42:nat)'
(opt (1_337 : nat))
```

Although we have directly inserted a new key value pair to the data partition (which we really should not),
we can retrieve it via the frontend canister.

```
$ dfx canister call kv_frontend get '(1:nat)'
(opt (42 : nat))
$ dfx canister call kv_frontend lookup '(1:nat)'
(1 : nat, "dmalx-m4aaa-aaaaa-qaanq-cai")
```

Of course, we can now overwrite it via the frontend:
```
$ dfx canister call kv_frontend put '(1:nat, 1337:nat)'
(opt (42 : nat))
$ time dfx canister call kv_frontend get '(1:nat)'
(opt (1_337 : nat))
```

We can also query it via a (duplicate) method doing update calls
```
$ time dfx canister call kv_frontend get_update '(1:nat)'
(opt (1_337 : nat))
```

It's also possible to do *two* query calls, first into the frontend and then into the data partition:
```
$ dfx canister call kv_frontend lookup '(1: nat)'
(1 : nat, "dmalx-m4aaa-aaaaa-qaanq-cai")
$ dfx canister call dmalx-m4aaa-aaaaa-qaanq-cai get '(1: nat)' --query
(1_337 : nat)
```