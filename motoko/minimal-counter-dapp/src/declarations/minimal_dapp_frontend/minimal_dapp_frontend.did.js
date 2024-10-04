export const idlFactory = ({ IDL }) => {
  const SetPermissions = IDL.Record({
    'prepare' : IDL.Vec(IDL.Principal),
    'commit' : IDL.Vec(IDL.Principal),
    'manage_permissions' : IDL.Vec(IDL.Principal),
  });
  const UpgradeArgs = IDL.Record({
    'set_permissions' : IDL.Opt(SetPermissions),
  });
  const InitArgs = IDL.Record({});
  const AssetCanisterArgs = IDL.Variant({
    'Upgrade' : UpgradeArgs,
    'Init' : InitArgs,
  });
  const ClearArguments = IDL.Record({});
  const BatchId = IDL.Nat;
  const Key = IDL.Text;
  const HeaderField = IDL.Tuple(IDL.Text, IDL.Text);
  const SetAssetPropertiesArguments = IDL.Record({
    'key' : Key,
    'headers' : IDL.Opt(IDL.Opt(IDL.Vec(HeaderField))),
    'is_aliased' : IDL.Opt(IDL.Opt(IDL.Bool)),
    'allow_raw_access' : IDL.Opt(IDL.Opt(IDL.Bool)),
    'max_age' : IDL.Opt(IDL.Opt(IDL.Nat64)),
  });
  const CreateAssetArguments = IDL.Record({
    'key' : Key,
    'content_type' : IDL.Text,
    'headers' : IDL.Opt(IDL.Vec(HeaderField)),
    'allow_raw_access' : IDL.Opt(IDL.Bool),
    'max_age' : IDL.Opt(IDL.Nat64),
    'enable_aliasing' : IDL.Opt(IDL.Bool),
  });
  const UnsetAssetContentArguments = IDL.Record({
    'key' : Key,
    'content_encoding' : IDL.Text,
  });
  const DeleteAssetArguments = IDL.Record({ 'key' : Key });
  const ChunkId = IDL.Nat;
  const SetAssetContentArguments = IDL.Record({
    'key' : Key,
    'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'chunk_ids' : IDL.Vec(ChunkId),
    'content_encoding' : IDL.Text,
  });
  const BatchOperationKind = IDL.Variant({
    'SetAssetProperties' : SetAssetPropertiesArguments,
    'CreateAsset' : CreateAssetArguments,
    'UnsetAssetContent' : UnsetAssetContentArguments,
    'DeleteAsset' : DeleteAssetArguments,
    'SetAssetContent' : SetAssetContentArguments,
    'Clear' : ClearArguments,
  });
  const CommitBatchArguments = IDL.Record({
    'batch_id' : BatchId,
    'operations' : IDL.Vec(BatchOperationKind),
  });
  const CommitProposedBatchArguments = IDL.Record({
    'batch_id' : BatchId,
    'evidence' : IDL.Vec(IDL.Nat8),
  });
  const ComputeEvidenceArguments = IDL.Record({
    'batch_id' : BatchId,
    'max_iterations' : IDL.Opt(IDL.Nat16),
  });
  const ConfigureArguments = IDL.Record({
    'max_batches' : IDL.Opt(IDL.Opt(IDL.Nat64)),
    'max_bytes' : IDL.Opt(IDL.Opt(IDL.Nat64)),
    'max_chunks' : IDL.Opt(IDL.Opt(IDL.Nat64)),
  });
  const DeleteBatchArguments = IDL.Record({ 'batch_id' : BatchId });
  const ConfigurationResponse = IDL.Record({
    'max_batches' : IDL.Opt(IDL.Nat64),
    'max_bytes' : IDL.Opt(IDL.Nat64),
    'max_chunks' : IDL.Opt(IDL.Nat64),
  });
  const Permission = IDL.Variant({
    'Prepare' : IDL.Null,
    'ManagePermissions' : IDL.Null,
    'Commit' : IDL.Null,
  });
  const GrantPermission = IDL.Record({
    'permission' : Permission,
    'to_principal' : IDL.Principal,
  });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
    'certificate_version' : IDL.Opt(IDL.Nat16),
  });
  const StreamingCallbackToken = IDL.Record({
    'key' : Key,
    'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'index' : IDL.Nat,
    'content_encoding' : IDL.Text,
  });
  const StreamingCallbackHttpResponse = IDL.Record({
    'token' : IDL.Opt(StreamingCallbackToken),
    'body' : IDL.Vec(IDL.Nat8),
  });
  const StreamingStrategy = IDL.Variant({
    'Callback' : IDL.Record({
      'token' : StreamingCallbackToken,
      'callback' : IDL.Func(
          [StreamingCallbackToken],
          [IDL.Opt(StreamingCallbackHttpResponse)],
          ['query'],
        ),
    }),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
    'streaming_strategy' : IDL.Opt(StreamingStrategy),
    'status_code' : IDL.Nat16,
  });
  const Time = IDL.Int;
  const ListPermitted = IDL.Record({ 'permission' : Permission });
  const RevokePermission = IDL.Record({
    'permission' : Permission,
    'of_principal' : IDL.Principal,
  });
  const ValidationResult = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : IDL.Text });
  return IDL.Service({
    'api_version' : IDL.Func([], [IDL.Nat16], ['query']),
    'authorize' : IDL.Func([IDL.Principal], [], []),
    'certified_tree' : IDL.Func(
        [IDL.Record({})],
        [
          IDL.Record({
            'certificate' : IDL.Vec(IDL.Nat8),
            'tree' : IDL.Vec(IDL.Nat8),
          }),
        ],
        ['query'],
      ),
    'clear' : IDL.Func([ClearArguments], [], []),
    'commit_batch' : IDL.Func([CommitBatchArguments], [], []),
    'commit_proposed_batch' : IDL.Func([CommitProposedBatchArguments], [], []),
    'compute_evidence' : IDL.Func(
        [ComputeEvidenceArguments],
        [IDL.Opt(IDL.Vec(IDL.Nat8))],
        [],
      ),
    'configure' : IDL.Func([ConfigureArguments], [], []),
    'create_asset' : IDL.Func([CreateAssetArguments], [], []),
    'create_batch' : IDL.Func(
        [IDL.Record({})],
        [IDL.Record({ 'batch_id' : BatchId })],
        [],
      ),
    'create_chunk' : IDL.Func(
        [IDL.Record({ 'content' : IDL.Vec(IDL.Nat8), 'batch_id' : BatchId })],
        [IDL.Record({ 'chunk_id' : ChunkId })],
        [],
      ),
    'create_chunks' : IDL.Func(
        [
          IDL.Record({
            'content' : IDL.Vec(IDL.Vec(IDL.Nat8)),
            'batch_id' : BatchId,
          }),
        ],
        [IDL.Record({ 'chunk_ids' : IDL.Vec(ChunkId) })],
        [],
      ),
    'deauthorize' : IDL.Func([IDL.Principal], [], []),
    'delete_asset' : IDL.Func([DeleteAssetArguments], [], []),
    'delete_batch' : IDL.Func([DeleteBatchArguments], [], []),
    'get' : IDL.Func(
        [IDL.Record({ 'key' : Key, 'accept_encodings' : IDL.Vec(IDL.Text) })],
        [
          IDL.Record({
            'content' : IDL.Vec(IDL.Nat8),
            'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
            'content_type' : IDL.Text,
            'content_encoding' : IDL.Text,
            'total_length' : IDL.Nat,
          }),
        ],
        ['query'],
      ),
    'get_asset_properties' : IDL.Func(
        [Key],
        [
          IDL.Record({
            'headers' : IDL.Opt(IDL.Vec(HeaderField)),
            'is_aliased' : IDL.Opt(IDL.Bool),
            'allow_raw_access' : IDL.Opt(IDL.Bool),
            'max_age' : IDL.Opt(IDL.Nat64),
          }),
        ],
        ['query'],
      ),
    'get_chunk' : IDL.Func(
        [
          IDL.Record({
            'key' : Key,
            'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
            'index' : IDL.Nat,
            'content_encoding' : IDL.Text,
          }),
        ],
        [IDL.Record({ 'content' : IDL.Vec(IDL.Nat8) })],
        ['query'],
      ),
    'get_configuration' : IDL.Func([], [ConfigurationResponse], []),
    'grant_permission' : IDL.Func([GrantPermission], [], []),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'http_request_streaming_callback' : IDL.Func(
        [StreamingCallbackToken],
        [IDL.Opt(StreamingCallbackHttpResponse)],
        ['query'],
      ),
    'list' : IDL.Func(
        [IDL.Record({})],
        [
          IDL.Vec(
            IDL.Record({
              'key' : Key,
              'encodings' : IDL.Vec(
                IDL.Record({
                  'modified' : Time,
                  'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
                  'length' : IDL.Nat,
                  'content_encoding' : IDL.Text,
                })
              ),
              'content_type' : IDL.Text,
            })
          ),
        ],
        ['query'],
      ),
    'list_authorized' : IDL.Func([], [IDL.Vec(IDL.Principal)], []),
    'list_permitted' : IDL.Func([ListPermitted], [IDL.Vec(IDL.Principal)], []),
    'propose_commit_batch' : IDL.Func([CommitBatchArguments], [], []),
    'revoke_permission' : IDL.Func([RevokePermission], [], []),
    'set_asset_content' : IDL.Func([SetAssetContentArguments], [], []),
    'set_asset_properties' : IDL.Func([SetAssetPropertiesArguments], [], []),
    'store' : IDL.Func(
        [
          IDL.Record({
            'key' : Key,
            'content' : IDL.Vec(IDL.Nat8),
            'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
            'content_type' : IDL.Text,
            'content_encoding' : IDL.Text,
          }),
        ],
        [],
        [],
      ),
    'take_ownership' : IDL.Func([], [], []),
    'unset_asset_content' : IDL.Func([UnsetAssetContentArguments], [], []),
    'validate_commit_proposed_batch' : IDL.Func(
        [CommitProposedBatchArguments],
        [ValidationResult],
        [],
      ),
    'validate_configure' : IDL.Func(
        [ConfigureArguments],
        [ValidationResult],
        [],
      ),
    'validate_grant_permission' : IDL.Func(
        [GrantPermission],
        [ValidationResult],
        [],
      ),
    'validate_revoke_permission' : IDL.Func(
        [RevokePermission],
        [ValidationResult],
        [],
      ),
    'validate_take_ownership' : IDL.Func([], [ValidationResult], []),
  });
};
export const init = ({ IDL }) => {
  const SetPermissions = IDL.Record({
    'prepare' : IDL.Vec(IDL.Principal),
    'commit' : IDL.Vec(IDL.Principal),
    'manage_permissions' : IDL.Vec(IDL.Principal),
  });
  const UpgradeArgs = IDL.Record({
    'set_permissions' : IDL.Opt(SetPermissions),
  });
  const InitArgs = IDL.Record({});
  const AssetCanisterArgs = IDL.Variant({
    'Upgrade' : UpgradeArgs,
    'Init' : InitArgs,
  });
  return [IDL.Opt(AssetCanisterArgs)];
};
