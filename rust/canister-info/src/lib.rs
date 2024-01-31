use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::{
    canister_info, CanisterChange,
    CanisterChangeDetails::{CodeDeployment, CodeUninstall, ControllersChange, Creation},
    CanisterChangeOrigin::{FromCanister, FromUser},
    CanisterInfoRequest, CanisterInfoResponse,
};
use serde::Deserialize;

/// Returns canister info with all available canister changes for a canister characterized by a given principal.
/// Traps if the canister_info management call is rejected (in particular, if the principal does not characterize a canister).
#[ic_cdk::update]
async fn info(canister_id: Principal) -> CanisterInfoResponse {
    let request = CanisterInfoRequest {
        canister_id,
        num_requested_changes: Some(20),
    };
    canister_info(request).await.unwrap().0
}

/// Returns all (reflexive and transitive) controllers of a canister characterized by a given principal
/// by implementing BFS over the controllers.
#[ic_cdk::update]
async fn reflexive_transitive_controllers(canister_id: Principal) -> Vec<Principal> {
    let mut ctrls = vec![canister_id];
    let mut queue = vec![canister_id];
    while !queue.is_empty() {
        let cur = queue.pop().unwrap();
        // check if the principal characterizes a canister by determining if it is an opaque principal
        if cur.as_slice().last() == Some(&0x01) {
            let info = info(cur).await;
            for c in info.controllers {
                if !ctrls.contains(&c) {
                    ctrls.push(c);
                    queue.push(c);
                }
            }
        }
    }
    ctrls
}

/// Specifies a canister snapshot by providing a Unix timestamp in nanoseconds
/// or a canister version.
#[derive(CandidType, Deserialize, Clone)]
pub enum CanisterSnapshot {
    #[serde(rename = "at_timestamp")]
    AtTimestamp(u64),
    #[serde(rename = "at_version")]
    AtVersion(u64),
}

/// Maps the latest canister change of the canister characterized by a given principal before or at the given `CanisterSnapshot`
/// and returns an optional integer characterizing the maximum clock skew (in nanoseconds)
/// between the subnet hosting the canister and the given `CanisterSnapshot`
/// (i.e., if this integer is `None`, then no assumptions on clock skew are needed).
/// Returns `None` if no change to map can be determined due to unavailability of canister changes in canister history
/// or due to ambiguity between canister changes with the same timestamp.
/// Traps if a canister_info call is rejected (in particular, if the given principal does not characterize a canister).
async fn map_canister_change<T>(
    canister_id: Principal,
    canister_deployment: CanisterSnapshot,
    f: impl Fn(&CanisterChange) -> Option<T>,
) -> Option<(T, Option<u64>)> {
    let info = info(canister_id).await;
    let mut map_change = None;
    let mut skew = None;
    for c in info.recent_changes {
        if let Some(x) = f(&c) {
            match &canister_deployment {
                CanisterSnapshot::AtTimestamp(t) => {
                    if *t == c.timestamp_nanos {
                        return None;
                    }
                    if *t >= c.timestamp_nanos {
                        map_change = Some(x);
                    }
                    let d = if *t >= c.timestamp_nanos {
                        *t - c.timestamp_nanos
                    } else {
                        c.timestamp_nanos - *t
                    };
                    skew = Some(std::cmp::min(d, skew.unwrap_or(d)));
                }
                CanisterSnapshot::AtVersion(v) => {
                    if *v >= c.canister_version {
                        map_change = Some(x);
                    } else {
                        break;
                    }
                }
            };
        }
    }
    map_change.map(|x| (x, skew))
}

