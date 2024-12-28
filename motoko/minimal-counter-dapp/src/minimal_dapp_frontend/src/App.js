import { html, render } from 'lit-html';
import { minimal_dapp_backend } from 'declarations/minimal_dapp_backend';
import logo from './logo2.svg';

class App {
  counter = '';

  constructor() {
    this.#init();
  }

  #init = async () => {
    this.counter = await minimal_dapp_backend.getCount();
    this.#render();
  }

  #increment = async (e) => {
    e.preventDefault();
    this.counter = await minimal_dapp_backend.increment();
    this.#render();
  };

  #decrement = async (e) => {
    e.preventDefault();
    this.counter = await minimal_dapp_backend.decrement();
    this.#render();
  };

  #reload = async (e) => {
    e.preventDefault();
    this.#init();
  }

  #reset = async (e) => {
    e.preventDefault();
    this.counter = await minimal_dapp_backend.reset();
    this.#render();
  }

  #render() {
    let body = html`
      <main>
        <img src="${logo}" alt="DFINITY logo" />
        <br />
        <br />
        <form action="#">
          <button id="increment-btn">Increment</button>
          <button id="decrement-btn">Decrement</button>
          <button id="reload-btn">Reload</button>
          <button id="reset-btn">Reset</button>
        </form>
        <section id="counter">Counter: ${this.counter}</section>
      </main>
    `;
    render(body, document.getElementById('root'));
    document.getElementById('increment-btn').addEventListener('click', this.#increment);
    document.getElementById('decrement-btn').addEventListener('click', this.#decrement);
    document.getElementById('reload-btn').addEventListener('click', this.#reload);
    document.getElementById('reset-btn').addEventListener('click', this.#reset);
    if (!this.counter) {
      document.getElementById('decrement-btn').disabled = true;
    } else {
      document.getElementById('decrement-btn').disabled = false;
    }
  }
}

export default App;
