//! This is a simple chat backend to demonstrate Canister Snapshots.
use std::{cell::RefCell, collections::HashSet};

thread_local! {
    static CHAT: RefCell<Vec<String>>  = Default::default();
    static LENGTH: RefCell<u64> = Default::default();
}

/// Appends a new message to the chat database.
#[ic_cdk_macros::update]
fn append(message: String) {
    // Ensure that the length of the chat is consistent with the actual chat
    // contents.
    assert_eq!(
        LENGTH.with_borrow(|len| *len),
        CHAT.with_borrow(|chat| chat.len() as u64)
    );
    CHAT.with_borrow_mut(|chat| chat.push(message));
    LENGTH.with_borrow_mut(|len| *len += 1);
}

/// Dumps all the chat messages.
#[ic_cdk_macros::query]
fn dump() -> Vec<String> {
    CHAT.with_borrow(|chat| chat.clone())
}

/// Removes messages containing spam keywords from the chat.
///
/// Returns the number of messages removed.
///
/// Move fast and break things! This method is BROKEN.
/// Can you spot the bug?
///
/// DISCLAIMER: While the Canister Snapshots feature is a valuable tool,
/// it should not be a substitute for comprehensive testing.
#[ic_cdk_macros::update]
fn remove_spam() -> u64 {
    let spam_keywords = HashSet::from(["coupon", "giveaway", "casino"]);

    let chat = CHAT.take();
    let mut new_chat = vec![];
    let mut spam = 0;
    for message in chat {
        if message.split(" ").any(|word| spam_keywords.contains(word)) {
            spam += 1;
            ic_cdk::println!("Found spam message: {message}");
            new_chat.push("(removed spam message)".into());
        } else {
            new_chat.push(message);
        }
    }
    if spam == 0 {
        ic_cdk::println!("No spam keywords found, the chat is unchanged.");
    } else {
        ic_cdk::println!("Removed {spam} messages, updating the chat...");
        CHAT.set(new_chat);
    }
    ic_cdk::println!("Filtered chat: {:?}", CHAT.with_borrow(|chat| chat.clone()));
    spam
}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    let chat = CHAT.with_borrow(|chat| chat.clone());
    let length = LENGTH.with_borrow(|len| *len);
    ic_cdk::storage::stable_save((chat, length)).expect("failed to save stable state");
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    let (chat, length): (Vec<String>, u64) =
        ic_cdk::storage::stable_restore().expect("failed to restore stable state");

    CHAT.set(chat);
    LENGTH.with_borrow_mut(|len| *len = length);
}
