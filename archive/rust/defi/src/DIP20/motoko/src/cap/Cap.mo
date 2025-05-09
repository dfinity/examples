import Result "mo:base/Result";
import Principal "mo:base/Principal";
import Debug "mo:base/Debug";
import Prelude "mo:base/Prelude";
import Cycles "mo:base/ExperimentalCycles";
import Root "Root";
import Types "Types";
import Router "Router";
// import ic "ic:aaaaa-aa";
import IC "IC";

module {
    public class Cap(canister_id: Principal, creation_cycles: Nat) {
        let router_id = "lj532-6iaaa-aaaah-qcc7a-cai";

        var rootBucket: ?Text = null;
        let ic: IC.ICActor = actor("aaaaa-aa");

        public func getTransaction(id: Nat64) : async Result.Result<Root.Event, Types.GetTransactionError> {
            await awaitForHandshake();

            let root = switch(rootBucket) {
                case(?r) { r };
                case(_) { Prelude.unreachable() };
            };
            let rb: Root.Self = actor(root);

            let transaction_response = await rb.get_transaction({ id=id; witness=false; }); 

            switch(transaction_response) {
                case (#Found(event, witness)) {
                    switch(event) {
                        case (null) {
                            #err(#invalidTransaction)
                        };
                        case (?event) {
                            #ok(event)
                        }
                    }
                };
                case (#Delegate(_, _)) {
                    #err(#unsupportedResponse)
                }
            }
        };

        public func insert(event: Root.IndefiniteEvent) : async Result.Result<Nat64, Types.InsertTransactionError> {
            await awaitForHandshake();

            let root = switch(rootBucket) {
                case(?r) { r };
                case(_) { Prelude.unreachable() };
            };
            let rb: Root.Self = actor(root);

            let insert_response = await rb.insert(event);

            #ok(insert_response)
        };


        /// Returns the principal of the root canister
        public func performHandshake(): async () {
            switch(rootBucket) {
                case(?r) { return; };
                case(_) { };
            };
            let router: Router.Self = actor(router_id);

            let result = await router.get_token_contract_root_bucket({
                witness=false;
                canister=canister_id;
            });

            switch(result.canister) {
                case(null) {
                    let settings: IC.CanisterSettings = {
                        controllers = ?[Principal.fromText(router_id)];
                        compute_allocation = null;
                        memory_allocation = null;
                        freezing_threshold = null;
                    };

                    let params: IC.CreateCanisterParams = {
                        settings = ?settings
                    };

                    // Add cycles and perform the create call
                    Cycles.add(creation_cycles);
                    let create_response = await ic.create_canister(params);

                    // Install the cap code
                    let canister = create_response.canister_id;
                    let router = (actor (router_id) : Router.Self);
                    await router.install_bucket_code(canister);

                    let result = await router.get_token_contract_root_bucket({
                        witness=false;
                        canister=canister_id;
                    });

                    switch(result.canister) {
                        case(null) {
                            // Debug.trap("Error while creating root bucket");
                            assert(false);
                        };
                        case(?canister) {
                            rootBucket := ?Principal.toText(canister);
                        };
                    };
                };
                case (?canister) {
                    rootBucket := ?Principal.toText(canister);
                };
            };
        };

        public func awaitForHandshake(): async () {
            if(rootBucket == null) {
                await performHandshake()
            } else {
                return;
            }
        }
    };
}