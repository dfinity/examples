import React, { useState } from "react";
import { backend } from "./actor.js";

function Create() {
  const [superheroId, setSuperheroId] = useState(null);
  const [name, setName] = useState("");
  const [superpowers, setSuperpowers] = useState([""]);

  async function handleCreate(event) {
    event.preventDefault();
    const id = await backend.create({ name, superpowers });
    setSuperheroId(Number(id));
  }

  function addSuperpower() {
    setSuperpowers([...superpowers, ""]);
  }

  function updateSuperpower(index, value) {
    const updated = [...superpowers];
    updated[index] = value;
    setSuperpowers(updated);
  }

  return (
    <div>
      <h2>Create a Superhero</h2>
      <form onSubmit={handleCreate}>
        <label>
          Name:{" "}
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </label>
        <br />
        {superpowers.map((sp, i) => (
          <div key={i}>
            <label>
              Superpower #{i + 1}:{" "}
              <input
                type="text"
                value={sp}
                onChange={(e) => updateSuperpower(i, e.target.value)}
              />
            </label>
            <br />
          </div>
        ))}
        <button type="button" onClick={addSuperpower}>
          Add Superpower
        </button>
        <br />
        <button type="submit">Create</button>
      </form>
      {superheroId !== null && (
        <pre>
          <code>{JSON.stringify({ superheroId }, null, 2)}</code>
        </pre>
      )}
    </div>
  );
}

function Read() {
  const [superheroId, setSuperheroId] = useState("");
  const [superhero, setSuperhero] = useState(null);

  async function handleRead(event) {
    event.preventDefault();
    const result = await backend.read(parseInt(superheroId, 10));
    setSuperhero(result);
  }

  return (
    <div>
      <h2>Read a Superhero</h2>
      <form onSubmit={handleRead}>
        <label>
          Identifier:{" "}
          <input
            type="number"
            value={superheroId}
            onChange={(e) => setSuperheroId(e.target.value)}
          />
        </label>
        <br />
        <button type="submit">Read</button>
      </form>
      {superhero !== undefined && (
        <pre>
          <code>{JSON.stringify({ superhero }, null, 2)}</code>
        </pre>
      )}
    </div>
  );
}

function Update() {
  const [superheroId, setSuperheroId] = useState("");
  const [name, setName] = useState("");
  const [superpowers, setSuperpowers] = useState([""]);
  const [success, setSuccess] = useState(null);

  async function handleUpdate(event) {
    event.preventDefault();
    const result = await backend.update(parseInt(superheroId, 10), {
      name,
      superpowers,
    });
    setSuccess(result);
  }

  function addSuperpower() {
    setSuperpowers([...superpowers, ""]);
  }

  function updateSuperpower(index, value) {
    const updated = [...superpowers];
    updated[index] = value;
    setSuperpowers(updated);
  }

  return (
    <div>
      <h2>Update a Superhero</h2>
      <form onSubmit={handleUpdate}>
        <label>
          Identifier:{" "}
          <input
            type="number"
            value={superheroId}
            onChange={(e) => setSuperheroId(e.target.value)}
          />
        </label>
        <br />
        <label>
          Name:{" "}
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </label>
        <br />
        {superpowers.map((sp, i) => (
          <div key={i}>
            <label>
              Superpower #{i + 1}:{" "}
              <input
                type="text"
                value={sp}
                onChange={(e) => updateSuperpower(i, e.target.value)}
              />
            </label>
            <br />
          </div>
        ))}
        <button type="button" onClick={addSuperpower}>
          Add Superpower
        </button>
        <br />
        <button type="submit">Update</button>
      </form>
      {success !== null && (
        <pre>
          <code>{JSON.stringify({ success }, null, 2)}</code>
        </pre>
      )}
    </div>
  );
}

function Delete() {
  const [superheroId, setSuperheroId] = useState("");
  const [success, setSuccess] = useState(null);

  async function handleDelete(event) {
    event.preventDefault();
    const result = await backend.delete(parseInt(superheroId, 10));
    setSuccess(result);
  }

  return (
    <div>
      <h2>Delete a Superhero</h2>
      <form onSubmit={handleDelete}>
        <label>
          Identifier:{" "}
          <input
            type="number"
            value={superheroId}
            onChange={(e) => setSuperheroId(e.target.value)}
          />
        </label>
        <br />
        <button type="submit">Delete</button>
      </form>
      {success !== null && (
        <pre>
          <code>{JSON.stringify({ success }, null, 2)}</code>
        </pre>
      )}
    </div>
  );
}

export default function App() {
  return (
    <div>
      <h1>Superheroes</h1>
      <p>
        A simple{" "}
        <a href="https://en.wikipedia.org/wiki/Create,_read,_update_and_delete">
          CRUD
        </a>{" "}
        application on the{" "}
        <a href="https://internetcomputer.org">Internet Computer</a> using{" "}
        <a href="https://docs.internetcomputer.org/motoko/home">Motoko</a> and{" "}
        <a href="https://reactjs.org">React</a>.
      </p>
      <hr />
      <Create />
      <hr />
      <Read />
      <hr />
      <Update />
      <hr />
      <Delete />
    </div>
  );
}
