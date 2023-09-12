import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface ActiveAnchorCounter {
  'counter' : bigint,
  'start_timestamp' : Timestamp,
}
export interface ActiveAnchorStatistics {
  'completed' : CompletedActiveAnchorStats,
  'ongoing' : OngoingActiveAnchorStats,
}
export type AddTentativeDeviceResponse = {
    'device_registration_mode_off' : null
  } |
  { 'another_device_tentatively_added' : null } |
  {
    'added_tentatively' : {
      'verification_code' : string,
      'device_registration_timeout' : Timestamp,
    }
  };
export interface AnchorCredentials {
  'recovery_phrases' : Array<PublicKey>,
  'credentials' : Array<WebAuthnCredential>,
  'recovery_credentials' : Array<WebAuthnCredential>,
}
export interface ArchiveConfig {
  'polling_interval_ns' : bigint,
  'entries_buffer_limit' : bigint,
  'module_hash' : Uint8Array | number[],
  'entries_fetch_limit' : number,
}
export interface ArchiveInfo {
  'archive_config' : [] | [ArchiveConfig],
  'archive_canister' : [] | [Principal],
}
export type AuthnMethod = { 'webauthn' : WebAuthn } |
  { 'pubkey' : PublicKeyAuthn };
export type AuthnMethodAddResponse = { 'ok' : null } |
  { 'invalid_metadata' : string };
export interface AuthnMethodData {
  'metadata' : MetadataMap,
  'protection' : AuthnMethodProtection,
  'last_authentication' : [] | [Timestamp],
  'authn_method' : AuthnMethod,
  'purpose' : Purpose,
}
export type AuthnMethodProtection = { 'unprotected' : null } |
  { 'protected' : null };
export interface AuthnMethodRegistrationInfo {
  'expiration' : Timestamp,
  'authn_method' : [] | [AuthnMethodData],
}
export interface BufferedArchiveEntry {
  'sequence_number' : bigint,
  'entry' : Uint8Array | number[],
  'anchor_number' : UserNumber,
  'timestamp' : Timestamp,
}
export interface Challenge {
  'png_base64' : string,
  'challenge_key' : ChallengeKey,
}
export type ChallengeKey = string;
export interface ChallengeResult { 'key' : ChallengeKey, 'chars' : string }
export interface CompletedActiveAnchorStats {
  'monthly_active_anchors' : [] | [ActiveAnchorCounter],
  'daily_active_anchors' : [] | [ActiveAnchorCounter],
}
export type CredentialId = Uint8Array | number[];
export interface Delegation {
  'pubkey' : PublicKey,
  'targets' : [] | [Array<Principal>],
  'expiration' : Timestamp,
}
export type DeployArchiveResult = { 'creation_in_progress' : null } |
  { 'success' : Principal } |
  { 'failed' : string };
export interface DeviceData {
  'alias' : string,
  'metadata' : [] | [MetadataMap],
  'origin' : [] | [string],
  'protection' : DeviceProtection,
  'pubkey' : DeviceKey,
  'key_type' : KeyType,
  'purpose' : Purpose,
  'credential_id' : [] | [CredentialId],
}
export type DeviceKey = PublicKey;
export type DeviceProtection = { 'unprotected' : null } |
  { 'protected' : null };
export interface DeviceRegistrationInfo {
  'tentative_device' : [] | [DeviceData],
  'expiration' : Timestamp,
}
export interface DeviceWithUsage {
  'alias' : string,
  'last_usage' : [] | [Timestamp],
  'metadata' : [] | [MetadataMap],
  'origin' : [] | [string],
  'protection' : DeviceProtection,
  'pubkey' : DeviceKey,
  'key_type' : KeyType,
  'purpose' : Purpose,
  'credential_id' : [] | [CredentialId],
}
export interface DomainActiveAnchorCounter {
  'start_timestamp' : Timestamp,
  'internetcomputer_org_counter' : bigint,
  'ic0_app_counter' : bigint,
  'both_ii_domains_counter' : bigint,
}
export interface DomainActiveAnchorStatistics {
  'completed' : DomainCompletedActiveAnchorStats,
  'ongoing' : DomainOngoingActiveAnchorStats,
}
export interface DomainCompletedActiveAnchorStats {
  'monthly_active_anchors' : [] | [DomainActiveAnchorCounter],
  'daily_active_anchors' : [] | [DomainActiveAnchorCounter],
}
export interface DomainOngoingActiveAnchorStats {
  'monthly_active_anchors' : Array<DomainActiveAnchorCounter>,
  'daily_active_anchors' : DomainActiveAnchorCounter,
}
export type FrontendHostname = string;
export type GetDelegationResponse = { 'no_such_delegation' : null } |
  { 'signed_delegation' : SignedDelegation };
