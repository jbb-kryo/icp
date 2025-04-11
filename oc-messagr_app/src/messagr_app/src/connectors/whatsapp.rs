use crate::{
    AuthConfig, Conversation, Message, MessageContent, User, 
    Attachment, Platform, Error, Result
};
use crate::auth::whatsapp;
use crate::storage::{conversations, messages};
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Initialize connection to WhatsApp
pub async fn init_connection(auth_config: &AuthConfig) -> Result<()> {
    // Verify token validity by making a test API call
    let business_profile = get_business_profile(auth_config).await?;
    ic_cdk::println!("Connected to WhatsApp Business: {}", business_profile.name);
    
    // Initialize conversations
    let contacts = get_contacts(auth_config).await?;
    store_contacts(auth_config, contacts).await?;
    
    Ok(())
}

// Sync messages from WhatsApp
pub async fn sync_messages(auth_config: &AuthConfig) -> Result<u64> {
    // Get all conversations
    let user_conversations = conversations::get_user_conversations(
        &ic_cdk::caller().to_string(), 
        Some(Platform::WhatsApp)
    );
    
    let mut total_synced = 0;
    
    // For each conversation, get recent messages
    for conversation in user_conversations {
        let wa_messages = get_conversation_messages(auth_config, &conversation.id).await?;
        
        // Store messages
        for msg in wa_messages {
            let message = whatsapp_message_to_message(msg, &conversation.id)?;
            messages::store_message(message)?;
            total_synced += 1;
        }
        
        // Update conversation with last message timestamp
        if let Some(latest) = wa_messages.first() {
            conversations::update_conversation_last_message(&conversation.id, latest.timestamp)?;
        }
    }
    
    Ok(total_synced)
}

// Get business profile
async fn get_business_profile(auth_config: &AuthConfig) -> Result<WhatsAppBusinessProfile> {
    // This would normally use HTTP outbound calls to the WhatsApp Business API
    // For demo purposes, we'll simulate the response
    
    let phone_number_id = auth_config.api_key.clone()
        .ok_or_else(|| Error::InvalidParameters("WhatsApp phone number ID is required".to_string()))?;
    
    Ok(WhatsAppBusinessProfile {
        id: phone_number_id,
        name: "Messagr Business".to_string(),
        description: Some("Messagr cross-platform messaging app demo".to_string()),
        vertical: Some("PROFESSIONAL_SERVICES".to_string()),
        about: Some("Available 24/7".to_string()),
        address: Some("123 Business St, Tech City".to_string()),
        email: Some("contact@messagr.example".to_string()),
        websites: Some(vec!["https://messagr.example".to_string()]),
        profile_picture_url: Some("https://platform-lookaside.fbsbx.com/platform/profilepic/?wa=123456789".to_string()),
    })
}

// Get contacts/conversations
async fn get_contacts(auth_config: &AuthConfig) -> Result<Vec<WhatsAppContact>> {
    // This would normally use HTTP outbound calls to the WhatsApp Business API
    // For demo purposes, we'll simulate the response
    
    Ok(vec![
        WhatsAppContact {
            wa_id: "12065550100".to_string(),
            phone_number: "+12065550100".to_string(),
            name: Some(WhatsAppProfileName {
                formatted_name: "Alice Johnson".to_string(),
                first_name: Some("Alice".to_string()),
                last_name: Some("Johnson".to_string()),
            }),
            profile: Some(WhatsAppProfile {
                status: Some("Hey there! I'm using WhatsApp.".to_string()),
                picture_url: Some("https://platform-lookaside.fbsbx.com/platform/profilepic/?wa=12065550100".to_string()),
            }),
        },
        WhatsAppContact {
            wa_id: "12065550101".to_string(),
            phone_number: "+12065550101".to_string(),
            name: Some(WhatsAppProfileName {
                formatted_name: "Bob Smith".to_string(),
                first_name: Some("Bob".to_string()),
                last_name: Some("Smith".to_string()),
            }),
            profile: Some(WhatsAppProfile {
                status: Some("Available".to_string()),
                picture_url: Some("https://platform-lookaside.fbsbx.com/platform/profilepic/?wa=12065550101".to_string()),
            }),
        },
    ])
}

