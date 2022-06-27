## Purpose
This canister is created to demonstrate usage of Internet Computer's newest feature: HTTP Requests for Canisters,
where canister can make remote HTTP calls. Currently, the feature is limited to access secure (HTTPS) remote services
that are served by nodes with IPv6 addresses. Trying to non-secure HTTP services or services with only IPv4 addresses
will trigger Internet Computer errors.

## How to use the sample dapp
There are two parts to the sample dapp, the frontend UI cansiter `exchange_rate_assets`, and the backend provider canister `exchange_rate`. Users should be able to interact with only the frontend UI canister, by selecting the start time and the
end time with the datetime pickers.

The returned rates may not exactly match the users time selection. (There could be gaps in between data points, or there could be smaller range being returned, or if lucky enough, the returned data set fully matches user's interest.) The reason
for that is because, to respect rate limiting on the remote service, we scatter our calls to remote service once per every
IC heartbeats. Consequently, the rate pulling can be a relatively-long asynchronous process. We store all the
previously-pulled rates into memory. As the user submits their interest, the rates that are already available from
previous pulls will be returned, while the ones that are not yet available will be pulled in parallel. If the user spots
gaps between requested rates and returned rates, the user simply wait for some time and retry the request, and likely the
full rates will be available then.

## Canister behaviors
This canister uses the example of pulling ICP<->USDC exchange rates from [Coinbase Candles API](https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getproductcandles). User requested rates will be put into a request pipe. And the
remote HTTP request will be attempted every 5 IC heartbeats. HTTP request pulls 200 data points, with each data point
cover 1 minute window of sample rate of Coinbase. As a result, each HTTP request to Coinbase covers 200 minutes of data. 

If the user interested time range is longer than a couple of years, the data points to be returned by backend canister
could potentially be out of canister response upper limit (2MB). As a result, we cap number of data points to be
returned by backend canister to frontend, and increase the sample interval in order to cover the full spectrum of 
interested range.

## Building the canister into wasm
`cd rust/exchange_rate`
`cargo build --target wasm32-unknown-unknown --release --p exchange_rate`
## Deploy the canister locally
`dfx deploy`

# Mainnet exchange_rate canister Id
mfwph-oqaaa-aaaam-qafsq-cai

Check the status of the canister: `dfx canister --network=ic status mfwph-oqaaa-aaaam-qafsq-cai`
