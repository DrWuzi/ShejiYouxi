use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemUpgradesResponse {
    pub definitions: Vec<ItemUpgradeDefinition>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemUpgradeDefinition {
    #[serde(rename = "ID")]
    pub id: String,
    pub item: Item,
    pub required_entitlement: Item,
    pub progression_schedule: ProgressionSchedule,
    pub reward_schedule: RewardSchedule,
    pub sidegrades: Option<Vec<Sidegrade>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    #[serde(rename = "ItemID")]
    pub item_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProgressionSchedule {
    pub name: String,
    #[serde(rename = "ProgressionCurrencyID")]
    pub progression_currency_id: String,
    pub progression_delta_per_level: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RewardSchedule {
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
    pub prerequisites: Option<serde_json::Value>,
    pub rewards_per_level: Option<Vec<RewardsPerLevel>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RewardsPerLevel {
    pub entitlement_rewards: Vec<EntitlementReward>,
    pub wallet_rewards: Option<serde_json::Value>,
    pub counter_rewards: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntitlementReward {
    pub amount: u32,
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    #[serde(rename = "ItemID")]
    pub item_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Sidegrade {
    #[serde(rename = "SidegradeID")]
    pub sidegrade_id: String,
    pub options: Vec<SidegradeOption>,
    pub prerequisites: Prerequisites,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SidegradeOption {
    #[serde(rename = "OptionID")]
    pub option_id: String,
    pub cost: Cost,
    pub rewards: Vec<EntitlementReward>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cost {
    pub wallet_costs: Vec<WalletCost>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WalletCost {
    #[serde(rename = "CurrencyID")]
    pub currency_id: String,
    pub amount_to_deduct: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Prerequisites {
    pub required_entitlements: Vec<Item>,
}
