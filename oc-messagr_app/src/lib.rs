use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::time;
use ic_cdk_macros::*;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use std::collections::HashMap;

mod auth;
mod connectors;
mod storage;
mod query;
mod indexing;
mod openchat;

// Type definitions matching our Candid interface
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum Platform {
    Telegram,
    Slack,
    Discord,
    Twitter,
    Facebook,
    WhatsApp,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AuthConfig {
    platform: Platform,
    token: String,
    api_key: Option<String>,
    api_secret: Option<String>,
    redirect_uri: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Attachment {
    attachment_type: String,
    url: Option<String>,
    content: Option<Vec<u8>>,
    name: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MessageContent {
    text: String,
    attachments: Vec<Attachment>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct User {
    id: String,
    name: String,
    platform: Platform,
    avatar_url: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Message {
    id: String,
    platform: Platform,
    conversation_id: String,
    sender: User,
    content: MessageContent,
    timestamp: u64,
    thread_id: Option<String>,
    reply_to: Option<String>,
    edited: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Conversation {
    id: String,
    platform: Platform,
    name: String,
    participants: Vec<User>,
    created_at: u64,
    last_message_at: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct QueryResult {
    messages: Vec<Message>,
    context: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum Error {
    NotAuthenticated,
    PlatformError(String),
    QueryError(String),
    InternalError(String),
    InvalidParameters(String),
}

type Memory = VirtualMemory<DefaultMemoryImpl>;
type Result<T> = std::result::Result<T, Error>;

// Stable memory storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = 
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    
    static AUTH_STORAGE: RefCell<StableBTreeMap<String, AuthConfig, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
    
    static CONVERSATIONS: RefCell<StableBTreeMap<String, Conversation, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
    
    static MESSAGES: RefCell<StableBTreeMap<String, Message, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
        )
    );
    
    static USER_DATA: RefCell<StableBTreeMap<Principal, String, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
        )
    );
}

// Authentication and setup
#[update]
async fn connect_platform(config: AuthConfig) -> Result<String> {
    let caller = ic_cdk::caller();
    let platform_key = format!("{}:{}", caller.to_string(), platform_to_string(&config.platform));
    
    // Validate auth config based on platform
    match &config.platform {
        Platform::Telegram => auth::telegram::validate_auth(&config)?,
        Platform::Slack => auth::slack::validate_auth(&config)?,
        Platform::Discord => auth::discord::validate_auth(&config)?,
        Platform::Twitter => auth::twitter::validate_auth(&config)?,
        Platform::Facebook => auth::facebook::validate_auth(&config)?,
        Platform::WhatsApp => auth::whatsapp::validate_auth(&config)?,
    }
    
    // Store auth config
    AUTH_STORAGE.with(|storage| {
        storage.borrow_mut().insert(platform_key.clone(), config.clone())
    });
    
    // Initialize platform connection
    match config.platform {
        Platform::Telegram => connectors::telegram::init_connection(&config).await,
        Platform::Slack => connectors::slack::init_connection(&config).await,
        Platform::Discord => connectors::discord::init_connection(&config).await,
        Platform::Twitter => connectors::twitter::init_connection(&config).await,
        Platform::Facebook => connectors::facebook::init_connection(&config).await,
        Platform::WhatsApp => connectors::whatsapp::init_connection(&config).await,
    }?;
    
    Ok(format!("Successfully connected to {}", platform_to_string(&config.platform)))
}

#[update]
fn disconnect_platform(platform: Platform) -> Result<bool> {
    let caller = ic_cdk::caller();
    let platform_key = format!("{}:{}", caller.to_string(), platform_to_string(&platform));
    
    AUTH_STORAGE.with(|storage| {
        if storage.borrow().get(&platform_key).is_some() {
            storage.borrow_mut().remove(&platform_key);
            Ok(true)
        } else {
            Err(Error::NotAuthenticated)
        }
    })
}

#[query]
fn get_connected_platforms() -> Vec<Platform> {
    let caller = ic_cdk::caller();
    let prefix = format!("{}:", caller.to_string());
    
    AUTH_STORAGE.with(|storage| {
        storage.borrow().iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v.platform.clone())
            .collect()
    })
}

// Data retrieval
#[update]
async fn sync_messages(platform: Platform) -> Result<u64> {
    let caller = ic_cdk::caller();
    let platform_key = format!("{}:{}", caller.to_string(), platform_to_string(&platform));
    
    // Get auth config
    let auth_config = AUTH_STORAGE.with(|storage| {
        storage.borrow().get(&platform_key)
            .ok_or(Error::NotAuthenticated)
    })?;
    
    // Sync messages from platform
    let count = match platform {
        Platform::Telegram => connectors::telegram::sync_messages(&auth_config).await,
        Platform::Slack => connectors::slack::sync_messages(&auth_config).await,
        Platform::Discord => connectors::discord::sync_messages(&auth_config).await,
        Platform::Twitter => connectors::twitter::sync_messages(&auth_config).await,
        Platform::Facebook => connectors::facebook::sync_messages(&auth_config).await,
        Platform::WhatsApp => connectors::whatsapp::sync_messages(&auth_config).await,
    }?;
    
    Ok(count)
}

#[query]
fn get_conversations(platform: Platform) -> Result<Vec<Conversation>> {
    let caller = ic_cdk::caller();
    let platform_string = platform_to_string(&platform);
    
    CONVERSATIONS.with(|storage| {
        let conversations = storage.borrow().iter()
            .filter(|(_, v)| 
                platform_to_string(&v.platform) == platform_string && 
                v.participants.iter().any(|p| p.id.starts_with(&caller.to_string()))
            )
            .map(|(_, v)| v.clone())
            .collect();
        
        Ok(conversations)
    })
}

#[query]
fn get_messages(conversation_id: String, limit: Option<u64>, offset: Option<u64>) -> Result<Vec<Message>> {
    let caller = ic_cdk::caller();
    
    // First check if the user has access to this conversation
    let conversation = CONVERSATIONS.with(|storage| {
        storage.borrow().get(&conversation_id)
            .ok_or(Error::InvalidParameters(format!("Conversation not found: {}", conversation_id)))
    })?;
    
    // Verify user has access to this conversation
    if !conversation.participants.iter().any(|p| p.id.starts_with(&caller.to_string())) {
        return Err(Error::NotAuthenticated);
    }
    
    // Get messages for the conversation
    let limit = limit.unwrap_or(100);
    let offset = offset.unwrap_or(0);
    
    MESSAGES.with(|storage| {
        let messages = storage.borrow().iter()
            .filter(|(_, v)| v.conversation_id == conversation_id)
            .map(|(_, v)| v.clone())
            .collect::<Vec<Message>>();
        
        // Sort by timestamp (newer first)
        let mut sorted_messages = messages;
        sorted_messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        // Apply pagination
        let paginated = sorted_messages.into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect();
        
        Ok(paginated)
    })
}

// Intelligent querying with advanced indexing
#[query]
fn query_conversations(query_text: String) -> Result<QueryResult> {
    let caller = ic_cdk::caller();
    
    // Parse the query to extract filters and structured information
    let (clean_query, filters) = indexing::search::SearchFilters::from_natural_language(&query_text);
    
    // Process the query using the advanced indexing system
    let search_results = storage::messages::search_messages(&clean_query, &filters)?;
    
    // Generate context information
    let context = generate_search_context(&query_text, &clean_query, &filters, &search_results);
    
    Ok(QueryResult {
        messages: search_results,
        context,
    })
}

// AI-enhanced query using OpenChat SDK
#[query]
async fn ai_enhanced_query(query_text: String) -> Result<QueryResult> {
    let caller = ic_cdk::caller();
    
    // Use OpenChat for enhanced querying
    openchat::query_with_openchat(&query_text, None, &caller.to_string(), 50).await
}

// Platform-specific AI query
#[query]
async fn ai_query_platform(query_text: String, platform: Platform) -> Result<QueryResult> {
    let caller = ic_cdk::caller();
    
    // Use OpenChat for platform-specific querying
    openchat::query_with_openchat(&query_text, Some(platform), &caller.to_string(), 50).await
}

// Perform topic analysis on messages
#[query]
fn analyze_topic(topic: String) -> Result<String> {
    let caller = ic_cdk::caller();
    
    // Get all messages the user has access to
    let user_conversations = storage::conversations::get_user_conversations(&caller.to_string(), None);
    let conversation_ids: Vec<String> = user_conversations.iter()
        .map(|c| c.id.clone())
        .collect();
    
    // Get messages from these conversations
    let mut all_messages = Vec::new();
    for conv_id in conversation_ids {
        let messages = storage::messages::get_conversation_messages(&conv_id, 500, None);
        all_messages.extend(messages);
    }
    
    // Generate topic summary
    let summary = openchat::query::generate_topic_summary(&all_messages, &topic);
    Ok(summary)
}

// Generate conversation insights
#[update]
async fn generate_conversation_insights(conversation_id: String) -> Result<openchat::types::QueryInsights> {
    let caller = ic_cdk::caller();
    
    // Check if user has access to this conversation
    let conversation = storage::conversations::get_conversation(&conversation_id)
        .ok_or(Error::InvalidParameters(format!("Conversation not found: {}", conversation_id)))?;
    
    // Verify user has access
    if !conversation.participants.iter().any(|p| p.id.starts_with(&caller.to_string())) {
        return Err(Error::NotAuthenticated);
    }
    
    // Get messages for this conversation
    let messages = storage::messages::get_conversation_messages(&conversation_id, 1000, None);
    
    // Format messages for analysis
    let analysis_messages: Vec<openchat::types::MessageForAnalysis> = messages.iter()
        .map(|msg| openchat::types::convert_to_analysis_message(msg))
        .collect();
    
    // Create analysis request
    let request = openchat::types::EnhancedAnalysisRequest {
        messages: analysis_messages,
        query: "Generate comprehensive insights".to_string(),
        analysis_type: openchat::types::AnalysisType::ComprehensiveAnalysis,
        options: openchat::types::AnalysisOptions {
            include_explanations: true,
            detail_level: openchat::types::DetailLevel::Detailed,
            max_results: Some(10),
        },
    };
    
    // Call OpenChat for analysis
    let client = openchat::client::create_openchat_client().await?;
    
    // In a real implementation, we would process the result from OpenChat
    // For this demo, we'll create a placeholder response
    let insights = generate_placeholder_insights(&messages);
    
    Ok(insights)
}

// Generate placeholder insights (in a real implementation, this would use the OpenChat response)
fn generate_placeholder_insights(messages: &[Message]) -> openchat::types::QueryInsights {
    use openchat::types::*;
    use std::collections::HashMap;
    
    // Extract entities (simple implementation)
    let mut entity_mentions = HashMap::new();
    for msg in messages {
        let words: Vec<&str> = msg.content.text.split_whitespace().collect();
        for word in words {
            if word.len() > 1 && word.chars().next().unwrap().is_uppercase() {
                *entity_mentions.entry(word.to_string()).or_insert(0) += 1;
            }
        }
    }
    
    // Create entities from the mentions
    let mut entities = Vec::new();
    for (name, count) in entity_mentions.iter().take(5) {
        entities.push(Entity {
            name: name.clone(),
            entity_type: EntityType::Other("Unknown".to_string()),
            mentions: *count,
            sentiment_score: Some(0.0),
            related_entities: Vec::new(),
        });
    }
    
    // Create placeholder topics
    let topics = vec![
        Topic {
            name: "Main Topic".to_string(),
            relevance_score: 0.9,
            message_count: messages.len() as i32,
            summary: "This is a placeholder summary of the main topic.".to_string(),
        },
        Topic {
            name: "Secondary Topic".to_string(),
            relevance_score: 0.7,
            message_count: (messages.len() / 2) as i32,
            summary: "This is a placeholder summary of the secondary topic.".to_string(),
        },
    ];
    
    // Create placeholder timeline if there are enough messages
    let timeline = if messages.len() >= 3 {
        // Sort by timestamp
        let mut sorted_msgs = messages.to_vec();
        sorted_msgs.sort_by_key(|m| m.timestamp);
        
        // Take a few key messages for the timeline
        let indices = [0, sorted_msgs.len() / 2, sorted_msgs.len() - 1];
        let timeline_events = indices.iter().map(|&i| {
            let msg = &sorted_msgs[i];
            TimelineEvent {
                timestamp: msg.timestamp,
                description: format!("{}: {}", msg.sender.name, truncate_text(&msg.content.text, 50)),
                related_message_ids: vec![msg.id.clone()],
                importance: 1,
            }
        }).collect();
        
        Some(timeline_events)
    } else {
        None
    };
    
    // Create placeholder sentiment analysis
    let sentiment = Some(SentimentAnalysis {
        overall_sentiment: 0.2,  // Slightly positive
        sentiment_breakdown: {
            let mut map = HashMap::new();
            map.insert("Positive".to_string(), 0.6);
            map.insert("Neutral".to_string(), 0.3);
            map.insert("Negative".to_string(), 0.1);
            map
        },
        key_positive_points: vec!["Placeholder positive point".to_string()],
        key_negative_points: vec!["Placeholder negative point".to_string()],
    });
    
    // Create placeholder conversation flow
    let conversation_flow = Some(ConversationFlow {
        main_threads: vec![
            ConversationThread {
                topic: "Main Discussion".to_string(),
                message_ids: messages.iter().map(|m| m.id.clone()).collect(),
                participants: messages.iter().map(|m| m.sender.name.clone()).collect::<std::collections::HashSet<_>>().into_iter().collect(),
                resolved: true,
            }
        ],
        key_decisions: vec!["Placeholder decision".to_string()],
        action_items: vec!["Placeholder action item".to_string()],
    });
    
    // Return the complete insights
    QueryInsights {
        entities,
        topics,
        timeline,
        sentiment,
        conversation_flow,
    }
}

// Helper function to truncate text
fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[0..max_length])
    }
}

