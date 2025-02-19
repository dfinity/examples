# PubSub Reloaded

This project enhances the original [PubSub example](link-to-original) to provide a clearer demonstration of inter-canister calls on the Internet Computer, specifically showing how functions can be passed as arguments between canisters. While maintaining the simplicity of the original design, this version improves the architecture by:

1. Clearly defining the three key roles in a pub/sub system:

   - Publisher: manages subscriptions and broadcasts messages
   - Subscribers: receive and process messages for their topics of interest
   - Content Creator: generates the content to be published (previously implicit in the original design)

2. Implementing a more intuitive message type: replacing the `Counter` type with a `NewsMessage` type that better represents a real-world pub/sub scenario

3. Supporting multiple subscribers out of the box, with a pre-configured setup that demonstrates how multiple subscribers can receive updates for the same topics

The example maintains the original's simplicity while providing a more practical and comprehensive demonstration of inter-canister communication.

## Overview and Architecture of the Original PubSub App

The original PubSub example implements a system similar to a mailing list or feed subscription service.

### Subscription

Subscribers can register their interest in specific topics through their public `init` function.

```motoko
  public func init(topic0 : Text) {
    Publisher.subscribe({
      topic = topic0;
      callback = updateCount;
    });
  };
```

Note that:

- `init` takes a topic as an argument, which is of type Text. The topic can be whatever topic: the subscriber is not just subscribing topics made available by the publisher, but any possible topic.

- `init` triggers an inter-canister call to the Publisher's `subscribe` function, passing, the topic they're interested in and a callback function (`updateCount`) that will be invoked when new messages arrive. This inter-canister communication is made possible by the Subscriber importing the Publisher canister: `import Publisher "canister:pub"` and the callback function being `public` in the Subscriber actor - Motoko automatically treats public functions from actors as shared when used as inter-canister calls.

The publisher's subscribe function definition and the definition of the type Subscriber and the list subscribers are the following:

```motoko
type Subscriber = {
	topic : Text;
	callback : shared Counter -> ();
};
stable var subscribers = List.nil<Subscriber>();

 public func subscribe(subscriber : Subscriber) {
    subscribers := List.push(subscriber, subscribers);
  };
```

As we can see from the definitions, when the subscribe function is called, an instance of the Subscriber type is added to the list of subscribers.

Note that:

1. The `subscribers` list doesn't track unique subscribers, but rather subscription entries. Each call to `init` adds a new entry to the list, regardless of whether the calling canister has already subscribed to the same or different topics. This means a single subscriber canister can appear multiple times in the list with different topic subscriptions.

2. Every subscriber passes the same function 'updateCount' as the callback function required in the Subscriber type. The different canisters are identified through the fact that the reference of the passed function is different.

3. The `shared` keyword in Motoko is used to designate functions that can be called across canisters. While public actor methods are implicitly shared, the type system needs explicit `shared` annotations when describing function types that will be used for inter-canister calls. For a detailed explanation of sharing functions between actors, see the [Motoko documentation on sharing](https://internetcomputer.org/docs/current/motoko/main/writing-motoko/sharing#the-shared-keyword).

### Content creation and publishing (broadcasting)

If we imagine the PubSub model as a mailing list or a blog, normally we have some content creators and subscribers of the content. The PubSub app resembles the model of a mailing list, where anyone can send a message. The message of the original PubSub app was of type Counter:

```motoko
type Counter = {
	topic : Text;
	value : Nat;
};
```

Each subscriber maintains a counter variable and an update function:

```motoko
var count: Nat = 0;
public func updateCount(counter : Counter) {
	count += counter.value;
};
```

For example, the topic could be "Astronauts" and the value "5". Every time a message of type Counter is published, if the subscriber has subscribed to that message's topic, its internal count variable is increased by the amount specified in the value.

So if a subscriber subscribes to "Astronauts", and then a Counter message is published with an "Astronauts" topic and a value of 5, and then another message with topic of "Astronauts" is published with value 3, the internal counter of the subscriber will be 8. Note that if a subscriber subscribes to multiple topics, the counter will maintain a unique sum for all of them.

## Enhancements

To make this small application more realistic, we will change the type of the broadcasted message to NewsMessage:

```motoko
type NewsMessage = {
    topic : Text;
    content : Text;
    readingTime : Nat;
};
```

This change makes the example more intuitive by:

- Keeping the topic-based subscription mechanism
- Adding actual content (Text) that represents the news message
- Replacing the arbitrary `value` field with a meaningful `readingTime` field that represents the estimated time to read the message

The `readingTime` field maintains the original example's counter functionality (subscribers can track total reading time for their topics) while making the application represent a more realistic news broadcasting scenario.

Therefore, the `count` state of the subscriber has been changed to `totalReadingTime`, which represents the time subscribers would have spent if they had read all the messages they subscribed to. In this context, it makes sense to have an increasing counter even if the subscriber subscribes to multiple topics, as it tracks total reading time across all subscriptions.

The function `init` has been renamed to `subscribeToTopic` as it better reflects its purpose - it's not really initializing anything and can be called multiple times. The new name makes the function's behavior more explicit and self-documenting.

Similarly, `updateCount` becomes `updateTotalReadingTime` to align with the new message type and state variable. This function now adds the reading time of each new message to the subscriber's total, providing a meaningful metric of content consumption.

Finally, the query function `getCount` is renamed to `getTotalReadingTime` to maintain consistency with the new terminology and provide a clearer indication of what information it returns.

### Summary of Changes

1. Message Type:

```motoko
// OLD
type Counter = {
    topic : Text;
    value : Nat;
};

// NEW
type NewsMessage = {
    topic : Text;
    content : Text;
    readingTime : Nat;
};
```

2. Subscriber State:

```motoko
// OLD
var count: Nat = 0;

// NEW
var totalReadingTime: Nat = 0;
```

3. Subscriber Functions:

```motoko
// OLD
public func init(topic0 : Text)

// NEW
public func subscribeToTopic(subscribedTopic : Text)
```

```motoko
// OLD
public func updateCount(counter : Counter) {
    count += counter.value;
};

// NEW
public func updateTotalReadingTime(message : NewsMessage) {
    totalReadingTime += message.readingTime;
};
```

```motoko
// OLD
public query func getCount() : async Nat

// NEW
public query func getTotalReadingTime() : async Nat
```

4. Publisher Type:

```motoko
// OLD
type Subscriber = {
    topic : Text;
    callback : shared Counter -> ();
};

// NEW
type Subscriber = {
    topic : Text;
    callback : shared NewsMessage -> ();
};
```

## Prerequisites

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

## Step 1: Setup the project environment

Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the commands:

```bash
cd examples/motoko/pub-sub
dfx start --background
```

## Step 2: Deploy the canisters:

```bash
dfx deploy
```

## Step 3: Subscribe to the "Apples" topic

```bash
dfx canister call sub init '("Apples")'
```

## Step 4: Publish to the "Apples" topic

```bash
dfx canister call pub publish '(record { "topic" = "Apples"; "value" = 2 })'
```

## Step 5: Receive your subscription

```bash
dfx canister call sub getCount
```

The output should resemble the following:

```bash
(2 : nat)
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app, since it makes inter-canister calls:

- [Be aware that state may change during inter-canister calls.](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview)
- [Only make inter-canister calls to trustworthy canisters.](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview)
- [Don't panic after await and don't lock shared resources across await boundaries.](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview)
