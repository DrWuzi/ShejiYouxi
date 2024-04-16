use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ContractResponse {
    version: u32,
    /// Player UUID
    subject: String,
    contracts: Vec<Contract>,
    processed_matches: Vec<ProcessedMatch>,
    /// UUID
    active_special_contract: String,
    missions: Vec<Mission>,
    mission_metadata: MissionMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Contract {
    #[serde(rename = "ContractDefinitionID")]
    contract_definition_id: String,
    contract_progression: ContractProgression,
    progression_level_reached: u32,
    progression_towards_next_level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ContractProgression {
    total_progression_earned: u32,
    total_progression_earned_version: u32,
    highest_rewarded_level: HashMap<String, RewardedLevel>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RewardedLevel {
    amount: u32,
    version: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ProcessedMatch {
    #[serde(rename = "ID")]
    id: String,
    /// Milliseconds since epoch
    start_time: u64,
    #[serde(rename = "XPGrants")]
    xp_grants: Option<XPGrants>,
    reward_grants: Option<HashMap<String, Value>>,
    mission_deltas: Option<HashMap<String, MissionDelta>>,
    contract_deltas: Option<HashMap<String, ContractDelta>>,
    could_progress_missions: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct XPGrants {
    game_played: u32,
    game_won: u32,
    round_played: u32,
    round_won: u32,
    missions: HashMap<String, Value>,
    modifier: Modifier,
    #[serde(rename = "NumAFKRounds")]
    num_afk_rounds: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Modifier {
    value: f32,
    base_multiplier_value: f32,
    modifiers: Vec<ModifierInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ModifierInfo {
    value: f32,
    name: ModifierName,
    base_only: bool,
}

#[derive(Debug, Serialize, Deserialize)]
enum ModifierName {
    #[serde(rename = "RESTRICTIONS_XP")]
    RestrictionsXP,
    #[serde(rename = "PREMIUM_CONTRACT_XP")]
    PremiumContractXP,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MissionDelta {
    #[serde(rename = "ID")]
    id: String,
    objectives: HashMap<String, u32>,
    objective_deltas: HashMap<String, ObjectiveDelta>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ObjectiveDelta {
    #[serde(rename = "ID")]
    id: String,
    progress_before: u32,
    progress_after: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ContractDelta {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TotalXPBefore")]
    total_xp_before: u32,
    #[serde(rename = "TotalXPAfter")]
    total_xp_after: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Mission {
    #[serde(rename = "ID")]
    id: String,
    objectives: HashMap<String, u32>,
    complete: bool,
    /// Date in ISO 8601 format
    expiration_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MissionMetadata {
    #[serde(rename = "NPECompleted")]
    npe_completed: bool,
    /// Date in ISO 8601 format
    weekly_checkpoint: String,
    /// Date in ISO 8601 format
    weekly_refill_time: String,
}
