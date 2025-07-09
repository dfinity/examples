import { Actor, HttpAgent } from "@dfinity/agent";

export const idlFactory = ({ IDL }) => {
    const ApiError = IDL.Variant({
        Unauthorized: IDL.Null,
        InvalidTokenId: IDL.Null,
        ZeroAddress: IDL.Null,
        Other: IDL.Null,
    });
    const OwnerResult = IDL.Variant({
        Err: ApiError,
        Ok: IDL.Principal,
    });
    const LogoResult = IDL.Record({
        logo_type: IDL.Text,
        data: IDL.Text,
    });
    const MetadataPurpose = IDL.Variant({
        Preview: IDL.Null,
        Rendered: IDL.Null,
    });
    const MetadataVal = IDL.Variant({
        TextContent: IDL.Text,
        BlobContent: IDL.Vec(IDL.Nat8),
        NatContent: IDL.Nat,
        Nat8Content: IDL.Nat8,
        Nat16Content: IDL.Nat16,
        Nat32Content: IDL.Nat32,
        Nat64Content: IDL.Nat64,
    });
    const MetadataKeyVal = IDL.Tuple(IDL.Text, MetadataVal);
    const MetadataPart = IDL.Record({
        purpose: MetadataPurpose,
        key_val_data: IDL.Vec(MetadataKeyVal),
        data: IDL.Vec(IDL.Nat8),
    });
    const MetadataDesc = IDL.Vec(MetadataPart);
    const MetadataResult = IDL.Variant({
        Err: ApiError,
        Ok: MetadataDesc,
    });
    return IDL.Service({
        getMetadataDip721: IDL.Func([IDL.Nat64], [MetadataResult], ['query']),
        ownerOfDip721: IDL.Func([IDL.Nat64], [OwnerResult], ['query']),
        logoDip721: IDL.Func([], [LogoResult], ['query']),
        nameDip721: IDL.Func([], [IDL.Text], ['query']),
        symbolDip721: IDL.Func([], [IDL.Text], ['query']),
    });
}

export const createActor = (canisterId, options) => {
    const agent = new HttpAgent({ ...options?.agentOptions });
    
    // Fetch root key for certificate validation during development
    if(process.env.NODE_ENV !== "production") {
        agent.fetchRootKey().catch(err=>{
            console.warn("Unable to fetch root key. Check to ensure that your local replica is running");
            console.error(err);
        });
    }
  
    // Creates an actor with using the candid interface and the HttpAgent
    return Actor.createActor(idlFactory, {
        agent,
        canisterId,
        ...options?.actorOptions,
    });
};

function createActorFactory(agentOptions, actorOptions) {
    return (canisterId) => createActor(canisterId, {agentOptions, actorOptions});
}

export default createActorFactory;
