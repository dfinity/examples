# HTTP Outcalls in Rust

This project demonstrates how to use the HTTP outcalls feature on the Internet Computer. It contains a single Rust canister with two methods that show how to make both replicated and non-replicated HTTP GET requests to external services.

## Prerequisites

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install).
- [x] Clone this project's repository.

## Step 1: Deploy the canister

Navigate into the project's root directory and run the following commands to start a local replica and deploy the canister:

```shell
dfx start --background --clean && dfx deploy
```

This will deploy a canister named `http_outcalls`.

## Step 2: Call the canister methods

Once deployed, you can test the canister's functions from the command line.

### Replicated Call with a Transform (`get_typicode_post`)

This method makes a call that goes through consensus to fetch a blog post. It uses a transform function to ensure the response is deterministic.

```shell
# Call the method with a post ID (e.g., 1), which calls out to https://jsonplaceholder.typicode.com/posts/1
dfx canister call http_outcalls get_typicode_post '(1: nat64)'
```

**Expected Output:**
You will see the JSON body of the post, indicating a successful call.
```
(
  Ok "{
  \"userId\": 1,
  \"id\": 1,
  \"title\": \"sunt aut facere repellat provident occaecati excepturi optio reprehenderit\",
  \"body\": \"quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto\"
}",
)
```

### Non-Replicated Call (`get_bitcoin_price`)

This method makes a non-replicated call to fetch volatile data—in this case, the price of Bitcoin.

```shell
dfx canister call http_outcalls get_bitcoin_price
```

**Expected Output:**
You will see a JSON response with the current price of Bitcoin in USD. The price will vary each time you call it.
```
(
  Ok "{\"bitcoin\":{\"usd\"::112973}}",
)
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.