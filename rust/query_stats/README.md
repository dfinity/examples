# Query stats

This example shows to work with the new query stats feature.

It shows how to programmatically access query stats and also how to monitor them over time by means of a timer and calculate rates out of the monotonically increasing counters of the feature.

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install/) at version 0.16.1 or higher.
- [x] Clone the following project files from GitHub: https://github.com/dfinity/examples/

Begin by opening a terminal window.

This is currently only for use from within DFINITY, as the replica shipped with `dfx` as well as nodes on mainnet do not enable the query stats feature at the moment.

Once enabled on mainnet, the following instructions will work without the `--network` argument.

From within DFINITY, it can be ran as following (e.g. with a deployment to the `small_with_query_stats` Farm testnet)

Variable `$BN_HOSTNAME` points at the hostname of a boundary node. For example, from field `['bn_aaaa_records']['url']` of the output of a Farm deployment

## Installing the canister

```
dfx canister create --network="$BN_HOSTNAME" query_stats
```

We will need the canister ID that has been assigned to our canister later.

```
dfx build --network=$BN_HOSTNAME query_stats
dfx canister install --network=$BN_HOSTNAME query_stats
```

## Issuing load

We now want to generate queries at a certain rate. One way to achieve this is by using `ic-workload-generator`, which is part of the [IC repo](https://github.com/dfinity/ic/tree/master/rs/workload_generator). 


```
ic-workload-generator $BN_HOSTNAME -r 10 -n 6000 --call-method "load" -m query --payload "4449444c0000" --no-status-check --canister-id=$CANISTER_ID
```


## Get query stats

There is a delay until query stats show up in the canister status. Depending on the configuration, this 
can reach from multiple minutes to an hour.

After that period of time, the following call shows the current stats:
```
dfx canister call --network=$BN_HOSTNAME query_stats get_current_query_stats_as_string '()'
```

This is the grant total of queries recorded since the canister was created.

If the query stats are 0, either the features is disabled on the nodes the canister was deployed to,
not enough time has passed since queries have been executed, or no queries have been recorded for this canister.

With the following call, we can query the current rates of those values:

```
dfx canister call --network=$BN_HOSTNAME query_stats get_current_query_stats_as_rates_string '(300)'
```

The argument to that call is the time in seconds over which to aggregate. This needs to be larger than the epoch length. On mainnet and real deployments, this therefore be at least one hour (3600 seconds).

Note that queries might be cached in boundary nodes, so a significantly lower number of calls might be
counted.

Similarly, the following call gets the raw data rather than its string representation:

```
dfx canister call --network=$BN_HOSTNAME query_stats get_current_query_stats_as_rates '(opt 300)'
```


