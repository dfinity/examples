import React, { useState } from 'react';
import { Principal } from '@dfinity/principal';
import StatusMessage from './StatusMessage';

const ApproveSpender = ({ actor, decimals }) => {
  const [spenderAddress, setSpenderAddress] = useState('');
  const [amount, setAmount] = useState('');
  const [fromSubaccount, setFromSubaccount] = useState('');
  const [status, setStatus] = useState({ message: '', isSuccess: null });

  const handleApprove = async (e) => {
    e.preventDefault();
    try {
      const result = await actor.icrc2_approve({
        spender: { owner: Principal.fromText(spenderAddress), subaccount: [] },
        amount: amount * Number(10 ** Number(decimals)),
        from_subaccount: fromSubaccount ? [fromSubaccount] : [],
        expires_at: [],
        expected_allowance: [],
        memo: [],
        fee: [],
        created_at_time: []
      });
      if ('Ok' in result) {
        setStatus({ message: 'Approval successful', isSuccess: true });
      } else if ('Err' in result) {
        setStatus({
          message: `Approval failed: ${Object.keys(result.Err)[0]}`,
          isSuccess: false
        });
      }
    } catch (error) {
      console.error('Approval failed:', error);
      setStatus({
        message: 'Approval failed failed: Unexpected error',
        isSuccess: false
      });
    }
  };

  const inputFields = [
    {
      name: 'fromSubaccount',
      value: fromSubaccount,
      setter: setFromSubaccount,
      placeholder: 'From Subaccount (optional)',
      type: 'text',
      required: false
    },
    {
      name: 'spenderAddress',
      value: spenderAddress,
      setter: setSpenderAddress,
      placeholder: 'Spender Address',
      type: 'text',
      required: true
    },
    {
      name: 'amount',
      value: amount,
      setter: setAmount,
      placeholder: 'Approved Amount',
      type: 'number',
      required: true,
      min: '0',
      step: '0.000001'
    }
  ];

  return (
    <div className="mb-8 rounded-lg bg-white p-8 shadow-md">
      <h2 className="mb-6 text-3xl font-bold text-gray-800">Approve Spender</h2>
      <form onSubmit={handleApprove} className="space-y-6">
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
          Approve Spender
        </button>
      </form>
      <StatusMessage message={status.message} isSuccess={status.isSuccess} />
    </div>
  );
};

export default ApproveSpender;
