import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type AccountDelegationError = { 'NoSuchDelegation' : null } |
  { 'InternalCanisterError' : string } |
  { 'Unauthorized' : Principal };
export interface AccountInfo {
  'name' : [] | [string],
  'origin' : string,
  'account_number' : [] | [AccountNumber],
  'last_used' : [] | [Timestamp],
}
export type AccountNumber = bigint;
export interface AccountUpdate { 'name' : [] | [string] }
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
export type AnalyticsConfig = {
    'Plausible' : {
      'domain' : [] | [string],
      'track_localhost' : [] | [boolean],
      'hash_mode' : [] | [boolean],
      'api_host' : [] | [string],
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
export type Aud = string;
export type AuthnMethod = { 'PubKey' : PublicKeyAuthn } |
  { 'WebAuthn' : WebAuthn };
export type AuthnMethodAddError = { 'InvalidMetadata' : string };
export interface AuthnMethodConfirmationCode {
  'confirmation_code' : string,
  'expiration' : Timestamp,
}
export type AuthnMethodConfirmationError = {
    'InternalCanisterError' : string
  } |
  { 'RegistrationModeOff' : null } |
  { 'Unauthorized' : Principal } |
  { 'NoAuthnMethodToConfirm' : null } |
  { 'WrongCode' : { 'retries_left' : number } };
export interface AuthnMethodData {
  'security_settings' : AuthnMethodSecuritySettings,
  'metadata' : MetadataMapV2,
  'last_authentication' : [] | [Timestamp],
  'authn_method' : AuthnMethod,
}
export type AuthnMethodMetadataReplaceError = { 'AuthnMethodNotFound' : null } |
  { 'InvalidMetadata' : string };
export type AuthnMethodProtection = { 'Protected' : null } |
  { 'Unprotected' : null };
export type AuthnMethodPurpose = { 'Recovery' : null } |
  { 'Authentication' : null };
export type AuthnMethodRegisterError = { 'RegistrationModeOff' : null } |
  { 'RegistrationAlreadyInProgress' : null } |
  { 'InvalidMetadata' : string };
export interface AuthnMethodRegistrationInfo {
  'expiration' : Timestamp,
  'session' : [] | [Principal],
  'authn_method' : [] | [AuthnMethodData],
}
export type AuthnMethodRegistrationModeEnterError = {
    'InvalidRegistrationId' : string
  } |
  { 'InternalCanisterError' : string } |
  { 'AlreadyInProgress' : null } |
  { 'Unauthorized' : Principal };
export type AuthnMethodRegistrationModeExitError = {
    'InternalCanisterError' : string
  } |
  { 'RegistrationModeOff' : null } |
  { 'Unauthorized' : Principal } |
  { 'InvalidMetadata' : string };
export type AuthnMethodReplaceError = { 'AuthnMethodNotFound' : null } |
  { 'InvalidMetadata' : string };
export interface AuthnMethodSecuritySettings {
  'protection' : AuthnMethodProtection,
  'purpose' : AuthnMethodPurpose,
}
export type AuthnMethodSecuritySettingsReplaceError = {
    'AuthnMethodNotFound' : null
  };
export interface AuthnMethodSessionInfo { 'name' : [] | [string] }
export interface BufferedArchiveEntry {
  'sequence_number' : bigint,
  'entry' : Uint8Array | number[],
  'anchor_number' : UserNumber,
  'timestamp' : Timestamp,
}
export interface CaptchaConfig {
  'max_unsolved_captchas' : bigint,
  'captcha_trigger' : {
      'Dynamic' : {
        'reference_rate_sampling_interval_s' : bigint,
        'threshold_pct' : number,
        'current_rate_sampling_interval_s' : bigint,
      }
    } |
    { 'Static' : { 'CaptchaDisabled' : null } | { 'CaptchaEnabled' : null } },
}
export type CaptchaResult = ChallengeResult;
export interface Challenge {
  'png_base64' : string,
  'challenge_key' : ChallengeKey,
}
export type ChallengeKey = string;
export interface ChallengeResult { 'key' : ChallengeKey, 'chars' : string }
export interface CheckCaptchaArg { 'solution' : string }
export type CheckCaptchaError = { 'NoRegistrationFlow' : null } |
  { 'UnexpectedCall' : { 'next_step' : RegistrationFlowNextStep } } |
  { 'WrongSolution' : { 'new_captcha_png_base64' : string } };
export type CreateAccountError = { 'AccountLimitReached' : null } |
  { 'InternalCanisterError' : string } |
  { 'Unauthorized' : Principal } |
  { 'NameTooLong' : null };
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
export interface DeviceKeyWithAnchor {
  'pubkey' : DeviceKey,
  'anchor_number' : UserNumber,
}
export type DeviceProtection = { 'unprotected' : null } |
  { 'protected' : null };
export interface DeviceRegistrationInfo {
  'tentative_device' : [] | [DeviceData],
  'expiration' : Timestamp,
  'tentative_session' : [] | [Principal],
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
export interface DummyAuthConfig { 'prompt_for_index' : boolean }
export type FrontendHostname = string;
export type GetAccountsError = { 'InternalCanisterError' : string } |
  { 'Unauthorized' : Principal };
export type GetDelegationResponse = { 'no_such_delegation' : null } |
  { 'signed_delegation' : SignedDelegation };
export type GetIdAliasError = { 'InternalCanisterError' : string } |
  { 'Unauthorized' : Principal } |
  { 'NoSuchCredentials' : string };
export interface GetIdAliasRequest {
  'rp_id_alias_jwt' : string,
  'issuer' : FrontendHostname,
  'issuer_id_alias_jwt' : string,
  'relying_party' : FrontendHostname,
  'identity_number' : IdentityNumber,
}
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
export interface IdAliasCredentials {
  'rp_id_alias_credential' : SignedIdAlias,
  'issuer_id_alias_credential' : SignedIdAlias,
}
export interface IdRegFinishArg {
  'name' : [] | [string],
  'authn_method' : AuthnMethodData,
}
export type IdRegFinishError = { 'NoRegistrationFlow' : null } |
  { 'UnexpectedCall' : { 'next_step' : RegistrationFlowNextStep } } |
  { 'InvalidAuthnMethod' : string } |
  { 'IdentityLimitReached' : null } |
  { 'StorageError' : string };
export interface IdRegFinishResult { 'identity_number' : bigint }
export interface IdRegNextStepResult { 'next_step' : RegistrationFlowNextStep }
export type IdRegStartError = { 'InvalidCaller' : null } |
  { 'AlreadyInProgress' : null } |
  { 'RateLimitExceeded' : null };
export interface IdentityAnchorInfo {
  'name' : [] | [string],
  'devices' : Array<DeviceWithUsage>,
  'openid_credentials' : [] | [Array<OpenIdCredential>],
  'device_registration' : [] | [DeviceRegistrationInfo],
}
export interface IdentityAuthnInfo {
  'authn_methods' : Array<AuthnMethod>,
  'recovery_authn_methods' : Array<AuthnMethod>,
}
export interface IdentityInfo {
  'authn_methods' : Array<AuthnMethodData>,
  'metadata' : MetadataMapV2,
  'name' : [] | [string],
  'authn_method_registration' : [] | [AuthnMethodRegistrationInfo],
  'openid_credentials' : [] | [Array<OpenIdCredential>],
}
export type IdentityInfoError = { 'InternalCanisterError' : string } |
  { 'Unauthorized' : Principal };
export type IdentityMetadataReplaceError = {
    'InternalCanisterError' : string
  } |
  { 'Unauthorized' : Principal } |
  {
    'StorageSpaceExceeded' : {
      'space_required' : bigint,
      'space_available' : bigint,
    }
  };
export type IdentityNumber = bigint;
export interface IdentityPropertiesReplace { 'name' : [] | [string] }
export type IdentityPropertiesReplaceError = {
    'InternalCanisterError' : string
  } |
  { 'Unauthorized' : Principal } |
  { 'NameTooLong' : { 'limit' : bigint } } |
  {
    'StorageSpaceExceeded' : {
      'space_required' : bigint,
      'space_available' : bigint,
    }
  };
export interface InternetIdentityInit {
  'fetch_root_key' : [] | [boolean],
  'openid_google' : [] | [[] | [OpenIdConfig]],
  'is_production' : [] | [boolean],
  'enable_dapps_explorer' : [] | [boolean],
  'assigned_user_number_range' : [] | [[bigint, bigint]],
  'new_flow_origins' : [] | [Array<string>],
  'archive_config' : [] | [ArchiveConfig],
  'canister_creation_cycles_cost' : [] | [bigint],
  'analytics_config' : [] | [[] | [AnalyticsConfig]],
  'related_origins' : [] | [Array<string>],
  'feature_flag_continue_from_another_device' : [] | [boolean],
  'captcha_config' : [] | [CaptchaConfig],
  'dummy_auth' : [] | [[] | [DummyAuthConfig]],
  'register_rate_limit' : [] | [RateLimitConfig],
}
export interface InternetIdentityStats {
  'storage_layout_version' : number,
  'users_registered' : bigint,
  'assigned_user_number_range' : [bigint, bigint],
  'archive_info' : ArchiveInfo,
  'canister_creation_cycles_cost' : bigint,
  'event_aggregations' : Array<[string, Array<[string, bigint]>]>,
}
export type Iss = string;
export type JWT = string;
export type KeyType = { 'platform' : null } |
  { 'seed_phrase' : null } |
  { 'cross_platform' : null } |
  { 'unknown' : null } |
  { 'browser_storage_key' : null };
export type LookupByRegistrationIdError = { 'InvalidRegistrationId' : string };
export type MetadataMap = Array<
  [
    string,
    { 'map' : MetadataMap } |
      { 'string' : string } |
      { 'bytes' : Uint8Array | number[] },
  ]
>;
export type MetadataMapV2 = Array<
  [
    string,
    { 'Map' : MetadataMapV2 } |
      { 'String' : string } |
      { 'Bytes' : Uint8Array | number[] },
  ]
>;
export interface OpenIDRegFinishArg { 'jwt' : JWT, 'salt' : Salt }
export interface OpenIdConfig { 'client_id' : string }
export interface OpenIdCredential {
  'aud' : Aud,
  'iss' : Iss,
  'sub' : Sub,
  'metadata' : MetadataMapV2,
  'last_usage_timestamp' : [] | [Timestamp],
}
export type OpenIdCredentialAddError = {
    'OpenIdCredentialAlreadyRegistered' : null
  } |
  { 'InternalCanisterError' : string } |
  { 'JwtExpired' : null } |
  { 'Unauthorized' : Principal } |
  { 'JwtVerificationFailed' : null };
export type OpenIdCredentialKey = [Iss, Sub];
export type OpenIdCredentialRemoveError = { 'InternalCanisterError' : string } |
  { 'OpenIdCredentialNotFound' : null } |
  { 'Unauthorized' : Principal };
export type OpenIdDelegationError = { 'NoSuchDelegation' : null } |
  { 'NoSuchAnchor' : null } |
  { 'JwtExpired' : null } |
  { 'JwtVerificationFailed' : null };
export interface OpenIdPrepareDelegationResponse {
  'user_key' : UserKey,
  'expiration' : Timestamp,
  'anchor_number' : UserNumber,
}
export interface PrepareAccountDelegation {
  'user_key' : UserKey,
  'expiration' : Timestamp,
}
export type PrepareIdAliasError = { 'InternalCanisterError' : string } |
  { 'Unauthorized' : Principal };
export interface PrepareIdAliasRequest {
  'issuer' : FrontendHostname,
  'relying_party' : FrontendHostname,
  'identity_number' : IdentityNumber,
}
export interface PreparedIdAlias {
  'rp_id_alias_jwt' : string,
  'issuer_id_alias_jwt' : string,
  'canister_sig_pk_der' : PublicKey,
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
export type RegistrationFlowNextStep = {
    'CheckCaptcha' : { 'captcha_png_base64' : string }
  } |
  { 'Finish' : null };
export type RegistrationId = string;
export type Salt = Uint8Array | number[];
export type SessionKey = PublicKey;
export interface SignedDelegation {
  'signature' : Uint8Array | number[],
  'delegation' : Delegation,
}
export interface SignedIdAlias {
  'credential_jws' : string,
  'id_alias' : Principal,
  'id_dapp' : Principal,
}
export interface StreamingCallbackHttpResponse {
  'token' : [] | [Token],
  'body' : Uint8Array | number[],
}
export type StreamingStrategy = {
    'Callback' : { 'token' : Token, 'callback' : [Principal, string] }
  };
export type Sub = string;
export type Timestamp = bigint;
export type Token = {};
export type UpdateAccountError = { 'AccountLimitReached' : null } |
  { 'InternalCanisterError' : string } |
  { 'Unauthorized' : Principal } |
  { 'NameTooLong' : null };
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
    { 'Ok' : null } |
      { 'Err' : AuthnMethodAddError }
  >,
  'authn_method_confirm' : ActorMethod<
    [IdentityNumber, string],
    { 'Ok' : null } |
      { 'Err' : AuthnMethodConfirmationError }
  >,
  'authn_method_metadata_replace' : ActorMethod<
    [IdentityNumber, PublicKey, MetadataMapV2],
    { 'Ok' : null } |
      { 'Err' : AuthnMethodMetadataReplaceError }
  >,
  'authn_method_register' : ActorMethod<
    [IdentityNumber, AuthnMethodData],
    { 'Ok' : AuthnMethodConfirmationCode } |
      { 'Err' : AuthnMethodRegisterError }
  >,
  'authn_method_registration_mode_enter' : ActorMethod<
    [IdentityNumber, [] | [RegistrationId]],
    { 'Ok' : { 'expiration' : Timestamp } } |
      { 'Err' : AuthnMethodRegistrationModeEnterError }
  >,
  'authn_method_registration_mode_exit' : ActorMethod<
    [IdentityNumber, [] | [AuthnMethodData]],
    { 'Ok' : null } |
      { 'Err' : AuthnMethodRegistrationModeExitError }
  >,
  'authn_method_remove' : ActorMethod<
    [IdentityNumber, PublicKey],
    { 'Ok' : null } |
      { 'Err' : null }
  >,
  'authn_method_replace' : ActorMethod<
    [IdentityNumber, PublicKey, AuthnMethodData],
    { 'Ok' : null } |
      { 'Err' : AuthnMethodReplaceError }
  >,
  'authn_method_security_settings_replace' : ActorMethod<
    [IdentityNumber, PublicKey, AuthnMethodSecuritySettings],
    { 'Ok' : null } |
      { 'Err' : AuthnMethodSecuritySettingsReplaceError }
  >,
  'authn_method_session_info' : ActorMethod<
    [IdentityNumber],
    [] | [AuthnMethodSessionInfo]
  >,
  'authn_method_session_register' : ActorMethod<
    [IdentityNumber],
    { 'Ok' : AuthnMethodConfirmationCode } |
      { 'Err' : AuthnMethodRegisterError }
  >,
  'check_captcha' : ActorMethod<
    [CheckCaptchaArg],
    { 'Ok' : IdRegNextStepResult } |
      { 'Err' : CheckCaptchaError }
  >,
  'config' : ActorMethod<[], InternetIdentityInit>,
  'create_account' : ActorMethod<
    [UserNumber, FrontendHostname, string],
    { 'Ok' : AccountInfo } |
      { 'Err' : CreateAccountError }
  >,
  'create_challenge' : ActorMethod<[], Challenge>,
  'deploy_archive' : ActorMethod<[Uint8Array | number[]], DeployArchiveResult>,
  'enter_device_registration_mode' : ActorMethod<[UserNumber], Timestamp>,
  'exit_device_registration_mode' : ActorMethod<[UserNumber], undefined>,
  'fetch_entries' : ActorMethod<[], Array<BufferedArchiveEntry>>,
  'get_account_delegation' : ActorMethod<
    [UserNumber, FrontendHostname, [] | [AccountNumber], SessionKey, Timestamp],
    { 'Ok' : SignedDelegation } |
      { 'Err' : AccountDelegationError }
  >,
  'get_accounts' : ActorMethod<
    [UserNumber, FrontendHostname],
    { 'Ok' : Array<AccountInfo> } |
      { 'Err' : GetAccountsError }
  >,
  'get_anchor_credentials' : ActorMethod<[UserNumber], AnchorCredentials>,
  'get_anchor_info' : ActorMethod<[UserNumber], IdentityAnchorInfo>,
  'get_delegation' : ActorMethod<
    [UserNumber, FrontendHostname, SessionKey, Timestamp],
    GetDelegationResponse
  >,
  'get_id_alias' : ActorMethod<
    [GetIdAliasRequest],
    { 'Ok' : IdAliasCredentials } |
      { 'Err' : GetIdAliasError }
  >,
  'get_principal' : ActorMethod<[UserNumber, FrontendHostname], Principal>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'http_request_update' : ActorMethod<[HttpRequest], HttpResponse>,
  'identity_authn_info' : ActorMethod<
    [IdentityNumber],
    { 'Ok' : IdentityAuthnInfo } |
      { 'Err' : null }
  >,
  'identity_info' : ActorMethod<
    [IdentityNumber],
    { 'Ok' : IdentityInfo } |
      { 'Err' : IdentityInfoError }
  >,
  'identity_metadata_replace' : ActorMethod<
    [IdentityNumber, MetadataMapV2],
    { 'Ok' : null } |
      { 'Err' : IdentityMetadataReplaceError }
  >,
  'identity_properties_replace' : ActorMethod<
    [IdentityNumber, IdentityPropertiesReplace],
    { 'Ok' : null } |
      { 'Err' : IdentityPropertiesReplaceError }
  >,
  'identity_registration_finish' : ActorMethod<
    [IdRegFinishArg],
    { 'Ok' : IdRegFinishResult } |
      { 'Err' : IdRegFinishError }
  >,
  'identity_registration_start' : ActorMethod<
    [],
    { 'Ok' : IdRegNextStepResult } |
      { 'Err' : IdRegStartError }
  >,
  'init_salt' : ActorMethod<[], undefined>,
  'lookup' : ActorMethod<[UserNumber], Array<DeviceData>>,
  'lookup_by_registration_mode_id' : ActorMethod<
    [RegistrationId],
    [] | [IdentityNumber]
  >,
  'lookup_device_key' : ActorMethod<
    [Uint8Array | number[]],
    [] | [DeviceKeyWithAnchor]
  >,
  'openid_credential_add' : ActorMethod<
    [IdentityNumber, JWT, Salt],
    { 'Ok' : null } |
      { 'Err' : OpenIdCredentialAddError }
  >,
  'openid_credential_remove' : ActorMethod<
    [IdentityNumber, OpenIdCredentialKey],
    { 'Ok' : null } |
      { 'Err' : OpenIdCredentialRemoveError }
  >,
  'openid_get_delegation' : ActorMethod<
    [JWT, Salt, SessionKey, Timestamp],
    { 'Ok' : SignedDelegation } |
      { 'Err' : OpenIdDelegationError }
  >,
  'openid_identity_registration_finish' : ActorMethod<
    [OpenIDRegFinishArg],
    { 'Ok' : IdRegFinishResult } |
      { 'Err' : IdRegFinishError }
  >,
  'openid_prepare_delegation' : ActorMethod<
    [JWT, Salt, SessionKey],
    { 'Ok' : OpenIdPrepareDelegationResponse } |
      { 'Err' : OpenIdDelegationError }
  >,
  'prepare_account_delegation' : ActorMethod<
    [
      UserNumber,
      FrontendHostname,
      [] | [AccountNumber],
      SessionKey,
      [] | [bigint],
    ],
    { 'Ok' : PrepareAccountDelegation } |
      { 'Err' : AccountDelegationError }
  >,
  'prepare_delegation' : ActorMethod<
    [UserNumber, FrontendHostname, SessionKey, [] | [bigint]],
    [UserKey, Timestamp]
  >,
  'prepare_id_alias' : ActorMethod<
    [PrepareIdAliasRequest],
    { 'Ok' : PreparedIdAlias } |
      { 'Err' : PrepareIdAliasError }
  >,
  'register' : ActorMethod<
    [DeviceData, ChallengeResult, [] | [Principal]],
    RegisterResponse
  >,
  'remove' : ActorMethod<[UserNumber, DeviceKey], undefined>,
  'replace' : ActorMethod<[UserNumber, DeviceKey, DeviceData], undefined>,
  'stats' : ActorMethod<[], InternetIdentityStats>,
  'update' : ActorMethod<[UserNumber, DeviceKey, DeviceData], undefined>,
  'update_account' : ActorMethod<
    [UserNumber, FrontendHostname, [] | [AccountNumber], AccountUpdate],
    { 'Ok' : AccountInfo } |
      { 'Err' : UpdateAccountError }
  >,
  'verify_tentative_device' : ActorMethod<
    [UserNumber, string],
    VerifyTentativeDeviceResponse
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
