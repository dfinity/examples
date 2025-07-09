export const idlFactory = ({ IDL }) => {
  const MetadataMap = IDL.Rec();
  const MetadataMapV2 = IDL.Rec();
  const ArchiveConfig = IDL.Record({
    'polling_interval_ns' : IDL.Nat64,
    'entries_buffer_limit' : IDL.Nat64,
    'module_hash' : IDL.Vec(IDL.Nat8),
    'entries_fetch_limit' : IDL.Nat16,
  });
  const CaptchaConfig = IDL.Record({
    'max_unsolved_captchas' : IDL.Nat64,
    'captcha_trigger' : IDL.Variant({
      'Dynamic' : IDL.Record({
        'reference_rate_sampling_interval_s' : IDL.Nat64,
        'threshold_pct' : IDL.Nat16,
        'current_rate_sampling_interval_s' : IDL.Nat64,
      }),
      'Static' : IDL.Variant({
        'CaptchaDisabled' : IDL.Null,
        'CaptchaEnabled' : IDL.Null,
      }),
    }),
  });
  const RateLimitConfig = IDL.Record({
    'max_tokens' : IDL.Nat64,
    'time_per_token_ns' : IDL.Nat64,
  });
  const InternetIdentityInit = IDL.Record({
    'assigned_user_number_range' : IDL.Opt(IDL.Tuple(IDL.Nat64, IDL.Nat64)),
    'archive_config' : IDL.Opt(ArchiveConfig),
    'canister_creation_cycles_cost' : IDL.Opt(IDL.Nat64),
    'captcha_config' : IDL.Opt(CaptchaConfig),
    'register_rate_limit' : IDL.Opt(RateLimitConfig),
  });
  const UserNumber = IDL.Nat64;
  MetadataMap.fill(
    IDL.Vec(
      IDL.Tuple(
        IDL.Text,
        IDL.Variant({
          'map' : MetadataMap,
          'string' : IDL.Text,
          'bytes' : IDL.Vec(IDL.Nat8),
        }),
      )
    )
  );
  const DeviceProtection = IDL.Variant({
    'unprotected' : IDL.Null,
    'protected' : IDL.Null,
  });
  const PublicKey = IDL.Vec(IDL.Nat8);
  const DeviceKey = PublicKey;
  const KeyType = IDL.Variant({
    'platform' : IDL.Null,
    'seed_phrase' : IDL.Null,
    'cross_platform' : IDL.Null,
    'unknown' : IDL.Null,
    'browser_storage_key' : IDL.Null,
  });
  const Purpose = IDL.Variant({
    'authentication' : IDL.Null,
    'recovery' : IDL.Null,
  });
  const CredentialId = IDL.Vec(IDL.Nat8);
  const DeviceData = IDL.Record({
    'alias' : IDL.Text,
    'metadata' : IDL.Opt(MetadataMap),
    'origin' : IDL.Opt(IDL.Text),
    'protection' : DeviceProtection,
    'pubkey' : DeviceKey,
    'key_type' : KeyType,
    'purpose' : Purpose,
    'credential_id' : IDL.Opt(CredentialId),
  });
  const Timestamp = IDL.Nat64;
  const AddTentativeDeviceResponse = IDL.Variant({
    'device_registration_mode_off' : IDL.Null,
    'another_device_tentatively_added' : IDL.Null,
    'added_tentatively' : IDL.Record({
      'verification_code' : IDL.Text,
      'device_registration_timeout' : Timestamp,
    }),
  });
  const IdentityNumber = IDL.Nat64;
  const AuthnMethodProtection = IDL.Variant({
    'Protected' : IDL.Null,
    'Unprotected' : IDL.Null,
  });
  const AuthnMethodPurpose = IDL.Variant({
    'Recovery' : IDL.Null,
    'Authentication' : IDL.Null,
  });
  const AuthnMethodSecuritySettings = IDL.Record({
    'protection' : AuthnMethodProtection,
    'purpose' : AuthnMethodPurpose,
  });
  MetadataMapV2.fill(
    IDL.Vec(
      IDL.Tuple(
        IDL.Text,
        IDL.Variant({
          'Map' : MetadataMapV2,
          'String' : IDL.Text,
          'Bytes' : IDL.Vec(IDL.Nat8),
        }),
      )
    )
  );
  const PublicKeyAuthn = IDL.Record({ 'pubkey' : PublicKey });
  const WebAuthn = IDL.Record({
    'pubkey' : PublicKey,
    'credential_id' : CredentialId,
  });
  const AuthnMethod = IDL.Variant({
    'PubKey' : PublicKeyAuthn,
    'WebAuthn' : WebAuthn,
  });
  const AuthnMethodData = IDL.Record({
    'security_settings' : AuthnMethodSecuritySettings,
    'metadata' : MetadataMapV2,
    'last_authentication' : IDL.Opt(Timestamp),
    'authn_method' : AuthnMethod,
  });
  const AuthnMethodAddError = IDL.Variant({ 'InvalidMetadata' : IDL.Text });
  const AuthnMethodConfirmationError = IDL.Variant({
    'RegistrationModeOff' : IDL.Null,
    'NoAuthnMethodToConfirm' : IDL.Null,
    'WrongCode' : IDL.Record({ 'retries_left' : IDL.Nat8 }),
  });
  const AuthnMethodMetadataReplaceError = IDL.Variant({
    'AuthnMethodNotFound' : IDL.Null,
    'InvalidMetadata' : IDL.Text,
  });
  const AuthnMethodConfirmationCode = IDL.Record({
    'confirmation_code' : IDL.Text,
    'expiration' : Timestamp,
  });
  const AuthnMethodRegisterError = IDL.Variant({
    'RegistrationModeOff' : IDL.Null,
    'RegistrationAlreadyInProgress' : IDL.Null,
    'InvalidMetadata' : IDL.Text,
  });
  const AuthnMethodReplaceError = IDL.Variant({
    'AuthnMethodNotFound' : IDL.Null,
    'InvalidMetadata' : IDL.Text,
  });
  const AuthnMethodSecuritySettingsReplaceError = IDL.Variant({
    'AuthnMethodNotFound' : IDL.Null,
  });
  const CheckCaptchaArg = IDL.Record({ 'solution' : IDL.Text });
  const RegistrationFlowNextStep = IDL.Variant({
    'CheckCaptcha' : IDL.Record({ 'captcha_png_base64' : IDL.Text }),
    'Finish' : IDL.Null,
  });
  const IdRegNextStepResult = IDL.Record({
    'next_step' : RegistrationFlowNextStep,
  });
  const CheckCaptchaError = IDL.Variant({
    'NoRegistrationFlow' : IDL.Null,
    'UnexpectedCall' : IDL.Record({ 'next_step' : RegistrationFlowNextStep }),
    'WrongSolution' : IDL.Record({ 'new_captcha_png_base64' : IDL.Text }),
  });
  const ChallengeKey = IDL.Text;
  const Challenge = IDL.Record({
    'png_base64' : IDL.Text,
    'challenge_key' : ChallengeKey,
  });
  const DeployArchiveResult = IDL.Variant({
    'creation_in_progress' : IDL.Null,
    'success' : IDL.Principal,
    'failed' : IDL.Text,
  });
  const BufferedArchiveEntry = IDL.Record({
    'sequence_number' : IDL.Nat64,
    'entry' : IDL.Vec(IDL.Nat8),
    'anchor_number' : UserNumber,
    'timestamp' : Timestamp,
  });
  const WebAuthnCredential = IDL.Record({
    'pubkey' : PublicKey,
    'credential_id' : CredentialId,
  });
  const AnchorCredentials = IDL.Record({
    'recovery_phrases' : IDL.Vec(PublicKey),
    'credentials' : IDL.Vec(WebAuthnCredential),
    'recovery_credentials' : IDL.Vec(WebAuthnCredential),
  });
  const DeviceWithUsage = IDL.Record({
    'alias' : IDL.Text,
    'last_usage' : IDL.Opt(Timestamp),
    'metadata' : IDL.Opt(MetadataMap),
    'origin' : IDL.Opt(IDL.Text),
    'protection' : DeviceProtection,
    'pubkey' : DeviceKey,
    'key_type' : KeyType,
    'purpose' : Purpose,
    'credential_id' : IDL.Opt(CredentialId),
  });
  const DeviceRegistrationInfo = IDL.Record({
    'tentative_device' : IDL.Opt(DeviceData),
    'expiration' : Timestamp,
  });
  const IdentityAnchorInfo = IDL.Record({
    'devices' : IDL.Vec(DeviceWithUsage),
    'device_registration' : IDL.Opt(DeviceRegistrationInfo),
  });
  const FrontendHostname = IDL.Text;
  const SessionKey = PublicKey;
  const Delegation = IDL.Record({
    'pubkey' : PublicKey,
    'targets' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'expiration' : Timestamp,
  });
  const SignedDelegation = IDL.Record({
    'signature' : IDL.Vec(IDL.Nat8),
    'delegation' : Delegation,
  });
  const GetDelegationResponse = IDL.Variant({
    'no_such_delegation' : IDL.Null,
    'signed_delegation' : SignedDelegation,
  });
  const GetIdAliasRequest = IDL.Record({
    'rp_id_alias_jwt' : IDL.Text,
    'issuer' : FrontendHostname,
    'issuer_id_alias_jwt' : IDL.Text,
    'relying_party' : FrontendHostname,
    'identity_number' : IdentityNumber,
  });
  const SignedIdAlias = IDL.Record({
    'credential_jws' : IDL.Text,
    'id_alias' : IDL.Principal,
    'id_dapp' : IDL.Principal,
  });
  const IdAliasCredentials = IDL.Record({
    'rp_id_alias_credential' : SignedIdAlias,
    'issuer_id_alias_credential' : SignedIdAlias,
  });
  const GetIdAliasError = IDL.Variant({
    'InternalCanisterError' : IDL.Text,
    'Unauthorized' : IDL.Principal,
    'NoSuchCredentials' : IDL.Text,
  });
  const HeaderField = IDL.Tuple(IDL.Text, IDL.Text);
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
    'certificate_version' : IDL.Opt(IDL.Nat16),
  });
  const Token = IDL.Record({});
  const StreamingCallbackHttpResponse = IDL.Record({
    'token' : IDL.Opt(Token),
    'body' : IDL.Vec(IDL.Nat8),
  });
  const StreamingStrategy = IDL.Variant({
    'Callback' : IDL.Record({
      'token' : Token,
      'callback' : IDL.Func(
          [Token],
          [StreamingCallbackHttpResponse],
          ['query'],
        ),
    }),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
    'upgrade' : IDL.Opt(IDL.Bool),
    'streaming_strategy' : IDL.Opt(StreamingStrategy),
    'status_code' : IDL.Nat16,
  });
  const IdentityAuthnInfo = IDL.Record({
    'authn_methods' : IDL.Vec(AuthnMethod),
    'recovery_authn_methods' : IDL.Vec(AuthnMethod),
  });
  const AuthnMethodRegistrationInfo = IDL.Record({
    'expiration' : Timestamp,
    'authn_method' : IDL.Opt(AuthnMethodData),
  });
  const IdentityInfo = IDL.Record({
    'authn_methods' : IDL.Vec(AuthnMethodData),
    'metadata' : MetadataMapV2,
    'authn_method_registration' : IDL.Opt(AuthnMethodRegistrationInfo),
  });
  const IdentityInfoError = IDL.Variant({
    'InternalCanisterError' : IDL.Text,
    'Unauthorized' : IDL.Principal,
  });
  const IdentityMetadataReplaceError = IDL.Variant({
    'InternalCanisterError' : IDL.Text,
    'Unauthorized' : IDL.Principal,
    'StorageSpaceExceeded' : IDL.Record({
      'space_required' : IDL.Nat64,
      'space_available' : IDL.Nat64,
    }),
  });
  const IdRegFinishArg = IDL.Record({ 'authn_method' : AuthnMethodData });
  const IdRegFinishResult = IDL.Record({ 'identity_number' : IDL.Nat64 });
  const IdRegFinishError = IDL.Variant({
    'NoRegistrationFlow' : IDL.Null,
    'UnexpectedCall' : IDL.Record({ 'next_step' : RegistrationFlowNextStep }),
    'InvalidAuthnMethod' : IDL.Text,
    'IdentityLimitReached' : IDL.Null,
    'StorageError' : IDL.Text,
  });
  const IdRegStartError = IDL.Variant({
    'InvalidCaller' : IDL.Null,
    'AlreadyInProgress' : IDL.Null,
    'RateLimitExceeded' : IDL.Null,
  });
  const UserKey = PublicKey;
  const PrepareIdAliasRequest = IDL.Record({
    'issuer' : FrontendHostname,
    'relying_party' : FrontendHostname,
    'identity_number' : IdentityNumber,
  });
  const PreparedIdAlias = IDL.Record({
    'rp_id_alias_jwt' : IDL.Text,
    'issuer_id_alias_jwt' : IDL.Text,
    'canister_sig_pk_der' : PublicKey,
  });
  const PrepareIdAliasError = IDL.Variant({
    'InternalCanisterError' : IDL.Text,
    'Unauthorized' : IDL.Principal,
  });
  const ChallengeResult = IDL.Record({
    'key' : ChallengeKey,
    'chars' : IDL.Text,
  });
  const RegisterResponse = IDL.Variant({
    'bad_challenge' : IDL.Null,
    'canister_full' : IDL.Null,
    'registered' : IDL.Record({ 'user_number' : UserNumber }),
  });
  const ArchiveInfo = IDL.Record({
    'archive_config' : IDL.Opt(ArchiveConfig),
    'archive_canister' : IDL.Opt(IDL.Principal),
  });
  const InternetIdentityStats = IDL.Record({
    'storage_layout_version' : IDL.Nat8,
    'users_registered' : IDL.Nat64,
    'assigned_user_number_range' : IDL.Tuple(IDL.Nat64, IDL.Nat64),
    'archive_info' : ArchiveInfo,
    'canister_creation_cycles_cost' : IDL.Nat64,
    'event_aggregations' : IDL.Vec(
      IDL.Tuple(IDL.Text, IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat64)))
    ),
  });
  const VerifyTentativeDeviceResponse = IDL.Variant({
    'device_registration_mode_off' : IDL.Null,
    'verified' : IDL.Null,
    'wrong_code' : IDL.Record({ 'retries_left' : IDL.Nat8 }),
    'no_device_to_verify' : IDL.Null,
  });
  return IDL.Service({
    'acknowledge_entries' : IDL.Func([IDL.Nat64], [], []),
    'add' : IDL.Func([UserNumber, DeviceData], [], []),
    'add_tentative_device' : IDL.Func(
        [UserNumber, DeviceData],
        [AddTentativeDeviceResponse],
        [],
      ),
    'authn_method_add' : IDL.Func(
        [IdentityNumber, AuthnMethodData],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : AuthnMethodAddError })],
        [],
      ),
    'authn_method_confirm' : IDL.Func(
        [IdentityNumber, IDL.Text],
        [
          IDL.Variant({
            'Ok' : IDL.Null,
            'Err' : AuthnMethodConfirmationError,
          }),
        ],
        [],
      ),
    'authn_method_metadata_replace' : IDL.Func(
        [IdentityNumber, PublicKey, MetadataMapV2],
        [
          IDL.Variant({
            'Ok' : IDL.Null,
            'Err' : AuthnMethodMetadataReplaceError,
          }),
        ],
        [],
      ),
    'authn_method_register' : IDL.Func(
        [IdentityNumber, AuthnMethodData],
        [
          IDL.Variant({
            'Ok' : AuthnMethodConfirmationCode,
            'Err' : AuthnMethodRegisterError,
          }),
        ],
        [],
      ),
    'authn_method_registration_mode_enter' : IDL.Func(
        [IdentityNumber],
        [
          IDL.Variant({
            'Ok' : IDL.Record({ 'expiration' : Timestamp }),
            'Err' : IDL.Null,
          }),
        ],
        [],
      ),
    'authn_method_registration_mode_exit' : IDL.Func(
        [IdentityNumber],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Null })],
        [],
      ),
    'authn_method_remove' : IDL.Func(
        [IdentityNumber, PublicKey],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Null })],
        [],
      ),
    'authn_method_replace' : IDL.Func(
        [IdentityNumber, PublicKey, AuthnMethodData],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : AuthnMethodReplaceError })],
        [],
      ),
    'authn_method_security_settings_replace' : IDL.Func(
        [IdentityNumber, PublicKey, AuthnMethodSecuritySettings],
        [
          IDL.Variant({
            'Ok' : IDL.Null,
            'Err' : AuthnMethodSecuritySettingsReplaceError,
          }),
        ],
        [],
      ),
    'check_captcha' : IDL.Func(
        [CheckCaptchaArg],
        [
          IDL.Variant({
            'Ok' : IdRegNextStepResult,
            'Err' : CheckCaptchaError,
          }),
        ],
        [],
      ),
    'config' : IDL.Func([], [InternetIdentityInit], ['query']),
    'create_challenge' : IDL.Func([], [Challenge], []),
    'deploy_archive' : IDL.Func([IDL.Vec(IDL.Nat8)], [DeployArchiveResult], []),
    'enter_device_registration_mode' : IDL.Func([UserNumber], [Timestamp], []),
    'exit_device_registration_mode' : IDL.Func([UserNumber], [], []),
    'fetch_entries' : IDL.Func([], [IDL.Vec(BufferedArchiveEntry)], []),
    'get_anchor_credentials' : IDL.Func(
        [UserNumber],
        [AnchorCredentials],
        ['query'],
      ),
    'get_anchor_info' : IDL.Func([UserNumber], [IdentityAnchorInfo], []),
    'get_delegation' : IDL.Func(
        [UserNumber, FrontendHostname, SessionKey, Timestamp],
        [GetDelegationResponse],
        ['query'],
      ),
    'get_id_alias' : IDL.Func(
        [GetIdAliasRequest],
        [IDL.Variant({ 'Ok' : IdAliasCredentials, 'Err' : GetIdAliasError })],
        ['query'],
      ),
    'get_principal' : IDL.Func(
        [UserNumber, FrontendHostname],
        [IDL.Principal],
        ['query'],
      ),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'http_request_update' : IDL.Func([HttpRequest], [HttpResponse], []),
    'identity_authn_info' : IDL.Func(
        [IdentityNumber],
        [IDL.Variant({ 'Ok' : IdentityAuthnInfo, 'Err' : IDL.Null })],
        ['query'],
      ),
    'identity_info' : IDL.Func(
        [IdentityNumber],
        [IDL.Variant({ 'Ok' : IdentityInfo, 'Err' : IdentityInfoError })],
        [],
      ),
    'identity_metadata_replace' : IDL.Func(
        [IdentityNumber, MetadataMapV2],
        [
          IDL.Variant({
            'Ok' : IDL.Null,
            'Err' : IdentityMetadataReplaceError,
          }),
        ],
        [],
      ),
    'identity_registration_finish' : IDL.Func(
        [IdRegFinishArg],
        [IDL.Variant({ 'Ok' : IdRegFinishResult, 'Err' : IdRegFinishError })],
        [],
      ),
    'identity_registration_start' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IdRegNextStepResult, 'Err' : IdRegStartError })],
        [],
      ),
    'init_salt' : IDL.Func([], [], []),
    'lookup' : IDL.Func([UserNumber], [IDL.Vec(DeviceData)], ['query']),
    'prepare_delegation' : IDL.Func(
        [UserNumber, FrontendHostname, SessionKey, IDL.Opt(IDL.Nat64)],
        [UserKey, Timestamp],
        [],
      ),
    'prepare_id_alias' : IDL.Func(
        [PrepareIdAliasRequest],
        [IDL.Variant({ 'Ok' : PreparedIdAlias, 'Err' : PrepareIdAliasError })],
        [],
      ),
    'register' : IDL.Func(
        [DeviceData, ChallengeResult, IDL.Opt(IDL.Principal)],
        [RegisterResponse],
        [],
      ),
    'remove' : IDL.Func([UserNumber, DeviceKey], [], []),
    'replace' : IDL.Func([UserNumber, DeviceKey, DeviceData], [], []),
    'stats' : IDL.Func([], [InternetIdentityStats], ['query']),
    'update' : IDL.Func([UserNumber, DeviceKey, DeviceData], [], []),
    'verify_tentative_device' : IDL.Func(
        [UserNumber, IDL.Text],
        [VerifyTentativeDeviceResponse],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const ArchiveConfig = IDL.Record({
    'polling_interval_ns' : IDL.Nat64,
    'entries_buffer_limit' : IDL.Nat64,
    'module_hash' : IDL.Vec(IDL.Nat8),
    'entries_fetch_limit' : IDL.Nat16,
  });
  const CaptchaConfig = IDL.Record({
    'max_unsolved_captchas' : IDL.Nat64,
    'captcha_trigger' : IDL.Variant({
      'Dynamic' : IDL.Record({
        'reference_rate_sampling_interval_s' : IDL.Nat64,
        'threshold_pct' : IDL.Nat16,
        'current_rate_sampling_interval_s' : IDL.Nat64,
      }),
      'Static' : IDL.Variant({
        'CaptchaDisabled' : IDL.Null,
        'CaptchaEnabled' : IDL.Null,
      }),
    }),
  });
  const RateLimitConfig = IDL.Record({
    'max_tokens' : IDL.Nat64,
    'time_per_token_ns' : IDL.Nat64,
  });
  const InternetIdentityInit = IDL.Record({
    'assigned_user_number_range' : IDL.Opt(IDL.Tuple(IDL.Nat64, IDL.Nat64)),
    'archive_config' : IDL.Opt(ArchiveConfig),
    'canister_creation_cycles_cost' : IDL.Opt(IDL.Nat64),
    'captcha_config' : IDL.Opt(CaptchaConfig),
    'register_rate_limit' : IDL.Opt(RateLimitConfig),
  });
  return [IDL.Opt(InternetIdentityInit)];
};
