use crate::{
    AuthConfig, Conversation, Message, MessageContent, User, 
    Attachment, Platform, Error, Result
};
use crate::auth::facebook;
use crate::storage::{conversations, messages};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Initialize connection to Facebook Messenger
pub async fn init_connection(auth_config: &AuthConfig) -> Result<()> {
    // Verify token validity by making a test API call
    let page_info = get_page_info(auth_config).await?;
    ic_cdk::println!("Connected to Facebook Page: {}", page_info.name);
    
    // Initialize conversations (Messenger threads)
    let conversations = get_conversations(auth_config).await?;
    store_conversations(auth_config, conversations).await?;
    
    Ok(())
}

// Sync messages from Facebook Messenger
pub async fn sync_messages(auth_config: &AuthConfig) -> Result<u64> {
    // Get all conversations
    let user_conversations = conversations::get_user_conversations(
        &ic_cdk::caller().to_string(), 
        Some(Platform::Facebook)
    );
    
    let mut total_synced = 0;
    
    // For each conversation, get recent messages
    for conversation in user_conversations {
        let fb_messages = get_conversation_messages(auth_config, &conversation.id).await?;
        
        // Store messages
        for msg in fb_messages {
            let message = facebook_message_to_message(msg, &conversation.id)?;
            messages::store_message(message)?;
            total_synced += 1;
        }
        
        // Update conversation with last message timestamp
        if let Some(latest) = fb_messages.first() {
            conversations::update_conversation_last_message(&conversation.id, latest.timestamp)?;
        }
    }
    
    Ok(total_synced)
}

// Get page info
async fn get_page_info(auth_config: &AuthConfig) -> Result<FacebookPage> {
    // This would normally use HTTP outbound calls to the Facebook Graph API
    // For demo purposes, we'll simulate the response
    
    Ok(FacebookPage {
        id: "123456789012345".to_string(),
        name: "Messagr Demo Page".to_string(),
        category: "App Page".to_string(),
        access_token: "PAGE_ACCESS_TOKEN".to_string(),
        tasks: vec!["ANALYZE".to_string(), "ADVERTISE".to_string(), "MESSAGING".to_string()],
        picture: Some(FacebookPicture {
            url: "https://platform-lookaside.fbsbx.com/platform/profilepic/?id=123456789012345".to_string(),
        }),
    })
}

// Get conversations (threads) from Facebook
async fn get_conversations(auth_config: &AuthConfig) -> Result<Vec<FacebookConversation>> {
    // This would normally use HTTP outbound calls to the Facebook Graph API
    // For demo purposes, we'll simulate the response
    
    Ok(vec![
        FacebookConversation {
            id: "t_123456789012345".to_string(),
            updated_time: "2023-01-01T12:00:00+0000".to_string(),
            link: "https://www.messenger.com/t/123456789012345".to_string(),
            name: None,
            message_count: 42,
            participants: FacebookParticipantData {
                data: vec![
                    FacebookParticipant {
                        id: "987654321098765".to_string(),
                        name: "John Smith".to_string(),
                        email: None,
                        profile_pic: Some("https://platform-lookaside.fbsbx.com/platform/profilepic/?id=987654321098765".to_string()),
                    },
                    FacebookParticipant {
                        id: "123456789012345".to_string(),
                        name: "Messagr Demo Page".to_string(),
                        email: None,
                        profile_pic: Some("https://platform-lookaside.fbsbx.com/platform/profilepic/?id=123456789012345".to_string()),
                    },
                ],
            },
            scoped_thread_key: Some("t_123456789012345".to_string()),
        },
        FacebookConversation {
            id: "t_567890123456789".to_string(),
            updated_time: "2023-01-02T14:30:00+0000".to_string(),
            link: "https://www.messenger.com/t/567890123456789".to_string(),
            name: Some("Marketing Team".to_string()),
            message_count: 128,
            participants: FacebookParticipantData {
                data: vec![
                    FacebookParticipant {
                        id: "111222333444555".to_string(),
                        name: "Jane Doe".to_string(),
                        email: None,
                        profile_pic: Some("https://platform-lookaside.fbsbx.com/platform/profilepic/?id=111222333444555".to_string()),
                    },
                    FacebookParticipant {
                        id: "555444333222111".to_string(),
                        name: "Bob Johnson".to_string(),
                        email: None,
                        profile_pic: Some("https://platform-lookaside.fbsbx.com/platform/profilepic/?id=555444333222111".to_string()),
                    },
                    FacebookParticipant {
                        id: "123456789012345".to_string(),
                        name: "Messagr Demo Page".to_string(),
                        email: None,
                        profile_pic: Some("https://platform-lookaside.fbsbx.com/platform/profilepic/?id=123456789012345".to_string()),
                    },
                ],
            },
            scoped_thread_key: Some("t_567890123456789".to_string()),
        },
    ])
}