export type HeaderField = [string, string];
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<HeaderField>,
  'certificate_version' : [] | [number],
}
export interface HttpResponse {
  'body' : Uint8Array | number[],
  'headers' : Array<HeaderField>,
  'upgrade' : [] | [boolean],
  'streaming_strategy' : [] | [StreamingStrategy],
  'status_code' : number,
}
export interface IdentityAnchorInfo {
  'devices' : Array<DeviceWithUsage>,
  'device_registration' : [] | [DeviceRegistrationInfo],
}
export interface IdentityInfo {
  'authn_methods' : Array<AuthnMethodData>,
  'authn_data_registration' : [] | [AuthnMethodRegistrationInfo],
}
export type IdentityInfoResponse = { 'ok' : IdentityInfo };
export type IdentityNumber = bigint;
export interface InternetIdentityInit {
  'max_num_latest_delegation_origins' : [] | [bigint],
  'assigned_user_number_range' : [] | [[bigint, bigint]],
  'archive_config' : [] | [ArchiveConfig],
  'canister_creation_cycles_cost' : [] | [bigint],
  'register_rate_limit' : [] | [RateLimitConfig],
}
export interface InternetIdentityStats {
  'storage_layout_version' : number,
  'users_registered' : bigint,
  'domain_active_anchor_stats' : [] | [DomainActiveAnchorStatistics],
  'max_num_latest_delegation_origins' : bigint,
  'assigned_user_number_range' : [bigint, bigint],
  'latest_delegation_origins' : Array<FrontendHostname>,
  'archive_info' : ArchiveInfo,
  'canister_creation_cycles_cost' : bigint,
  'active_anchor_stats' : [] | [ActiveAnchorStatistics],
}
export type KeyType = { 'platform' : null } |
  { 'seed_phrase' : null } |
  { 'cross_platform' : null } |
  { 'unknown' : null };
export type MetadataMap = Array<
  [
    string,
    { 'map' : MetadataMap } |
      { 'string' : string } |
      { 'bytes' : Uint8Array | number[] },
  ]
>;
export interface OngoingActiveAnchorStats {
  'monthly_active_anchors' : Array<ActiveAnchorCounter>,
  'daily_active_anchors' : ActiveAnchorCounter,
}
export type PublicKey = Uint8Array | number[];
export interface PublicKeyAuthn { 'pubkey' : PublicKey }
export type Purpose = { 'authentication' : null } |
  { 'recovery' : null };
export interface RateLimitConfig {
  'max_tokens' : bigint,
  'time_per_token_ns' : bigint,
}
export type RegisterResponse = { 'bad_challenge' : null } |
  { 'canister_full' : null } |
  { 'registered' : { 'user_number' : UserNumber } };
export type SessionKey = PublicKey;
export interface SignedDelegation {
  'signature' : Uint8Array | number[],
  'delegation' : Delegation,
}
export interface StreamingCallbackHttpResponse {
  'token' : [] | [Token],
  'body' : Uint8Array | number[],
}
export type StreamingStrategy = {
    'Callback' : { 'token' : Token, 'callback' : [Principal, string] }
  };
export type Timestamp = bigint;
export type Token = {};
export type UserKey = PublicKey;
export type UserNumber = bigint;
export type VerifyTentativeDeviceResponse = {
    'device_registration_mode_off' : null
  } |
  { 'verified' : null } |
  { 'wrong_code' : { 'retries_left' : number } } |
  { 'no_device_to_verify' : null };
export interface WebAuthn {
  'pubkey' : PublicKey,
  'credential_id' : CredentialId,
}
export interface WebAuthnCredential {
  'pubkey' : PublicKey,
  'credential_id' : CredentialId,
}
export interface _SERVICE {
  'acknowledge_entries' : ActorMethod<[bigint], undefined>,
  'add' : ActorMethod<[UserNumber, DeviceData], undefined>,
  'add_tentative_device' : ActorMethod<
    [UserNumber, DeviceData],
    AddTentativeDeviceResponse
  >,
  'authn_method_add' : ActorMethod<
    [IdentityNumber, AuthnMethodData],
    [] | [AuthnMethodAddResponse]
  >,
  'create_challenge' : ActorMethod<[], Challenge>,
  'deploy_archive' : ActorMethod<[Uint8Array | number[]], DeployArchiveResult>,
  'enter_device_registration_mode' : ActorMethod<[UserNumber], Timestamp>,
  'exit_device_registration_mode' : ActorMethod<[UserNumber], undefined>,
  'fetch_entries' : ActorMethod<[], Array<BufferedArchiveEntry>>,
  'get_anchor_credentials' : ActorMethod<[UserNumber], AnchorCredentials>,
  'get_anchor_info' : ActorMethod<[UserNumber], IdentityAnchorInfo>,
  'get_delegation' : ActorMethod<
    [UserNumber, FrontendHostname, SessionKey, Timestamp],
    GetDelegationResponse
  >,
  'get_principal' : ActorMethod<[UserNumber, FrontendHostname], Principal>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'http_request_update' : ActorMethod<[HttpRequest], HttpResponse>,
  'identity_info' : ActorMethod<[IdentityNumber], [] | [IdentityInfoResponse]>,
  'init_salt' : ActorMethod<[], undefined>,
  'lookup' : ActorMethod<[UserNumber], Array<DeviceData>>,
  'prepare_delegation' : ActorMethod<
    [UserNumber, FrontendHostname, SessionKey, [] | [bigint]],
    [UserKey, Timestamp]
  >,
  'register' : ActorMethod<
    [DeviceData, ChallengeResult, [] | [Principal]],
    RegisterResponse
  >,
  'remove' : ActorMethod<[UserNumber, DeviceKey], undefined>,
  'replace' : ActorMethod<[UserNumber, DeviceKey, DeviceData], undefined>,
  'stats' : ActorMethod<[], InternetIdentityStats>,
  'update' : ActorMethod<[UserNumber, DeviceKey, DeviceData], undefined>,
  'verify_tentative_device' : ActorMethod<
    [UserNumber, string],
    VerifyTentativeDeviceResponse
  >,
}
