use serde::{Deserialize, Serialize};

use super::common::ItemTypeID;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OwnedItemsResponse {
    pub entitlements_by_types: Vec<EntitlementsByType>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntitlementsByType {
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: ItemTypeID,
    pub entitlements: Vec<Entitlement>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entitlement {
    #[serde(rename = "TypeID")]
    pub type_id: String,
    #[serde(rename = "ItemID")]
    pub item_id: String,
    #[serde(default)]
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,
}
