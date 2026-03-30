use crate::models::{ClientAccount, ClientId, Transaction, TransactionType};
use std::collections::HashMap;

/// Handles all common state of the App.
#[derive(Default, Debug)]
pub struct AppState {
    /// Map of every account handled by the system.
    pub client_accounts: HashMap<ClientId, ClientAccount>,
}

impl AppState {
    // CONSTRUCTORS -----------------------------------------------------------

    /// Creates a new empty [AppState].
    pub fn new() -> Self {
        Self::default()
    }

    // METHODS ----------------------------------------------------------------

    /// Process a [Transaction] and update the state accordingly.
    pub fn process_transaction(&mut self, transaction: Transaction) {
        // Gets or creates the account for the user.
        let account = self
            .client_accounts
            .entry(transaction.client)
            .or_insert_with(|| ClientAccount::new(transaction.client));

        // When an account is locked, we should ignore every other operation but in some cases,
        // like in crypto, someone can send you funds to your account without your permission or any
        // other kind of control. Therefore, I ignore everything BUT deposits.
        if account.locked && !matches!(transaction.kind, TransactionType::Deposit) {
            return;
        }

        match transaction.kind {
            TransactionType::Deposit => {
                account.available += transaction.amount;
                account.total += transaction.amount;

                // Store the transaction for future disputes.
                account.add_transaction(transaction.transaction, transaction.amount);
            }
            TransactionType::Withdrawal => {
                // Cannot withdraw more quantity than available.
                if account.available < transaction.amount {
                    return;
                }

                account.available -= transaction.amount;
                account.total -= transaction.amount;
            }
            TransactionType::Dispute => {
                // Get the past transaction ignoring the dispute if it does not exist.
                let past_transaction =
                    match account.past_transactions.get_mut(&transaction.transaction) {
                        Some(v) => v,
                        None => return,
                    };

                // Ignore if already disputed.
                if past_transaction.disputed {
                    return;
                }

                account.available -= past_transaction.amount;
                account.held += past_transaction.amount;
                past_transaction.disputed = true;
            }
            TransactionType::Resolve => {
                // Get the past transaction ignoring the resolve if it does not exist.
                let past_transaction =
                    match account.past_transactions.get_mut(&transaction.transaction) {
                        Some(v) => v,
                        None => return,
                    };

                // Ignore if not yet disputed.
                if !past_transaction.disputed {
                    return;
                }

                account.available += past_transaction.amount;
                account.held -= past_transaction.amount;
            }
            TransactionType::Chargeback => {
                // Get the past transaction ignoring the chargeback if it does not exist.
                let past_transaction =
                    match account.past_transactions.get_mut(&transaction.transaction) {
                        Some(v) => v,
                        None => return,
                    };

                // Ignore if not yet disputed.
                if !past_transaction.disputed {
                    return;
                }

                account.held -= past_transaction.amount;
                account.total -= past_transaction.amount;
                account.locked = true;
            }
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DepositTransactionState;

    #[test]
    pub fn test_deposit_and_withdrawal_transactions() {
        let client = 0;
        let mut app_state = AppState::new();

        app_state.process_transaction(Transaction {
            kind: TransactionType::Deposit,
            client,
            transaction: 0,
            amount: 10.0.into(),
        });

        app_state.process_transaction(Transaction {
            kind: TransactionType::Deposit,
            client,
            transaction: 1,
            amount: 20.0.into(),
        });

        // Ignored
        app_state.process_transaction(Transaction {
            kind: TransactionType::Withdrawal,
            client,
            transaction: 2,
            amount: 50.0.into(),
        });

        app_state.process_transaction(Transaction {
            kind: TransactionType::Withdrawal,
            client,
            transaction: 2,
            amount: 15.0.into(),
        });

        app_state.process_transaction(Transaction {
            kind: TransactionType::Withdrawal,
            client,
            transaction: 2,
            amount: 5.0.into(),
        });

        let account = app_state.client_accounts.get(&client).unwrap();

        assert_eq!(account.available, 10.0.into());
        assert_eq!(account.held, 0.0.into());
        assert_eq!(account.total, 10.0.into());
    }

    #[test]
    pub fn test_dispute_transactions() {
        let client = 0;
        let mut app_state = AppState::new();
        let account = app_state
            .client_accounts
            .entry(client)
            .or_insert_with(|| ClientAccount::new(client));

        account.available = 15.0.into();
        account.total = 15.0.into();
        account.past_transactions.insert(
            0,
            DepositTransactionState {
                amount: 20.0.into(),
                disputed: false,
            },
        );
        account.past_transactions.insert(
            1,
            DepositTransactionState {
                amount: 30.0.into(),
                disputed: true,
            },
        );

        // Dispute transaction 0.
        app_state.process_transaction(Transaction {
            kind: TransactionType::Dispute,
            client,
            transaction: 0,
            amount: 0.0.into(),
        });

        let account = app_state.client_accounts.get(&client).unwrap();

        assert_eq!(account.available, (-5.0).into());
        assert_eq!(account.held, 20.0.into());
        assert_eq!(account.total, 15.0.into());
        assert_eq!(
            account.past_transactions.get(&0).unwrap(),
            &DepositTransactionState {
                amount: 20.0.into(),
                disputed: true,
            }
        );
        assert_eq!(
            account.past_transactions.get(&1).unwrap(),
            &DepositTransactionState {
                amount: 30.0.into(),
                disputed: true,
            }
        );

        let account_clone = account.clone();

        // Skip disputing transaction 1.
        app_state.process_transaction(Transaction {
            kind: TransactionType::Dispute,
            client,
            transaction: 1,
            amount: 0.0.into(),
        });

        assert_eq!(
            app_state.client_accounts.get(&client).unwrap(),
            &account_clone
        );
    }

    #[test]
    pub fn test_resolve_transactions() {
        let client = 0;
        let mut app_state = AppState::new();
        let account = app_state
            .client_accounts
            .entry(client)
            .or_insert_with(|| ClientAccount::new(client));

        account.available = (-5.0).into();
        account.held = 20.0.into();
        account.total = 15.0.into();
        account.past_transactions.insert(
            0,
            DepositTransactionState {
                amount: 15.0.into(),
                disputed: true,
            },
        );
        account.past_transactions.insert(
            1,
            DepositTransactionState {
                amount: 30.0.into(),
                disputed: false,
            },
        );

        // Resolve transaction 0.
        app_state.process_transaction(Transaction {
            kind: TransactionType::Resolve,
            client,
            transaction: 0,
            amount: 0.0.into(),
        });

        let account = app_state.client_accounts.get(&client).unwrap();

        assert_eq!(account.available, 10.0.into());
        assert_eq!(account.held, 5.0.into());
        assert_eq!(account.total, 15.0.into());
        assert_eq!(
            account.past_transactions.get(&0).unwrap(),
            &DepositTransactionState {
                amount: 15.0.into(),
                disputed: true,
            }
        );
        assert_eq!(
            account.past_transactions.get(&1).unwrap(),
            &DepositTransactionState {
                amount: 30.0.into(),
                disputed: false,
            }
        );

        let account_clone = account.clone();

        // Skip resolving transaction 1.
        app_state.process_transaction(Transaction {
            kind: TransactionType::Resolve,
            client,
            transaction: 1,
            amount: 0.0.into(),
        });

        assert_eq!(
            app_state.client_accounts.get(&client).unwrap(),
            &account_clone
        );
    }

    #[test]
    pub fn test_chargeback_transactions() {
        let client = 0;
        let mut app_state = AppState::new();
        let account = app_state
            .client_accounts
            .entry(client)
            .or_insert_with(|| ClientAccount::new(client));

        account.available = (-5.0).into();
        account.held = 20.0.into();
        account.total = 15.0.into();
        account.past_transactions.insert(
            0,
            DepositTransactionState {
                amount: 20.0.into(),
                disputed: true,
            },
        );
        account.past_transactions.insert(
            1,
            DepositTransactionState {
                amount: 30.0.into(),
                disputed: false,
            },
        );

        // Charge back transaction 0.
        app_state.process_transaction(Transaction {
            kind: TransactionType::Chargeback,
            client,
            transaction: 0,
            amount: 0.0.into(),
        });

        let account = app_state.client_accounts.get(&client).unwrap();

        assert_eq!(account.available, (-5.0).into());
        assert_eq!(account.held, 0.0.into());
        assert_eq!(account.total, (-5.0).into());
        assert_eq!(
            account.past_transactions.get(&0).unwrap(),
            &DepositTransactionState {
                amount: 20.0.into(),
                disputed: true,
            }
        );
        assert_eq!(
            account.past_transactions.get(&1).unwrap(),
            &DepositTransactionState {
                amount: 30.0.into(),
                disputed: false,
            }
        );

        let account_clone = account.clone();

        // Skip transaction 1.
        app_state.process_transaction(Transaction {
            kind: TransactionType::Chargeback,
            client,
            transaction: 1,
            amount: 0.0.into(),
        });

        assert_eq!(
            app_state.client_accounts.get(&client).unwrap(),
            &account_clone
        );
    }

    #[test]
    pub fn test_locked_accounts_accept_deposits() {
        let client = 0;
        let mut app_state = AppState::new();
        let account = app_state
            .client_accounts
            .entry(client)
            .or_insert_with(|| ClientAccount::new(client));

        account.locked = true;

        // Process deposit transaction.
        app_state.process_transaction(Transaction {
            kind: TransactionType::Deposit,
            client,
            transaction: 0,
            amount: 50.5.into(),
        });

        let account = app_state.client_accounts.get(&client).unwrap();

        assert_eq!(account.available, 50.5.into());
        assert_eq!(account.held, 0.0.into());
        assert_eq!(account.total, 50.5.into());
    }

    #[test]
    pub fn test_locked_accounts_block_transactions() {
        let client = 0;
        let mut app_state = AppState::new();
        let account = app_state
            .client_accounts
            .entry(client)
            .or_insert_with(|| ClientAccount::new(client));

        account.available = 200.0.into();
        account.total = 200.0.into();
        account.locked = true;
        account.past_transactions.insert(
            8,
            DepositTransactionState {
                amount: 10.0.into(),
                disputed: false,
            },
        );
        account.past_transactions.insert(
            9,
            DepositTransactionState {
                amount: 10.0.into(),
                disputed: true,
            },
        );

        let account_clone = account.clone();

        // Process withdrawal transaction.
        app_state.process_transaction(Transaction {
            kind: TransactionType::Withdrawal,
            client,
            transaction: 10,
            amount: 10.0.into(),
        });

        assert_eq!(
            app_state.client_accounts.get(&client).unwrap(),
            &account_clone
        );

        // Process dispute transaction.
        app_state.process_transaction(Transaction {
            kind: TransactionType::Dispute,
            client,
            transaction: 8,
            amount: 0.0.into(),
        });

        assert_eq!(
            app_state.client_accounts.get(&client).unwrap(),
            &account_clone
        );

        // Process resolve transaction.
        app_state.process_transaction(Transaction {
            kind: TransactionType::Resolve,
            client,
            transaction: 9,
            amount: 0.0.into(),
        });

        assert_eq!(
            app_state.client_accounts.get(&client).unwrap(),
            &account_clone
        );

        // Process chargeback transaction.
        app_state.process_transaction(Transaction {
            kind: TransactionType::Chargeback,
            client,
            transaction: 9,
            amount: 0.0.into(),
        });

        assert_eq!(
            app_state.client_accounts.get(&client).unwrap(),
            &account_clone
        );
    }
}
