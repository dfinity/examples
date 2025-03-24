import React, { useState } from 'react';
import { Principal } from '@dfinity/principal';
import { backend } from 'declarations/backend';

const BalanceChecker = ({ decimals }) => {
  const [principal, setPrincipal] = useState('');
  const [subaccount, setSubaccount] = useState('');
  const [balance, setBalance] = useState(null);
  const [error, setError] = useState('');

  const handleCheckBalance = async (e) => {
    e.preventDefault();
    setBalance(null);
    setError('');

    try {
      const owner = Principal.fromText(principal);
      const subaccountArray = subaccount
        ? [new Uint8Array(subaccount.split(',').map((num) => parseInt(num.trim(), 10)))]
        : [];

      const result = await backend.icrc1_balance_of({
        owner: owner,
        subaccount: subaccountArray
      });

      const supplyScaler = (s) => {
        return Number(s) / Number(10n ** decimals);
      };
      setBalance(supplyScaler(result).toString());
    } catch (err) {
      console.error('Error checking balance:', err);
      setError('Failed to check balance. Please ensure the principal is valid.');
    }
  };

  const inputFields = [
    {
      name: 'principal',
      value: principal,
      setter: setPrincipal,
      placeholder: 'Principal ID',
      type: 'text',
      required: true
    },
    {
      name: 'subaccount',
      value: subaccount,
      setter: setSubaccount,
      placeholder: 'Subaccount (optional)',
      type: 'text',
      required: false
    }
  ];

  return (
    <div className="mb-8 rounded-lg bg-white p-8 shadow-md">
      <h2 className="mb-6 text-3xl font-bold text-gray-800">Check Balance</h2>
      <form onSubmit={handleCheckBalance} className="space-y-6">
        {inputFields.map(({ name, value, setter, placeholder, type, required }) => (
          <input
            key={name}
            type={type}
            value={value}
            onChange={(e) => setter(e.target.value)}
            placeholder={placeholder}
            required={required}
            className="w-full rounded-md border px-3 py-2"
          />
        ))}
        <button type="submit" className="bg-infinite hover:bg-dark-infinite w-full rounded-md px-4 py-2 text-white">
          Check Balance
        </button>
      </form>
      {balance !== null && <div className="mt-4 rounded-md border bg-gray-100 p-3">Balance: {balance}</div>}
      {error && <div className="mt-4 rounded-md border border-red-400 bg-red-100 p-3 text-red-700">{error}</div>}{' '}
    </div>
  );
};

export default BalanceChecker;
