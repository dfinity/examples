import reactTSCanister from 'ic:canisters/dfx_react_typescript';

import React from 'react';
import ReactDOM from 'react-dom';

class App extends React.Component {
  state = {
    count: 0,
  };

  constructor(props) {
    super(props);
  }

  async updateCount(val) {
    const count = +(val === '+' ? await reactTSCanister.increment() : await reactTSCanister.decrement());
    this.setState({ count });
  }

  render() {
    const { count } = this.state;

    return (
      <section>
        <h2>Count: {count}</h2>
        <h2>
          <button onClick={() => this.updateCount('+')}>Click to Add</button>
          <button onClick={() => this.updateCount('-')}>Click to Subtract</button>
        </h2>
      </section>
    );
  }
}

/*
NB: dfx bootstrap's index.html generated looks like this:

<app id="app"><progress class="ic_progress" id="ic-progress">Loading...</progress></app>
*/
ReactDOM.render(<App />, document.getElementById('app'));
