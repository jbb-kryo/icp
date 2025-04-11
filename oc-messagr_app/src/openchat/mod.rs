mod client;
mod types;
mod query;
mod embeddings;

use crate::{Message, Conversation, QueryResult, Error, Result, Platform};
use openchat_sdk::{OpenChatClient, QueryResponse, MessageContent, QueryRequest as OCQueryRequest};
use ic_cdk::api::{call, management_canister::main::{CanisterIdRecord, CanisterStatusResponse, canister_status}};
use ic_cdk::api::time;
use ic_cdk::trap;
use candid::Principal;
use std::collections::HashMap;

// OpenChat Community canister ID on mainnet
const OPENCHAT_COMMUNITY_CANISTER_ID: &str = "xomae-vyaaa-aaaaq-aabhq-cai";

// AI-assisted query interface with OpenChat SDK
pub async fn query_with_openchat(
    query_text: &str,
    filter_platform: Option<Platform>,
    user_id: &str,
    limit: usize
) -> Result<QueryResult> {
    // Create an OpenChat client
    let client = client::create_openchat_client().await?;
    
    // Get messages filtered by platform if specified
    let messages = if let Some(platform) = filter_platform {
        crate::storage::messages::get_messages_by_platform(&platform, 1000)?
    } else {
        crate::storage::messages::get_all_messages()?
    };
    
    // Filter messages by user access
    let user_conversations = crate::storage::conversations::get_user_conversations(user_id, None);
    let user_conversation_ids: Vec<String> = user_conversations.iter()
        .map(|c| c.id.clone())
        .collect();
    
    let accessible_messages: Vec<Message> = messages.into_iter()
        .filter(|m| user_conversation_ids.contains(&m.conversation_id))
        .collect();
    
    // Format messages for OpenChat SDK
    let formatted_messages = format_messages_for_openchat(&accessible_messages);
    
    // Prepare query with context window optimization
    let optimized_messages = optimize_context_window(formatted_messages, query_text);
    
    // Build the query request
    let request = OCQueryRequest {
        query: query_text.to_string(),
        sources: optimized_messages,
        max_responses: limit,
        conversation_id: None,
    };
    
    // Execute the query using OpenChat SDK
    let response = client.query(request).await?;
    
    // Process the response
    process_query_response(response, &accessible_messages, query_text)
}

// Format messages for the OpenChat SDK
fn format_messages_for_openchat(messages: &[Message]) -> Vec<MessageContent> {
    messages.iter()
        .map(|msg| {
            MessageContent {
                id: msg.id.clone(),
                text: msg.content.text.clone(),
                timestamp: msg.timestamp,
                sender: msg.sender.name.clone(),
                metadata: Some(format!(
                    "Platform: {}, Conversation: {}, Has Attachments: {}",
                    format_platform(&msg.platform),
                    msg.conversation_id,
                    !msg.content.attachments.is_empty()
                )),
            }
        })
        .collect()
}

// Optimize context window by selecting most relevant messages
fn optimize_context_window(messages: Vec<MessageContent>, query: &str) -> Vec<MessageContent> {
    // Use vector embeddings to find semantic similarity with query
    let embedded_query = embeddings::embed_text(query);
    
    // Score and rank messages by relevance to query
    let mut scored_messages: Vec<(MessageContent, f32)> = messages.into_iter()
        .map(|msg| {
            let embedded_msg = embeddings::embed_text(&msg.text);
            let score = embeddings::calculate_similarity(&embedded_query, &embedded_msg);
            (msg, score)
        })
        .collect();
    
    // Sort by score (descending)
    scored_messages.sort_by(|(_, score_a), (_, score_b)| {
        score_b.partial_cmp(score_a).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Take top messages that fit within context limit (typically 8k tokens for Claude)
    let mut selected_messages = Vec::new();
    let mut token_count = 0;
    let token_limit = 8000;  // Approximate token limit for Claude
    
    for (msg, _) in scored_messages {
        let approx_tokens = msg.text.len() / 4;  // Rough approximation of tokens
        if token_count + approx_tokens < token_limit {
            token_count += approx_tokens;
            selected_messages.push(msg);
        } else {
            break;
        }
    }
    
    selected_messages
}

// Process query response from OpenChat
fn process_query_response(
    response: QueryResponse,
    original_messages: &[Message],
    query_text: &str
) -> Result<QueryResult> {
    // Extract message IDs from response
    let message_ids: Vec<String> = response.message_ids;
    
    // Find original messages by ID
    let mut result_messages: Vec<Message> = Vec::new();
    for id in &message_ids {
        if let Some(msg) = original_messages.iter().find(|m| m.id == *id) {
            result_messages.push(msg.clone());
        }
    }
    
    // Sort messages by timestamp (newest first)
    result_messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    // Generate enhanced context using AI explanation
    let context = if let Some(explanation) = response.explanation {
        format!("AI Analysis: {}\n\nFound {} relevant messages.", explanation, result_messages.len())
    } else {
        format!("Found {} messages related to your query.", result_messages.len())
    };
    
    Ok(QueryResult {
        messages: result_messages,
        context,
    })
}

// Helper to format platform name
fn format_platform(platform: &Platform) -> String {
    match platform {
        Platform::Telegram => "Telegram".to_string(),
        Platform::Slack => "Slack".to_string(),
        Platform::Discord => "Discord".to_string(),
        Platform::Twitter => "Twitter".to_string(),
        Platform::Facebook => "Facebook".to_string(),
        Platform::WhatsApp => "WhatsApp".to_string(),
    }
}