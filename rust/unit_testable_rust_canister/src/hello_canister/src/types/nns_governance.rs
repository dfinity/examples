// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use candid::{self, CandidType, Deserialize};
use ic_cdk::api::call::CallResult;

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronId {
    pub id: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Followees {
    pub followees: Vec<NeuronId>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct KnownNeuronData {
    name: String,
    description: Option<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct KnownNeuron {
    pub id: Option<NeuronId>,
    pub known_neuron_data: Option<KnownNeuronData>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct FulfillSubnetRentalRequest {
    pub user: Option<candid::Principal>,
    pub replica_version_id: Option<String>,
    pub node_ids: Option<Vec<candid::Principal>>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Spawn {
    pub percentage_to_spawn: Option<u32>,
    pub new_controller: Option<candid::Principal>,
    pub nonce: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Split {
    amount_e8s: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Follow {
    topic: i32,
    followees: Vec<NeuronId>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct AccountIdentifier {
    hash: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Account {
    owner: Option<candid::Principal>,
    subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct DisburseMaturity {
    pub to_account_identifier: Option<AccountIdentifier>,
    pub to_account: Option<Account>,
    pub percentage_to_disburse: u32,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct RefreshVotingPower {}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ClaimOrRefreshNeuronFromAccount {
    pub controller: Option<candid::Principal>,
    pub memo: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum By {
    NeuronIdOrSubaccount {},
    MemoAndController(ClaimOrRefreshNeuronFromAccount),
    Memo(u64),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ClaimOrRefresh {
    by: Option<By>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct RemoveHotKey {
    hot_key_to_remove: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct AddHotKey {
    new_hot_key: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ChangeAutoStakeMaturity {
    pub requested_setting_for_auto_stake_maturity: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct IncreaseDissolveDelay {
    additional_dissolve_delay_seconds: u32,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SetVisibility {
    visibility: Option<i32>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SetDissolveTimestamp {
    dissolve_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Operation {
    RemoveHotKey(RemoveHotKey),
    AddHotKey(AddHotKey),
    ChangeAutoStakeMaturity(ChangeAutoStakeMaturity),
    StopDissolving {},
    StartDissolving {},
    IncreaseDissolveDelay(IncreaseDissolveDelay),
    SetVisibility(SetVisibility),
    JoinCommunityFund {},
    LeaveCommunityFund {},
    SetDissolveTimestamp(SetDissolveTimestamp),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Configure {
    operation: Option<Operation>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ProposalId {
    pub id: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct RegisterVote {
    vote: i32,
    proposal: Option<ProposalId>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Merge {
    source_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct DisburseToNeuron {
    pub dissolve_delay_seconds: u64,
    pub kyc_verified: bool,
    pub amount_e8s: u64,
    pub new_controller: Option<candid::Principal>,
    pub nonce: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct FolloweesForTopic {
    pub topic: Option<i32>,
    pub followees: Option<Vec<NeuronId>>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SetFollowing {
    topic_following: Option<Vec<FolloweesForTopic>>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct StakeMaturity {
    percentage_to_stake: Option<u32>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct MergeMaturity {
    percentage_to_merge: u32,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Amount {
    e8s: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Disburse {
    pub to_account: Option<AccountIdentifier>,
    pub amount: Option<Amount>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Command {
    Spawn(Spawn),
    Split(Split),
    Follow(Follow),
    DisburseMaturity(DisburseMaturity),
    RefreshVotingPower(RefreshVotingPower),
    ClaimOrRefresh(ClaimOrRefresh),
    Configure(Configure),
    RegisterVote(RegisterVote),
    Merge(Merge),
    DisburseToNeuron(DisburseToNeuron),
    SetFollowing(SetFollowing),
    MakeProposal(Box<Proposal>),
    StakeMaturity(StakeMaturity),
    MergeMaturity(MergeMaturity),
    Disburse(Disburse),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum NeuronIdOrSubaccount {
    Subaccount(Vec<u8>),
    NeuronId(NeuronId),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ManageNeuron {
    pub id: Option<NeuronId>,
    pub command: Option<Command>,
    pub neuron_id_or_subaccount: Option<NeuronIdOrSubaccount>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Controllers {
    controllers: Vec<candid::Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct CanisterSettings {
    pub freezing_threshold: Option<u64>,
    pub wasm_memory_threshold: Option<u64>,
    pub controllers: Option<Controllers>,
    pub log_visibility: Option<i32>,
    pub wasm_memory_limit: Option<u64>,
    pub memory_allocation: Option<u64>,
    pub compute_allocation: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct UpdateCanisterSettings {
    pub canister_id: Option<candid::Principal>,
    pub settings: Option<CanisterSettings>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct InstallCode {
    pub skip_stopping_before_installing: Option<bool>,
    pub wasm_module_hash: Option<Vec<u8>>,
    pub canister_id: Option<candid::Principal>,
    pub arg_hash: Option<Vec<u8>>,
    pub install_mode: Option<i32>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct StopOrStartCanister {
    pub action: Option<i32>,
    pub canister_id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Percentage {
    basis_points: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Duration {
    seconds: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Tokens {
    e8s: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct VotingRewardParameters {
    pub reward_rate_transition_duration: Option<Duration>,
    pub initial_reward_rate: Option<Percentage>,
    pub final_reward_rate: Option<Percentage>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct GovernanceParameters {
    pub neuron_maximum_dissolve_delay_bonus: Option<Percentage>,
    pub neuron_maximum_age_for_age_bonus: Option<Duration>,
    pub neuron_maximum_dissolve_delay: Option<Duration>,
    pub neuron_minimum_dissolve_delay_to_vote: Option<Duration>,
    pub neuron_maximum_age_bonus: Option<Percentage>,
    pub neuron_minimum_stake: Option<Tokens>,
    pub proposal_wait_for_quiet_deadline_increase: Option<Duration>,
    pub proposal_initial_voting_period: Option<Duration>,
    pub proposal_rejection_fee: Option<Tokens>,
    pub voting_reward_parameters: Option<VotingRewardParameters>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Image {
    base64_encoding: Option<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct LedgerParameters {
    pub transaction_fee: Option<Tokens>,
    pub token_symbol: Option<String>,
    pub token_logo: Option<Image>,
    pub token_name: Option<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Canister {
    id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronBasketConstructionParameters {
    pub dissolve_delay_interval: Option<Duration>,
    pub count: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct GlobalTimeOfDay {
    seconds_after_utc_midnight: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Countries {
    iso_codes: Vec<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SwapParameters {
    pub minimum_participants: Option<u64>,
    pub neurons_fund_participation: Option<bool>,
    pub duration: Option<Duration>,
    pub neuron_basket_construction_parameters: Option<NeuronBasketConstructionParameters>,
    pub confirmation_text: Option<String>,
    pub maximum_participant_icp: Option<Tokens>,
    pub minimum_icp: Option<Tokens>,
    pub minimum_direct_participation_icp: Option<Tokens>,
    pub minimum_participant_icp: Option<Tokens>,
    pub start_time: Option<GlobalTimeOfDay>,
    pub maximum_direct_participation_icp: Option<Tokens>,
    pub maximum_icp: Option<Tokens>,
    pub neurons_fund_investment_icp: Option<Tokens>,
    pub restricted_countries: Option<Countries>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SwapDistribution {
    total: Option<Tokens>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronDistribution {
    pub controller: Option<candid::Principal>,
    pub dissolve_delay: Option<Duration>,
    pub memo: Option<u64>,
    pub vesting_period: Option<Duration>,
    pub stake: Option<Tokens>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct DeveloperDistribution {
    developer_neurons: Vec<NeuronDistribution>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct InitialTokenDistribution {
    pub treasury_distribution: Option<SwapDistribution>,
    pub developer_distribution: Option<DeveloperDistribution>,
    pub swap_distribution: Option<SwapDistribution>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct CreateServiceNervousSystem {
    pub url: Option<String>,
    pub governance_parameters: Option<GovernanceParameters>,
    pub fallback_controller_principal_ids: Vec<candid::Principal>,
    pub logo: Option<Image>,
    pub name: Option<String>,
    pub ledger_parameters: Option<LedgerParameters>,
    pub description: Option<String>,
    pub dapp_canisters: Vec<Canister>,
    pub swap_parameters: Option<SwapParameters>,
    pub initial_token_distribution: Option<InitialTokenDistribution>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ExecuteNnsFunction {
    nns_function: i32,
    payload: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NodeProvider {
    pub id: Option<candid::Principal>,
    pub reward_account: Option<AccountIdentifier>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct RewardToNeuron {
    dissolve_delay_seconds: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct RewardToAccount {
    to_account: Option<AccountIdentifier>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum RewardMode {
    RewardToNeuron(RewardToNeuron),
    RewardToAccount(RewardToAccount),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct RewardNodeProvider {
    pub node_provider: Option<NodeProvider>,
    pub reward_mode: Option<RewardMode>,
    pub amount_e8s: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronBasketConstructionParameters_1 {
    pub dissolve_delay_interval_seconds: u64,
    pub count: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Params {
    pub min_participant_icp_e8s: u64,
    pub neuron_basket_construction_parameters: Option<NeuronBasketConstructionParameters_1>,
    pub max_icp_e8s: u64,
    pub swap_due_timestamp_seconds: u64,
    pub min_participants: u32,
    pub sns_token_e8s: u64,
    pub sale_delay_seconds: Option<u64>,
    pub max_participant_icp_e8s: u64,
    pub min_direct_participation_icp_e8s: Option<u64>,
    pub min_icp_e8s: u64,
    pub max_direct_participation_icp_e8s: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct OpenSnsTokenSwap {
    pub community_fund_investment_e8s: Option<u64>,
    pub target_swap_canister_id: Option<candid::Principal>,
    pub params: Option<Params>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct TimeWindow {
    start_timestamp_seconds: u64,
    end_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SetOpenTimeWindowRequest {
    open_time_window: Option<TimeWindow>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SetSnsTokenSwapOpenTimeWindow {
    pub request: Option<SetOpenTimeWindowRequest>,
    pub swap_canister_id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SetDefaultFollowees {
    default_followees: Vec<(i32, Followees)>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct RewardNodeProviders {
    pub use_registry_derived_rewards: Option<bool>,
    pub rewards: Vec<RewardNodeProvider>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct VotingPowerEconomics {
    pub start_reducing_voting_power_after_seconds: Option<u64>,
    pub neuron_minimum_dissolve_delay_to_vote_seconds: Option<u64>,
    pub clear_following_after_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Decimal {
    human_readable: Option<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronsFundMatchedFundingCurveCoefficients {
    pub contribution_threshold_xdr: Option<Decimal>,
    pub one_third_participation_milestone_xdr: Option<Decimal>,
    pub full_participation_milestone_xdr: Option<Decimal>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronsFundEconomics {
    pub maximum_icp_xdr_rate: Option<Percentage>,
    pub neurons_fund_matched_funding_curve_coefficients:
        Option<NeuronsFundMatchedFundingCurveCoefficients>,
    pub max_theoretical_neurons_fund_participation_amount_xdr: Option<Decimal>,
    pub minimum_icp_xdr_rate: Option<Percentage>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NetworkEconomics {
    pub neuron_minimum_stake_e8s: u64,
    pub voting_power_economics: Option<VotingPowerEconomics>,
    pub max_proposals_to_keep_per_topic: u32,
    pub neuron_management_fee_per_proposal_e8s: u64,
    pub reject_cost_e8s: u64,
    pub transaction_fee_e8s: u64,
    pub neuron_spawn_dissolve_delay_seconds: u64,
    pub minimum_icp_xdr_rate: u64,
    pub maximum_node_provider_rewards_e8s: u64,
    pub neurons_fund_economics: Option<NeuronsFundEconomics>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Principals {
    principals: Vec<candid::Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Change {
    ToRemove(NodeProvider),
    ToAdd(NodeProvider),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct AddOrRemoveNodeProvider {
    change: Option<Change>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Motion {
    motion_text: String,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Action {
    RegisterKnownNeuron(KnownNeuron),
    FulfillSubnetRentalRequest(FulfillSubnetRentalRequest),
    ManageNeuron(ManageNeuron),
    UpdateCanisterSettings(UpdateCanisterSettings),
    InstallCode(InstallCode),
    StopOrStartCanister(StopOrStartCanister),
    CreateServiceNervousSystem(CreateServiceNervousSystem),
    ExecuteNnsFunction(ExecuteNnsFunction),
    RewardNodeProvider(RewardNodeProvider),
    OpenSnsTokenSwap(OpenSnsTokenSwap),
    SetSnsTokenSwapOpenTimeWindow(SetSnsTokenSwapOpenTimeWindow),
    SetDefaultFollowees(SetDefaultFollowees),
    RewardNodeProviders(RewardNodeProviders),
    ManageNetworkEconomics(NetworkEconomics),
    ApproveGenesisKyc(Principals),
    AddOrRemoveNodeProvider(AddOrRemoveNodeProvider),
    Motion(Motion),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Proposal {
    pub url: String,
    pub title: Option<String>,
    pub action: Option<Action>,
    pub summary: String,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct MakingSnsProposal {
    pub proposal: Option<Box<Proposal>>,
    pub caller: Option<candid::Principal>,
    pub proposer_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct XdrConversionRate {
    pub xdr_permyriad_per_icp: Option<u64>,
    pub timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct MonthlyNodeProviderRewards {
    pub minimum_xdr_permyriad_per_icp: Option<u64>,
    pub registry_version: Option<u64>,
    pub node_providers: Vec<NodeProvider>,
    pub timestamp: u64,
    pub rewards: Vec<RewardNodeProvider>,
    pub xdr_conversion_rate: Option<XdrConversionRate>,
    pub maximum_node_provider_rewards_e8s: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronSubsetMetrics {
    pub total_maturity_e8s_equivalent: Option<u64>,
    pub maturity_e8s_equivalent_buckets: Vec<(u64, u64)>,
    pub voting_power_buckets: Vec<(u64, u64)>,
    pub total_staked_e8s: Option<u64>,
    pub count: Option<u64>,
    pub deciding_voting_power_buckets: Vec<(u64, u64)>,
    pub total_staked_maturity_e8s_equivalent: Option<u64>,
    pub total_potential_voting_power: Option<u64>,
    pub total_deciding_voting_power: Option<u64>,
    pub staked_maturity_e8s_equivalent_buckets: Vec<(u64, u64)>,
    pub staked_e8s_buckets: Vec<(u64, u64)>,
    pub total_voting_power: Option<u64>,
    pub potential_voting_power_buckets: Vec<(u64, u64)>,
    pub count_buckets: Vec<(u64, u64)>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct GovernanceCachedMetrics {
    pub total_maturity_e8s_equivalent: u64,
    pub not_dissolving_neurons_e8s_buckets: Vec<(u64, f64)>,
    pub dissolving_neurons_staked_maturity_e8s_equivalent_sum: u64,
    pub garbage_collectable_neurons_count: u64,
    pub dissolving_neurons_staked_maturity_e8s_equivalent_buckets: Vec<(u64, f64)>,
    pub neurons_with_invalid_stake_count: u64,
    pub not_dissolving_neurons_count_buckets: Vec<(u64, u64)>,
    pub ect_neuron_count: u64,
    pub total_supply_icp: u64,
    pub neurons_with_less_than_6_months_dissolve_delay_count: u64,
    pub dissolved_neurons_count: u64,
    pub community_fund_total_maturity_e8s_equivalent: u64,
    pub total_staked_e8s_seed: u64,
    pub total_staked_maturity_e8s_equivalent_ect: u64,
    pub total_staked_e8s: u64,
    pub fully_lost_voting_power_neuron_subset_metrics: Option<NeuronSubsetMetrics>,
    pub not_dissolving_neurons_count: u64,
    pub total_locked_e8s: u64,
    pub neurons_fund_total_active_neurons: u64,
    pub total_voting_power_non_self_authenticating_controller: Option<u64>,
    pub total_staked_maturity_e8s_equivalent: u64,
    pub not_dissolving_neurons_e8s_buckets_ect: Vec<(u64, f64)>,
    pub spawning_neurons_count: u64,
    pub declining_voting_power_neuron_subset_metrics: Option<NeuronSubsetMetrics>,
    pub total_staked_e8s_ect: u64,
    pub not_dissolving_neurons_staked_maturity_e8s_equivalent_sum: u64,
    pub dissolved_neurons_e8s: u64,
    pub total_staked_e8s_non_self_authenticating_controller: Option<u64>,
    pub dissolving_neurons_e8s_buckets_seed: Vec<(u64, f64)>,
    pub neurons_with_less_than_6_months_dissolve_delay_e8s: u64,
    pub not_dissolving_neurons_staked_maturity_e8s_equivalent_buckets: Vec<(u64, f64)>,
    pub dissolving_neurons_count_buckets: Vec<(u64, u64)>,
    pub dissolving_neurons_e8s_buckets_ect: Vec<(u64, f64)>,
    pub non_self_authenticating_controller_neuron_subset_metrics: Option<NeuronSubsetMetrics>,
    pub dissolving_neurons_count: u64,
    pub dissolving_neurons_e8s_buckets: Vec<(u64, f64)>,
    pub total_staked_maturity_e8s_equivalent_seed: u64,
    pub community_fund_total_staked_e8s: u64,
    pub not_dissolving_neurons_e8s_buckets_seed: Vec<(u64, f64)>,
    pub public_neuron_subset_metrics: Option<NeuronSubsetMetrics>,
    pub timestamp_seconds: u64,
    pub seed_neuron_count: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct RestoreAgingNeuronGroup {
    pub count: Option<u64>,
    pub previous_total_stake_e8s: Option<u64>,
    pub current_total_stake_e8s: Option<u64>,
    pub group_type: i32,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct RestoreAgingSummary {
    pub groups: Vec<RestoreAgingNeuronGroup>,
    pub timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct RewardEvent {
    pub rounds_since_last_distribution: Option<u64>,
    pub day_after_genesis: u64,
    pub actual_timestamp_seconds: u64,
    pub total_available_e8s_equivalent: u64,
    pub latest_round_available_e8s_equivalent: Option<u64>,
    pub distributed_e8s_equivalent: u64,
    pub settled_proposals: Vec<ProposalId>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronStakeTransfer {
    pub to_subaccount: Vec<u8>,
    pub neuron_stake_e8s: u64,
    pub from: Option<candid::Principal>,
    pub memo: u64,
    pub from_subaccount: Vec<u8>,
    pub transfer_timestamp: u64,
    pub block_height: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct GovernanceError {
    error_message: String,
    error_type: i32,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Ballot {
    vote: i32,
    voting_power: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SwapParticipationLimits {
    pub min_participant_icp_e8s: Option<u64>,
    pub max_participant_icp_e8s: Option<u64>,
    pub min_direct_participation_icp_e8s: Option<u64>,
    pub max_direct_participation_icp_e8s: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronsFundNeuronPortion {
    pub controller: Option<candid::Principal>,
    pub hotkeys: Vec<candid::Principal>,
    pub is_capped: Option<bool>,
    pub maturity_equivalent_icp_e8s: Option<u64>,
    pub nns_neuron_id: Option<NeuronId>,
    pub amount_icp_e8s: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronsFundSnapshot {
    pub neurons_fund_neuron_portions: Vec<NeuronsFundNeuronPortion>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct IdealMatchedParticipationFunction {
    pub serialized_representation: Option<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronsFundParticipation {
    pub total_maturity_equivalent_icp_e8s: Option<u64>,
    pub intended_neurons_fund_participation_icp_e8s: Option<u64>,
    pub direct_participation_icp_e8s: Option<u64>,
    pub swap_participation_limits: Option<SwapParticipationLimits>,
    pub max_neurons_fund_swap_participation_icp_e8s: Option<u64>,
    pub neurons_fund_reserves: Option<NeuronsFundSnapshot>,
    pub ideal_matched_participation_function: Option<IdealMatchedParticipationFunction>,
    pub allocated_neurons_fund_participation_icp_e8s: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronsFundData {
    pub final_neurons_fund_participation: Option<NeuronsFundParticipation>,
    pub initial_neurons_fund_participation: Option<NeuronsFundParticipation>,
    pub neurons_fund_refunds: Option<NeuronsFundSnapshot>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct CanisterStatusResultV2 {
    pub status: Option<i32>,
    pub freezing_threshold: Option<u64>,
    pub controllers: Vec<candid::Principal>,
    pub memory_size: Option<u64>,
    pub cycles: Option<u64>,
    pub idle_cycles_burned_per_day: Option<u64>,
    pub module_hash: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct CanisterSummary {
    pub status: Option<CanisterStatusResultV2>,
    pub canister_id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SwapBackgroundInformation {
    pub ledger_index_canister_summary: Option<CanisterSummary>,
    pub fallback_controller_principal_ids: Vec<candid::Principal>,
    pub ledger_archive_canister_summaries: Vec<CanisterSummary>,
    pub ledger_canister_summary: Option<CanisterSummary>,
    pub swap_canister_summary: Option<CanisterSummary>,
    pub governance_canister_summary: Option<CanisterSummary>,
    pub root_canister_summary: Option<CanisterSummary>,
    pub dapp_canister_summaries: Vec<CanisterSummary>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct DerivedProposalInformation {
    pub swap_background_information: Option<SwapBackgroundInformation>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Tally {
    no: u64,
    yes: u64,
    total: u64,
    timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct WaitForQuietState {
    current_deadline_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ProposalData {
    pub id: Option<ProposalId>,
    pub topic: Option<i32>,
    pub failure_reason: Option<GovernanceError>,
    pub ballots: Vec<(u64, Ballot)>,
    pub proposal_timestamp_seconds: u64,
    pub reward_event_round: u64,
    pub failed_timestamp_seconds: u64,
    pub neurons_fund_data: Option<NeuronsFundData>,
    pub reject_cost_e8s: u64,
    pub derived_proposal_information: Option<DerivedProposalInformation>,
    pub latest_tally: Option<Tally>,
    pub total_potential_voting_power: Option<u64>,
    pub sns_token_swap_lifecycle: Option<i32>,
    pub decided_timestamp_seconds: u64,
    pub proposal: Option<Box<Proposal>>,
    pub proposer: Option<NeuronId>,
    pub wait_for_quiet_state: Option<WaitForQuietState>,
    pub executed_timestamp_seconds: u64,
    pub original_total_community_fund_maturity_e8s_equivalent: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Command_2 {
    Spawn(NeuronId),
    Split(Split),
    Configure(Configure),
    Merge(Merge),
    DisburseToNeuron(DisburseToNeuron),
    SyncCommand {},
    ClaimOrRefreshNeuron(ClaimOrRefresh),
    MergeMaturity(MergeMaturity),
    Disburse(Disburse),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronInFlightCommand {
    command: Option<Command_2>,
    timestamp: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct BallotInfo {
    vote: i32,
    proposal_id: Option<ProposalId>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct MaturityDisbursement {
    pub account_identifier_to_disburse_to: Option<AccountIdentifier>,
    pub timestamp_of_disbursement_seconds: Option<u64>,
    pub amount_e8s: Option<u64>,
    pub account_to_disburse_to: Option<Account>,
    pub finalize_disbursement_timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum DissolveState {
    DissolveDelaySeconds(u64),
    WhenDissolvedTimestampSeconds(u64),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Neuron {
    pub id: Option<NeuronId>,
    pub staked_maturity_e8s_equivalent: Option<u64>,
    pub controller: Option<candid::Principal>,
    pub recent_ballots: Vec<BallotInfo>,
    pub voting_power_refreshed_timestamp_seconds: Option<u64>,
    pub kyc_verified: bool,
    pub potential_voting_power: Option<u64>,
    pub neuron_type: Option<i32>,
    pub not_for_profit: bool,
    pub maturity_e8s_equivalent: u64,
    pub deciding_voting_power: Option<u64>,
    pub cached_neuron_stake_e8s: u64,
    pub created_timestamp_seconds: u64,
    pub auto_stake_maturity: Option<bool>,
    pub aging_since_timestamp_seconds: u64,
    pub hot_keys: Vec<candid::Principal>,
    pub account: Vec<u8>,
    pub joined_community_fund_timestamp_seconds: Option<u64>,
    pub maturity_disbursements_in_progress: Option<Vec<MaturityDisbursement>>,
    pub dissolve_state: Option<DissolveState>,
    pub followees: Vec<(i32, Followees)>,
    pub neuron_fees_e8s: u64,
    pub visibility: Option<i32>,
    pub transfer: Option<NeuronStakeTransfer>,
    pub known_neuron_data: Option<KnownNeuronData>,
    pub spawn_at_timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Governance {
    pub default_followees: Vec<(i32, Followees)>,
    pub making_sns_proposal: Option<MakingSnsProposal>,
    pub most_recent_monthly_node_provider_rewards: Option<MonthlyNodeProviderRewards>,
    pub maturity_modulation_last_updated_at_timestamp_seconds: Option<u64>,
    pub wait_for_quiet_threshold_seconds: u64,
    pub metrics: Option<GovernanceCachedMetrics>,
    pub neuron_management_voting_period_seconds: Option<u64>,
    pub node_providers: Vec<NodeProvider>,
    pub cached_daily_maturity_modulation_basis_points: Option<i32>,
    pub economics: Option<NetworkEconomics>,
    pub restore_aging_summary: Option<RestoreAgingSummary>,
    pub spawning_neurons: Option<bool>,
    pub latest_reward_event: Option<RewardEvent>,
    pub to_claim_transfers: Vec<NeuronStakeTransfer>,
    pub short_voting_period_seconds: u64,
    pub proposals: Vec<(u64, ProposalData)>,
    pub xdr_conversion_rate: Option<XdrConversionRate>,
    pub in_flight_commands: Vec<(u64, NeuronInFlightCommand)>,
    pub neurons: Vec<(u64, Neuron)>,
    pub genesis_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Result {
    Ok,
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Result_1 {
    Error(GovernanceError),
    NeuronId(NeuronId),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ClaimOrRefreshNeuronFromAccountResponse {
    result: Option<Result_1>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Result_2 {
    Ok(Neuron),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Result_3 {
    Ok(GovernanceCachedMetrics),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Result_4 {
    Ok(MonthlyNodeProviderRewards),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronInfo {
    pub dissolve_delay_seconds: u64,
    pub recent_ballots: Vec<BallotInfo>,
    pub voting_power_refreshed_timestamp_seconds: Option<u64>,
    pub potential_voting_power: Option<u64>,
    pub neuron_type: Option<i32>,
    pub deciding_voting_power: Option<u64>,
    pub created_timestamp_seconds: u64,
    pub state: i32,
    pub stake_e8s: u64,
    pub joined_community_fund_timestamp_seconds: Option<u64>,
    pub retrieved_at_timestamp_seconds: u64,
    pub visibility: Option<i32>,
    pub known_neuron_data: Option<KnownNeuronData>,
    pub voting_power: u64,
    pub age_seconds: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Result_5 {
    Ok(NeuronInfo),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct GetNeuronsFundAuditInfoRequest {
    nns_proposal_id: Option<ProposalId>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronsFundAuditInfo {
    pub final_neurons_fund_participation: Option<NeuronsFundParticipation>,
    pub initial_neurons_fund_participation: Option<NeuronsFundParticipation>,
    pub neurons_fund_refunds: Option<NeuronsFundSnapshot>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Ok {
    neurons_fund_audit_info: Option<NeuronsFundAuditInfo>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Result_6 {
    Ok(Ok),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct GetNeuronsFundAuditInfoResponse {
    result: Option<Result_6>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Result_7 {
    Ok(NodeProvider),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ProposalInfo {
    pub id: Option<ProposalId>,
    pub status: i32,
    pub topic: i32,
    pub failure_reason: Option<GovernanceError>,
    pub ballots: Vec<(u64, Ballot)>,
    pub proposal_timestamp_seconds: u64,
    pub reward_event_round: u64,
    pub deadline_timestamp_seconds: Option<u64>,
    pub failed_timestamp_seconds: u64,
    pub reject_cost_e8s: u64,
    pub derived_proposal_information: Option<DerivedProposalInformation>,
    pub latest_tally: Option<Tally>,
    pub total_potential_voting_power: Option<u64>,
    pub reward_status: i32,
    pub decided_timestamp_seconds: u64,
    pub proposal: Option<Box<Proposal>>,
    pub proposer: Option<NeuronId>,
    pub executed_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ListKnownNeuronsResponse {
    known_neurons: Vec<KnownNeuron>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronSubaccount {
    subaccount: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ListNeurons {
    pub page_size: Option<u64>,
    pub include_public_neurons_in_full_neurons: Option<bool>,
    pub neuron_ids: Vec<u64>,
    pub page_number: Option<u64>,
    pub include_empty_neurons_readable_by_caller: Option<bool>,
    pub neuron_subaccounts: Option<Vec<NeuronSubaccount>>,
    pub include_neurons_readable_by_caller: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ListNeuronsResponse {
    pub neuron_infos: Vec<(u64, NeuronInfo)>,
    pub full_neurons: Vec<Neuron>,
    pub total_pages_available: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct DateRangeFilter {
    pub start_timestamp_seconds: Option<u64>,
    pub end_timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ListNodeProviderRewardsRequest {
    date_filter: Option<DateRangeFilter>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ListNodeProviderRewardsResponse {
    pub rewards: Vec<MonthlyNodeProviderRewards>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ListNodeProvidersResponse {
    node_providers: Vec<NodeProvider>,
}

#[derive(CandidType, Deserialize, Debug, Default, Clone, serde::Serialize)]
pub struct ListProposalInfo {
    pub include_reward_status: Vec<i32>,
    pub omit_large_fields: Option<bool>,
    pub before_proposal: Option<ProposalId>,
    pub limit: u32,
    pub exclude_topic: Vec<i32>,
    pub include_all_manage_neuron_proposals: Option<bool>,
    pub include_status: Vec<i32>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ListProposalInfoResponse {
    pub proposal_info: Vec<ProposalInfo>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct InstallCodeRequest {
    pub arg: Option<Vec<u8>>,
    pub wasm_module: Option<Vec<u8>>,
    pub skip_stopping_before_installing: Option<bool>,
    pub canister_id: Option<candid::Principal>,
    pub install_mode: Option<i32>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum ProposalActionRequest {
    RegisterKnownNeuron(KnownNeuron),
    FulfillSubnetRentalRequest(FulfillSubnetRentalRequest),
    ManageNeuron(Box<ManageNeuronRequest>),
    UpdateCanisterSettings(UpdateCanisterSettings),
    InstallCode(InstallCodeRequest),
    StopOrStartCanister(StopOrStartCanister),
    CreateServiceNervousSystem(CreateServiceNervousSystem),
    ExecuteNnsFunction(ExecuteNnsFunction),
    RewardNodeProvider(RewardNodeProvider),
    RewardNodeProviders(RewardNodeProviders),
    ManageNetworkEconomics(NetworkEconomics),
    ApproveGenesisKyc(Principals),
    AddOrRemoveNodeProvider(AddOrRemoveNodeProvider),
    Motion(Motion),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct MakeProposalRequest {
    pub url: String,
    pub title: Option<String>,
    pub action: Option<ProposalActionRequest>,
    pub summary: String,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum ManageNeuronCommandRequest {
    Spawn(Spawn),
    Split(Split),
    Follow(Follow),
    DisburseMaturity(DisburseMaturity),
    RefreshVotingPower(RefreshVotingPower),
    ClaimOrRefresh(ClaimOrRefresh),
    Configure(Configure),
    RegisterVote(RegisterVote),
    Merge(Merge),
    DisburseToNeuron(DisburseToNeuron),
    SetFollowing(SetFollowing),
    MakeProposal(MakeProposalRequest),
    StakeMaturity(StakeMaturity),
    MergeMaturity(MergeMaturity),
    Disburse(Disburse),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ManageNeuronRequest {
    pub id: Option<NeuronId>,
    pub command: Option<ManageNeuronCommandRequest>,
    pub neuron_id_or_subaccount: Option<NeuronIdOrSubaccount>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SpawnResponse {
    created_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct DisburseMaturityResponse {
    amount_disbursed_e8s: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct RefreshVotingPowerResponse {}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ClaimOrRefreshResponse {
    refreshed_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct MergeResponse {
    pub target_neuron: Option<Neuron>,
    pub source_neuron: Option<Neuron>,
    pub target_neuron_info: Option<NeuronInfo>,
    pub source_neuron_info: Option<NeuronInfo>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SetFollowingResponse {}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct MakeProposalResponse {
    pub message: Option<String>,
    pub proposal_id: Option<ProposalId>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct StakeMaturityResponse {
    maturity_e8s: u64,
    staked_maturity_e8s: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct MergeMaturityResponse {
    merged_maturity_e8s: u64,
    new_stake_e8s: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct DisburseResponse {
    transfer_block_height: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Command_1 {
    Error(GovernanceError),
    Spawn(SpawnResponse),
    Split(SpawnResponse),
    Follow {},
    DisburseMaturity(DisburseMaturityResponse),
    RefreshVotingPower(RefreshVotingPowerResponse),
    ClaimOrRefresh(ClaimOrRefreshResponse),
    Configure {},
    RegisterVote {},
    Merge(MergeResponse),
    DisburseToNeuron(SpawnResponse),
    SetFollowing(SetFollowingResponse),
    MakeProposal(MakeProposalResponse),
    StakeMaturity(StakeMaturityResponse),
    MergeMaturity(MergeMaturityResponse),
    Disburse(DisburseResponse),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct ManageNeuronResponse {
    command: Option<Command_1>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Committed {
    pub total_direct_contribution_icp_e8s: Option<u64>,
    pub total_neurons_fund_contribution_icp_e8s: Option<u64>,
    pub sns_governance_canister_id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Result_8 {
    Committed(Committed),
    Aborted {},
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SettleCommunityFundParticipation {
    pub result: Option<Result_8>,
    pub open_sns_token_swap_proposal_id: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Committed_1 {
    pub total_direct_participation_icp_e8s: Option<u64>,
    pub total_neurons_fund_participation_icp_e8s: Option<u64>,
    pub sns_governance_canister_id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Result_9 {
    Committed(Committed_1),
    Aborted {},
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SettleNeuronsFundParticipationRequest {
    pub result: Option<Result_9>,
    pub nns_proposal_id: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct NeuronsFundNeuron {
    pub controller: Option<candid::Principal>,
    pub hotkeys: Option<Principals>,
    pub is_capped: Option<bool>,
    pub nns_neuron_id: Option<u64>,
    pub amount_icp_e8s: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct Ok_1 {
    neurons_fund_neuron_portions: Vec<NeuronsFundNeuron>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub enum Result_10 {
    Ok(Ok_1),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct SettleNeuronsFundParticipationResponse {
    result: Option<Result_10>,
}

#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]
pub struct UpdateNodeProvider {
    reward_account: Option<AccountIdentifier>,
}
