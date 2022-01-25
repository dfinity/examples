<script>
  import CanisterIds from "./components/CanisterIds.svelte";
  import Links from "./components/Links.svelte";
  import BalanceInfo from './components/BalanceInfo.svelte';
  import Orders from './components/Orders.svelte';
  import { library } from '@fortawesome/fontawesome-svg-core';
  import { faMinus, faPlus } from '@fortawesome/free-solid-svg-icons';

 // Add fontawesome icons
 const icons = [faMinus, faPlus]
 library.add(icons);
</script>

<main>
  <a
    href="https://dfinity.org"
    target="_blank"
    rel="noopener noreferrer"
    class="logo"
  >
    <img src="images/dfinity.svg" alt="DFINITY logo" />
  </a>
  <h1>DeFi DEX Example</h1>
  <BalanceInfo />
  <Orders />
  <Links />
  <CanisterIds />
</main>

<style>
  img {
    height: 22px;
  }
  .logo {
    display: inline-block;
  }

  main {
    text-align: center;
    padding: 1em;
  }

  h1 {
    text-transform: uppercase;
    font-size: 3em;
    font-weight: 400;
    line-height: 1.09;
  }

  @media (min-width: 640px) {
    main {
      max-width: 800px;
      margin: 0 auto;
    }
    h1 {
      font-size: 4em;
    }
  }
</style>
