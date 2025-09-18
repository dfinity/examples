import React from 'react';

const Leaderboard = ({ entries }) => {
  return (
    <div style={{ marginTop: '20px' }}>
      <h2>Leaderboard</h2>
      <table style={{ margin: '0 auto', borderCollapse: 'collapse' }}>
        <thead>
          <tr>
            <th
              style={{
                padding: '5px 10px',
                border: '1px solid black'
              }}
            >
              Rank
            </th>
            <th
              style={{
                padding: '5px 10px',
                border: '1px solid black'
              }}
            >
              Name
            </th>
            <th
              style={{
                padding: '5px 10px',
                border: '1px solid black'
              }}
            >
              Score
            </th>
          </tr>
        </thead>
        <tbody>
          {entries.map(({ name, score }, index) => (
            <tr key={index}>
              <td
                style={{
                  padding: '5px 10px',
                  border: '1px solid black'
                }}
              >
                {index + 1}
              </td>
              <td
                style={{
                  padding: '5px 10px',
                  border: '1px solid black'
                }}
              >
                {name}
              </td>
              <td
                style={{
                  padding: '5px 10px',
                  border: '1px solid black'
                }}
              >
                {score.toString()}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Leaderboard;
