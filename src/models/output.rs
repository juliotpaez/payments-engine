use crate::models::{ClientAccount, ClientId};
use serde::{Deserialize, Serialize};

/// The representation of a line in the output csv.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LineOutput {
    pub client: ClientId,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

impl LineOutput {
    // CONSTRUCTORS -----------------------------------------------------------

    /// Builds a new [LineOutput] from a [ClientAccount].
    pub fn from_account(account: &ClientAccount) -> Self {
        Self {
            client: account.client,
            available: account.available.into(),
            held: account.held.into(),
            total: account.total.into(),
            locked: account.locked,
        }
    }
}
