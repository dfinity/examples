use candid::{CandidType, Deserialize};
use serde::Serialize;

// =============================================================================
// REQUEST TYPES (for API evolution)
// =============================================================================

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetCounterRequest {}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct IncrementCounterRequest {}

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
pub struct GetCounterResponse {
    pub count: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IncrementCounterResponse {
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

pub mod nns_governance {
    use candid::{CandidType, Deserialize};
    use serde::Serialize;
    use std::collections::BTreeMap;

    #[derive(
        CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
    )]
    pub enum Topic {
        Unspecified = 0,
        ManageNeuron = 1,
        ExchangeRate = 2,
        NetworkEconomics = 3,
        Governance = 4,
        NodeAdmin = 5,
        ParticipantManagement = 6,
        SubnetManagement = 7,
        NetworkCanisterManagement = 8,
        Kyc = 9,
        NodeProviderRewards = 10,
        SnsDecentralizationSale = 11,
        SubnetRental = 12,
        ReplicaVersionManagement = 13,
        SnsAndCommunityFund = 14,
        ApiBoundaryNodeManagement = 15,
        SubnetReplicaVersionManagement = 16,
        ReplicaVersionManagement2 = 17,
        IcOsVersionElection = 18,
        IcOsVersionDeployment = 19,
        ServiceNervousSystemManagement = 20,
    }

