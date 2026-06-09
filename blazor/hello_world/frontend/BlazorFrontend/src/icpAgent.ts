import { Actor, HttpAgent } from "@dfinity/agent";

const idlFactory = ({ IDL }: any) =>
  IDL.Service({
    getGreeting: IDL.Func([], [IDL.Text], ["query"]),
    setGreeting: IDL.Func([IDL.Text], [IDL.Text], []),
    hello: IDL.Func([IDL.Text], [IDL.Text], ["query"]),
  });

function getCanisterId(): string {
  const decoded = decodeURIComponent(document.cookie);
  const match = decoded
    .split("; ")
    .find((c) => c.startsWith("ic_env="));

  if (match) {
    const params = new URLSearchParams(match.replace("ic_env=", ""));
    const id = params.get("PUBLIC_CANISTER_ID:backend");
    if (id) return id;
  }

  throw new Error("Canister ID not found. Make sure the app is accessed via the canister subdomain.");
}

let _actor: any = null;

async function getActor() {
  if (!_actor) {
    const isLocal =
      window.location.hostname === "localhost" ||
      window.location.hostname.endsWith(".localhost");

    const agent = await HttpAgent.create({
      host: isLocal ? "http://localhost:8000" : "https://ic0.app",
    });

    if (isLocal) {
      await agent.fetchRootKey();
    }

    _actor = Actor.createActor(idlFactory, {
      agent,
      canisterId: getCanisterId(),
    });
  }
  return _actor;
}

export async function getGreeting(): Promise<string> {
  return (await getActor()).getGreeting();
}

export async function setGreeting(name: string): Promise<string> {
  return (await getActor()).setGreeting(name);
}

export async function hello(name: string): Promise<string> {
  return (await getActor()).hello(name);
}
