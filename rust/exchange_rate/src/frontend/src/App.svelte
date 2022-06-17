<script>
  import Chart from "./components/Chart.svelte";
</script>

<main>
  <h1>ICP Price Chart</h1>

  <Chart />

  <a
    href="https://dfinity.org"
    target="_blank"
    rel="noopener noreferrer"
    class="logo"
  >
    <img src="images/ic-badge-powered-by-crypto_label-stripe-white-text.png" alt="DFINITY logo" />
  </a>
</main>

<style>
  h1 {
    color: #a9a9a9;
    font-weight: 700;
  }

  .logo {
    display: inline-block;
    margin-bottom: 64px;
    margin-top: 64px;
  }

  main {
    text-align: center;
    padding: 1em;
  }

  @media (min-width: 640px) {
    main {
      max-width: 800px;
      margin: 0 auto;
    }
  }
</style>