    #[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum ProposalStatus {
        Unspecified = 0,
        Open = 1,
        Rejected = 2,
        Adopted = 3,
        Executed = 4,
        Failed = 5,
    }

    /// NNS Governance ListProposals request (based on IC patterns)
    #[derive(CandidType, Deserialize, Clone, Debug, Default)]
    pub struct ListProposals {
        /// Maximum number of proposals to return. Defaults to 10 if not specified.
        pub limit: Option<u32>,
        /// Return proposals strictly before this proposal ID (for pagination)
        pub before_proposal: Option<u64>,
        /// Exclude proposals with these topics
        pub exclude_topic: Option<Vec<Topic>>,
        /// Include only proposals with these statuses
        pub include_status: Option<Vec<ProposalStatus>>,
        /// Include only proposals rewarding votes on these topics
        pub include_reward_status: Option<Vec<bool>>,
        /// Include only proposals with these topics
        pub include_all_manage_neuron_proposals: Option<bool>,
        /// Omit large fields from the response for performance
        pub omit_large_fields: Option<bool>,
    }

    /// NNS Governance ListProposals response (based on IC patterns)
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct ListProposalsResponse {
        /// List of proposals matching the request criteria
        pub proposals: Vec<ProposalInfo>,
        /// Total number of proposals in the governance canister
        pub total_proposal_count: Option<u64>,
    }

    /// Detailed proposal information (simplified version)
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    pub struct ProposalInfo {
        /// Unique identifier for the proposal
        pub id: u64,
        /// The neuron ID that submitted the proposal
        pub proposer: u64,
        /// Timestamp when the proposal was submitted (nanoseconds since epoch)
        pub proposal_timestamp_seconds: u64,
        /// Current status of the proposal
        pub status: ProposalStatus,
        /// Topic category of the proposal
        pub topic: Topic,
        /// Title/summary of the proposal
        pub title: String,
        /// Detailed description of the proposal
        pub summary: String,
        /// URL to additional proposal documentation
        pub url: String,
    }

    /// GetProposal request for getting individual proposal details
    #[derive(CandidType, Deserialize, Clone, Debug)]
    pub struct GetProposal {
        pub proposal_id: u64,
    }

    /// GetProposal response containing the proposal or error
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct GetProposalResponse {
        pub proposal_info: Option<ProposalInfo>,
    }

    /// Neuron ID type
    pub type NeuronId = u64;

    /// Followees for a specific topic
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct Followees {
        pub followees: Vec<NeuronId>,
    }

    /// Network economics parameters for governance canister
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct NetworkEconomics {
        pub neuron_minimum_stake_e8s: u64,
        pub max_proposals_to_keep_per_topic: u32,
        pub neuron_management_fee_per_proposal_e8s: u64,
        pub reject_cost_e8s: u64,
        pub transaction_fee_e8s: u64,
        pub neuron_spawn_dissolve_delay_seconds: u64,
        pub minimum_icp_xdr_rate: u64,
        pub maximum_node_provider_rewards_e8s: u64,
    }

    impl Default for NetworkEconomics {
        fn default() -> Self {
            Self {
                neuron_minimum_stake_e8s: 100_000_000, // 1 ICP
                max_proposals_to_keep_per_topic: 100,
                neuron_management_fee_per_proposal_e8s: 1_000_000, // 0.01 ICP
                reject_cost_e8s: 1_000_000,                        // 0.01 ICP
                transaction_fee_e8s: 10_000,                       // 0.0001 ICP
                neuron_spawn_dissolve_delay_seconds: 7 * 24 * 60 * 60, // 7 days
                minimum_icp_xdr_rate: 100,
                maximum_node_provider_rewards_e8s: 1_000_000_000_000, // 10,000 ICP
            }
        }
    }

    /// Governance canister initialization arguments (complete structure)
    /// Based on the actual IC governance canister requirements
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct GovernanceCanisterInit {
        /// Network economics parameters
        pub economics: Option<NetworkEconomics>,
        /// Default followees for each topic - REQUIRED field according to error message
        pub default_followees: BTreeMap<Topic, Followees>,
        /// Wait for quiet threshold in seconds
        pub wait_for_quiet_threshold_seconds: u64,
        /// Short voting period in seconds  
        pub short_voting_period_seconds: u64,
        /// Initial voting period
        pub initial_voting_period: u64,
        /// Proposal wait for quiet threshold
        pub proposal_wait_for_quiet_threshold_seconds: u64,
        /// Proposal initial voting period
        pub proposal_initial_voting_period: u64,
        /// Neuron management voting period (optional)
        pub neuron_management_voting_period_seconds: Option<u64>,
        /// Neurons fund economics (optional, can be None for testing)
        pub neurons_fund_economics: Option<()>, // Simplified - in real governance this is a complex type
        /// Voting rewards parameters (optional, can be None for testing)
        pub voting_rewards_parameters: Option<()>, // Simplified
        /// Genesis timestamp (optional)
        pub genesis_timestamp_seconds: Option<u64>,
    }

    impl Default for GovernanceCanisterInit {
        fn default() -> Self {
            // Create default followees with empty lists for all topics
            let mut default_followees = BTreeMap::new();

            // Add empty followees for key governance topics
            // In a real setup, these would point to foundation neurons or other trusted entities
            let topics_to_init = vec![
                Topic::Governance,
                Topic::NetworkEconomics,
                Topic::NodeAdmin,
                Topic::SubnetManagement,
                Topic::NetworkCanisterManagement,
                Topic::ExchangeRate,
            ];

            for topic in topics_to_init {
                default_followees.insert(
                    topic,
                    Followees {
                        followees: vec![], // Empty for testing - real deployment would have foundation neurons
                    },
                );
            }

            Self {
                economics: Some(NetworkEconomics::default()),
                default_followees,
                wait_for_quiet_threshold_seconds: 60, // 1 minute for testing
                short_voting_period_seconds: 300,     // 5 minutes for testing
                initial_voting_period: 300,           // 5 minutes for testing
                proposal_wait_for_quiet_threshold_seconds: 60,
                proposal_initial_voting_period: 300,
                neuron_management_voting_period_seconds: Some(300), // 5 minutes for testing
                neurons_fund_economics: None,                       // Not needed for basic testing
                voting_rewards_parameters: None,                    // Not needed for basic testing
                genesis_timestamp_seconds: Some(1640000000),        // Placeholder timestamp
            }
        }
    }
}

/// ICP Ledger canister types and initialization arguments  
/// Based on ICRC-1 standard for modern token ledgers
pub mod icp_ledger {
    use candid::{CandidType, Deserialize, Nat, Principal};
    use serde::Serialize;