/// Returns the controllers of the canister characterized by a given principal and at the given `CanisterSnapshot`
/// and an optional integer characterizing the maximum clock skew (in nanoseconds)
/// between the subnet hosting the canister and the given `CanisterSnapshot`
/// (i.e., if this integer is `None`, then no assumptions on clock skew are needed).
/// Returns `None` if the controllers cannot be determined due to unavailability of canister changes in canister history
/// or due to ambiguity between canister changes with the same timestamp.
/// Traps if a canister_info call is rejected (in particular, if the given principal does not characterize a canister).
#[ic_cdk::update]
async fn canister_controllers(
    canister_id: Principal,
    canister_deployment: CanisterSnapshot,
) -> Option<(Vec<Principal>, Option<u64>)> {
    map_canister_change(canister_id, canister_deployment, |c| match &c.details {
        Creation(creation) => Some(creation.controllers.clone()),
        ControllersChange(ctrls) => Some(ctrls.controllers.clone()),
        _ => None,
    })
    .await
}

/// Returns the module hash of the canister characterized by a given principal and at the given `CanisterSnapshot`
/// and an optional integer characterizing the maximum clock skew (in nanoseconds)
/// between the subnet hosting the canister and the given `CanisterSnapshot`
/// (i.e., if this integer is `None`, then no assumptions on clock skew are needed).
/// Returns `None` if the module hash cannot be determined due to unavailability of canister changes in canister history
/// or due to ambiguity between canister changes with the same timestamp.
/// Traps if a canister_info call is rejected (in particular, if the given principal does not characterize a canister).
#[ic_cdk::update]
async fn canister_module_hash(
    canister_id: Principal,
    canister_deployment: CanisterSnapshot,
) -> Option<(Option<Vec<u8>>, Option<u64>)> {
    map_canister_change(canister_id, canister_deployment, |c| match &c.details {
        Creation(_) => Some(None),
        CodeUninstall => Some(None),
        CodeDeployment(code_deployment) => Some(Some(code_deployment.module_hash.clone())),
        _ => None,
    })
    .await
}

/// Returns the deployment chain of the canister characterized by a given principal and at the given `CanisterSnapshot`:
/// a list of canister changes starting with the change resulting in the canister deployment characterized by the given principal and at the given `CanisterSnapshot`
/// and with each subsequent change resulting in the canister deployment triggering the previous change,
/// and an optional integer characterizing the maximum clock skew (in nanoseconds)
/// between the subnets hosting the canisters from the deployment chain and the given `CanisterSnapshot`
/// (i.e., if this integer is `None`, then no assumptions on clock skew are needed).
/// The deployment chain stops if a canister change in the deployment chain cannot be determined (due to unavailability of canister changes in canister history
/// or due to ambiguity between canister changes with the same timestamp)
/// or upon encountering a change from a user principal or upon encountering a loop among canisters from the deployment chain.
/// Traps if a canister_info call is rejected (in particular, if the given principal does not characterize a canister).
#[ic_cdk::update]
async fn canister_deployment_chain(
    canister_id: Principal,
    canister_deployment: CanisterSnapshot,
) -> (Vec<CanisterChange>, Option<u64>) {
    let mut current_canister_id = canister_id;
    let mut current_canister_deployment = canister_deployment;
    let mut visited_canister_ids = vec![]; // canister IDs of canisters from the deployment chain
    let mut deployment_chain = vec![];
    let mut skew = None;
    loop {
        visited_canister_ids.push(current_canister_id);
        match map_canister_change(
            current_canister_id,
            current_canister_deployment.clone(),
            |c| match &c.details {
                CodeDeployment(_) => Some(c.clone()),
                _ => None,
            },
        )
        .await
        {
            Some((c, s)) => {
                let mut done = false;
                match &c.origin {
                    FromUser(_) => {
                        done = true;
                    }
                    FromCanister(o) => {
                        if visited_canister_ids.contains(&o.canister_id) {
                            done = true;
                        } else {
                            current_canister_id = o.canister_id;
                            current_canister_deployment = match o.canister_version {
                                None => CanisterSnapshot::AtTimestamp(c.timestamp_nanos),
                                Some(v) => CanisterSnapshot::AtVersion(v),
                            };
                        }
                    }
                };
                deployment_chain.push(c);
                skew = s
                    .map(|dt| Some(std::cmp::min(dt, skew.unwrap_or(dt))))
                    .unwrap_or(skew);
                if done {
                    break;
                }
            }
            None => {
                break;
            }
        };
    }
    (deployment_chain, skew)
}
