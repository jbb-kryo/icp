use crate::{
    AuthConfig, Conversation, Message, MessageContent, User, 
    Attachment, Platform, Error, Result
};
use crate::auth::twitter;
use crate::storage::{conversations, messages};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Initialize connection to Twitter
pub async fn init_connection(auth_config: &AuthConfig) -> Result<()> {
    // Verify token validity by making a test API call
    let user_info = get_user_info(auth_config).await?;
    ic_cdk::println!("Connected to Twitter as: @{}", user_info.screen_name);
    
    // Initialize conversations (direct messages)
    let dms = get_direct_messages(auth_config).await?;
    store_direct_messages(auth_config, dms).await?;
    
    Ok(())
}

// Sync messages from Twitter
pub async fn sync_messages(auth_config: &AuthConfig) -> Result<u64> {
    // Get all conversations
    let user_conversations = conversations::get_user_conversations(
        &ic_cdk::caller().to_string(), 
        Some(Platform::Twitter)
    );
    
    let mut total_synced = 0;
    
    // For each conversation, get recent messages
    for conversation in user_conversations {
        let timeline = match conversation.name.starts_with("DM:") {
            true => get_direct_message_events(auth_config, &conversation.id).await?,
            false => get_timeline_tweets(auth_config, &conversation.id).await?,
        };
        
        // Store messages
        for tweet in timeline {
            let message = twitter_message_to_message(tweet, &conversation.id)?;
            messages::store_message(message)?;
            total_synced += 1;
        }
        
        // Update conversation with last message timestamp if available
        if let Some(latest) = timeline.first() {
            if let Some(timestamp) = latest.created_at_millis {
                conversations::update_conversation_last_message(&conversation.id, timestamp)?;
            }
        }
    }
    
    Ok(total_synced)
}

// Get user info from Twitter
async fn get_user_info(auth_config: &AuthConfig) -> Result<TwitterUserResponse> {
    // This would normally use HTTP outbound calls to the Twitter API with OAuth 1.0a
    // For demo purposes, we'll simulate the response
    
    Ok(TwitterUserResponse {
        id: 12345678,
        id_str: "12345678".to_string(),
        name: "Messagr User".to_string(),
        screen_name: "messagr_user".to_string(),
        location: Some("Internet Computer".to_string()),
        description: Some("Testing the Messagr app for ICP".to_string()),
        verified: false,
        followers_count: 42,
        friends_count: 100,
        profile_image_url_https: Some("https://pbs.twimg.com/profile_images/default_image.png".to_string()),
    })
}

// Get direct messages
async fn get_direct_messages(auth_config: &AuthConfig) -> Result<Vec<TwitterDirectMessageEvent>> {
    // This would normally use HTTP outbound calls to the Twitter API
    // For demo purposes, we'll simulate the response
    
    Ok(vec![
        TwitterDirectMessageEvent {
            id: "1234567890".to_string(),
            created_timestamp: "1609459200000".to_string(),
            created_at_millis: Some(1609459200000),
            message_create: TwitterDirectMessageCreate {
                target: TwitterMessageTarget {
                    recipient_id: "987654321".to_string(),
                },
                sender_id: "12345678".to_string(),
                message_data: TwitterMessageData {
                    text: "Hello, this is a direct message!".to_string(),
                    entities: None,
                    attachment: None,
                },
            },
        },
    ])
}

// Store direct messages as conversations
async fn store_direct_messages(
    auth_config: &AuthConfig,
    dms: Vec<TwitterDirectMessageEvent>,
) -> Result<()> {
    let caller = ic_cdk::caller();
    let caller_info = get_user_info(auth_config).await?;
    
    // Extract unique conversation partners
    let mut conversation_partners = HashMap::new();
    for dm in &dms {
        let partner_id = if dm.message_create.sender_id == caller_info.id_str {
            dm.message_create.target.recipient_id.clone()
        } else {
            dm.message_create.sender_id.clone()
        };
        
        // In a real implementation, we'd fetch user details for each partner
        // For now, just use IDs with a placeholder name
        conversation_partners.insert(partner_id.clone(), format!("User {}", partner_id));
    }
    
    // Create conversations for each partner
    for (partner_id, partner_name) in conversation_partners {
        // Create conversation
        let conversation_id = format!("dm-{}-{}", caller_info.id_str, partner_id);
        let conversation = Conversation {
            id: conversation_id,
            platform: Platform::Twitter,
            name: format!("DM: @{}", partner_name),
            participants: vec![
                User {
                    id: caller_info.id_str.clone(),
                    name: caller_info.name.clone(),
                    platform: Platform::Twitter,
                    avatar_url: caller_info.profile_image_url_https.clone(),
                },
                User {
                    id: partner_id.clone(),
                    name: partner_name,
                    platform: Platform::Twitter,
                    avatar_url: None,
                },
            ],
            created_at: time(),
            last_message_at: None,
        };
        
        // Store conversation
        conversations::store_conversation(conversation)?;
    }
    
    Ok(())
}

// Get timeline tweets
async fn get_timeline_tweets(auth_config: &AuthConfig, timeline_id: &str) -> Result<Vec<TwitterMessage>> {
    // This would normally use HTTP outbound calls to the Twitter API
    // For demo purposes, we'll simulate the response
    
    Ok(vec![
        TwitterMessage {
            id: "1234567890".to_string(),
            id_str: "1234567890".to_string(),
            text: "Hello, this is a tweet!".to_string(),
            created_at: "Wed Apr 15 00:00:00 +0000 2020".to_string(),
            created_at_millis: Some(1586908800000),
            user: Some(TwitterUser {
                id: 12345678,
                id_str: "12345678".to_string(),
                name: "Messagr User".to_string(),
                screen_name: "messagr_user".to_string(),
                profile_image_url_https: Some("https://pbs.twimg.com/profile_images/default_image.png".to_string()),
            }),
            in_reply_to_status_id_str: None,
            entities: None,
            extended_entities: None,
        },
    ])
}