// Generate detailed context information for the search results
fn generate_search_context(
    original_query: &str, 
    clean_query: &str, 
    filters: &indexing::search::SearchFilters,
    results: &[Message]
) -> String {
    let mut context_parts = Vec::new();
    
    // Original query
    context_parts.push(format!("Query: \"{}\"", original_query));
    
    // Applied filters
    if clean_query != original_query {
        context_parts.push(format!("Search terms: \"{}\"", clean_query));
    }
    
    // Platform filter
    if let Some(platform) = &filters.platform {
        context_parts.push(format!("Platform: {}", platform));
    }
    
    // Time range
    if filters.start_time.is_some() || filters.end_time.is_some() {
        let time_range = match (filters.start_time, filters.end_time) {
            (Some(start), Some(end)) => format!(
                "Time range: {} to {}", 
                format_timestamp(start), 
                format_timestamp(end)
            ),
            (Some(start), None) => format!("After: {}", format_timestamp(start)),
            (None, Some(end)) => format!("Before: {}", format_timestamp(end)),
            _ => String::new(),
        };
        
        if !time_range.is_empty() {
            context_parts.push(time_range);
        }
    }
    
    // Conversation filter
    if let Some(conv_id) = &filters.conversation_id {
        // Try to get the conversation name
        let conv_name = storage::conversations::get_conversation(conv_id)
            .map(|c| c.name)
            .unwrap_or_else(|| format!("Conversation {}", conv_id));
            
        context_parts.push(format!("In conversation: {}", conv_name));
    }
    
    // Sender filter
    if let Some(sender_name) = &filters.sender_name {
        context_parts.push(format!("From: {}", sender_name));
    }
    
    // Attachment filters
    if filters.has_attachments {
        if let Some(att_type) = &filters.attachment_type {
            context_parts.push(format!("With {} attachments", att_type));
        } else {
            context_parts.push("With attachments".to_string());
        }
    }
    
    // Other filters
    if filters.is_reply {
        context_parts.push("Only replies".to_string());
    }
    
    if filters.in_thread {
        context_parts.push("In threads".to_string());
    }
    
    if filters.is_edited {
        context_parts.push("Edited messages".to_string());
    }
    
    // Result summary
    let result_count = results.len();
    
    // Count unique conversations and platforms
    let mut conversation_ids = std::collections::HashSet::new();
    let mut platforms = std::collections::HashSet::new();
    
    for msg in results {
        conversation_ids.insert(&msg.conversation_id);
        platforms.insert(&msg.platform);
    }
    
    context_parts.push(format!(
        "Found {} messages across {} conversations from {} platforms",
        result_count,
        conversation_ids.len(),
        platforms.len()
    ));
    
    // Join all parts
    context_parts.join("\n")
}

