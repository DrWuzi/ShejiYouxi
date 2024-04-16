use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ItemTypeID {
    #[serde(rename = "01bb38e1-da47-4e6a-9b3d-945fe4655707")]
    Agents,
    #[serde(rename = "f85cb6f7-33e5-4dc8-b609-ec7212301948")]
    Contracts,
    #[serde(rename = "d5f120f8-ff8c-4aac-92ea-f2b5acbe9475")]
    Sprays,
    #[serde(rename = "dd3bf334-87f3-40bd-b043-682a57a8dc3a")]
    GunBuddies,
    #[serde(rename = "3f296c07-64c3-494c-923b-fe692a4fa1bd")]
    Cards,
    #[serde(rename = "e7c63390-eda7-46e0-bb7a-a6abdacd2433")]
    Skins,
    #[serde(rename = "3ad1b2b2-acdb-4524-852f-954a76ddae0a")]
    SkinVariants,
    #[serde(rename = "de7caa6b-adf7-4588-bbd1-143831e786c6")]
    Titles,
}
