import React, { useState } from 'react';
import { backend } from 'declarations/backend';
import { JsonView, allExpanded, defaultStyles } from 'react-json-view-lite';
import 'react-json-view-lite/dist/index.css';

function Block() {
  const [loading, setLoading] = useState(false);
  const [block, setBlock] = useState();
  const [error, setError] = useState();
  const [blockNumber, setBlockNumber] = useState('');

  const fetchBlock = async () => {
    try {
      setLoading(true);
      setError(undefined);
      // Convert the block number string to BigInt for the backend call
      const block = await backend.get_evm_block(BigInt(blockNumber || 420));
      setBlock(block);
    } catch (err) {
      console.error(err);
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <h1 className="mb-6 text-center text-2xl font-bold text-gray-800">EVM Block Explorer</h1>

      <div className="space-y-4">
        <div className="flex items-center space-x-4">
          <input
            type="number"
            value={blockNumber}
            onChange={(e) => setBlockNumber(e.target.value)}
            placeholder="Enter block number"
            className="w-full rounded-lg border border-gray-300 p-3 text-sm focus:border-blue-500 focus:ring-blue-500 md:w-1/2"
          />
          <button
            onClick={fetchBlock}
            disabled={loading}
            className={`w-full rounded-lg px-6 py-3 text-sm font-medium md:w-auto ${loading ? 'cursor-not-allowed bg-gray-300 text-gray-700' : 'bg-blue-500 text-white hover:bg-blue-600'}`}
          >
            {loading ? 'Loading...' : 'Fetch block'}
          </button>
        </div>

        {error && <p className="text-sm text-red-500">{error}</p>}

        {!!block && (
          <div className="mt-4 rounded-lg border border-gray-200 bg-white p-4 shadow-sm">
            <JsonView
              data={block}
              shouldExpandNode={allExpanded}
              style={{
                ...defaultStyles,
                container: 'bg-gray-50 overflow-x-auto p-4 rounded-lg',
                label: 'font-semibold text-sm',
                value: 'text-xs text-gray-700',
                item: 'mb-2'
              }}
            />
          </div>
        )}
      </div>
    </div>
  );
}

export default Block;