// Format timestamp for display
fn format_timestamp(timestamp: u64) -> String {
    let seconds = timestamp / 1000; // Convert from milliseconds to seconds
    let now = (ic_cdk::api::time() / 1_000_000) as u64; // Current time in seconds
    
    if now - seconds < 60 {
        "just now".to_string()
    } else if now - seconds < 3600 {
        format!("{} minutes ago", (now - seconds) / 60)
    } else if now - seconds < 86400 {
        format!("{} hours ago", (now - seconds) / 3600)
    } else {
        format!("{} days ago", (now - seconds) / 86400)
    }
}

// User management
#[update]
fn set_username(username: String) -> Result<bool> {
    let caller = ic_cdk::caller();
    
    if username.is_empty() {
        return Err(Error::InvalidParameters("Username cannot be empty".to_string()));
    }
    
    USER_DATA.with(|storage| {
        storage.borrow_mut().insert(caller, username);
    });
    
    Ok(true)
}

#[query]
fn get_username() -> String {
    let caller = ic_cdk::caller();
    
    USER_DATA.with(|storage| {
        storage.borrow().get(&caller)
            .unwrap_or_else(|| "Anonymous".to_string())
    })
}

// Advanced indexing functions
#[update]
fn optimize_indices() -> Result<bool> {
    storage::messages::optimize_indices()?;
    Ok(true)
}

