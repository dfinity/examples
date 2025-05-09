module {
  public type GetTransactionError = {
    #invalidTransaction;
    #unsupportedResponse;
  };

  public type InsertTransactionError = {
    #invalidTransaction;
    #unsupportedResponse;
  };
}