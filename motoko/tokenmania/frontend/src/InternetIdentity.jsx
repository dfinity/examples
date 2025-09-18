import React, { useEffect, useState } from 'react';
import { AuthClient } from '@dfinity/auth-client';
import { createActor, canisterId } from 'declarations/backend';

const network = process.env.DFX_NETWORK;
const identityProvider =
  network === 'ic'
    ? 'https://id.ai/' // Mainnet
    : 'http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943'; // Local

const InternetIdentity = ({ setActor, isAuthenticated, setIsAuthenticated }) => {
  const [authClient, setAuthClient] = useState();
  const [principal, setPrincipal] = useState();
  useEffect(() => {
    updateActor();
  }, []);

  async function updateActor() {
    const authClient = await AuthClient.create();
    const identity = authClient.getIdentity();
    const actor = createActor(canisterId, {
      agentOptions: {
        identity
      }
    });
    const isAuthenticated = await authClient.isAuthenticated();

    setActor(actor);
    setAuthClient(authClient);
    setIsAuthenticated(isAuthenticated);
    setPrincipal(identity.getPrincipal().toString());
  }

  async function login() {
    await authClient.login({
      identityProvider,
      onSuccess: updateActor
    });
  }

  async function logout() {
    await authClient.logout();
    updateActor();
  }

  return (
    <div className="flex items-center space-x-4">
      {isAuthenticated ? (
        <>
          <p className="text-sm">
            <span className="font-mono">{principal}</span>
          </p>
          <button
            onClick={logout}
            className="transform rounded-lg bg-red-500 px-3 py-1 text-sm font-bold text-white shadow-md transition duration-300 ease-in-out hover:scale-105 hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-opacity-50"
          >
            Sign Out
          </button>
        </>
      ) : (
        <button
          onClick={login}
          className="transform rounded-lg bg-white px-3 py-1 text-sm font-bold text-blue-600 shadow-md transition duration-300 ease-in-out hover:scale-105 focus:outline-none focus:ring-2 focus:ring-white focus:ring-opacity-50"
        >
          Sign In with Internet Identity
        </button>
      )}
    </div>
  );
};

export default InternetIdentity;
