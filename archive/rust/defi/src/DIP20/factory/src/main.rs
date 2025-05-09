use ic_kit::{
    candid::{candid_method, encode_args, CandidType, Deserialize, Nat},
    ic,
    interfaces::{management, Method},
    macros::*,
    Principal, RejectionCode,
};

#[derive(CandidType, Deserialize)]
pub enum TokenType {
    DIP20Motoko,
}

#[derive(CandidType, Deserialize)]
pub enum FactoryError {
    CreateCanisterError,
    CanisterStatusNotAvailableError,
    EncodeError,
    CodeAlreadyInstalled,
    InstallCodeError,
}

const WASM: &[u8] = include_bytes!("../../motoko/.dfx/local/canisters/token/token.wasm");

#[update]
#[candid_method(update)]
pub async fn create(
    logo: String,
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: Nat,
    owner: Principal,
    mut controllers: Vec<Principal>,
    cycles: u64,
    fee: Nat,
    _token_type: TokenType,
) -> Result<Principal, FactoryError> {
    assert_eq!(
        ic_kit::ic::caller(),
        owner,
        "only the owner of this contract can call the create method"
    );

    // create canister
    controllers.push(ic_kit::ic::id());
    let create_settings = management::CanisterSettings {
        controllers: Some(controllers),
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
    };
    use management::{CanisterStatus, InstallMode, WithCanisterId};

    let arg = management::CreateCanisterArgument {
        settings: Some(create_settings),
    };
    let (res,) = match management::CreateCanister::perform_with_payment(
        Principal::management_canister(),
        (arg,),
        cycles,
    )
    .await
    {
        Err(_) => return Err(FactoryError::CreateCanisterError),
        Ok(res) => res,
    };

    let canister_id = res.canister_id;

    // install code
    let (response,) = match CanisterStatus::perform(
        Principal::management_canister(),
        (WithCanisterId { canister_id },),
    )
    .await
    {
        Err(_) => return Err(FactoryError::CanisterStatusNotAvailableError),
        Ok(res) => res,
    };

    if response.module_hash.is_some() {
        return Err(FactoryError::CodeAlreadyInstalled);
    }

    #[derive(CandidType, Deserialize)]
    struct InstallCodeArgumentBorrowed<'a> {
        mode: InstallMode,
        canister_id: Principal,
        #[serde(with = "serde_bytes")]
        wasm_module: &'a [u8],
        arg: Vec<u8>,
    }

    let arg = match encode_args((logo, name, symbol, decimals, total_supply, owner, fee)) {
        Err(_) => return Err(FactoryError::EncodeError),
        Ok(res) => res,
    };

    let install_config = InstallCodeArgumentBorrowed {
        mode: InstallMode::Install,
        canister_id,
        wasm_module: WASM,
        arg,
    };

    if (ic::call(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    )
    .await as Result<(), (RejectionCode, std::string::String)>)
        .is_err()
    {
        return Err(FactoryError::InstallCodeError);
    }

    Ok(canister_id)
}

#[init]
pub fn init(owner: Principal) {
    ic_kit::ic::store(owner);
}

#[pre_upgrade]
pub fn pre_upgragde() {
    ic_kit::ic::stable_store((owner(),)).expect("unable to store data in stable storage")
}

#[post_upgrade]
pub fn post_upgragde() {
    let (owner,) = ic_kit::ic::stable_restore::<(Principal,)>()
        .expect("unable to restore data in stable storage");
    ic_kit::ic::store(owner);
}

#[query]
#[candid_method(query)]
pub fn owner() -> Principal {
    *ic_kit::ic::get_maybe::<Principal>().expect("owner not set")
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