// Store WhatsApp contacts as conversations
async fn store_contacts(
    auth_config: &AuthConfig,
    contacts: Vec<WhatsAppContact>,
) -> Result<()> {
    let caller = ic_cdk::caller();
    let business_profile = get_business_profile(auth_config).await?;
    
    for contact in contacts {
        // Create participants list
        let mut participants = Vec::new();
        
        // Add the business account
        participants.push(User {
            id: business_profile.id.clone(),
            name: business_profile.name.clone(),
            platform: Platform::WhatsApp,
            avatar_url: business_profile.profile_picture_url.clone(),
        });
        
        // Add the contact
        let contact_name = contact.name.as_ref()
            .map(|n| n.formatted_name.clone())
            .unwrap_or_else(|| contact.phone_number.clone());
        
        participants.push(User {
            id: contact.wa_id.clone(),
            name: contact_name.clone(),
            platform: Platform::WhatsApp,
            avatar_url: contact.profile.as_ref().and_then(|p| p.picture_url.clone()),
        });
        
        // Create conversation ID (format: whatsapp_business_id_contact_id)
        let conversation_id = format!("wa_{}_{}", business_profile.id, contact.wa_id);
        
        // Create conversation
        let conversation = Conversation {
            id: conversation_id,
            platform: Platform::WhatsApp,
            name: format!("Chat with {}", contact_name),
            participants,
            created_at: time(),
            last_message_at: None,
        };
        
        // Store conversation
        conversations::store_conversation(conversation)?;
    }
    
    Ok(())
}

// Get messages from a WhatsApp conversation
async fn get_conversation_messages(
    auth_config: &AuthConfig,
    conversation_id: &str,
) -> Result<Vec<WhatsAppMessage>> {
    // This would normally use HTTP outbound calls to the WhatsApp Business API
    // For demo purposes, we'll simulate the response
    
    // Parse conversation ID to get contact ID
    // Format: wa_business_id_contact_id
    let parts: Vec<&str> = conversation_id.split('_').collect();
    if parts.len() < 3 {
        return Err(Error::InvalidParameters(format!("Invalid conversation ID: {}", conversation_id)));
    }
    
    let contact_id = parts[2];
    
    Ok(vec![
        WhatsAppMessage {
            id: "wamid.abcd1234".to_string(),
            from: contact_id.to_string(),
            timestamp: 1672670400000, // Jan 2, 2023, 14:30 UTC in milliseconds
            type_field: "text".to_string(),
            text: Some(WhatsAppText {
                body: "Hello! Can you help me with my order?".to_string(),
            }),
            image: None,
            audio: None,
            document: None,
            video: None,
            location: None,
            contacts: None,
            interactive: None,
        },
        WhatsAppMessage {
            id: "wamid.efgh5678".to_string(),
            from: auth_config.api_key.clone().unwrap_or_default(),
            timestamp: 1672670700000, // Jan 2, 2023, 14:35 UTC in milliseconds
            type_field: "text".to_string(),
            text: Some(WhatsAppText {
                body: "Hi there! Yes, I'd be happy to help with your order. Could you please provide your order number?".to_string(),
            }),
            image: None,
            audio: None,
            document: None,
            video: None,
            location: None,
            contacts: None,
            interactive: None,
        },
        WhatsAppMessage {
            id: "wamid.ijkl9012".to_string(),
            from: contact_id.to_string(),
            timestamp: 1672671000000, // Jan 2, 2023, 14:40 UTC in milliseconds
            type_field: "image".to_string(),
            text: None,
            image: Some(WhatsAppMedia {
                id: "1234567890",
                mime_type: "image/jpeg".to_string(),
                sha256: "abcdef1234567890".to_string(),
                caption: Some("Here's a screenshot of my order confirmation".to_string()),
                url: Some("https://lookaside.fbsbx.com/whatsapp_business/attachments/12345.jpg".to_string()),
            }),
            audio: None,
            document: None,
            video: None,
            location: None,
            contacts: None,
            interactive: None,
        },
    ])
}