// Get direct message events
async fn get_direct_message_events(
    auth_config: &AuthConfig, 
    conversation_id: &str
) -> Result<Vec<TwitterMessage>> {
    // This would fetch DMs and convert them to our common format
    let dm_events = get_direct_messages(auth_config).await?;
    
    // Convert DM events to the common TwitterMessage format
    let messages = dm_events
        .into_iter()
        .filter(|dm| {
            // Extract conversation ID format: "dm-{user1}-{user2}"
            let parts: Vec<&str> = conversation_id.split('-').collect();
            if parts.len() != 3 {
                return false;
            }
            
            let user1 = parts[1];
            let user2 = parts[2];
            
            // Check if this DM is part of the conversation
            (dm.message_create.sender_id == user1 && dm.message_create.target.recipient_id == user2) ||
            (dm.message_create.sender_id == user2 && dm.message_create.target.recipient_id == user1)
        })
        .map(|dm| {
            // We would normally fetch user info for the sender
            // Here using placeholder data
            let timestamp = dm.created_timestamp.parse::<u64>().unwrap_or(0);
            
            TwitterMessage {
                id: dm.id.clone(),
                id_str: dm.id,
                text: dm.message_create.message_data.text,
                created_at: "".to_string(), // Not used for DMs
                created_at_millis: Some(timestamp),
                user: Some(TwitterUser {
                    id: 0,
                    id_str: dm.message_create.sender_id,
                    name: "Twitter User".to_string(),
                    screen_name: "twitter_user".to_string(),
                    profile_image_url_https: None,
                }),
                in_reply_to_status_id_str: None,
                entities: None,
                extended_entities: None,
            }
        })
        .collect();
    
    Ok(messages)
}

// Convert Twitter message to our domain model
fn twitter_message_to_message(tweet: TwitterMessage, conversation_id: &str) -> Result<Message> {
    let user = tweet.user.ok_or_else(|| {
        Error::InternalError("Tweet missing user information".to_string())
    })?;
    
    let mut attachments = Vec::new();
    
    // Process media entities if available
    if let Some(entities) = &tweet.extended_entities {
        if let Some(media) = &entities.media {
            for m in media {
                attachments.push(Attachment {
                    attachment_type: "image".to_string(),
                    url: Some(m.media_url_https.clone()),
                    content: None,
                    name: Some(format!("Media {}", m.id_str)),
                });
            }
        }
    }
    
    // Get timestamp
    let timestamp = tweet.created_at_millis.unwrap_or_else(|| {
        // Parse Twitter's date format if milliseconds not available
        // In a real implementation we'd parse the date string properly
        ic_cdk::api::time() // Fallback to current time
    });
    
    let sender = User {
        id: user.id_str.clone(),
        name: format!("{} (@{})", user.name, user.screen_name),
        platform: Platform::Twitter,
        avatar_url: user.profile_image_url_https,
    };
    
    Ok(Message {
        id: tweet.id_str.clone(),
        platform: Platform::Twitter,
        conversation_id: conversation_id.to_string(),
        sender,
        content: MessageContent {
            text: tweet.text,
            attachments,
        },
        timestamp,
        thread_id: None,
        reply_to: tweet.in_reply_to_status_id_str,
        edited: false,
    })
}

// Twitter API response structures
#[derive(Debug, Serialize, Deserialize)]
struct TwitterUserResponse {
    id: i64,
    id_str: String,
    name: String,
    screen_name: String,
    location: Option<String>,
    description: Option<String>,
    verified: bool,
    followers_count: i32,
    friends_count: i32,
    profile_image_url_https: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterUser {
    id: i64,
    id_str: String,
    name: String,
    screen_name: String,
    profile_image_url_https: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterEntities {
    hashtags: Option<Vec<TwitterHashtag>>,
    urls: Option<Vec<TwitterUrl>>,
    user_mentions: Option<Vec<TwitterMention>>,
    media: Option<Vec<TwitterMedia>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterHashtag {
    text: String,
    indices: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterUrl {
    url: String,
    expanded_url: String,
    display_url: String,
    indices: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterMention {
    screen_name: String,
    name: String,
    id: i64,
    id_str: String,
    indices: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterMedia {
    id: i64,
    id_str: String,
    indices: Vec<i32>,
    media_url: String,
    media_url_https: String,
    url: String,
    display_url: String,
    expanded_url: String,
    #[serde(rename = "type")]
    media_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterMessage {
    id: String,
    id_str: String,
    text: String,
    created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at_millis: Option<u64>,
    user: Option<TwitterUser>,
    in_reply_to_status_id_str: Option<String>,
    entities: Option<TwitterEntities>,
    extended_entities: Option<TwitterEntities>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterDirectMessageEvent {
    id: String,
    created_timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at_millis: Option<u64>,
    message_create: TwitterDirectMessageCreate,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterDirectMessageCreate {
    target: TwitterMessageTarget,
    sender_id: String,
    message_data: TwitterMessageData,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterMessageTarget {
    recipient_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterMessageData {
    text: String,
    entities: Option<TwitterEntities>,
    attachment: Option<TwitterMessageAttachment>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterMessageAttachment {
    #[serde(rename = "type")]
    attachment_type: String,
    media: Option<TwitterAttachmentMedia>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitterAttachmentMedia {
    id: i64,
    id_str: String,
    media_url: String,
    media_url_https: String,
    url: String,
    #[serde(rename = "type")]
    media_type: String,
}