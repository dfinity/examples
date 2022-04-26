//@ts-check
import { writable } from "svelte/store";
import { TransactionStorage } from "./transactionStorage";

export const localStorageHistory = new TransactionStorage();

export const transactionHistory = writable([], () => {
  initialize();
  // return () => console.log("No more transactions");
});

async function initialize() {
  const history = await localStorageHistory.getTransactionHistory();
  transactionHistory.set(history);
}

export function getTransactions() {
  return transactionHistory;
}

export function addTransaction(index, detailString) {
  const newTransaction = {
    date: new Date(),
    nft: index,
    detail: detailString,
  };
  let combinedHistory = [];
  //updating writable storage
  transactionHistory.update((history) => {
    console.log("current history in update", history);
    combinedHistory = [newTransaction, ...history];
    return combinedHistory;
  });

  console.log("addTransactions called", transactionHistory);
  //updating to local storage
  localStorageHistory.setTransactionHistory(combinedHistory);
}
