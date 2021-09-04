import React from 'react';

import { superheroes } from "../../declarations";

const $ = document.getElementById.bind(document);
const idl = require('../utilities/idl');

class Delete extends React.Component {

  constructor() {
    super();
    this.state = { success: null };
  }

  delete(event) {
    event.preventDefault();
    const superheroId = parseInt($('delete-superhero-id').value, 10);
    superheroes.delete(superheroId).then((success) => {
      this.setState({ success });
    });
  }

  render() {
    return (
      <div>
        <h2>Delete a Superhero</h2>
        <form onSubmit={ this.delete.bind(this) }>
          <label htmlFor="delete-superhero-id">Identifier: </label>
          <input id="delete-superhero-id" type="number"/>
          <br/>
          <button type="submit">Submit</button>
        </form>
        <div id="delete-response">
          <pre>
            <code>{ JSON.stringify(this.state, null, 2) }</code>
          </pre>
        </div>
      </div>
    );
  }
}

export default Delete;
