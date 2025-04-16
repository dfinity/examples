import React, { useState, useEffect } from 'react';
import ApproveSpender from './TokenApprove';
import AuthWarning from './AuthWarning';
import BalanceChecker from './BalanceChecker';
import Header from './Header';
import TransferFrom from './TokenTransfer';
import TokenInfo from './TokenInfo';
import TokenSender from './TokenSender';
import CreateToken from './CreateToken';

const App = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [totalSupply, setTotalSupply] = useState('');
  const [actor, setActor] = useState();
  const [tokenCreated, setTokenCreated] = useState(false);
  const [decimals, setDecimals] = useState(0n);

  const updateSupply = async () => {
    try {
      const supply = await actor.icrc1_total_supply();
      const decimals = BigInt(await actor.icrc1_decimals());
      setTotalSupply(`${Number(supply) / Number(10n ** decimals)}`);
      setDecimals(decimals);
    } catch (error) {
      console.error('Error fetching total supply:', error);
    }
  };

  const checkTokenCreated = async () => {
    try {
      const result = await actor.token_created();
      setTokenCreated(result);
    } catch (error) {
      console.error('Error fetching token created status:', error);
    }
  };

  useEffect(() => {
    if (isAuthenticated || tokenCreated) {
      updateSupply();
    }
  }, [isAuthenticated, tokenCreated]);

  useEffect(() => {
    if (actor) {
      checkTokenCreated();
    }
  }, [actor]);

  return (
    <div className="min-h-screen bg-gray-100">
      <Header
        actor={actor}
        setActor={setActor}
        isAuthenticated={isAuthenticated}
        setIsAuthenticated={setIsAuthenticated}
        tokenCreated={tokenCreated}
        setTokenCreated={setTokenCreated}
      />
      {tokenCreated ? (
        <div>
          <TokenInfo totalSupply={totalSupply} />
          <div className="mx-auto">
            {isAuthenticated ? (
              <div className="grid grid-cols-1 gap-8 px-4 md:grid-cols-4 lg:grid-cols-3">
                <BalanceChecker decimals={decimals} />
                <TokenSender actor={actor} updateSupply={updateSupply} decimals={decimals} />
                <ApproveSpender actor={actor} decimals={decimals} />
                <TransferFrom actor={actor} decimals={decimals} />
              </div>
            ) : (
              <AuthWarning />
            )}
          </div>
        </div>
      ) : (
        <div>{isAuthenticated ? <CreateToken actor={actor} setTokenCreated={setTokenCreated} /> : <AuthWarning />}</div>
      )}
    </div>
  );
};

export default App;
