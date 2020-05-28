import "regenerator-runtime/runtime.js";

import reactCanister from 'ic:canisters/dfx_react';

import React, { useState } from 'react';
import ReactDOM from 'react-dom';

async function asyncUpdateCount(val, callback) {
  let bigIntCount = 0;

  switch (val) {
    case '+':
      bigIntCount = await reactCanister.increment();
      break;
    case '-':
      bigIntCount = await reactCanister.decrement();
      break;
  }
  const count = parseInt(BigInt(bigIntCount).toString(), 10);
  callback(count);
}

export function App() {
  const [count, updateCount] = useState(0);

  return (
    <section>
      <h2>Count: {count}</h2>
      <h2>
        <button onClick={() => asyncUpdateCount('+', updateCount)}>Click to Increase</button>
        <button onClick={() => asyncUpdateCount('-', updateCount)}>Click to Decrease</button>
      </h2>
    </section>
  );
}

/*
NB: dfx bootstrap's index.html generated looks like this:

<app id="app"><progress class="ic_progress" id="ic-progress">Loading...</progress></app>
*/
ReactDOM.render(<App />, document.getElementById('app'));
