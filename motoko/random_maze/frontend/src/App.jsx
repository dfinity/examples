import React, { useState } from 'react';
import { backend } from './actor';

const App = () => {
  const [size, setSize] = useState('');
  const [maze, setMaze] = useState('');
  const [loading, setLoading] = useState(false);

  const handleGenerate = async () => {
    const n = parseInt(size, 10);
    if (!Number.isInteger(n) || n < 0) return;
    setLoading(true);
    try {
      const result = await backend.generate(BigInt(n));
      setMaze(result);
    } catch (error) {
      console.error('Error generating maze:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ backgroundColor: 'powderblue', textAlign: 'center', minHeight: '100vh', fontFamily: 'sans-serif', fontSize: '1.5rem' }}>
      <img src="/logo.png" alt="DFINITY logo" style={{ maxWidth: '50vw', maxHeight: '25vw', display: 'block', margin: 'auto' }} />
      <div>
        <label htmlFor="size">Approximate size of maze?*</label>
        <input
          id="size"
          type="number"
          min="0"
          value={size}
          onChange={(e) => setSize(e.target.value)}
        />
        <button onClick={handleGenerate} disabled={loading || size === '' || parseInt(size, 10) < 0}>
          {loading ? 'Generating…' : 'Generate!'}
        </button>
      </div>
      <pre>*rounded down to odd number.</pre>
      <pre id="maze" style={{ lineHeight: '1' }}>{maze}</pre>
    </div>
  );
};

export default App;
