# 1. Authentication

### 1.1. Make sure any action that only a specific user should be able to do requires  authentication
* Motoko: Done
* Rust: Done

### 1.2. Disallow the anonymous principal in authenticated calls
* Motoko: Done
* Rust: Done

# 2. Input Validation
* Motoko: Done
* Rust: Done

# 3. Asset Certification

### 3.1. Use HTTP asset certification and avoid serving your dapp through raw.ic0.app
* Motoko: Done
* Rust: Done

# 4. Canister Storage

### 4.1. Use thread_local! with Cell/RefCell for state variables and put all your globals in one basket. 
* Motoko: Not applicable
* Rust: Done

### 4.2. Limit the amount of data that can be stored in a canister per user
* Motoko: Done
* Rust: Done

### 4.3. Consider using stable memory, version it, test it
* Motoko: !TODO!
* Rust: Done (untested)

### 4.4. Don’t store sensitive data on canisters (unless it is encrypted)
* Motoko: Done
* Rust: Done

### 4.5. Create backups
* Motoko: Future
* Rust: Future

# 5. Inter-Canister Calls and Rollbacks

### 5.1. Don’t panic after await and don’t lock shared resources across await boundaries
* Motoko: Done (we don't use await)
* Rust: Done (we don't use await)

### 5.2. Be aware that state may change during inter-canister calls
* Motoko: Done (we have no inter-canister calls)
* Rust: Done (we have no inter-canister calls)

### 5.3. Only make inter-canister calls to trustworthy canisters
* Motoko: Done (we have no inter-canister calls)
* Rust: Done (we have no inter-canister calls)

### 5.4. Make sure there are no loops in call graphs
* Motoko: Done
* Rust: Done

# 6. Canister Upgrades

### 6.1. Don’t panic/trap during upgrades:   
* Motoko: Done, assuming that [`Iter.toArray`](https://github.com/dfinity/motoko-base/blob/master/src/Iter.mo) and [`Map.fromIter`](https://github.com/dfinity/motoko-base/blob/master/src/HashMap.mo) do not trap.
* Rust: Done, assuming that [`borrow_mut`](https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html#tymethod.borrow_mut), [`std::mem::take`](https://doc.rust-lang.org/stable/std/mem/fn.take.html), and [`ic_cdk::storage::stable_save`](https://docs.rs/ic-cdk/latest/ic_cdk/storage/fn.stable_save.html) do not panic. 

# 7. Rust-specific issues

### 7.1. Don’t use unsafe Rust code: 
* Rust: Done

### 7.2. Avoid integer overflows: 
* Rust: Done

# 8. Miscellaneous

### 8.1. For expensive calls, consider using captchas or proof of work
* Motoko: Future
* Rust: Future

### 8.2. Test your canister code even in presence of System API calls
* Motoko: Future
* Rust: Future

### 8.3. Make canister builds reproducible
* Motoko: Done (via Docker)
* Rust: Done (via Docker)

### 8.4. Expose metrics from your canister
* Motoko: Future
* Rust: Future

### 8.5. Don’t rely on time being strictly monotonic
* Motoko: Done
* Rust: Done

### 8.6. Protect against draining the cycles balance
* Motoko: Future
* Rust: Future


# 9. Efficiency considerations

### 10.1. submit_ciphertexts
* Adding submit_ciphertexts is currently O(C*D) where `C =  ciphertexts.size()` and `D = store.device_list.size()`