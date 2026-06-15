import React, { useState } from 'react';
import { backend } from './actor';

const App = () => {
  const [size, setSize] = useState(10);
  const [maze, setMaze] = useState('');
  const [loading, setLoading] = useState(false);

  const handleGenerate = async () => {
    setLoading(true);
    try {
      const result = await backend.generate(BigInt(size));
      setMaze(result);
    } catch (error) {
      console.error('Error generating maze:', error);
    }
    setLoading(false);
  };

  return (
    <div style={{ padding: '2rem', fontFamily: 'sans-serif' }}>
      <h1>Random Maze</h1>
      <div>
        <label htmlFor="size">Maze size: </label>
        <input
          id="size"
          type="number"
          min="1"
          max="50"
          value={size}
          onChange={(e) => setSize(Number(e.target.value))}
          style={{ marginRight: '1rem' }}
        />
        <button onClick={handleGenerate} disabled={loading}>
          {loading ? 'Generating…' : 'Generate!'}
        </button>
      </div>
      {maze && (
        <pre
          id="maze"
          style={{
            marginTop: '1rem',
            lineHeight: '1',
            fontSize: '1rem',
            whiteSpace: 'pre',
          }}
        >
          {maze}
        </pre>
      )}
    </div>
  );
};

export default App;
