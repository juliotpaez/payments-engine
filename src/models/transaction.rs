use crate::models::{ClientId, Fixed4, TransactionId};
use serde::{Deserialize, Serialize};

/// The representation of a transaction in the system.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique identifier of the client that performs the transaction.
    #[serde(rename = "type")]
    pub kind: TransactionType,

    /// Unique identifier of the client that performs the transaction.
    pub client: ClientId,

    /// Unique identifier of this transaction or the referenced transaction by the type.
    #[serde(rename = "tx")]
    pub transaction: TransactionId,

    /// Amount processed by the transaction.
    /// Ignored for the types that do not require an amount.
    #[serde(default)]
    pub amount: Fixed4,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Different types of the transaction.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
