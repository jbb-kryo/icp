use crate::{Message, Conversation, QueryResult, Error, Result};
use crate::storage::{messages, conversations};
use crate::query::parser::{QueryRequest, FilterType};
use ic_cdk::api::time;

pub async fn process_query(query_request: QueryRequest) -> Result<QueryResult> {
    // First, filter conversations based on filters
    let relevant_conversations = filter_conversations(&query_request.filters)?;
    
    // Then, search for messages in those conversations
    let mut relevant_messages = Vec::new();
    
    for conversation in &relevant_conversations {
        let conversation_messages = messages::get_conversation_messages(
            &conversation.id, 
            1000, // Temporary high limit for filtering
            None
        );
        
        for message in conversation_messages {
            if message_matches_filters(&message, &query_request.filters) {
                relevant_messages.push(message);
            }
        }
    }
    
    // Sort by recency
    relevant_messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    // Apply limit
    if let Some(limit) = query_request.limit {
        relevant_messages.truncate(limit as usize);
    }
    
    // Generate context information for the response
    let context = generate_context(&query_request, &relevant_conversations, &relevant_messages);
    
    Ok(QueryResult {
        messages: relevant_messages,
        context,
    })
}

fn filter_conversations(filters: &[FilterType]) -> Result<Vec<Conversation>> {
    // Get all user conversations
    let caller = ic_cdk::caller();
    let user_conversations = conversations::get_user_conversations(&caller.to_string(), None);
    
    // Apply filters
    let filtered = user_conversations.into_iter()
        .filter(|conversation| {
            // Check if conversation matches all relevant filters
            for filter in filters {
                match filter {
                    FilterType::Platform(platform) => {
                        if &conversation.platform != platform {
                            return false;
                        }
                    },
                    FilterType::Conversation(id) => {
                        if &conversation.id != id {
                            return false;
                        }
                    },
                    // Other filters don't apply at the conversation level
                    _ => {}
                }
            }
            true
        })
        .collect();
    
    Ok(filtered)
}

fn message_matches_filters(message: &Message, filters: &[FilterType]) -> bool {
    for filter in filters {
        match filter {
            FilterType::Platform(platform) => {
                if &message.platform != platform {
                    return false;
                }
            },
            FilterType::TimeRange { start, end } => {
                if message.timestamp < *start {
                    return false;
                }
                if let Some(end_time) = end {
                    if message.timestamp > *end_time {
                        return false;
                    }
                }
            },
            FilterType::Sender(sender_id) => {
                if &message.sender.id != sender_id {
                    return false;
                }
            },
            FilterType::Keyword(keyword) => {
                if !message.content.text.to_lowercase().contains(&keyword.to_lowercase()) {
                    return false;
                }
            },
            FilterType::Conversation(conv_id) => {
                if &message.conversation_id != conv_id {
                    return false;
                }
            },
        }
    }
    true
}

fn generate_context(
    query: &QueryRequest, 
    conversations: &[Conversation], 
    messages: &[Message]
) -> String {
    // Create a descriptive context for the query results
    let mut context = String::new();
    
    // Query description
    context.push_str(&format!("Query: \"{}\"\n", query.query_text));
    
    // Filter description
    if !query.filters.is_empty() {
        context.push_str("Filters applied:\n");
        for filter in &query.filters {
            match filter {
                FilterType::Platform(platform) => {
                    context.push_str(&format!("- Platform: {:?}\n", platform));
                },
                FilterType::TimeRange { start, end } => {
                    let start_time = format_timestamp(*start);
                    let end_desc = if let Some(end_time) = end {
                        format!("to {}", format_timestamp(*end_time))
                    } else {
                        "to now".to_string()
                    };
                    context.push_str(&format!("- Time range: {} {}\n", start_time, end_desc));
                },
                FilterType::Sender(sender) => {
                    context.push_str(&format!("- From sender: {}\n", sender));
                },
                FilterType::Keyword(keyword) => {
                    context.push_str(&format!("- Containing: \"{}\"\n", keyword));
                },
                FilterType::Conversation(conv_id) => {
                    // Look up conversation name
                    let conv_name = conversations.iter()
                        .find(|c| &c.id == conv_id)
                        .map_or_else(|| conv_id.clone(), |c| c.name.clone());
                    context.push_str(&format!("- In conversation: {}\n", conv_name));
                },
            }
        }
    }
    
    // Result summary
    context.push_str(&format!("\nFound {} messages across {} conversations.\n", 
        messages.len(), 
        conversations.len()
    ));
    
    // Add platforms covered
    let platforms: Vec<String> = conversations.iter()
        .map(|c| format!("{:?}", c.platform))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
        
    if !platforms.is_empty() {
        context.push_str(&format!("Platforms: {}\n", platforms.join(", ")));
    }
    
    context
}

fn format_timestamp(timestamp: u64) -> String {
    // Convert milliseconds timestamp to a human-readable format
    // In a real implementation, we'd use a proper date formatting library
    
    // Simple conversion to seconds for now
    let seconds = timestamp / 1000;
    let now = (time() / 1_000_000) as u64; // Current time in seconds
    
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