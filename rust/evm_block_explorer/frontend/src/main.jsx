import React from 'react';
import ReactDOM from 'react-dom/client';
import '../index.css';
import Block from './Block';

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <div className="mx-auto my-10 max-w-4xl rounded-lg bg-white p-8 shadow-lg">
      <div className="space-y-6 rounded-lg bg-gray-50 p-6">
        <Block />
      </div>
    </div>
  </React.StrictMode>
);
