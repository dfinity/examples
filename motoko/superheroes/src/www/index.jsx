import React from 'react';
import ReactDOM from 'react-dom';

import Create from './components/create.jsx';
import Read from './components/read.jsx';
import Update from './components/update.jsx';
import Delete from './components/delete.jsx';

class App extends React.Component {

  render() {
    return (
      <div>
        <h1>Superheroes</h1>
        <p>A simple example that demonstrates how to build a <a href="https://en.wikipedia.org/wiki/Create,_read,_update_and_delete">CRUD</a> application on the <a href="https://dfinity.org">Internet Computer</a> using <a href="https://sdk.dfinity.org/docs/language-guide/motoko.html">Motoko</a> and <a href="https://reactjs.org">React</a>.</p>
        <hr/>
        <Create/>
        <hr/>
        <Read/>
        <hr/>
        <Update/>
        <hr/>
        <Delete/>
      </div>
    );
  }
}

export default App;

ReactDOM.render(<App/>, document.getElementById('app'));
