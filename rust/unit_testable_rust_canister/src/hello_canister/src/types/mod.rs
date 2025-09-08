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

    /// Tokens amount in e8s (1 ICP = 100,000,000 e8s)
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    pub struct Tokens {
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
        pub trigger_threshold: u64,
        pub num_blocks_to_archive: u64,
        pub controller_id: Principal,
        pub cycles_for_archive_creation: Option<u64>,
    }

    /// ICP Ledger initialization payload - this is the main variant type
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub enum LedgerArg {
        Init(LedgerInit),
    }

    /// ICP Ledger initialization record
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct LedgerInit {
        pub minting_account: String, // Account identifier as hex string
        pub initial_values: Vec<(String, Tokens)>, // (account_id, balance) pairs
        pub send_whitelist: Vec<Principal>,
        pub transfer_fee: Option<Tokens>,
        pub token_symbol: Option<String>,
        pub token_name: Option<String>,
        pub archive_options: Option<ArchiveOptions>,
    }

    impl LedgerInit {
        pub fn default_for_tests() -> Self {
            // Use proper account identifier hex strings for testing
            let minting_account =
                "5b315d2f6702cb3a27d826161797d7b2c2e131cd312aece51d4d5574d1247087".to_string();
            let test_account =
                "2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138".to_string();

            Self {
                minting_account,
                initial_values: vec![
                    (test_account, Tokens::from_icp(10_000)), // 10,000 ICP for testing
                ],
                send_whitelist: vec![],
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
            }
        }
    }

    /// Account identifier for old ICP ledger (not ICRC-1)
    pub type AccountIdentifier = String;

    /// Transfer arguments for the old ICP ledger transfer method
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct TransferArgs {
        pub memo: u64,
        pub amount: Tokens,
        pub fee: Tokens,
        pub from_subaccount: Option<Vec<u8>>,
        pub to: AccountIdentifier, // Account identifier as hex string
        pub created_at_time: Option<u64>,
    }
}
