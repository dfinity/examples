import React, { useState, useEffect } from 'react';
import { authClient, createBackendActor } from './actor';

// Reusable button component
const Button = ({ onClick, children }) => <button onClick={onClick}>{children}</button>;

const App = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(authClient.isAuthenticated());
  const [principal, setPrincipal] = useState('Click "Whoami" to see your principal ID');

  // Track the sign-in status, including changes made in another tab.
  useEffect(() => authClient.subscribe(() => setIsAuthenticated(authClient.isAuthenticated())), []);

  const login = async () => {
    try {
      await authClient.signIn();
    } catch (error) {
      console.error('Sign-in failed:', error);
    }
  };

  const logout = async () => {
    await authClient.signOut();
  };

  const whoami = async () => {
    setPrincipal('Loading...');

    // Read the identity at call time: it is installed only once signIn() completes.
    const identity = authClient.isAuthenticated() ? await authClient.getIdentity() : undefined;
    const result = await createBackendActor(identity).whoami();
    setPrincipal(result.toString());
  };

  return (
    <div>
      <h1>Who Am I?</h1>
      <div id="info-box" className="info-box">
        <div className="info-content">
          <p>
            <i className="fas fa-info-circle"></i> A <strong>principal</strong> is a unique identifier in the Internet
            Computer ecosystem.
          </p>
          <p>
            It represents an entity (user, canister smart contract, or other) and is used for identification and
            authorization purposes.
          </p>
          <p>
            In this example, click "Whoami" to find out the principal ID with which you're interacting with the backend.
            If you're not signed in, you will see that you're using the so-called anonymous principal, "2vxsx-fae".
          </p>
          <p>
            After you've logged in with Internet Identity, you'll see a longer principal, which is unique to your
            identity and the dapp you're using.
          </p>
        </div>
      </div>

      {!isAuthenticated ? (
        <Button onClick={login}>Login with Internet Identity</Button>
      ) : (
        <Button onClick={logout}>Logout</Button>
      )}

      <Button onClick={whoami}>Whoami</Button>

      {principal && (
        <div>
          <h2>Your principal ID is:</h2>
          <h4>{principal}</h4>
        </div>
      )}
    </div>
  );
};

export default App;
