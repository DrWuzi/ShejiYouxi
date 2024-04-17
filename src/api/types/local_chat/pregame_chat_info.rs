use serde::{Deserialize, Serialize};

use super::common::ChatType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatUIState {
    pub changed_since_hidden: bool,
    pub hidden: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatConversation {
    pub cid: String,
    pub direct_messages: bool,
    pub global_readership: bool,
    pub message_history: bool,
    pub mid: String,
    pub muted: bool,
    pub muted_restriction: bool,
    pub r#type: ChatType,
    pub ui_state: ChatUIState,
    pub unread_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PreGameChatInfoResponse {
    pub conversations: Vec<ChatConversation>,
}
