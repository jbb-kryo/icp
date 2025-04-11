use crate::{Message, Error, Result, QueryResult};
use crate::storage::messages;
use super::embeddings;
use openchat_sdk::{MessageContent, EntityExtraction, TopicAnalysis};
use std::collections::{HashMap, HashSet};

// Different query types that can be detected
pub enum QueryIntent {
    Search(String),                      // Basic search
    EntityLookup(Vec<String>),           // Find mentions of entities
    TimeAnalysis(i64, i64),              // Analyze messages in time range
    TopicSummary(String),                // Summarize messages on a topic
    TrendAnalysis(String),               // Analyze trends for a topic
    SentimentAnalysis(String),           // Analyze sentiment for a topic
    RelationshipAnalysis(Vec<String>),   // Analyze relationships between entities
    Unknown,                             // Can't determine intent
}

// Detect the intent of a query
pub fn detect_query_intent(query: &str) -> QueryIntent {
    let query_lower = query.to_lowercase();
    
    // Check for time analysis patterns
    if query_lower.contains("trend") || query_lower.contains("over time") || query_lower.contains("historical") {
        if let Some(topic) = extract_main_topic(&query_lower) {
            return QueryIntent::TrendAnalysis(topic);
        }
    }
    
    // Check for sentiment analysis patterns
    if query_lower.contains("feel") || query_lower.contains("sentiment") || 
       query_lower.contains("opinion") || query_lower.contains("attitude") {
        if let Some(topic) = extract_main_topic(&query_lower) {
            return QueryIntent::SentimentAnalysis(topic);
        }
    }
    
    // Check for relationship analysis
    if query_lower.contains("relationship") || query_lower.contains("between") || 
       query_lower.contains("connection") {
        let entities = extract_entities(&query_lower);
        if entities.len() >= 2 {
            return QueryIntent::RelationshipAnalysis(entities);
        }
    }
    
    // Check for topic summary
    if query_lower.contains("summarize") || query_lower.contains("summary") || 
       query_lower.starts_with("what") {
        if let Some(topic) = extract_main_topic(&query_lower) {
            return QueryIntent::TopicSummary(topic);
        }
    }
    
    // Default to search
    QueryIntent::Search(query.to_string())
}

// Extract entities from a query
fn extract_entities(query: &str) -> Vec<String> {
    // This is a placeholder - a real implementation would use NLP
    // For now, we'll do a very simple extraction of capitalized words
    let words = query.split_whitespace();
    let mut entities = Vec::new();
    
    for word in words {
        if !word.is_empty() && word.chars().next().unwrap().is_uppercase() {
            entities.push(word.to_string());
        }
    }
    
    entities
}

// Extract the main topic from a query
fn extract_main_topic(query: &str) -> Option<String> {
    // This is a placeholder - a real implementation would use NLP
    // For now, we'll do simple keyword extraction
    let words: Vec<&str> = query.split_whitespace().collect();
    
    if words.is_empty() {
        return None;
    }
    
    // Look for words following key phrases
    for (i, word) in words.iter().enumerate() {
        if *word == "about" || *word == "regarding" || *word == "on" {
            if i < words.len() - 1 {
                return Some(words[i+1].to_string());
            }
        }
    }
    
    // Fallback to the last word if we can't find a better topic
    Some(words[words.len() - 1].to_string())
}

// Generate a summary of messages about a particular topic
pub fn generate_topic_summary(messages: &[Message], topic: &str) -> String {
    // Placeholder for a real implementation that would use OpenChat's AI capabilities
    let relevant_messages = filter_messages_by_topic(messages, topic);
    
    let message_count = relevant_messages.len();
    if message_count == 0 {
        return format!("No messages found about {}.", topic);
    }
    
    // Count mentions by platform
    let mut platform_counts = HashMap::new();
    for msg in &relevant_messages {
        *platform_counts.entry(format!("{:?}", msg.platform)).or_insert(0) += 1;
    }
    
    // Format summary
    let mut summary = format!("Found {} messages about {}.\n\n", message_count, topic);
    summary.push_str("Distribution by platform:\n");
    
    for (platform, count) in platform_counts {
        summary.push_str(&format!("- {}: {} messages\n", platform, count));
    }
    
    // Find most recent message
    if let Some(latest) = relevant_messages.iter().max_by_key(|m| m.timestamp) {
        summary.push_str(&format!("\nMost recent message ({}): {}", 
            format_timestamp(latest.timestamp),
            truncate_text(&latest.content.text, 100)
        ));
    }
    
    summary
}

// Filter messages by topic
fn filter_messages_by_topic(messages: &[Message], topic: &str) -> Vec<Message> {
    let topic_embedding = embeddings::embed_text(topic);
    
    let mut scored_messages: Vec<(Message, f32)> = messages.iter()
        .map(|msg| {
            let msg_embedding = embeddings::embed_text(&msg.content.text);
            let score = embeddings::calculate_similarity(&topic_embedding, &msg_embedding);
            (msg.clone(), score)
        })
        .filter(|(_, score)| *score > 0.1) // Threshold for relevance
        .collect();
    
    // Sort by score (descending)
    scored_messages.sort_by(|(_, score_a), (_, score_b)| {
        score_b.partial_cmp(score_a).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Extract just the messages
    scored_messages.into_iter()
        .map(|(msg, _)| msg)
        .collect()
}

// Format timestamp as a human-readable string
fn format_timestamp(timestamp: u64) -> String {
    let now = ic_cdk::api::time() / 1_000_000; // Convert to seconds
    let seconds = timestamp / 1000; // Convert from milliseconds to seconds
    
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

// Truncate text to a maximum length
fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[0..max_length])
    }
}