use crate::{Message, Conversation, QueryResult, Error, Result, Platform};
use crate::storage::{messages, conversations};
use crate::query::parser::QueryRequest;
use ic_cdk::api::time;
use openchat_sdk::types::{
    CanisterId, MessageContent as OpenChatMessage,
    QueryRequest as OpenChatQueryRequest, 
    EventsRequest as OpenChatEventsRequest
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

const OPENCHAT_COMMUNITY_CANISTER_ID: &str = "xomae-vyaaa-aaaaq-aabhq-cai";

// Integrate with OpenChat SDK for enhanced query capabilities
pub async fn process_query_with_openchat(
    query_text: &str,
    user_id: &str,
    limit: Option<u64>
) -> Result<QueryResult> {
    // Create a community canister ID from the OpenChat mainnet canister
    let community_id = CanisterId::from_str(OPENCHAT_COMMUNITY_CANISTER_ID)
        .map_err(|e| Error::InternalError(format!("Invalid canister ID: {}", e)))?;
    
    // Get relevant conversations and messages for the query
    let (conversations, messages) = collect_query_data(user_id)?;
    
    // Prepare the OpenChat request
    let request = OpenChatQueryRequest {
        query: query_text.to_string(),
        sources: format_messages_for_openchat(&messages),
        limit: limit.unwrap_or(25) as usize,
    };
    
    // This would typically call the OpenChat SDK to query the messages
    // In the canister environment, we'd use inter-canister calls
    // For now, we'll simulate the response locally
    
    let results = simulate_openchat_query(&request, &messages)?;
    
    // Format the results into our QueryResult structure
    let result_messages = results
        .into_iter()
        .map(|id| {
            messages.iter()
                .find(|m| m.id == id)
                .cloned()
                .unwrap_or_default()
        })
        .collect();
    
    let context = format!(
        "AI-powered query results for: \"{}\"\nFound {} relevant messages across {} platforms.",
        query_text,
        result_messages.len(),
        count_platforms(&result_messages)
    );
    
    Ok(QueryResult {
        messages: result_messages,
        context,
    })
}

// Collect all messages and conversations for the query
fn collect_query_data(user_id: &str) -> Result<(Vec<Conversation>, Vec<Message>)> {
    // Get all conversations for the user
    let user_conversations = conversations::get_user_conversations(user_id, None);
    
    // Get messages for each conversation
    let mut all_messages = Vec::new();
    for conversation in &user_conversations {
        let conv_messages = messages::get_conversation_messages(&conversation.id, 1000, None);
        all_messages.extend(conv_messages);
    }
    
    Ok((user_conversations, all_messages))
}

// Format messages for the OpenChat query system
fn format_messages_for_openchat(messages: &[Message]) -> Vec<String> {
    messages
        .iter()
        .map(|msg| {
            format!(
                "[{}] {}: {}",
                platform_to_string(&msg.platform),
                msg.sender.name,
                msg.content.text
            )
        })
        .collect()
}

// Simulate OpenChat AI query processing
// In a real implementation, this would call the OpenChat SDK
fn simulate_openchat_query(
    request: &OpenChatQueryRequest,
    messages: &[Message]
) -> Result<Vec<String>> {
    // For demo purposes, we'll implement a simple keyword search
    // In a real implementation, this would leverage the AI capabilities of OpenChat
    
    let query_terms: Vec<&str> = request.query.to_lowercase().split_whitespace().collect();
    
    let mut scored_messages: Vec<(String, usize)> = messages
        .iter()
        .map(|msg| {
            let text = msg.content.text.to_lowercase();
            let score = query_terms.iter()
                .filter(|term| text.contains(*term))
                .count();
            (msg.id.clone(), score)
        })
        .filter(|(_, score)| *score > 0)
        .collect();
    
    // Sort by score (descending)
    scored_messages.sort_by(|(_, a), (_, b)| b.cmp(a));
    
    // Take the top results based on limit
    let results: Vec<String> = scored_messages
        .into_iter()
        .take(request.limit)
        .map(|(id, _)| id)
        .collect();
    
    Ok(results)
}

// Count unique platforms in messages
fn count_platforms(messages: &[Message]) -> usize {
    messages
        .iter()
        .map(|m| platform_to_string(&m.platform))
        .collect::<std::collections::HashSet<_>>()
        .len()
}

// Convert platform enum to string
fn platform_to_string(platform: &Platform) -> String {
    match platform {
        Platform::Telegram => "Telegram".to_string(),
        Platform::Slack => "Slack".to_string(),
        Platform::Discord => "Discord".to_string(),
        Platform::Twitter => "Twitter".to_string(),
        Platform::Facebook => "Facebook".to_string(),
        Platform::WhatsApp => "WhatsApp".to_string(),
    }
}