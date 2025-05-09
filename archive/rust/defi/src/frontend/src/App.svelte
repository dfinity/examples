<script>
  import Nav from './components/Nav.svelte';
  import Orders from './components/Orders.svelte';
  import { library } from '@fortawesome/fontawesome-svg-core';
  import { faMinus, faPlus, faCheck, faTimes, faCopy, faArrowRight, faArrowLeft } from '@fortawesome/free-solid-svg-icons';
  import { auth, plugWallet } from './store/auth';
  import BalanceInfo from './components/BalanceInfo.svelte';

 // Add fontawesome icons
 const icons = [faMinus, faPlus, faCheck, faTimes, faCopy, faArrowRight, faArrowLeft];
 library.add(icons);
</script>

<main>
  <Nav />
  <h1>DeFi DEX</h1>
  <div class="main-container">
    {#if $auth.loggedIn || $plugWallet.isConnected}
     <BalanceInfo />
    {/if}
    <Orders />
  </div>
  <div class="footer">
    <a class="ic-badge-link" href="https://www.smartcontracts.org">
      <picture>
        <img src="images/ic-badge.png" alt="Powered by the Internet Computer" />
      </picture>
    </a>
  </div>
</main>

<style>


  .main-container {
    align-items: left;
  }

  main {
    text-align: left;
    padding: 1em;
  }

  h1 {
    text-transform: uppercase;
    font-size: 3em;
    font-weight: 400;
    line-height: 1.09;
  }

  .footer {
    position: fixed;
    bottom: 0;
    width: 100vw;
    padding: 20px;
    text-align: center;
  }

  .ic-badge-link {
    cursor: pointer;
  }

  @media (max-width: 640px) {
    main {
      max-width: 800px;
      margin: 0 auto;
    }
    h1 {
      font-size: 2em;
    }
  }
</style>
