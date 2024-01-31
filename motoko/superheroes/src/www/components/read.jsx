import React from 'react';

import { superheroes } from "../../declarations";

const $ = document.getElementById.bind(document);
const idl = require('../utilities/idl');

class Read extends React.Component {

  constructor() {
    super();
    this.state = { superhero: null };
  }

  read(event) {
    event.preventDefault();
    const superheroId = parseInt($('read-superhero-id').value, 10);
    superheroes.read(superheroId).then((result) => {
      const superhero = idl.fromOptional(result);
      if (superhero) {
        superhero.superpowers = idl.fromList(superhero.superpowers);
      };
      this.setState({ superhero });
    });
  }

  render() {
    return (
      <div>
        <h2>Read a Superhero</h2>
        <form onSubmit={ this.read.bind(this) }>
          <label htmlFor="read-superhero-id">Identifier: </label>
          <input id="read-superhero-id" type="number"/>
          <br/>
          <button type="submit">Submit</button>
        </form>
        <div id="read-response">
          <pre>
            <code>{ JSON.stringify(this.state, null, 2) }</code>
          </pre>
        </div>
      </div>
    );
  }
}

export default Read;