#[update]
fn rebuild_indices() -> Result<bool> {
    storage::messages::rebuild_indices()?;
    Ok(true)
}

// Advanced search interface with structured filters
#[query]
fn advanced_search(query: String, platform: Option<Platform>, 
                  start_time: Option<u64>, end_time: Option<u64>,
                  conversation_id: Option<String>, sender_id: Option<String>,
                  has_attachments: Option<bool>, attachment_type: Option<String>,
                  is_reply: Option<bool>, in_thread: Option<bool>, is_edited: Option<bool>,
                  sort_by: String, sort_direction: String,
                  limit: Option<u64>, offset: Option<u64>) -> Result<QueryResult> {
    
    // Create search filters from parameters
    let mut filters = indexing::search::SearchFilters::default();
    
    // Set platform filter
    if let Some(p) = platform {
        filters.platform = Some(p);
    }
    
    // Set time range filters
    filters.start_time = start_time;
    filters.end_time = end_time;
    
    // Set conversation filter
    filters.conversation_id = conversation_id;
    
    // Set sender filter
    filters.sender_id = sender_id;
    
    // Set attachment filters
    if let Some(has_att) = has_attachments {
        filters.has_attachments = has_att;
    }
    filters.attachment_type = attachment_type;
    
    // Set message type filters
    if let Some(reply) = is_reply {
        filters.is_reply = reply;
    }
    if let Some(thread) = in_thread {
        filters.in_thread = thread;
    }
    if let Some(edited) = is_edited {
        filters.is_edited = edited;
    }
    
    // Set sort options
    match sort_by.as_str() {
        "time" | "timestamp" => filters.sort_by = indexing::search::SortField::Timestamp,
        "platform" => filters.sort_by = indexing::search::SortField::Platform,
        _ => filters.sort_by = indexing::search::SortField::Relevance,
    }
    
    match sort_direction.as_str() {
        "asc" | "ascending" => filters.sort_direction = indexing::search::SortDirection::Ascending,
        _ => filters.sort_direction = indexing::search::SortDirection::Descending,
    }
    
    // Set pagination
    if let Some(lim) = limit {
        filters.limit = lim as usize;
    }
    if let Some(off) = offset {
        filters.offset = off as usize;
    }
    
    // Perform the search
    let search_results = storage::messages::search_messages(&query, &filters)?;
    
    // Generate context information
    let context = generate_search_context(&query, &query, &filters, &search_results);
    
    Ok(QueryResult {
        messages: search_results,
        context,
    })
}

