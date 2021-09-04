import React from 'react';

import { superheroes } from "../../declarations";

const $ = document.getElementById.bind(document);
const idl = require('../utilities/idl');

class Create extends React.Component {

  constructor() {
    super();
    this.state = { superheroId: null };
  }

  create(event) {
    event.preventDefault();
    const name = $('create-name').value;
    const n = parseInt($('create-superpowers-count').value, 10);
    const superpowers = [];
    for (var i = 0; i < n; i++) {
      const superpower = $('create-superpower-' + i).value;
      superpowers.push(superpower);
    };
    const superhero = { name, superpowers };
    superhero.superpowers = idl.toList(superhero.superpowers);
    superheroes.create(superhero).then((superheroId) => {
      this.setState({ superheroId });
    });
  }

  toggle() {
    const n = parseInt($('create-superpowers-count').value, 10);
    const container = $('create-superpowers-container');
    while (container.hasChildNodes()) {
      container.removeChild(container.lastChild);
    };
    for (var i = 0; i < n; i++) {
      const label = document.createElement('label');
      label.setAttribute('for', 'create-superpower-' + i);
      label.innerHTML = 'Superpower #' + (i + 1) + ': ';
      container.appendChild(label);
      const input = document.createElement('input');
      input.id = 'create-superpower-' + i;
      input.type = 'text';
      container.appendChild(input);
      const br = document.createElement('br');
      container.appendChild(br);
    };
  }

  render() {
    return (
      <div>
        <h2>Create a Superhero</h2>
        <form onSubmit={ this.create.bind(this) }>
          <label htmlFor="create-name">Name: </label>
          <input id="create-name" type="text"/>
          <br/>
          <label htmlFor="create-superpowers-count">Superpowers: </label>
          <select id="create-superpowers-count" onChange={ this.toggle }>
            <option value="0">0</option>
            <option value="1">1</option>
            <option value="2">2</option>
            <option value="3">3</option>
            <option value="4">4</option>
            <option value="5">5</option>
          </select>
          <br/>
          <div id="create-superpowers-container"/>
          <button type="submit">Submit</button>
        </form>
        <div id="create-response">
          <pre>
            <code>{ JSON.stringify(this.state, null, 2) }</code>
          </pre>
        </div>
      </div>
    );
  }
}

export default Create;
