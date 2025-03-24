import React, { useState, useEffect } from 'react';
import { backend, canisterId } from 'declarations/backend';
import CardDisplay from './CardDisplay';

const TokenInfo = ({ totalSupply }) => {
  const [tokenInfo, setTokenInfo] = useState({
    name: '',
    symbol: '',
    loading: true
  });

  useEffect(() => {
    const fetchTokenInfo = async () => {
      try {
        const metadata = await backend.icrc1_metadata();
        const newTokenInfo = metadata.reduce((acc, [key, value]) => {
          const parsedKey = key.split(':')[1].trim();
          if (parsedKey === 'name' || parsedKey === 'symbol') {
            acc[parsedKey] = value.Text;
          }
          return acc;
        }, {});

        setTokenInfo((prevState) => ({
          ...prevState,
          ...newTokenInfo,
          loading: false
        }));
      } catch (error) {
        console.error('Error fetching token info:', error);
        setTokenInfo((prevState) => ({ ...prevState, loading: false }));
      }
    };

    fetchTokenInfo();
  }, []);

  if (tokenInfo.loading) {
    return (
      <div className="flex h-48 items-center justify-center">
        <div className="border-t-3 border-b-3 h-12 w-12 animate-spin rounded-full border-blue-500"></div>
      </div>
    );
  }

  const cardInfo = [
    { icon: '💰', title: 'Name', value: tokenInfo.name },
    { icon: '🏷️', title: 'Symbol', value: tokenInfo.symbol },
    { icon: '📊', title: 'Total Supply', value: totalSupply },
    { icon: '💳', title: 'Token Address (ICRC-2)', value: canisterId }
  ];

  return <CardDisplay cards={cardInfo} />;
};

export default TokenInfo;
