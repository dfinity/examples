// import { writable } from 'svelte/store';
import { AuthClient } from "@dfinity/auth-client";
import { HttpAgent, Actor } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import * as wallet from "./declarations/nftwallet/nftwallet.did.js";
import dipCanisterFactory from "./dip721";
import { CID } from "multiformats/cid";
import { Base64 } from "js-base64";
import mime from "mime/lite";

const hostOptions = {
  host: isMainnet() ? "https://ic0.app" : "http://localhost:8000",
};

let dipCanister = dipCanisterFactory(hostOptions);

export function fetchNftRouter(canister, index) {
  return (context, trigger) => {
    fetchNft(
      context.params[canister],
      Number.parseInt(context.params[index])
    ).then(
      (nft) => {
        context.state.nft = nft;
        trigger();
      },
      (error) => {
        context.state.nftError = error;
        console.log(error);
        trigger();
      }
    );
  };
}

export async function isOwner(nft) {
  const canister = dipCanister(nft.canister);
  const result = await canister.ownerOfDip721(nft.index);
  if ("Err" in result) {
    throw result.Err;
  } else {
    return result.Ok.toString() === process.env.NFTWALLET_CANISTER_ID;
  }
}

export async function fetchNft(principal, index, checkOwner) {
  if (
    checkOwner !== false &&
    !(await isOwner({ canister: principal, index }))
  ) {
    return null;
  }
  const canister = dipCanister(principal);
  const info = await fetchCollectionInfo(principal);
  const nft = { ...info, canister: principal, index };
  const metadata = await canister.getMetadataDip721(index);
  if ("Err" in metadata) {
    throw metadata.Err;
  }
  for (const part of metadata.Ok) {
    let contentType;
    let location;
    let locationType;
    for (const [key, value] of part.key_val_data) {
      switch (key) {
        case "contentType":
          contentType = value.TextContent;
          break;
        case "locationType":
          locationType = value.Nat8Content;
          break;
        case "location":
          location = value;
          break;
      }
    }
    contentType ||= "application/octet-stream";
    let url;
    switch (locationType) {
      case 1:
        let cid;
        try {
          cid = CID.decode(new Uint8Array(location.BlobContent));
        } catch {
          cid = CID.parse(String.fromCharCode(...location.BlobContent));
        }
        url = `https://${cid.toString()}.ipfs.ipfs.io`;
        break;
      case 2:
        url = `https://${location.TextContent}.raw.ic0.app/${index}.${mime.getExtension(contentType) || "bin"
          }`;
        break;
      case 3:
        url = location.TextContent;
        break;
    }
    let value;
    if (part.data.length !== 0) {
      const blob = new Blob([new Uint8Array(part.data)], { type: contentType });
      value = URL.createObjectURL(blob);
      objectUrls.push(value);
    } else {
      value = url;
    }
    if ("preview" in part.purpose) {
      if (!nft.preview && url) {
        nft.preview = { url, value, contentType, locationType };
      }
    } else {
      (nft.content ||= []).push({
        url,
        value,
        contentType,
        locationType,
      });
    }
  }
  if (!nft.preview) {
    nft.preview = nft.content?.find((part) =>
      part.contentType.startsWith("image")
    );
  }
  return nft;
}

export async function fetchAllOwnedNfts() {
  const walletCanister = await getWalletCanister();
  const nfts = await walletCanister.owned_nfts();
  const coll = [];
  for (const { canister, index } of nfts) {
    coll.push(await fetchNft(canister, index, false));
  }
  return coll;
}

export async function fetchAllOwnedNftsForCollection(principal) {
  const walletCanister = await getWalletCanister();
  const nfts = await walletCanister.owned_nfts();
  const coll = [];
  for (const { canister, index } of nfts) {
    if (canister.toString() === principal.toString()) {
      coll.push(await fetchNft(canister, index, false));
    }
  }
  return coll;
}

