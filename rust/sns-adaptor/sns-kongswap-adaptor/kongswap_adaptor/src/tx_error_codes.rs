#[allow(dead_code)]
pub(crate) enum TransactionErrorCodes {
    PreConditionCode,
    PostConditionCode,
    CallFailedCode,
    BackendCode,
    TemporaryUnavailableCode,
    GenericErrorCode,
}

impl From<TransactionErrorCodes> for u64 {
    fn from(value: TransactionErrorCodes) -> Self {
        match value {
            TransactionErrorCodes::PreConditionCode => 0,
            TransactionErrorCodes::PostConditionCode => 1,
            TransactionErrorCodes::CallFailedCode => 2,
            TransactionErrorCodes::BackendCode => 3,
            TransactionErrorCodes::TemporaryUnavailableCode => 4,
            TransactionErrorCodes::GenericErrorCode => 5,
        }
    }
}
