use crate::{
    AuthConfig, Conversation, Message, MessageContent, User, 
    Attachment, Platform, Error, Result
};
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use std::collections::HashMap;
use crate::storage::{conversations, messages};

// Initialize connection to Slack
pub async fn init_connection(auth_config: &AuthConfig) -> Result<()> {
    // Verify token validity by making a test API call
    let user_info = get_user_info(auth_config).await?;
    ic_cdk::println!("Connected to Slack as: {}", user_info.user.name);
    
    // Populate initial conversations
    let channels = get_channels(auth_config).await?;
    store_channels(auth_config, channels).await?;
    
    Ok(())
}

// Sync messages from Slack
pub async fn sync_messages(auth_config: &AuthConfig) -> Result<u64> {
    // Get all channels/conversations
    let user_conversations = get_user_conversations(auth_config).await?;
    
    let mut total_synced = 0;
    
    // For each conversation, get recent messages
    for conversation in user_conversations {
        let history = get_conversation_history(auth_config, &conversation.id).await?;
        
        // Store messages
        for msg in history.messages {
            let message = slack_message_to_message(msg, &conversation.id)?;
            messages::store_message(message)?;
            total_synced += 1;
        }
        
        // Update conversation with last message timestamp
        if let Some(latest_ts) = history.messages.first().map(|m| m.ts.clone()) {
            if let Ok(ts_millis) = parse_slack_timestamp(&latest_ts) {
                conversations::update_conversation_last_message(&conversation.id, ts_millis)?;
            }
        }
    }
    
    Ok(total_synced)
}

// Get user info
async fn get_user_info(auth_config: &AuthConfig) -> Result<SlackUserInfoResponse> {
    // This would normally use HTTP outbound calls to the Slack API
    // For demo purposes, we'll simulate the response
    
    Ok(SlackUserInfoResponse {
        ok: true,
        user: SlackUser {
            id: "U0123456789".to_string(),
            name: "messagr_user".to_string(),
            real_name: Some("Messagr User".to_string()),
            profile: SlackUserProfile {
                avatar_hash: "abc123".to_string(),
                image_72: Some("https://avatars.slack-edge.com/abc123_72.png".to_string()),
            },
        },
    })
}

// Get channels
async fn get_channels(auth_config: &AuthConfig) -> Result<SlackChannelsResponse> {
    // This would normally use HTTP outbound calls to the Slack API
    // For demo purposes, we'll simulate the response
    
    Ok(SlackChannelsResponse {
        ok: true,
        channels: vec![
            SlackChannel {
                id: "C0123456789".to_string(),
                name: "general".to_string(),
                is_channel: true,
                is_group: false,
                is_im: false,
                created: 1609459200, // January 1, 2021
                creator: "U0123456789".to_string(),
                is_archived: false,
                is_general: true,
                members: Some(vec!["U0123456789".to_string(), "U9876543210".to_string()]),
                topic: SlackChannelTopic {
                    value: "Company-wide announcements and work-based matters".to_string(),
                    creator: "U0123456789".to_string(),
                    last_set: 1609459200,
                },
                purpose: SlackChannelPurpose {
                    value: "This channel is for company-wide communication".to_string(),
                    creator: "U0123456789".to_string(),
                    last_set: 1609459200,
                },
            },
            SlackChannel {
                id: "C9876543210".to_string(),
                name: "random".to_string(),
                is_channel: true,
                is_group: false,
                is_im: false,
                created: 1609459200, // January 1, 2021
                creator: "U0123456789".to_string(),
                is_archived: false,
                is_general: false,
                members: Some(vec!["U0123456789".to_string(), "U9876543210".to_string()]),
                topic: SlackChannelTopic {
                    value: "Non-work banter and water cooler conversation".to_string(),
                    creator: "U0123456789".to_string(),
                    last_set: 1609459200,
                },
                purpose: SlackChannelPurpose {
                    value: "A place for non-work-related flimflam".to_string(),
                    creator: "U0123456789".to_string(),
                    last_set: 1609459200,
                },
            },
        ],
    })
}

// Store channels as conversations
async fn store_channels(auth_config: &AuthConfig, channels_response: SlackChannelsResponse) -> Result<()> {
    let caller = ic_cdk::caller();
    
    for channel in channels_response.channels {
        // Get channel members (in a real implementation, we'd fetch user details)
        let members = channel.members.unwrap_or_default();
        let mut participants = Vec::new();
        
        for member_id in members {
            participants.push(User {
                id: member_id.clone(),
                name: format!("User {}", member_id),
                platform: Platform::Slack,
                avatar_url: None,
            });
        }
        
        // Add the current user if not already in participants
        if !participants.iter().any(|p| p.id.starts_with(&caller.to_string())) {
            participants.push(User {
                id: caller.to_string(),
                name: "Current User".to_string(),
                platform: Platform::Slack,
                avatar_url: None,
            });
        }
        
        // Create conversation
        let conversation = Conversation {
            id: channel.id.clone(),
            platform: Platform::Slack,
            name: format!("#{}", channel.name),
            participants,
            created_at: (channel.created as u64) * 1000, // Convert to milliseconds
            last_message_at: None,
        };
        
        // Store conversation
        conversations::store_conversation(conversation)?;
    }
    
    Ok(())
}

