import React from 'react';

const StatusMessage = ({ message, isSuccess }) => {
  if (!message) return null;

  return (
    <div
      className={`mt-4 flex items-center rounded-md border p-3 ${
        isSuccess ? 'border-green-300 bg-green-100 text-green-800' : 'border-red-300 bg-red-100 text-red-800'
      }`}
    >
      <span
        className={`mr-2 inline-block h-5 w-5 rounded-full ${
          isSuccess ? 'bg-green-500' : 'bg-red-500'
        } flex items-center justify-center`}
      >
        {isSuccess ? <span className="text-xs text-white">✓</span> : <span className="text-xs text-white">✕</span>}
      </span>
      {message}
    </div>
  );
};

export default StatusMessage;
