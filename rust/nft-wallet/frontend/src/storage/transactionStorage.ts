interface ITransactionStorage {
  getTransactionHistory(): Promise<Array<TransactionHistory>>;
  setTransactionHistory(transaction: TransactionHistory[]): Promise<void>;
}
export interface TransactionHistory {
  date: Date;
  nft: number;
  detail: string;
}

export class TransactionStorage implements ITransactionStorage {
  async getTransactionHistory(): Promise<TransactionHistory[]> {
    const history = window.localStorage.getItem("TRANSACTION_HISTORY");
    if (history) {
      return JSON.parse(history);
    } else {
      return [];
    }
  }

  async setTransactionHistory(
    transaction: TransactionHistory[]
  ): Promise<void> {
    const transactionToAdd = JSON.stringify(transaction);

    window.localStorage.setItem("TRANSACTION_HISTORY", transactionToAdd);
  }
}
