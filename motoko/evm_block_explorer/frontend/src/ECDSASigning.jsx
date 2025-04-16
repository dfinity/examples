import React, { useState } from 'react';
import { backend } from 'declarations/backend';

function Block() {
  const [loading, setLoading] = useState(false);
  const [loading2, setLoading2] = useState(false);
  const [key, setKey] = useState();
  const [error, setError] = useState();
  const [message, setMessage] = useState('');
  const [signature, setSignature] = useState('');

  const showPublicKeyECDSA = async () => {
    try {
      setLoading(true);
      setError(undefined);
      const key = await backend.get_ecdsa_public_key();
      setKey(key);
    } catch (err) {
      console.error(err);
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const signMessage = async () => {
    try {
      setLoading2(true);
      setError(undefined);
      const signature = await backend.sign_message_with_ecdsa(message);
      setSignature(JSON.stringify(signature));
    } catch (err) {
      console.error(err);
      setError(String(err));
    } finally {
      setLoading2(false);
    }
  };

  return (
    <div className="space-y-4">
      <h1 className="mb-6 text-center text-2xl font-bold text-gray-800">ECDSA Signing</h1>

      <div className="flex items-center space-x-4">
        <div className="w-full rounded-lg border border-gray-300 p-3 text-sm focus:border-blue-500 focus:ring-blue-500 md:w-1/2">
          <div className="w-full overflow-y-auto text-xs">
            {key ? (
              <pre className="whitespace-pre-wrap break-all">{key}</pre>
            ) : (
              <p className="text-gray-500">Public key will be shown here</p>
            )}
          </div>
        </div>
        <button
          onClick={showPublicKeyECDSA}
          disabled={loading}
          className={`w-full rounded-lg px-6 py-3 text-sm font-medium md:w-auto ${loading ? 'cursor-not-allowed bg-gray-300 text-gray-700' : 'bg-blue-500 text-white hover:bg-blue-600'}`}
        >
          {loading ? 'Loading...' : 'Get ECDSA public key'}
        </button>
      </div>

      <div className="mt-4 flex items-center space-x-4">
        <div className="w-full rounded-lg border border-gray-300 p-3 text-sm focus:border-blue-500 focus:ring-blue-500 md:w-1/2">
          <div className="h-24 overflow-y-auto text-xs">
            <h3 className="mb-2 text-sm font-medium text-gray-800">Enter a message to be signed:</h3>
            <input
              type="text"
              value={message}
              onChange={(e) => setMessage(e.target.value)}
              placeholder="Enter message to sign"
              className="w-full bg-transparent text-xs outline-none"
            />
          </div>
        </div>
        <button
          onClick={signMessage}
          disabled={loading2 || !message}
          className={`w-full rounded-lg px-6 py-3 text-sm font-medium md:w-auto ${loading2 ? 'cursor-not-allowed bg-gray-300 text-gray-700' : 'bg-blue-500 text-white hover:bg-blue-600'}`}
        >
          {loading2 ? 'Signing...' : 'Sign'}
        </button>
      </div>

      {signature && (
        <div className="rounded-lg border border-gray-200 bg-white p-4 shadow-sm">
          <h3 className="mb-2 text-sm font-medium text-gray-800">ECDSA signature:</h3>
          <pre className="whitespace-pre-wrap break-all text-xs">{signature}</pre>
        </div>
      )}
    </div>
  );
}

export default Block;
