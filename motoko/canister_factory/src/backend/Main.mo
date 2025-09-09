import Child "Child";
import AnotherChild "AnotherChild";
import Principal "mo:core/Principal";
import Management "ic:aaaaa-aa";

/// # Motoko Canister Creation Demo
///
/// This actor demonstrates various approaches to creating and managing 
/// canisters on the Internet Computer, including:
/// - Creating canisters using actor classes with the `system` keyword
/// - Installing actor classes on existing canisters
/// - Upgrading and reinstalling canisters
/// - Manual canister creation using the management canister
///
/// ## Key Differences: Actor Class vs Management Canister
///
/// **Actor Class Management** (high-level approach):
/// - Simpler API with automatic WASM installation
/// - Limited canister settings available:
///   - `controllers`, `compute_allocation`, `memory_allocation`, `freezing_threshold`
/// - Good for most common use cases
/// - See: [Actor Class Management Documentation](https://internetcomputer.org/docs/motoko/language-manual#actor-class-management)
///
/// **Management Canister** (low-level approach):
/// - Full control over canister creation and settings
/// - Access to all canister settings including:
///   - `reserved_cycles_limit`, `wasm_memory_limit`, `log_visibility`, `wasm_memory_threshold`
/// - Requires separate steps for creation and code installation
/// - Necessary when advanced canister configuration is needed
/// - See: [IC create_canister Method](https://internetcomputer.org/docs/references/ic-interface-spec#ic-create_canister)
///
/// The actor showcases both high-level actor class operations and 
/// low-level management canister interactions.
persistent actor Main {

  /// Creates a new canister using an actor class with automatic installation.
  ///
  /// This function demonstrates the high-level approach to canister creation
  /// using the `system` keyword with actor classes. The new canister is 
  /// automatically created and the Child actor class is installed in one operation.
  ///
  /// ## Parameters
  /// - `cycles`: The number of cycles to attach to the canister creation
  ///
  /// ## Returns
  /// The Principal ID of the newly created canister
  ///
  /// ## Example
  /// ```motoko
  /// let canisterId = await newActorClass(2_000_000_000_000); // 2T cycles
  /// ```
  ///
  /// ## Note
  /// Actor class management only exposes limited canister settings 
  /// (`controllers`, `compute_allocation`, `memory_allocation`, `freezing_threshold`).
  /// For advanced settings like `wasm_memory_limit` or `log_visibility`, 
  /// use `createAndInstallCanisterManually()` instead.
  ///
  /// ## Reference
  /// [Actor Class Management Documentation](https://internetcomputer.org/docs/motoko/language-manual#actor-class-management)
  public shared ({ caller }) func newActorClass(cycles : Nat) : async Principal {
    let settings = {
      settings = ?{
        controllers = ?[caller, Principal.fromActor(Main)];
        compute_allocation = null;
        freezing_threshold = null;
        memory_allocation = null;
      };
    };
    let canister = await (with cycles) (system Child.Child)(#new settings)();
    return Principal.fromActor(canister);
  };

  /// Creates a new canister and installs an actor class using two-step process.
  ///
  /// This function demonstrates the two-step approach: first creating an empty
  /// canister using the management canister, then installing the Child actor 
  /// class on the existing canister ID.
  ///
  /// ## Parameters
  /// - `cycles`: The number of cycles to attach to the canister creation
  ///
  /// ## Returns
  /// The Principal ID of the canister with the installed actor class
  ///
  /// ## Example
  /// ```motoko
  /// let canisterId = await installActorClass(2_000_000_000_000); // 2T cycles
  /// ```
  ///
  /// ## Reference
  /// [Actor Class Management Documentation](https://internetcomputer.org/docs/motoko/language-manual#actor-class-management)
  public shared ({ caller }) func installActorClass(cycles : Nat) : async Principal {
    let canisterId = await createCanisterWithCycles(caller, cycles);
    let canister = await (system Child.Child)(#install canisterId)();
    return Principal.fromActor(canister);
  };

  /// Upgrades an existing canister to use a different actor class.
  ///
  /// This function demonstrates canister upgrades by taking an existing
  /// canister running the Child actor class and upgrading it to use the
  /// AnotherChild actor class instead. The canister's state is preserved
  /// during the upgrade process.
  ///
  /// ## Parameters
  /// - `canisterId`: The Principal ID of the existing canister to upgrade
  ///
  /// ## Returns
  /// The Principal ID of the upgraded canister instance
  ///
  /// ## Example
  /// ```motoko
  /// let upgradedCanister = await upgradeActorClass(existingCanisterId);
  /// ```
  ///
  /// ## Reference
  /// [Actor Class Management Documentation](https://internetcomputer.org/docs/motoko/language-manual#actor-class-management)
  public func upgradeActorClass(canisterId : Principal) : async Principal {
    let instance = actor (Principal.toText(canisterId)) : Child.Child;
    let newInstance = await (system AnotherChild.AnotherChild)(#upgrade instance)();
    Principal.fromActor(newInstance);
  };

  /// Reinstalls an existing canister with a different actor class.
  ///
  /// This function demonstrates canister reinstallation by taking an existing
  /// canister running the Child actor class and reinstalling it with the
  /// AnotherChild actor class. Unlike upgrade, reinstallation does NOT 
  /// preserve the canister's state - all data is lost.
  ///
  /// ## Parameters
  /// - `canisterId`: The Principal ID of the existing canister to reinstall
  ///
  /// ## Returns
  /// The Principal ID of the reinstalled canister instance
  ///
  /// ## Example
  /// ```motoko
  /// let reinstalledCanister = await reinstallActorClass(existingCanisterId);
  /// ```
  ///
  /// ## Warning
  /// This operation will destroy all existing state in the canister!
  ///
  /// ## Reference
  /// [Actor Class Management Documentation](https://internetcomputer.org/docs/motoko/language-manual#actor-class-management)
  public func reinstallActorClass(canisterId : Principal) : async Principal {
    let instance = actor (Principal.toText(canisterId)) : Child.Child;
    let newInstance = await (system AnotherChild.AnotherChild)(#reinstall instance)();
    Principal.fromActor(newInstance);
  };

  /// Creates a canister manually using the management canister and installs empty WASM code.
  ///
  /// This function demonstrates the low-level approach to canister creation
  /// by directly using the management canister APIs. It first creates an 
  /// empty canister, then installs a minimal empty WASM module on it.
  /// 
  /// This approach gives you full control over the canister creation process
  /// and access to ALL management canister settings that are NOT available 
  /// through actor class management, such as:
  /// - `reserved_cycles_limit` - Upper limit on reserved cycles
  /// - `wasm_memory_limit` - WASM heap memory consumption limit  
  /// - `log_visibility` - Controls who can access canister logs
  /// - `wasm_memory_threshold` - Threshold for low memory hooks
  ///
  /// ## Parameters
  /// - `cycles`: The number of cycles to attach to the canister creation
  ///
  /// ## Returns
  /// The Principal ID of the manually created canister
  ///
  /// ## Example
  /// ```motoko
  /// let canisterId = await createAndInstallCanisterManually(2_000_000_000_000); // 2T cycles
  /// ```
  ///
  /// ## Note
  /// This creates a canister with empty WASM code, not a full actor class.
  /// Use this when you need advanced canister settings not available in actor class management.
  ///
  /// ## Reference
  /// [IC create_canister Method](https://internetcomputer.org/docs/references/ic-interface-spec#ic-create_canister)
  public shared ({ caller }) func createAndInstallCanisterManually(cycles : Nat) : async Principal {
    let canisterId = await createCanisterWithCycles(caller, cycles);
    await installCode(canisterId);
    return canisterId;
  };

  /// Creates an empty canister using the management canister with specified cycles.
  ///
  /// This is a helper function that handles the low-level canister creation
  /// using the management canister API. Unlike actor class management, this
  /// approach provides access to ALL canister settings available in the
  /// management canister's `create_canister` method, including:
  /// - `log_visibility`, `reserved_cycles_limit`, `wasm_memory_limit`, `wasm_memory_threshold`
  /// - Standard settings: `controllers`, `compute_allocation`, `memory_allocation`, `freezing_threshold`
  ///
  /// The created canister will have the caller and this Main actor as controllers.
  ///
  /// ## Parameters
  /// - `caller`: The Principal who initiated the request (becomes a controller)
  /// - `cycles`: The number of cycles to attach to the canister creation
  ///
  /// ## Returns
  /// The Principal ID of the newly created empty canister
  ///
  /// ## Reference
  /// [IC create_canister Method](https://internetcomputer.org/docs/references/ic-interface-spec#ic-create_canister)
  func createCanisterWithCycles(caller : Principal, cycles : Nat) : async Principal {
    let result = await (with cycles) Management.create_canister({
      sender_canister_version = null;
      settings = ?{
        log_visibility = null;
        reserved_cycles_limit = null;
        compute_allocation = null;
        memory_allocation = null;
        controllers = ?[caller, Principal.fromActor(Main)];
        freezing_threshold = null;
        wasm_memory_limit = null;
        wasm_memory_threshold = null;
      };
    });
    return result.canister_id;
  };

  /// Installs empty WASM code on a canister using the management canister.
  ///
  /// This helper function installs a minimal empty WASM module on the specified
  /// canister. The WASM module `"\00\61\73\6d\01\00\00\00"` represents an empty
  /// WebAssembly module that creates a basic empty actor.
  ///
  /// ## Parameters
  /// - `canisterId`: The Principal ID of the canister to install code on
  ///
  /// ## Note
  /// This installs empty WASM code, not a full Motoko actor class.
  func installCode(canisterId : Principal) : async () {
    await Management.install_code({
      canister_id = canisterId;
      mode = #install;
      wasm_module : Blob = "\00\61\73\6d\01\00\00\00"; // this is an empty actor
      arg : Blob = "";
      sender_canister_version = null;
    });
  };
};