// Get conversation history
async fn get_conversation_history(auth_config: &AuthConfig, channel_id: &str) -> Result<SlackHistoryResponse> {
    // This would normally use HTTP outbound calls to the Slack API
    // For demo purposes, we'll simulate the response
    
    Ok(SlackHistoryResponse {
        ok: true,
        messages: vec![
            SlackMessage {
                type_field: "message".to_string(),
                user: Some("U0123456789".to_string()),
                text: "Hello world!".to_string(),
                ts: "1609459200.000100".to_string(),
                thread_ts: None,
                reply_count: None,
                replies: None,
                attachments: None,
            },
            SlackMessage {
                type_field: "message".to_string(),
                user: Some("U9876543210".to_string()),
                text: "Hi there!".to_string(),
                ts: "1609459300.000200".to_string(),
                thread_ts: None,
                reply_count: None,
                replies: None,
                attachments: None,
            },
        ],
        has_more: false,
    })
}

// Get all conversations/channels the user is part of
async fn get_user_conversations(auth_config: &AuthConfig) -> Result<Vec<Conversation>> {
    // In a real implementation, we'd make API calls
    // For demo purposes, we'll use our stored conversations
    
    let caller = ic_cdk::caller();
    Ok(conversations::get_user_conversations(&caller.to_string(), Some(Platform::Slack)))
}

// Parse Slack timestamp (e.g., "1609459200.000100") to milliseconds
fn parse_slack_timestamp(ts: &str) -> Result<u64> {
    let parts: Vec<&str> = ts.split('.').collect();
    if parts.len() != 2 {
        return Err(Error::InternalError(format!("Invalid Slack timestamp: {}", ts)));
    }
    
    let seconds = parts[0].parse::<u64>()
        .map_err(|e| Error::InternalError(format!("Invalid seconds in timestamp: {}", e)))?;
    
    let microseconds = parts[1].parse::<u64>()
        .map_err(|e| Error::InternalError(format!("Invalid microseconds in timestamp: {}", e)))?;
    
    Ok(seconds * 1000 + microseconds / 1000)
}

// Convert Slack message to our domain model
fn slack_message_to_message(msg: SlackMessage, channel_id: &str) -> Result<Message> {
    let user_id = msg.user.clone().unwrap_or_else(|| "unknown".to_string());
    
    let message_id = msg.ts.clone();
    let timestamp = parse_slack_timestamp(&msg.ts)?;
    
    let mut attachments = Vec::new();
    if let Some(slack_attachments) = msg.attachments {
        for attachment in slack_attachments {
            attachments.push(Attachment {
                attachment_type: "link".to_string(),
                url: Some(attachment.fallback),
                content: None,
                name: Some(attachment.title.unwrap_or_default()),
            });
        }
    }
    
    let sender = User {
        id: user_id.clone(),
        name: format!("User {}", user_id),
        platform: Platform::Slack,
        avatar_url: None,
    };
    
    Ok(Message {
        id: message_id,
        platform: Platform::Slack,
        conversation_id: channel_id.to_string(),
        sender,
        content: MessageContent {
            text: msg.text,
            attachments,
        },
        timestamp,
        thread_id: msg.thread_ts,
        reply_to: None, // Would need to parse replies in a real implementation
        edited: false,
    })
}

// Slack API response structures
#[derive(Debug, Serialize, Deserialize)]
struct SlackUserInfoResponse {
    ok: bool,
    user: SlackUser,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackUser {
    id: String,
    name: String,
    real_name: Option<String>,
    profile: SlackUserProfile,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackUserProfile {
    avatar_hash: String,
    image_72: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackChannelsResponse {
    ok: bool,
    channels: Vec<SlackChannel>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackChannel {
    id: String,
    name: String,
    is_channel: bool,
    is_group: bool,
    is_im: bool,
    created: u32,
    creator: String,
    is_archived: bool,
    is_general: bool,
    members: Option<Vec<String>>,
    topic: SlackChannelTopic,
    purpose: SlackChannelPurpose,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackChannelTopic {
    value: String,
    creator: String,
    last_set: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackChannelPurpose {
    value: String,
    creator: String,
    last_set: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackHistoryResponse {
    ok: bool,
    messages: Vec<SlackMessage>,
    has_more: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackMessage {
    #[serde(rename = "type")]
    type_field: String,
    user: Option<String>,
    text: String,
    ts: String,
    thread_ts: Option<String>,
    reply_count: Option<u32>,
    replies: Option<Vec<SlackReply>>,
    attachments: Option<Vec<SlackAttachment>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackReply {
    user: String,
    ts: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackAttachment {
    fallback: String,
    title: Option<String>,
    text: Option<String>,
    image_url: Option<String>,
    thumb_url: Option<String>,
}