import React from 'react';

const Score = ({ score, color }) => {
  const scoreStyle = {
    position: 'absolute',
    top: '20px',
    left: '50%',
    transform: 'translateX(-50%)',
    fontSize: '24px',
    color
  };

  return <div style={scoreStyle}>Score: {score}</div>;
};

export default Score;
