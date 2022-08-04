## Purpose
This canister is created to demonstrate usage of Internet Computer's newest feature: HTTP Requests
for Canisters, where canisters can make remote HTTP calls. Currently, the feature is limited to
access secure (HTTPS) remote services that are served by nodes with IPv6 addresses. Trying to access
non-secure HTTP services or services with only IPv4 addresses will trigger Internet Computer errors.

## How to use the sample dapp
There are two parts to the sample dapp, the frontend UI canister `exchange_rate_assets`, and the
backend provider canister `exchange_rate`. Users should be able to interact with only the frontend
UI canister, by selecting the start time and the end time with the datetime pickers.

The returned rates may not exactly match the user's time selection. (There could be gaps in between
data points, or there could be smaller ranges being returned, or if lucky enough, the returned
dataset fully matches the user's request.) The reason for that is because, to respect rate limiting
on the remote service, we scatter our calls to remote service once per every IC heartbeat.
Consequently, the rate pulling can be a relatively-long asynchronous process. We store all the
previously-pulled rates in memory. As the user submits their request, the rates that are already
available from previous pulls will be returned, while the ones that are not yet available will be
pulled in parallel. If the user spots gaps between requested rates and returned rates, the user
simply needs to wait for some time and retry the request, and likely the full rates will be available then.

## Canister behaviors
This canister uses the example of pulling ICP<->USDC exchange rates from
[Coinbase Candles API](https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getproductcandles).
User requested rates will be put into a request pipe. And the remote HTTP request will be
attempted every 5 IC heartbeats. HTTP request pulls 200 data points, with each data point covering a
1 minute window of sample rate of Coinbase. As a result, each HTTP request to Coinbase covers
200 minutes of data. 

If the user-requested time range is longer than a couple of years, the data points to be returned
by backend canister could potentially be out of canister response upper limit (2MB). As a result,
we cap number of data points to be returned by backend canister to frontend, and increase the
sample interval in order to cover the full spectrum of interested range.

This canister is designed to be as cost effective as possible. There are 2 major factors that affect
cycles usage when it comes to the Canister HTTP Request feature:
- The number of requests being made
- The size of each request and response

And between these 2 factors, the first one (number of remote requests made) has a much higher
effect on cycles cost. So the goal of the canister is to:
- Make as few remote calls as possible
- Make each remote HTTP call as small as possible

However, note that these 2 goals are conflicting each other. Consider 1 year's exchange rate
data, that is a static amount of data that needs to be downloaded. The fewer remote calls we make, the
bigger amount of data each call needs to fetch. The less amount of data each call fetches, the
more remote call the canister has to make. And we bias towards the 1st approach, which is
maximize data fetched by each call as much as possible, to reduce number of calls needed. For the reason
mentioned above, that the number of calls costs much more than call size.

On top of that, we cache data that's already fetched, to save from future user requests
triggering remote HTTP calls again.

## Building the canister into wasm
`cd rust/exchange_rate`
`cargo build --target wasm32-unknown-unknown --release --p exchange_rate`

## Deploy the canister locally
- `dfx start --enable-canister-http` to start a local IC instance with Canister HTTP feature enabled
- `dfx deploy --with-cycles 100000000000` to deploy the `exchange_rate` and `exchange_rate_assets`
  canisters to local IC
- `dfx canister status {canister_id}` to check the status of the canister
