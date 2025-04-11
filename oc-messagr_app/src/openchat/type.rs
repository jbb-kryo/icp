use crate::{Message, Error, Result};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Enhanced query result with AI insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedQueryResult {
    // Original messages
    pub messages: Vec<Message>,
    
    // AI-generated context explanation
    pub explanation: String,
    
    // Optional AI-generated insights
    pub insights: Option<QueryInsights>,
    
    // Message IDs ranked by relevance
    pub ranked_ids: Vec<String>,
}

// AI-generated insights about the query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInsights {
    // Key entities mentioned in the results
    pub entities: Vec<Entity>,
    
    // Key topics identified in the results
    pub topics: Vec<Topic>,
    
    // Timeline of important events
    pub timeline: Option<Vec<TimelineEvent>>,
    
    // Sentiment analysis
    pub sentiment: Option<SentimentAnalysis>,
    
    // Conversation flow analysis
    pub conversation_flow: Option<ConversationFlow>,
}

// Entity representation (person, place, organization, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
    pub mentions: i32,
    pub sentiment_score: Option<f32>,
    pub related_entities: Vec<String>,
}

// Types of entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Person,
    Organization,
    Location,
    Date,
    Product,
    Other(String),
}

// Topic identified in messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub name: String,
    pub relevance_score: f32,
    pub message_count: i32,
    pub summary: String,
}

// Event on the timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub timestamp: u64,
    pub description: String,
    pub related_message_ids: Vec<String>,
    pub importance: i32,
}

// Sentiment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentAnalysis {
    pub overall_sentiment: f32,  // -1.0 to 1.0
    pub sentiment_breakdown: HashMap<String, f32>,
    pub key_positive_points: Vec<String>,
    pub key_negative_points: Vec<String>,
}

// Conversation flow analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationFlow {
    pub main_threads: Vec<ConversationThread>,
    pub key_decisions: Vec<String>,
    pub action_items: Vec<String>,
}

// Thread in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationThread {
    pub topic: String,
    pub message_ids: Vec<String>,
    pub participants: Vec<String>,
    pub resolved: bool,
}

// OpenChat API request for enhanced analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedAnalysisRequest {
    pub messages: Vec<MessageForAnalysis>,
    pub query: String,
    pub analysis_type: AnalysisType,
    pub options: AnalysisOptions,
}

// Message formatted for analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageForAnalysis {
    pub id: String,
    pub text: String,
    pub timestamp: u64,
    pub platform: String,
    pub sender: String,
    pub conversation_id: String,
}

// Types of analysis that can be requested
#[derive(Debug, Serialize, Deserialize)]
pub enum AnalysisType {
    Sentiment,
    Topics,
    Entities,
    Timeline,
    ConversationFlow,
    ComprehensiveAnalysis,
}

// Options for analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisOptions {
    pub include_explanations: bool,
    pub detail_level: DetailLevel,
    pub max_results: Option<usize>,
}

// Level of detail in the analysis
#[derive(Debug, Serialize, Deserialize)]
pub enum DetailLevel {
    Basic,
    Standard,
    Detailed,
}

// Convert our Message type to MessageForAnalysis
pub fn convert_to_analysis_message(msg: &Message) -> MessageForAnalysis {
    MessageForAnalysis {
        id: msg.id.clone(),
        text: msg.content.text.clone(),
        timestamp: msg.timestamp,
        platform: format!("{:?}", msg.platform),
        sender: msg.sender.name.clone(),
        conversation_id: msg.conversation_id.clone(),
    }
}