// Convert WhatsApp message to our domain model
fn whatsapp_message_to_message(msg: WhatsAppMessage, conversation_id: &str) -> Result<Message> {
    // Parse conversation ID to get business ID
    // Format: wa_business_id_contact_id
    let parts: Vec<&str> = conversation_id.split('_').collect();
    if parts.len() < 3 {
        return Err(Error::InvalidParameters(format!("Invalid conversation ID: {}", conversation_id)));
    }
    
    let business_id = parts[1];
    
    // Determine if the message is from the business or the contact
    let is_from_business = msg.from == business_id;
    
    // Create sender information
    let sender = User {
        id: msg.from.clone(),
        name: if is_from_business { 
            "Messagr Business".to_string() 
        } else { 
            format!("Contact {}", msg.from) 
        },
        platform: Platform::WhatsApp,
        avatar_url: None,
    };
    
    // Create message text and attachments
    let mut message_text = String::new();
    let mut attachments = Vec::new();
    
    match msg.type_field.as_str() {
        "text" => {
            if let Some(text) = &msg.text {
                message_text = text.body.clone();
            }
        },
        "image" => {
            if let Some(image) = &msg.image {
                message_text = image.caption.clone().unwrap_or_default();
                attachments.push(Attachment {
                    attachment_type: "image".to_string(),
                    url: image.url.clone(),
                    content: None,
                    name: Some("Image".to_string()),
                });
            }
        },
        "audio" => {
            if let Some(audio) = &msg.audio {
                attachments.push(Attachment {
                    attachment_type: "audio".to_string(),
                    url: audio.url.clone(),
                    content: None,
                    name: Some("Audio".to_string()),
                });
            }
        },
        "document" => {
            if let Some(document) = &msg.document {
                message_text = document.caption.clone().unwrap_or_default();
                attachments.push(Attachment {
                    attachment_type: "file".to_string(),
                    url: document.url.clone(),
                    content: None,
                    name: document.filename.clone(),
                });
            }
        },
        "video" => {
            if let Some(video) = &msg.video {
                message_text = video.caption.clone().unwrap_or_default();
                attachments.push(Attachment {
                    attachment_type: "video".to_string(),
                    url: video.url.clone(),
                    content: None,
                    name: Some("Video".to_string()),
                });
            }
        },
        "location" => {
            if let Some(location) = &msg.location {
                message_text = format!(
                    "Location: {} (Latitude: {}, Longitude: {})",
                    location.name.clone().unwrap_or_default(),
                    location.latitude,
                    location.longitude
                );
            }
        },
        _ => {
            message_text = format!("[Unsupported message type: {}]", msg.type_field);
        }
    }
    
    Ok(Message {
        id: msg.id.clone(),
        platform: Platform::WhatsApp,
        conversation_id: conversation_id.to_string(),
        sender,
        content: MessageContent {
            text: message_text,
            attachments,
        },
        timestamp: msg.timestamp,
        thread_id: None,
        reply_to: None,
        edited: false,
    })
}

// WhatsApp API response structures
#[derive(Debug, Serialize, Deserialize)]
struct WhatsAppBusinessProfile {
    id: String,
    name: String,
    description: Option<String>,
    vertical: Option<String>,
    about: Option<String>,
    address: Option<String>,
    email: Option<String>,
    websites: Option<Vec<String>>,
    profile_picture_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhatsAppContact {
    wa_id: String,
    phone_number: String,
    name: Option<WhatsAppProfileName>,
    profile: Option<WhatsAppProfile>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhatsAppProfileName {
    formatted_name: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhatsAppProfile {
    status: Option<String>,
    picture_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhatsAppMessage {
    id: String,
    from: String,
    timestamp: u64,
    #[serde(rename = "type")]
    type_field: String,
    text: Option<WhatsAppText>,
    image: Option<WhatsAppMedia>,
    audio: Option<WhatsAppMedia>,
    document: Option<WhatsAppDocument>,
    video: Option<WhatsAppMedia>,
    location: Option<WhatsAppLocation>,
    contacts: Option<Vec<WhatsAppContact>>,
    interactive: Option<WhatsAppInteractive>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhatsAppText {
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhatsAppMedia {
    id: String,
    mime_type: String,
    sha256: String,
    caption: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhatsAppDocument {
    id: String,
    mime_type: String,
    sha256: String,
    filename: Option<String>,
    caption: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhatsAppLocation {
    latitude: f64,
    longitude: f64,
    name: Option<String>,
    address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhatsAppInteractive {
    #[serde(rename = "type")]
    type_field: String,
    button_reply: Option<WhatsAppButtonReply>,
    list_reply: Option<WhatsAppListReply>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhatsAppButtonReply {
    id: String,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhatsAppListReply {
    id: String,
    title: String,
    description: Option<String>,
}