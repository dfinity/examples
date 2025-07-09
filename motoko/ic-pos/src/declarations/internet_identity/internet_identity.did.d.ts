import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

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
export type AuthnMethod = { 'PubKey' : PublicKeyAuthn } |
  { 'WebAuthn' : WebAuthn };
export type AuthnMethodAddError = { 'InvalidMetadata' : string };
export interface AuthnMethodConfirmationCode {
  'confirmation_code' : string,
  'expiration' : Timestamp,
}
export type AuthnMethodConfirmationError = { 'RegistrationModeOff' : null } |
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
  'authn_method' : [] | [AuthnMethodData],
}
export type AuthnMethodReplaceError = { 'AuthnMethodNotFound' : null } |
  { 'InvalidMetadata' : string };
export interface AuthnMethodSecuritySettings {
  'protection' : AuthnMethodProtection,
  'purpose' : AuthnMethodPurpose,
}
export type AuthnMethodSecuritySettingsReplaceError = {
    'AuthnMethodNotFound' : null
  };
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
export type FrontendHostname = string;
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
export interface IdRegFinishArg { 'authn_method' : AuthnMethodData }
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
  'devices' : Array<DeviceWithUsage>,
  'device_registration' : [] | [DeviceRegistrationInfo],
}
export interface IdentityAuthnInfo {
  'authn_methods' : Array<AuthnMethod>,
  'recovery_authn_methods' : Array<AuthnMethod>,
}
export interface IdentityInfo {
  'authn_methods' : Array<AuthnMethodData>,
  'metadata' : MetadataMapV2,
  'authn_method_registration' : [] | [AuthnMethodRegistrationInfo],
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
export interface InternetIdentityInit {
  'assigned_user_number_range' : [] | [[bigint, bigint]],
  'archive_config' : [] | [ArchiveConfig],
  'canister_creation_cycles_cost' : [] | [bigint],
  'captcha_config' : [] | [CaptchaConfig],
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
export type KeyType = { 'platform' : null } |
  { 'seed_phrase' : null } |
  { 'cross_platform' : null } |
  { 'unknown' : null } |
  { 'browser_storage_key' : null };
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
    [IdentityNumber],
    { 'Ok' : { 'expiration' : Timestamp } } |
      { 'Err' : null }
  >,
  'authn_method_registration_mode_exit' : ActorMethod<
    [IdentityNumber],
    { 'Ok' : null } |
      { 'Err' : null }
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
  'check_captcha' : ActorMethod<
    [CheckCaptchaArg],
    { 'Ok' : IdRegNextStepResult } |
      { 'Err' : CheckCaptchaError }
  >,
  'config' : ActorMethod<[], InternetIdentityInit>,
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
  'verify_tentative_device' : ActorMethod<
    [UserNumber, string],
    VerifyTentativeDeviceResponse
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
