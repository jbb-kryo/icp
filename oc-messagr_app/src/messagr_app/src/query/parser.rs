use crate::{Error, Result, Platform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterType {
    Platform(Platform),
    TimeRange { start: u64, end: Option<u64> },
    Sender(String),
    Keyword(String),
    Conversation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    pub query_text: String,
    pub filters: Vec<FilterType>,
    pub limit: Option<u64>,
}

// Parse a natural language query into structured filters
pub fn parse_query(query_text: &str) -> Result<QueryRequest> {
    // In a production system, we would use a more sophisticated NLP approach
    // Or integrate with the OpenChat SDK's query capabilities
    
    let mut filters = Vec::new();
    let mut cleaned_query = query_text.to_string();
    
    // Simple keyword-based parsing
    
    // Check for platform filters
    if query_text.to_lowercase().contains("telegram") {
        filters.push(FilterType::Platform(Platform::Telegram));
        cleaned_query = cleaned_query.replace("telegram", "").replace("Telegram", "");
    }
    if query_text.to_lowercase().contains("slack") {
        filters.push(FilterType::Platform(Platform::Slack));
        cleaned_query = cleaned_query.replace("slack", "").replace("Slack", "");
    }
    if query_text.to_lowercase().contains("discord") {
        filters.push(FilterType::Platform(Platform::Discord));
        cleaned_query = cleaned_query.replace("discord", "").replace("Discord", "");
    }
    if query_text.to_lowercase().contains("twitter") {
        filters.push(FilterType::Platform(Platform::Twitter));
        cleaned_query = cleaned_query.replace("twitter", "").replace("Twitter", "");
    }
    if query_text.to_lowercase().contains("facebook") {
        filters.push(FilterType::Platform(Platform::Facebook));
        cleaned_query = cleaned_query.replace("facebook", "").replace("Facebook", "");
    }
    if query_text.to_lowercase().contains("whatsapp") {
        filters.push(FilterType::Platform(Platform::WhatsApp));
        cleaned_query = cleaned_query.replace("whatsapp", "").replace("WhatsApp", "");
    }
    
    // Check for time-based filters
    if query_text.to_lowercase().contains("yesterday") {
        // This is a simplification - in real code we'd calculate actual timestamps
        let now = ic_cdk::api::time() / 1_000_000; // Convert to seconds
        let yesterday_start = now - 86400; // 24 hours ago
        filters.push(FilterType::TimeRange { 
            start: yesterday_start * 1000, // Convert to millis 
            end: Some(now * 1000) 
        });
        cleaned_query = cleaned_query.replace("yesterday", "").replace("Yesterday", "");
    }
    if query_text.to_lowercase().contains("last week") {
        let now = ic_cdk::api::time() / 1_000_000; // Convert to seconds
        let week_start = now - (7 * 86400); // 7 days ago
        filters.push(FilterType::TimeRange { 
            start: week_start * 1000, 
            end: Some(now * 1000) 
        });
        cleaned_query = cleaned_query.replace("last week", "").replace("Last week", "");
    }
    
    // Sender filters would be detected from names in the query
    // This is a simplified approach - in a real system we'd use entity extraction
    
    // Clean up the query
    cleaned_query = cleaned_query.trim().to_string();
    
    // Add the remaining text as a keyword filter if not empty
    if !cleaned_query.is_empty() {
        filters.push(FilterType::Keyword(cleaned_query.clone()));
    }
    
    Ok(QueryRequest {
        query_text: cleaned_query,
        filters,
        limit: Some(50), // Default limit
    })
}