export async function fetchCollectionInfo(principal) {
  const canister = dipCanister(principal);
  const name = await canister.nameDip721();
  const symbol = await canister.symbolDip721();
  const logo = await canister.logoDip721();
  const icon = URL.createObjectURL(
    new Blob([Base64.toUint8Array(logo.data)], { type: logo.logo_type })
  );
  objectUrls.push(icon);
  return { name, symbol, icon };
}

let authClient;

async function getAuthClient() {
  if (!authClient) {
    authClient = await AuthClient.create();
    dipCanister = dipCanisterFactory({
      identity: authClient.getIdentity(),
      ...hostOptions,
    });
  }
  return authClient;
}

let walletCanister;

export async function isAuthenticated() {
  if (overrideAuthenticated) {
    return true; // in the middle of login.onSuccess, before authClient.isAuthenticated will return true
  }
  const authClient = await getAuthClient();
  return await authClient.isAuthenticated();
}

export async function isAuthorized() {
  const walletCanister = await getWalletCanister();
  return walletCanister && (await walletCanister.is_authorized());
}

export async function getWalletCanister() {
  if (walletCanister) {
    return walletCanister;
  }
  await getAuthClient();
  walletCanister = createWalletCanister();
  return walletCanister;
}

let overrideAuthenticated;

export async function authenticate(onSuccess) {
  const authClient = await getAuthClient();
  await authClient.login({
    onSuccess: async () => {
      overrideAuthenticated = true;
      try {
        walletCanister = createWalletCanister();
        dipCanister = dipCanisterFactory({
          identity: authClient.getIdentity(),
          ...hostOptions,
        });
        onSuccess();
      } finally {
        overrideAuthenticated = false;
      }
    },
    onError: async (e) => {
      throw e;
    },
    identityProvider: isMainnet()
      ? "https://identity.ic0.app/#authorize"
      : `http://${process.env.INTERNET_IDENTITY_CANISTER_ID}.localhost:8000/#authorize`,
  });
}

export async function logout() {
  if (authClient) {
    await authClient.logout();
  }
}

export async function getPrincipal() {
  const authClient = await getAuthClient();
  return authClient.getIdentity().getPrincipal();
}

export function getCanisterId() {
  return process.env.NFTWALLET_CANISTER_ID;
}

export function isMainnet() {
  return process.env.DFX_NETWORK === "ic";
}

export async function transfer(nft, to, notify) {
  const walletCanister = await getWalletCanister();
  const walletNft = {
    canister: Principal.fromText(nft.canister),
    index: nft.index,
  };
  const result = await walletCanister.transfer(
    walletNft,
    Principal.fromText(to),
    notify == null ? [] : [!!notify]
  );
  return result;
}

export async function register(canister, index) {
  const walletCanister = await getWalletCanister();

  try {
    const res = await walletCanister.register({
      canister: Principal.fromText(canister),
      index,
    });
    const response = {
      status: null,
      message: "",
    };

    if ("Err" in res) {
      console.error(JSON.stringify(res.Err));
      response.status = "fail";
      response.message =
        "Failed to register. \n\nThe NFT by that index may not exist, or your NFT Wallet is not the owner. Ensure that the wallet canister owns the NFT.";
    }
    if ("Ok" in res) {
      response.status = "success";
      response.message = "Successfully registered.";
    }
    return response;
  } catch (err) {
    console.error(err);
    return {
      status: "fail",
      message: `There was an error. Do you have the correct Principal ID for the NFT?`,
    };
  }
}

let objectUrls = [];

export function releaseObjectUrls() {
  for (const url of dataURLs) {
    URL.revokeObjectURL(url);
  }
  objectUrls = [];
}

function createWalletCanister() {
  const agent = new HttpAgent({
    identity: authClient.getIdentity(),
    ...hostOptions,
  });

  // Fetch root key for certificate validation during development
  if (process.env.DFX_NETWORK !== "ic") {
    agent.fetchRootKey().catch((err) => {
      console.warn(
        "Unable to fetch root key. Check to ensure that your local replica is running"
      );
      console.error(err);
    });
  }

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(wallet.idlFactory, {
    agent,
    canisterId: getCanisterId(),
  });
}