// Get index statistics for monitoring
#[query]
fn get_index_stats() -> Result<IndexStats> {
    // Get stats about the indices
    let message_count = MESSAGES.with(|store| store.borrow().len());
    
    // We would collect more detailed stats from the indexing system
    // in a real implementation
    
    Ok(IndexStats {
        message_count,
        indexed_count: message_count, // Assume all messages are indexed
        last_optimization: None,      // Would track this in a real implementation
        index_size_bytes: 0,          // Would calculate in a real implementation
    })
}

// Structure for index statistics
#[derive(CandidType, Deserialize)]
struct IndexStats {
    message_count: u64,
    indexed_count: u64,
    last_optimization: Option<u64>,
    index_size_bytes: u64,
}

// System
#[query]
fn get_version() -> String {
    "Messagr App v0.1.0".to_string()
}

// Utility functions
fn platform_to_string(platform: &Platform) -> String {
    match platform {
        Platform::Telegram => "telegram".to_string(),
        Platform::Slack => "slack".to_string(),
        Platform::Discord => "discord".to_string(),
        Platform::Twitter => "twitter".to_string(),
        Platform::Facebook => "facebook".to_string(),
        Platform::WhatsApp => "whatsapp".to_string(),
    }
}

// Required for Candid interface generation
ic_cdk::export_candid!();