use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PricesResponse {
    offers: Vec<Offer>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Offer {
    #[serde(rename = "OfferID")]
    offer_id: String,
    is_direct_purchase: bool,
    start_date: String,
    cost: HashMap<String, f64>,
    rewards: Vec<Reward>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Reward {
    #[serde(rename = "ItemTypeID")]
    item_type_id: String,
    #[serde(rename = "ItemID")]
    item_id: String,
    quantity: u64,
}
