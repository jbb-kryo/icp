pub mod telegram;
pub mod slack;
pub mod discord;
pub mod twitter;
pub mod facebook;
pub mod whatsapp;

use crate::{AuthConfig, Conversation, Message, Error, Result, User};
use ic_cdk::api::time;
use std::collections::HashMap;

// Common traits for platform connectors
#[async_trait::async_trait]
pub trait PlatformConnector {
    async fn init(&self, auth_config: &AuthConfig) -> Result<()>;
    async fn fetch_conversations(&self) -> Result<Vec<Conversation>>;
    async fn fetch_messages(&self, conversation_id: &str, limit: u64, before_id: Option<&str>) -> Result<Vec<Message>>;
    async fn send_message(&self, conversation_id: &str, text: &str) -> Result<Message>;
}