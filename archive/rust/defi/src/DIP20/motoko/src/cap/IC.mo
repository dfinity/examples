module {
    public type CanisterSettings = {
        controllers : ?[Principal];
        compute_allocation : ?Nat;
        memory_allocation : ?Nat;
        freezing_threshold : ?Nat;
    };
    public type CanisterId = {
        canister_id: Principal;
    };
    public type InstallMode = {
        #install;
        #reinstall;
        #upgrade;
    };
    public type InstallCodeParams = {
        mode: InstallMode;
        canister_id: Principal;
        wasm_module: Blob;
        arg: Blob;
    };
    public type UpdateSettingsParams = {
        canister_id: Principal;
        settings: CanisterSettings;
    };
    public type CreateCanisterParams = {
        settings: ?CanisterSettings;
    };
    public type Status = {
        #running;
        #stopping;
        #stopped;
    };
    public type CanisterStatus = {
        status: Status;
        settings: CanisterSettings;
        module_hash: ?Blob;
        memory_size: Nat;
        cycles: Nat;
    };
    public type ICActor = actor {
        create_canister: shared(settings: CreateCanisterParams) -> async CanisterId;
        update_settings: shared(params: UpdateSettingsParams) -> async ();
        install_code: shared(params: InstallCodeParams) -> async ();
        canister_status: query(canister_id: CanisterId) -> async CanisterStatus;
    };
}