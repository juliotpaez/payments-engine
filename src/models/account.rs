use crate::models::{ClientId, Fixed4, TransactionId};
use std::collections::HashMap;

/// The account of a specific client.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClientAccount {
    /// Unique identifier of the client that owns the account.
    pub client: ClientId,

    /// Amount that is available for trading, i.e. not held for any reason.
    pub available: Fixed4,

    /// Amount that is not available for the user to use.
    pub held: Fixed4,

    /// Total amount in the account, i.e. available + held.
    pub total: Fixed4,

    /// Whether the account is locked or not.
    pub locked: bool,

    /// Map of past deposit transactions that could be disputed.
    pub past_transactions: HashMap<TransactionId, DepositTransactionState>,
}

impl ClientAccount {
    // CONSTRUCTORS -----------------------------------------------------------

    /// Builds a new empty [ClientAccount] for a client.
    pub fn new(client: ClientId) -> Self {
        Self {
            client,
            available: Fixed4::default(),
            held: Fixed4::default(),
            total: Fixed4::default(),
            locked: false,
            past_transactions: HashMap::new(),
        }
    }

    // METHODS ----------------------------------------------------------------

    /// Adds a transaction to the map for this account.
    pub fn add_transaction(&mut self, transaction_id: TransactionId, amount: Fixed4) {
        self.past_transactions.insert(
            transaction_id,
            DepositTransactionState {
                amount,
                disputed: false,
            },
        );
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// The state of a deposit transaction.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DepositTransactionState {
    /// The amount of the transaction.
    pub amount: Fixed4,

    /// Whether it was disputed or not.
    pub disputed: bool,
}
