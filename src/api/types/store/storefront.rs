use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::common::ItemTypeID;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct StorefrontResponse {
    featured_bundle: FeaturedBundle,
    skins_panel_layout: SkinsPanelLayout,
    upgrade_currency_store: UpgradeCurrencyStore,
    accessory_store: AccessoryStore,
    bonus_store: Option<BonusStore>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FeaturedBundle {
    bundle: Bundle,
    bundles: Vec<Bundle>,
    bundle_remaining_duration_in_seconds: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bundle {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "DataAssetID")]
    data_asset_id: String,
    #[serde(rename = "CurrencyID")]
    currency_id: String,
    items: Vec<BundleItem>,
    item_offers: Option<Vec<BundleItemOffer>>,
    total_base_cost: Option<HashMap<String, f64>>,
    total_discounted_cost: Option<HashMap<String, f64>>,
    total_discount_percent: f64,
    duration_remaining_in_seconds: u64,
    wholesale_only: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BundleItem {
    item: Item,
    base_price: f64,
    #[serde(rename = "CurrencyID")]
    currency_id: String,
    discount_percent: f64,
    discounted_price: f64,
    is_promo_item: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    #[serde(rename = "ItemTypeID")]
    item_type_id: ItemTypeID,
    #[serde(rename = "ItemID")]
    item_id: String,
    amount: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BundleItemOffer {
    #[serde(rename = "BundleItemOfferID")]
    bundle_item_offer_id: String,
    offer: Offer,
    discount_percent: f64,
    discounted_cost: HashMap<String, f64>,
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
    item_type_id: ItemTypeID,
    #[serde(rename = "ItemID")]
    item_id: String,
    quantity: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkinsPanelLayout {
    single_item_offers: Vec<String>,
    single_item_store_offers: Vec<SingleItemStoreOffer>,
    single_item_offers_remaining_duration_in_seconds: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SingleItemStoreOffer {
    #[serde(rename = "OfferID")]
    offer_id: String,
    is_direct_purchase: bool,
    start_date: String,
    cost: HashMap<String, f64>,
    rewards: Vec<Reward>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpgradeCurrencyStore {
    upgrade_currency_offers: Vec<UpgradeCurrencyOffer>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpgradeCurrencyOffer {
    #[serde(rename = "OfferID")]
    offer_id: String,
    #[serde(rename = "StorefrontItemID")]
    storefront_item_id: String,
    offer: Offer,
    discounted_percent: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccessoryStore {
    accessory_store_offers: Vec<AccessoryStoreOffer>,
    accessory_store_remaining_duration_in_seconds: u64,
    #[serde(rename = "StorefrontID")]
    storefront_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccessoryStoreOffer {
    offer: Offer,
    #[serde(rename = "ContractID")]
    contract_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BonusStore {
    bonus_store_offers: Vec<BonusStoreOffer>,
    bonus_store_remaining_duration_in_seconds: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BonusStoreOffer {
    #[serde(rename = "BonusOfferID")]
    bonus_offer_id: String,
    offer: Offer,
    discount_percent: f64,
    discount_costs: HashMap<String, f64>,
    is_seen: bool,
}
