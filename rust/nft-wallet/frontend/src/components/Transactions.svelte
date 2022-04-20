<script>
    import { transactionHistory } from '../storage/transactionHistory.js';
    import { isAuthorized } from "../nft.js";
    import { DataTable, InlineLoading } from "carbon-components-svelte";
    import page from "page";
    let theme = "g70";
    $: transaction_rows = $transactionHistory.map((transaction, i) => {
      const date = (new Date(transaction.date)).toLocaleString();
      return {
        "id": i,
        "date": date,
        "nft": transaction.nft,
        "detail": transaction.detail,
      }
    });

  </script>

<div id="transactions">
  {#await isAuthorized() then isAuthorized}
  {#if isAuthorized}
    {#if $transactionHistory.length > 0}
    <DataTable
      sortable
      bind:selecte={theme}
      title ="Transactions"
      headers={[
        { key: "date", value: "Date" },
        { key: "nft", value: "NFT" },
        { key: "detail", value: "Detail" },
      ]}
      rows= {transaction_rows}
      size = "tall"
    />
    {:else}
        <div class="transaction">
          No transactions record to display.
        </div>
    {/if}
    {:else}
      <div class="transaction">
        <div class="inlineLoad"><InlineLoading />Redirecting...</div>
        No transactions record, you must login first.
      </div>
      {#await setTimeout(()=> page('/'), 3000)}
      ...
      {/await}
  {/if}
  {/await}
</div>

<style>
  .inlineLoad {
        padding-bottom: 20px;
        margin: auto;
        width: fit-content;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 14px;
    }
  .transaction {
    padding-top: 15%;
    padding-bottom: 15%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    margin: 15px 0;
    font-size: 16px;
  }
  #transactions {
    margin: auto;
    text-align: center;
  }
</style>
