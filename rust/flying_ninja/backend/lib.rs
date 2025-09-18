use candid::{CandidType, Nat};
use ic_cdk::api::management_canister;

use std::cell::RefCell;

thread_local! {
    // Global state of the canister. Will be wiped when upgrading the canister.
    static STATE: RefCell<State> = RefCell::new(State::default());
}

const LEADERBOARD_SIZE: usize = 10;
type Leaderboard = Vec<LeaderboardEntry>;

#[derive(Debug, Default, Clone, CandidType)]
struct State {
    leaderboard: Leaderboard,
}

#[derive(Debug, Clone, CandidType)]
struct LeaderboardEntry {
    pub name: String,
    pub score: Nat,
}

// Query function to display the leaderboard.
#[ic_cdk::query]
fn get_leaderboard() -> Leaderboard {
    STATE.with(|s| s.borrow().leaderboard.clone())
}

// Query function to determine if a certain score would be added to the leaderboard.
#[ic_cdk::query]
fn is_high_score(score: Nat) -> bool {
    STATE.with(|s| {
        let state = s.borrow();
        if state.leaderboard.len() < LEADERBOARD_SIZE {
            true
        } else {
            // Leaderboard is sorted - we can assume the last entry has the lowest score.
            let last_leaderboard_idx = state.leaderboard.len() - 1;
            let lowest_leaderboard_entry = state.leaderboard.get(last_leaderboard_idx).unwrap();
            score > lowest_leaderboard_entry.score
        }
    })
}

// Update function to add an entry to the leaderboard if it is good enough to warrant an entry.
#[ic_cdk::update]
fn add_leaderboard_entry(name: String, score: Nat) -> Leaderboard {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.leaderboard.push(LeaderboardEntry {
            name, score
        });
        // Sort the leaderboard from highest to lowest score.
        state.leaderboard.sort_by_key(|entry| entry.score.clone());
        state.leaderboard.reverse();
        if state.leaderboard.len() > LEADERBOARD_SIZE {
            state.leaderboard.pop();
        }
        state.leaderboard.clone()
    })
}

// Update function to provide secure randomness as the game seed.
#[ic_cdk::update]
async fn get_randomness() -> Vec<u8> {
    management_canister::main::raw_rand().await.unwrap().0
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
