import React, { useState } from 'react';
import { Principal } from '@dfinity/principal';
import StatusMessage from './StatusMessage';

const TransferFrom = ({ actor, decimals }) => {
  const [fromAddress, setFromAddress] = useState('');
  const [toAddress, setToAddress] = useState('');
  const [amount, setAmount] = useState();
  const [spenderSubaccount, setSpenderSubaccount] = useState('');
  const [status, setStatus] = useState({ message: '', isSuccess: null });

  const handleTransferFrom = async (e) => {
    e.preventDefault();
    try {
      const result = await actor.icrc2_transfer_from({
        from: { owner: Principal.fromText(fromAddress), subaccount: [] },
        to: { owner: Principal.fromText(toAddress), subaccount: [] },
        amount: amount * Number(10 ** Number(decimals)),
        spender_subaccount: spenderSubaccount ? [spenderSubaccount] : [],
        fee: [],
        memo: [],
        created_at_time: []
      });
      if ('Ok' in result) {
        setStatus({ message: 'Transfer successful', isSuccess: true });
      } else if ('Err' in result) {
        setStatus({
          message: `Transfer failed: ${Object.keys(result.Err)[0]}`,
          isSuccess: false
        });
      }
    } catch (error) {
      console.error('Transfer failed:', error);
      setStatus({
        message: 'Transfer failed: Unexpected error',
        isSuccess: false
      });
    }
  };

  const inputFields = [
    {
      name: 'fromAddress',
      value: fromAddress,
      setter: setFromAddress,
      placeholder: 'From Address',
      type: 'text',
      required: true
    },
    {
      name: 'toAddress',
      value: toAddress,
      setter: setToAddress,
      placeholder: 'To Address',
      type: 'text',
      required: true
    },
    {
      name: 'amount',
      value: amount,
      setter: setAmount,
      placeholder: 'Amount',
      type: 'number',
      required: true,
      min: '0',
      step: '0.000001'
    },
    {
      name: 'spenderSubaccount',
      value: spenderSubaccount,
      setter: setSpenderSubaccount,
      placeholder: 'Spender Subaccount (optional)',
      type: 'text',
      required: false
    }
  ];

  return (
    <div className="mb-8 rounded-lg bg-white p-8 shadow-md">
      <h2 className="mb-6 text-3xl font-bold text-gray-800">Transfer From</h2>
      <form onSubmit={handleTransferFrom} className="space-y-6">
        {inputFields.map(({ name, value, setter, placeholder, type, required, min, step }) => (
          <input
            key={name}
            type={type}
            value={value}
            onChange={(e) => setter(e.target.value)}
            placeholder={placeholder}
            required={required}
            min={min}
            step={step}
            className="w-full rounded-md border px-3 py-2"
          />
        ))}
        <button type="submit" className="bg-infinite hover:bg-dark-infinite w-full rounded-md px-4 py-2 text-white">
          Transfer From
        </button>
      </form>
      <StatusMessage message={status.message} isSuccess={status.isSuccess} />
    </div>
  );
};

export default TransferFrom;
