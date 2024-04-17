use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WalletResponse {
    balances: HashMap<String, f64>,
}