    /// Account identifier for ICRC-1 compatible ledgers
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Account {
        /// The principal that owns the account
        pub owner: Principal,
        /// Optional subaccount (32 bytes)
        pub subaccount: Option<Vec<u8>>,
    }

    impl Account {
        /// Create a new account with just an owner (no subaccount)
        pub fn new(owner: Principal) -> Self {
            Self {
                owner,
                subaccount: None,
            }
        }

        /// Create an account with owner and subaccount
        pub fn new_with_subaccount(owner: Principal, subaccount: Vec<u8>) -> Self {
            Self {
                owner,
                subaccount: Some(subaccount),
            }
        }
    }

    /// Metadata for the token (name, symbol, etc.)
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct MetadataEntry {
        pub key: String,
        pub value: MetadataValue,
    }

    /// Different types of metadata values
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub enum MetadataValue {
        Text(String),
        Blob(Vec<u8>),
        Nat(Nat),
        Int(i64),
    }

    /// Feature flags for the ledger
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct FeatureFlags {
        pub icrc2: bool,
    }

    /// Archive options for transaction storage
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct ArchiveOptions {
        /// Number of blocks to archive at once
        pub trigger_threshold: u64,
        /// Maximum number of blocks to store in archive
        pub num_blocks_to_archive: u64,
        /// Controller of archive canisters
        pub controller_id: Principal,
        /// More archive cycles threshold
        pub more_controller_ids: Option<Vec<Principal>>,
        /// Cycles for archive creation
        pub cycles_for_archive_creation: Option<u64>,
        /// Maximum transactions per response
        pub max_transactions_per_response: Option<u64>,
    }

    /// ICP Ledger initialization arguments
    /// Based on ICRC-1 standard with ICP-specific extensions
    #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
    pub struct LedgerInitArgs {
        /// Account that can mint tokens
        pub minting_account: Account,
        /// Fee for transfer operations in e8s (10^-8 tokens)
        pub transfer_fee: u64,
        /// Token symbol (e.g., "ICP")
        pub token_symbol: String,
        /// Token name (e.g., "Internet Computer Protocol")
        pub token_name: String,
        /// Token metadata
        pub metadata: Vec<MetadataEntry>,
        /// Initial account balances
        pub initial_balances: Vec<(Account, u64)>,
        /// Archive options for storing old transactions
        pub archive_options: ArchiveOptions,
        /// Number of decimal places for the token
        pub decimals: Option<u8>,
        /// Feature flags
        pub feature_flags: Option<FeatureFlags>,
        /// Maximum number of accounts
        pub max_memo_length: Option<u16>,
    }

    impl Default for LedgerInitArgs {
        fn default() -> Self {
            // Use a test principal for minting account
            let minting_account = Account::new(
                Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai")
                    .expect("Invalid minting account principal"),
            );

            // Default archive options
            let archive_options = ArchiveOptions {
                trigger_threshold: 2000,
                num_blocks_to_archive: 1000,
                controller_id: Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai")
                    .expect("Invalid controller principal"),
                more_controller_ids: None,
                cycles_for_archive_creation: Some(1_000_000_000_000), // 1T cycles
                max_transactions_per_response: Some(2000),
            };

            // Default metadata for ICP
            let metadata = vec![
                MetadataEntry {
                    key: "icrc1:logo".to_string(),
                    value: MetadataValue::Text(
                        "https://cryptologos.cc/logos/internet-computer-icp-logo.png".to_string(),
                    ),
                },
                MetadataEntry {
                    key: "icrc1:description".to_string(),
                    value: MetadataValue::Text(
                        "The Internet Computer Protocol (ICP) token".to_string(),
                    ),
                },
            ];

            Self {
                minting_account,
                transfer_fee: 10_000, // 0.0001 ICP in e8s
                token_symbol: "ICP".to_string(),
                token_name: "Internet Computer Protocol".to_string(),
                metadata,
                initial_balances: vec![], // Empty for production-like setup
                archive_options,
                decimals: Some(8), // ICP uses 8 decimal places
                feature_flags: Some(FeatureFlags { icrc2: true }),
                max_memo_length: Some(32),
            }
        }
    }
}