// Store Facebook conversations
async fn store_conversations(
    auth_config: &AuthConfig,
    fb_conversations: Vec<FacebookConversation>,
) -> Result<()> {
    let caller = ic_cdk::caller();
    let page_info = get_page_info(auth_config).await?;
    
    for fb_conv in fb_conversations {
        // Create participants list
        let mut participants = Vec::new();
        
        for participant in fb_conv.participants.data {
            participants.push(User {
                id: participant.id.clone(),
                name: participant.name.clone(),
                platform: Platform::Facebook,
                avatar_url: participant.profile_pic,
            });
        }
        
        // Make sure page is included in participants
        if !participants.iter().any(|p| p.id == page_info.id) {
            participants.push(User {
                id: page_info.id.clone(),
                name: page_info.name.clone(),
                platform: Platform::Facebook,
                avatar_url: page_info.picture.as_ref().map(|p| p.url.clone()),
            });
        }
        
        // Parse the updated time to get a timestamp
        // In a real implementation, we'd use a proper date parsing
        // Here we're using a simple estimate
        let timestamp = time();
        
        // Create conversation
        let conversation = Conversation {
            id: fb_conv.id.clone(),
            platform: Platform::Facebook,
            name: fb_conv.name.unwrap_or_else(|| {
                // For direct conversations, use the other person's name
                for participant in &fb_conv.participants.data {
                    if participant.id != page_info.id {
                        return format!("Chat with {}", participant.name);
                    }
                }
                "Facebook Conversation".to_string()
            }),
            participants,
            created_at: timestamp,
            last_message_at: Some(timestamp),
        };
        
        // Store conversation
        conversations::store_conversation(conversation)?;
    }
    
    Ok(())
}

// Get messages from a Facebook conversation
async fn get_conversation_messages(
    auth_config: &AuthConfig,
    conversation_id: &str,
) -> Result<Vec<FacebookMessage>> {
    // This would normally use HTTP outbound calls to the Facebook Graph API
    // For demo purposes, we'll simulate the response
    
    Ok(vec![
        FacebookMessage {
            id: "m_123456789012345".to_string(),
            message: "Hello, this is a test message!".to_string(),
            created_time: "2023-01-02T14:30:00+0000".to_string(),
            timestamp: 1672670400000, // Jan 2, 2023, 14:30 UTC in milliseconds
            from: FacebookMessageSender {
                id: "987654321098765".to_string(),
                name: "John Smith".to_string(),
                email: None,
            },
            attachments: Some(FacebookAttachmentData {
                data: vec![
                    FacebookAttachment {
                        id: "123456789".to_string(),
                        mime_type: "image/jpeg".to_string(),
                        name: Some("photo.jpg".to_string()),
                        size: Some(12345),
                        image_data: Some(FacebookImageData {
                            width: 800,
                            height: 600,
                            render_as_sticker: false,
                            url: "https://scontent.xx.fbcdn.net/v/image.jpg".to_string(),
                        }),
                        video_data: None,
                        file_url: Some("https://scontent.xx.fbcdn.net/v/image.jpg".to_string()),
                    },
                ],
            }),
        },
        FacebookMessage {
            id: "m_987654321098765".to_string(),
            message: "Thanks for your message! How can I help you today?".to_string(),
            created_time: "2023-01-02T14:35:00+0000".to_string(),
            timestamp: 1672670700000, // Jan 2, 2023, 14:35 UTC in milliseconds
            from: FacebookMessageSender {
                id: "123456789012345".to_string(),
                name: "Messagr Demo Page".to_string(),
                email: None,
            },
            attachments: None,
        },
    ])
}

// Convert Facebook message to our domain model
fn facebook_message_to_message(msg: FacebookMessage, conversation_id: &str) -> Result<Message> {
    let sender = User {
        id: msg.from.id.clone(),
        name: msg.from.name.clone(),
        platform: Platform::Facebook,
        avatar_url: Some(format!("https://graph.facebook.com/{}/picture", msg.from.id)),
    };
    
    let mut attachments = Vec::new();
    
    // Process attachments if available
    if let Some(attachment_data) = msg.attachments {
        for attachment in attachment_data.data {
            let attachment_type = if attachment.mime_type.starts_with("image/") {
                "image"
            } else if attachment.mime_type.starts_with("video/") {
                "video"
            } else {
                "file"
            };
            
            attachments.push(Attachment {
                attachment_type: attachment_type.to_string(),
                url: attachment.file_url.clone(),
                content: None,
                name: attachment.name.clone(),
            });
        }
    }
    
    Ok(Message {
        id: msg.id.clone(),
        platform: Platform::Facebook,
        conversation_id: conversation_id.to_string(),
        sender,
        content: MessageContent {
            text: msg.message,
            attachments,
        },
        timestamp: msg.timestamp,
        thread_id: None,
        reply_to: None,
        edited: false,
    })
}

// Facebook API response structures
#[derive(Debug, Serialize, Deserialize)]
struct FacebookPage {
    id: String,
    name: String,
    category: String,
    access_token: String,
    tasks: Vec<String>,
    picture: Option<FacebookPicture>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FacebookPicture {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FacebookConversation {
    id: String,
    updated_time: String,
    link: String,
    name: Option<String>,
    message_count: i32,
    participants: FacebookParticipantData,
    scoped_thread_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FacebookParticipantData {
    data: Vec<FacebookParticipant>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FacebookParticipant {
    id: String,
    name: String,
    email: Option<String>,
    profile_pic: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FacebookMessage {
    id: String,
    message: String,
    created_time: String,
    timestamp: u64,
    from: FacebookMessageSender,
    attachments: Option<FacebookAttachmentData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FacebookMessageSender {
    id: String,
    name: String,
    email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FacebookAttachmentData {
    data: Vec<FacebookAttachment>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FacebookAttachment {
    id: String,
    mime_type: String,
    name: Option<String>,
    size: Option<i32>,
    image_data: Option<FacebookImageData>,
    video_data: Option<FacebookVideoData>,
    file_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FacebookImageData {
    width: i32,
    height: i32,
    render_as_sticker: bool,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FacebookVideoData {
    width: i32,
    height: i32,
    url: String,
    preview_url: String,
}