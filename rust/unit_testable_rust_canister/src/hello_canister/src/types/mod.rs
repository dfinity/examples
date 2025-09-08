use candid::{CandidType, Deserialize};
use serde::Serialize;

// =============================================================================
// REQUEST TYPES (for API evolution)
// =============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetCountRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IncrementCountRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DecrementCountRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ListProposalsRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetProposalInfoRequest {
    pub proposal_id: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetProposalCountRequest {}

// =============================================================================
// RESPONSE TYPES (for API evolution)
// =============================================================================

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetCountResponse {
    pub count: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IncrementCountResponse {
    pub new_count: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct DecrementCountResponse {
    pub new_count: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ListProposalsResponse {
    pub proposal_ids: Option<Vec<u64>>,
    pub error: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetProposalInfoResponse {
    pub proposal: Option<nns_governance::ProposalInfo>,
    pub error: Option<String>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetProposalCountResponse {
    pub count: Option<usize>,
    pub error: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetProposalTitlesRequest {
    pub limit: Option<u32>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetProposalTitlesResponse {
    pub titles: Option<Vec<String>>,
    pub error: Option<String>,
}

// =============================================================================
// NNS GOVERNANCE TYPES (from IC monorepo patterns)
// =============================================================================

pub mod nns_governance;

/// ICP Ledger canister types based on the official ledger.did
/// These match the actual ICP ledger canister interface
pub mod icp_ledger {
    use candid::{CandidType, Deserialize, Principal};
    use serde::Serialize;
    use std::collections::HashMap;

    /// Tokens amount in e8s (1 ICP = 100,000,000 e8s)
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    pub struct Tokens {
        #[n(0)]
        pub e8s: u64,
    }

    impl Tokens {
        pub fn from_e8s(e8s: u64) -> Self {
            Self { e8s }
        }

        pub fn from_icp(icp: u64) -> Self {
            Self {
                e8s: icp * 100_000_000,
            }
        }
    }

    /// Archive options for the ledger
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct ArchiveOptions {
        pub trigger_threshold: usize,
        pub num_blocks_to_archive: usize,
        pub controller_id: Principal,
        pub cycles_for_archive_creation: Option<u64>,
    }

    /// ICP Ledger initialization payload - this is the main variant type
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub enum LedgerArg {
        Init(LedgerInit),
    }

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub struct AccountIdentifier {
        pub hash: [u8; 28],
    }
    pub type Subaccount = [u8; 32];

    #[derive(Serialize, CandidType, Deserialize, Clone, Debug, Copy)]
    pub struct Account {
        #[cbor(n(0), with = "icrc_cbor::principal")]
        pub owner: Principal,
        #[cbor(n(1), with = "minicbor::bytes")]
        pub subaccount: Option<Subaccount>,
    }

    /// ICP Ledger initialization record (not all fields)
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct LedgerInit {
        pub minting_account: AccountIdentifier,
        pub icrc1_minting_account: Option<Account>,
        pub max_message_size_bytes: Option<usize>,
        pub initial_values: HashMap<AccountIdentifier, Tokens>,
        pub archive_options: Option<ArchiveOptions>,
        pub transfer_fee: Option<Tokens>,
        pub token_symbol: Option<String>,
        pub token_name: Option<String>,
    }

    impl LedgerInit {
        pub fn default_for_tests() -> Self {
            Self {
                minting_account,
                icrc1_minting_account: None,
                transfer_fee: Some(Tokens::from_e8s(10_000)), // 0.0001 ICP
                token_symbol: Some("LICP".to_string()),
                token_name: Some("Local ICP".to_string()),
                archive_options: Some(ArchiveOptions {
                    trigger_threshold: 2000,
                    num_blocks_to_archive: 1000,
                    controller_id: Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai")
                        .expect("Invalid archive controller"),
                    cycles_for_archive_creation: Some(1_000_000_000_000), // 1T cycles
                }),
                max_message_size_bytes: None,
                initial_values: Default::default(),
            }
        }
    }

    #[derive(
        Copy,
        Clone,
        Eq,
        PartialEq,
        Ord,
        PartialOrd,
        Hash,
        Debug,
        Default,
        CandidType,
        Deserialize,
        Serialize,
    )]
    pub struct Memo(pub u64);

    #[derive(
        Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, CandidType, Deserialize, Serialize,
    )]
    pub struct TimeStamp {
        #[n(0)]
        timestamp_nanos: u64,
    }

    /// Transfer arguments for the old ICP ledger transfer method
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct TransferArgs {
        pub memo: Memo,
        pub amount: Tokens,
        pub fee: Tokens,
        pub from_subaccount: Option<Vec<u8>>,
        pub to: AccountIdentifier, // Account identifier as hex string
        pub created_at_time: Option<u64>,
    }
}
